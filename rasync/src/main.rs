use rand::Rng;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{mpsc, Arc, LazyLock, Mutex};
use std::collections::HashMap;
use futures::task::{self, ArcWake};

struct Delay {
    when: Instant,
}

#[derive(Clone, Copy, Debug)]
enum ErrorType {
    Io,
    Cpu
}

#[derive(Clone, Copy, Debug)]
struct TaskError {
    et: ErrorType
}

impl TaskError {
    fn new(et: ErrorType) -> Self {
        Self { et }
    }
}

#[derive(Clone, Copy, Debug)]
struct DelayResult {
    result: Option<TaskError>,
    roll: u32,
}

impl Future for Delay {
    type Output = DelayResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            let mut rng = rand::rng();
            let mut et = None;
            let roll = rng.random::<u32>() % 100;
            if roll < 5 {
                et = Some(TaskError::new(ErrorType::Cpu));
            } else if roll < 20 {
                et = Some(TaskError::new(ErrorType::Io));
            }
            let result = et;
            return Poll::Ready(DelayResult{ result, roll });
        }

        let waker = cx.waker().clone();
        let when = self.when;
        thread::spawn(move || {
            let now = Instant::now();
            if now < when {
                thread::sleep(when - now);
            }
            waker.wake();
        });
        Poll::Pending
    }
}

struct TaskFuture {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    poll: Poll<()>,
}

impl TaskFuture {
    fn new(future: impl Future<Output = ()> + Send + 'static) -> Self {
        TaskFuture {
            future: Box::pin(future),
            poll: Poll::Pending
        }
    }
    
    fn poll(&mut self, cx: &mut Context<'_>) {
        if self.poll.is_pending() {
            self.poll = self.future.as_mut().poll(cx);
        }
    }
}

struct Task {
    task_future: Mutex<TaskFuture>,
    executor: mpsc::Sender<Arc<Task>>,
}

impl Task {
    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut task_future = self.task_future.try_lock().unwrap();

        task_future.poll(&mut cx);
    }

    fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task>>)
        where
            F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            task_future: Mutex::new(TaskFuture::new(future)),
            executor: sender.clone(),
        });
        let _ = sender.send(task);
    }

    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

struct MiniTokio {
    scheduled: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}

impl MiniTokio {
    fn new() -> Self {
        let (sender, scheduled) = mpsc::channel();
        Self { scheduled, sender }
    }

    fn spawn<F>(&self, future: F)
        where
            F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }

    // fn stop(&self) {
    // }
}

#[derive(Clone, Copy, Debug)]
struct TaskStats {
    total: u32,
    roll_avg: f32,
    success_rate: f32,
}

fn sum_stats(t1: TaskStats, t2: TaskStats) -> TaskStats {
    let ftotal = t1.total as f32 + t2.total as f32;
    assert!(ftotal > 0.0);
    let ratio1 : f32 = t1.total as f32 / ftotal;
    let ratio2 : f32 = t2.total as f32 / ftotal;

    let roll_avg = (t1.roll_avg * ratio1) + (t2.roll_avg * ratio2);
    let success_rate = (t1.success_rate * ratio1) + (t2.success_rate * ratio2);
    let total = t1.total + t2.total;
    let out = TaskStats { total, roll_avg, success_rate };
    out
}

struct TaskBin {
    tasks : LazyLock<Mutex<HashMap<u32, TaskStats>>>,
}

impl TaskBin {
    const fn new() -> Self {
        TaskBin {
            tasks: LazyLock::new(|| { Mutex::new(HashMap::new()) }),
        }
    }
    fn add(&self, id: u32, result: DelayResult) {
        let _ = id;
        let id = 0;

        let mut tasks = self.tasks.lock().unwrap();
        let success_rate : f32;
        match result.result {
            None => success_rate = 1.0,
            Some(_err) => success_rate = 0.0,
        }
        let roll_avg : f32 = result.roll as f32;
        let mut stats = TaskStats{ total:1, roll_avg, success_rate };
        let prev_stats = tasks.insert(id, stats);
        if let Some(prev) = prev_stats {
            stats = sum_stats(stats, prev);
            tasks.insert(id, stats);
        }
        println!("--> id:{} stats:{:?}", id, stats);
    }
}

static TASK_BIN : TaskBin = TaskBin::new();

fn main() {
    let mt = MiniTokio::new();
    for i in 0..100 {
        let cl = async move {
            let when = Instant::now() + Duration::from_millis(100);
            let future = Delay { when };
            let result = future.await;
            TASK_BIN.add(i, result);
        };
        mt.spawn(cl);
    }
    mt.run();
}

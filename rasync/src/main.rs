use rand::Rng;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;
use futures::task::{self, ArcWake};

struct Delay {
    id: u32,
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
    id: u32,
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
            return Poll::Ready(DelayResult{ id: self.id, result, roll });
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
    // sender: mpsc::Sender<Arc<Task>>,
}

impl MiniTokio {
    fn new(scheduled: mpsc::Receiver<Arc<Task>>) -> Self {
        Self { scheduled }
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

struct TaskBin {
    tasks: HashMap<u32, TaskStats>,
    // thread: std::thread::JoinHandle,
    rcvr: mpsc::Receiver<DelayResult>,
}

impl TaskBin {
    fn new(rcvr: mpsc::Receiver<DelayResult>) -> Self {
        TaskBin {
            tasks: HashMap::new(),
            rcvr,
            // thread
        }
    }

    fn run(&mut self) {
        while let Ok(result) = self.rcvr.recv() {
            self.add(result);
        }
    }

    fn add(&mut self, result: DelayResult) {
        let id = result.id;

        let success_rate : f32;
        match result.result {
            None => success_rate = 1.0,
            Some(_err) => success_rate = 0.0,
        }
        let roll_avg : f32 = result.roll as f32;
        let mut stats = TaskStats{ total:1, roll_avg, success_rate };
        let prev_stats = self.tasks.insert(id, stats);
        if let Some(prev) = prev_stats {
            stats = Self::sum_stats(stats, prev);
            self.tasks.insert(id, stats);
        }
        println!("--> id:{} stats:{:?}", id, stats);
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

}

fn main() {
    let (mt_sender, mt_rcvr) = mpsc::channel();
    let mt_sender = Arc::new(mt_sender);

    let mut threads = vec![];
    threads.push(thread::spawn(|| {
        let mt = MiniTokio::new(mt_rcvr);
        mt.run();
    }));

    let (stat_sender, stat_rcvr) = mpsc::channel();
    threads.push(thread::spawn(|| {
        let mut tb = TaskBin::new(stat_rcvr);
        tb.run();
    }));

    for id in 0..2 {
        let stat_sender = stat_sender.clone();
        let cl = async move {
            let future = Delay { 
                id,
                when: Instant::now() + Duration::from_millis(200),
            };
            let result = future.await;
            let _ = stat_sender.send(result);
        };
        Task::spawn(cl, &mt_sender.clone());
    };

    for t in threads {
        let _ = t.join();
    }

}

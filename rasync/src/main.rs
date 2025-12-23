use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use futures::task::{self, ArcWake};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            return Poll::Ready("done");
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

fn main() {
    let mt = MiniTokio::new();
    mt.spawn(async {
        println!("1: enter");
        let when = Instant::now() + Duration::from_millis(200);
        let future = Delay { when };
        println!("1: await");
        let out = future.await;
        println!("1: await done. {}", out);
    });
    mt.run();
}

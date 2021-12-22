use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake};
use thiserror::Error;

type BoxedError = Box<dyn std::error::Error>;
type BoxedResult = Result<(), BoxedError>; 

#[derive(Debug, Error)]
pub enum Error {
    #[error("Task returned error: {0}")]
    Task(BoxedError)
}

pub type Result<T, E = crate::Error> = core::result::Result<T, E>;

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = BoxedResult> + Send>>>,
    executor: flume::Sender<Arc<Task>>,
}

impl Task {
    fn poll(self: Arc<Self>) -> Result<()> {
        let waker = Arc::new(Waker { task: self.clone() }).into();
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();

        if let Poll::Ready(Err(err)) = future.as_mut().poll(&mut cx) {
            return Err(Error::Task(err));
        }
        Ok(())
    }

    fn spawn<F>(future: F, sender: &flume::Sender<Arc<Self>>)
    where
        F: Future<Output = BoxedResult> + Send + 'static,
    {
        let task = Self {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        };
        sender.send(Arc::new(task)).unwrap();
    }
}

pub struct Aklete {
    scheduled: flume::Receiver<Arc<Task>>,
    sender: Option<flume::Sender<Arc<Task>>>,
}

struct Waker {
    task: Arc<Task>,
}

impl Wake for Waker {
    fn wake(self: Arc<Self>) {
        let _ = self.task.executor.send(self.task.clone());
    }
}

impl Aklete {
    pub fn new() -> Self {
        let (sender, scheduled) = flume::unbounded();
        Self {
            scheduled,
            sender: Some(sender),
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = BoxedResult> + Send + 'static,
    {
        Task::spawn(future, self.sender.as_ref().unwrap());
    }

    pub fn run(&mut self) -> Result<()> {
        self.sender = None;
        while let Ok(task) = self.scheduled.recv() {
            task.poll()?;
        }
        let (sender, scheduled) = flume::unbounded();
        self.scheduled = scheduled;
        self.sender = Some(sender);
        Ok(())
    }
}

impl Default for Aklete {
    fn default() -> Self {
        Self::new()
    }
}

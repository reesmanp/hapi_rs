mod job;
mod worker;
mod message;

use std::error::Error;
use std::{fmt, thread};
use std::sync::{mpsc, Arc, Mutex};
use self::{
  job::Job,
  message::Message,
  worker::Worker
};

#[derive(Debug)]
pub struct ThreadPoolError {
  pub error: String
}

impl Error for ThreadPoolError {
  fn description(&self) -> &str {
    self.error.as_str()
  }
}

impl fmt::Display for ThreadPoolError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.error)
  }
}

pub struct ThreadPool {
  pub workers: Vec<Worker>,
  pub sender: mpsc::Sender<Message>
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    println!("Shutting down all workers.");

    for worker in &mut self.workers {
      println!("Shutting down worker {}.", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

impl ThreadPool {
  pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolError> {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    Ok(
      ThreadPool {
        workers,
        sender
      }
    )
  }

  pub fn execute<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

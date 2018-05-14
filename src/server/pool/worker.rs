use std::thread;
use server::pool::job::Job;
use std::sync::{mpsc, Arc, Mutex};
use super::message::Message;

pub struct Worker {
  pub id: usize,
  pub thread: Option<thread::JoinHandle<()>>
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let unlocked = match receiver.lock() {
          Ok(t) => t,
          Err(e) => continue
        };

        let message = match unlocked.recv() {
          Err(error) => {
            println!("Worker error: {:?}", error);
            continue
          },
          Ok(message) => message
        };

        match message {
          Message::NewJob(job) => {
            println!("Worker {} got a job; executing...", id);
            job.call_box();
          },
          Message::Terminate => {
            println!("Worker {} was told to terminate...", id);
            break;
          }
        }
      }
    });

    Worker {
      id,
      thread: Some(thread)
    }
  }
}

use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use super::message::Message;

pub struct Worker {
    id: u8,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    pub fn new(id: u8, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        Self {
            id,
            thread: Some(thread::spawn(move || {
                loop {
                    let unlocked = match receiver.lock() {
                        Ok(t) => t,
                        Err(_) => break
                    };

                    let message = match unlocked.recv() {
                        Ok(message) => message,
                        Err(error) => {
                            println!("Worker error: {:?}", error);
                            break;
                        }
                    };

                    match message {
                        Message::NewContinuousJob(job) => {
                            println!("Worker {} got a new continuous job!", id);
                            drop(unlocked);
                            job.call_box();
                        },
                        Message::NewJob(job) => {
                            println!("Worker {} got a new job!", id);
                            job.call_box();
                        },
                        Message::HandlerJob(handler, req, mut res) => {
                            println!("Worker {} got a new handler job!", id);
                            handler(&req, &mut res);
                        }
                        Message::Terminate => {
                            println!("Worker {} is shutting down", id);
                            break;
                        }
                    }
                }
            }))
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        println!("Dropping worker {}", self.id);
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap_or_default();
        }
    }
}

/*impl Clone for Worker {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            thread: match self.thread {
                None => None,
                Some(ref T) => Some(*T)
            }
        }
    }
}*/

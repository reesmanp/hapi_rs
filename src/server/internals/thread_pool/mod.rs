pub mod job;
mod message;
mod worker;

use std::vec::Vec;
use std::sync::{Arc, mpsc, Mutex};
use self::job::{FnBox};
use self::worker::Worker;
use self::message::Message;
use super::route::RouteHandler;
use super::super::super::http::{
    request::Request,
    response::Response
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::SyncSender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        assert!(size as u8 <= u8::max_value());

        let (sender, receiver) = mpsc::sync_channel(2);
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id as u8, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender
        }
    }

    pub fn execute_handler(&self, handler: Arc<RouteHandler>, req: Request, res: Response) {
        match self.sender.send(Message::HandlerJob(handler, req, res)) {
            Ok(_) => return,
            Err(t) => self.log_error(t)
        }
    }

    pub fn execute_job<F>(&self, f: F)
        where F: FnBox + FnOnce() + Send + Clone + 'static
    {
        let job = Box::new(f);
        for _worker in &self.workers
            {
            let cloned_job = job.clone();
            match self.sender.send(Message::NewContinuousJob(cloned_job)) {
                Ok(_) => return,
                Err(t) => self.log_error(t)
            }
        }
    }

    fn log_error(&self, err: mpsc::SendError<Message>) {
        eprintln!("{}", err);
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers!");
        println!("{}", self.workers.len());

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers!");

        for worker in self.workers.iter() {
            drop(worker);
        }
    }
}

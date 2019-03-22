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
        self.sender.send(Message::HandlerJob(handler, req, res)).unwrap();
    }

    pub fn execute_job<F>(&self, f: F)
        where F: FnBox + FnOnce() + Send + Clone + 'static
    {
        let job = Box::new(f);
        for _worker in &self.workers
            {
            let cloned_job = job.clone();
            self.sender.send(Message::NewContinuousJob(cloned_job)).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers!");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers!");

        for worker in &mut self.workers {
            println!("Shutting down worker {}.", worker.get_id());

            drop(worker);
        }
    }
}

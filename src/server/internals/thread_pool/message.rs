use super::job::Job;
use super::RouteHandler;
use super::{Request, Response};

pub enum Message {
    NewContinuousJob(Job),
    NewJob(Job),
    HandlerJob(RouteHandler, Request, Response),
    Terminate
}

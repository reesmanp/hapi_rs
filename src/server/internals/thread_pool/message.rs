use super::job::Job;
use super::RouteHandler;
use super::{Request, Response};
use std::sync::Arc;

pub enum Message {
    NewContinuousJob(Job),
    NewJob(Job),
    HandlerJob(Arc<RouteHandler>, Request, Response),
    Terminate
}

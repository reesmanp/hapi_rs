use super::job::Job;
use super::RouteHandler;
use super::{Request, Response};
use std::sync::Arc;

#[allow(dead_code)]
pub enum Message {
    NewContinuousJob(Job),
    NewJob(Job),
    HandlerJob(Arc<RouteHandler>, Request, Response),
    Terminate
}

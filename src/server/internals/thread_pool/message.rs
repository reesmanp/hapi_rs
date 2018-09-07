use super::job::Job;

pub enum Message {
    NewContinuousJob(Job),
    NewJob(Job),
    Terminate
}
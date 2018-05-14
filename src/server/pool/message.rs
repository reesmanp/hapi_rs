use super::job::Job;

pub enum Message {
    NewJob(Job),
    Terminate
}

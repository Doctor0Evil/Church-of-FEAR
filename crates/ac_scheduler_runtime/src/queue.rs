use crate::job::Job;
use std::collections::VecDeque;

#[derive(Default)]
pub struct JobQueue {
    items: VecDeque<Job>,
}

impl JobQueue {
    pub fn push(&mut self, job: Job) {
        self.items.push_back(job);
    }

    pub fn pop(&mut self) -> Option<Job> {
        self.items.pop_front()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

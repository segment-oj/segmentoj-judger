use std::collections::VecDeque;
use crate::task::*;

pub struct Queue {
    task_que: VecDeque<content::Task>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            task_que: VecDeque::new(),
        }
    }

    pub fn add(&mut self, t: content::Task) {
        self.task_que.push_back(t);
    }

    pub fn is_idle(&self) -> bool {
        self.task_que.len() == 0
    }

    pub fn get(&mut self) -> Option<content::Task> {
        self.task_que.pop_front()
    }
}

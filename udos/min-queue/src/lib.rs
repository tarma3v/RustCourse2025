#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    data: VecDeque<T>,
    min_deque: VecDeque<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            min_deque: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push_back(val.clone());
        while let Some(back) = self.min_deque.back() {
            if *back > val {
                self.min_deque.pop_back();
            } else {
                break;
            }
        }
        self.min_deque.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(val) = self.data.pop_front() {
            if let Some(front) = self.min_deque.front() {
                if *front == val {
                    self.min_deque.pop_front();
                }
            }
            Some(val)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.min_deque.front()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

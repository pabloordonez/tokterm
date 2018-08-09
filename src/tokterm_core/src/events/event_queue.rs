use events::event::Event;
use std::collections::vec_deque::VecDeque;

#[allow(dead_code)]
pub struct EventQueue {
    queue: VecDeque<Event>,
    events: usize,
}

#[allow(dead_code)]
impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            queue: VecDeque::new(),
            events: 0,
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.queue.push_back(event);
    }

    pub fn get_event(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
}

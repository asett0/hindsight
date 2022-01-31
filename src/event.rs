use crate::events::fill::FillEvent;
use crate::events::market::MarketEvent;
use crate::events::order::OrderEvent;
use std::collections::VecDeque;

pub enum Event {
    Market(MarketEvent),
    Order(OrderEvent),
    Fill(FillEvent),
    Terminate,
}

pub trait EventGenerator {
    fn react_to_event(&mut self, _: &Event) -> Option<Event> {
        None
    }

    fn heartbeat(&mut self) -> Option<Event> {
        None
    }
}

pub struct EventQueue(VecDeque<Event>);

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue(VecDeque::new())
    }

    pub fn pop_front(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add_event(&mut self, option_event: Option<Event>) {
        if let Some(event) = option_event {
            self.0.push_back(event);
        }
    }
}

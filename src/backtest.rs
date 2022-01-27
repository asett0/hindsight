use crate::event::{Event, EventGenerator, EventQueue};

pub fn backtest<T: Send>(event_generators: &mut Vec<Box<dyn EventGenerator>>) {
    let mut event_queue = EventQueue::new();

    'heartbeat: loop {
        for event_generator in event_generators.iter_mut() {
            event_queue.add_event(event_generator.heartbeat());
        }

        while !event_queue.is_empty() {
            let event = event_queue.pop_front().unwrap();

            if let Event::Terminate = event {
                break 'heartbeat;
            }

            for event_generator in event_generators.iter_mut() {
                event_queue.add_event(event_generator.react_to_event(&event));
            }
        }
    }
}

#[cfg(test)]
mod test {}

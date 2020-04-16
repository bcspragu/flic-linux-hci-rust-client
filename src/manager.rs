use crate::events;
use crate::Result;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::client::Client;

type Handler = Box<dyn Fn(&events::Event) + Send + 'static>;

pub struct Manager {
    pub client: Client,
    handlers: Mutex<HashMap<events::Opcode, Vec<Handler>>>,
}

impl Manager {
    pub fn new(host: &str) -> Result<Manager> {
        let client = Client::new(host)?;
        Ok(Manager {
            client,
            handlers: Mutex::new(HashMap::new()),
        })
    }

    pub fn register_handler<F>(&self, opcode: events::Opcode, f: F)
    where
        F: Fn(&events::Event) + Send + 'static,
    {
        let mut handlers = self.handlers.lock().unwrap();
        let v = handlers.entry(opcode).or_insert(vec![]);
        v.push(Box::new(f));
    }

    pub fn start(&self) -> Result<()> {
        loop {
            let (evt, opcode) = self.client.next_event()?;

            let handlers = self.handlers.lock().unwrap();
            let handlers = match handlers.get(&opcode) {
                Some(handlers) => handlers,
                None => continue,
            };

            for event_fn in handlers {
                event_fn(&evt);
            }
        }
    }
}

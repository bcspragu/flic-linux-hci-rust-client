use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;

use crate::commands;
use crate::events;
use crate::Result;

type Handler = Box<dyn Fn(&events::Event) + Send + 'static>;

pub struct Client {
    writer: Mutex<TcpStream>,
    reader: Mutex<TcpStream>,
    handlers: Mutex<HashMap<events::Opcode, Vec<Handler>>>,
}

impl Client {
    pub fn new(host: &str) -> Result<Client> {
        let reader = TcpStream::connect(host)?;
        let writer = reader.try_clone()?;
        Ok(Client {
            writer: Mutex::new(writer),
            reader: Mutex::new(reader),
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

    pub fn listen(&self) -> Result<()> {
        loop {
            let (evt, opcode) = self.next_event()?;

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

    pub fn next_event(&self) -> Result<(events::Event, events::Opcode)> {
        let mut stream = self.reader.lock().unwrap();

        let mut header = [0 as u8; 2];
        stream
            .read_exact(&mut header)
            .expect("failed to read header");

        let len = u16::from_le_bytes([header[0], header[1]]);

        let mut body = vec![0u8; len as usize];
        stream.read_exact(&mut body).expect("failed to read body");

        events::unmarshal(&body)
    }

    pub fn send_command<C>(&self, cmd: C) -> Result<()>
    where
        C: commands::Command,
    {
        let mut stream = self.writer.lock().unwrap();

        let opcode = cmd.opcode();
        let mut body = cmd.marshal();
        // Prepend opcode
        body.insert(0, opcode);

        // Get the length, and prepend that as the length header, little endian.
        let len = body.len().to_le_bytes();
        body.insert(0, len[1]);
        body.insert(0, len[0]);

        stream.write(body.as_slice())?;
        stream.flush()?;

        Ok(())
    }
}

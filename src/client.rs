use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;

use crate::commands;
use crate::events;
use crate::Result;

use crate::error::FlicError;

pub struct Client {
    stream: Mutex<Option<TcpStream>>,
    handlers: Mutex<HashMap<events::Opcode, Vec<fn(&events::Event)>>>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            stream: Mutex::new(None),
            handlers: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_handler(
        &self,
        opcode: events::Opcode,
        event_fn: fn(&events::Event),
    ) {
        let mut handlers = self.handlers.lock().unwrap();
        let v = handlers.entry(opcode).or_insert(vec![]);
        v.push(event_fn);
    }

    pub fn listen(&self, host: &str) -> Result<()> {
        {
            let mut stream = self.stream.lock().unwrap();
            match *stream {
                Some(_) => return Err(FlicError::Generic(String::from("stream already open"))),
                None => (), // This is expected.
            }

            *stream = Some(TcpStream::connect(host)?);

        }

        loop {
            let (evt, opcode) = self.next_event()?;
            println!("Event: {:?}, Opcode: {:?}", evt, opcode);

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
        let stream = self.stream.lock().unwrap();
        let mut stream = match stream.as_ref() {
            Some(stream) => stream,
            None => return Err(FlicError::Generic(String::from("no stream open"))),
        };

        let mut header = [0 as u8; 2];
        stream
            .read_exact(&mut header)
            .expect("failed to read header");
        let len = u16::from_le_bytes([header[0], header[1]]);

        let mut body = vec![0u8; len as usize];
        stream.read_exact(&mut body).expect("failed to read body");

        events::unmarshal(&body)
    }

    pub fn send_command(&self, cmd: Box<dyn commands::Command>) -> Result<()> {
        let stream = self.stream.lock().unwrap();

        let mut stream = match stream.as_ref() {
            Some(stream) => stream,
            None => return Err(FlicError::Generic(String::from("no stream open"))),
        };

        let opcode = cmd.opcode();
        let mut body = cmd.marshal();
        // Prepend opcode
        body.insert(0, opcode);

        // Get the length, and prepend that as the length header, little endian.
        let len = body.len().to_le_bytes();
        body.insert(0, len[1]);
        body.insert(0, len[0]);

        stream.write(body.as_slice())?;
        Ok(())
    }
}

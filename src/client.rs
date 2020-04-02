use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::commands;
use crate::events;
use crate::Result;

use crate::error::FlicError;

pub struct Client {
    stream: Option<TcpStream>,
    handlers: HashMap<events::Opcode, Vec<Box<dyn Fn(&events::Event)>>>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            stream: None,
            handlers: HashMap::new(),
        }
    }

    pub fn register_handler(
        &mut self,
        opcode: events::Opcode,
        event_fn: Box<dyn Fn(&events::Event)>,
    ) {
        let v = self.handlers.entry(opcode).or_insert(vec![]);
        v.push(event_fn);
    }

    pub fn listen(&mut self, host: &str) -> Result<()> {
        match self.stream {
            Some(_) => return Err(FlicError::Generic(String::from("stream already open"))),
            None => (), // This is expected.
        }

        self.stream = Some(TcpStream::connect(host)?);

        loop {
            let (evt, opcode) = self.next_event()?;
            println!("Event: {:?}, Opcode: {:?}", evt, opcode);

            let handlers = match self.handlers.get(&opcode) {
                Some(handlers) => handlers,
                None => continue,
            };

            for event_fn in handlers {
                event_fn(&evt);
            }
        }
    }

    pub fn send_command(&mut self, cmd: Box<dyn commands::Command>) -> Result<()> {
        let mut stream = match self.stream.as_ref() {
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

    pub fn next_event(&mut self) -> Result<(events::Event, events::Opcode)> {
        if let None = self.stream {
            return Err(FlicError::Generic(String::from("no stream open")));
        }
        let mut stream = self.stream.as_ref().unwrap();

        let mut header = [0 as u8; 3];
        stream
            .read_exact(&mut header)
            .expect("failed to read header");
        let len = u16::from_le_bytes([header[0], header[1]]);
        let opcode = header[2];

        // Minus one for the opcode
        let mut body = vec![0u8; (len - 1) as usize];
        stream.read_exact(&mut body).expect("failed to read body");

        events::unmarshal(opcode, &body)
    }
}

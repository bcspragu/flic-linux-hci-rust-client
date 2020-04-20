use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;
use std::time::Duration;

use crate::commands;
use crate::error::FlicError;
use crate::events;
use crate::Result;

pub struct Client {
    writer: Mutex<TcpStream>,
    reader: Mutex<TcpStream>,
}

impl Client {
    pub fn new(host: &str) -> Result<Client> {
        let reader = TcpStream::connect(host)?;
        let writer = reader.try_clone()?;
        Ok(Client {
            writer: Mutex::new(writer),
            reader: Mutex::new(reader),
        })
    }

    pub fn next_event(&self) -> Result<(events::Event, events::Opcode)> {
        Ok(self
            .next_event_with_timeout(None)?
            .expect("call is blocking, event must be returned"))
    }

    pub fn next_event_with_timeout(
        &self,
        timeout: Option<Duration>,
    ) -> Result<Option<(events::Event, events::Opcode)>> {
        let mut stream = self.reader.lock().unwrap();
        stream.set_read_timeout(timeout)?;

        let mut header = [0 as u8; 2];
        match stream.read_exact(&mut header) {
            Ok(_) => {}
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock | ErrorKind::TimedOut => return Ok(None),
                _ => return Err(FlicError::FlicD(err)),
            },
        }

        let len = u16::from_le_bytes([header[0], header[1]]);

        let mut body = vec![0u8; len as usize];
        match stream.read_exact(&mut body) {
            Ok(_) => {}
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock | ErrorKind::TimedOut => return Ok(None),
                _ => return Err(FlicError::FlicD(err)),
            },
        }

        Ok(Some(events::unmarshal(&body)?))
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

impl Iterator for Client {
    type Item = Result<(events::Event, events::Opcode)>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_event())
    }
}

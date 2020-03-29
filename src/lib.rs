#[macro_use]
extern crate num_derive;

use std::io::{Read, Write};
use std::net::TcpStream;

pub mod commands;
pub mod enums;
pub mod events;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(host: &str) -> std::io::Result<Client> {
        let stream = TcpStream::connect(host)?;
        Ok(Client { stream })
    }

    pub fn send_command(&mut self, cmd: Box<dyn commands::Command>) -> std::io::Result<()> {
        let opcode = cmd.opcode();
        let mut body = cmd.marshal();
        // Prepend opcode
        body.insert(0, opcode);

        // Get the length, and prepend that as the length header, little endian.
        let len = body.len().to_le_bytes();
        body.insert(0, len[1]);
        body.insert(0, len[0]);

        self.stream
            .write(body.as_slice())
            .expect("failed to write command");
        Ok(())
    }

    pub fn next_event(&mut self) -> events::Result<events::Event> {
        let mut header = [0 as u8; 3];
        self.stream
            .read_exact(&mut header)
            .expect("failed to read header");
        let len = u16::from_le_bytes([header[0], header[1]]);
        let opcode = header[2];

        // Minus one for the opcode
        let mut body = vec![0u8; (len - 1) as usize];
        self.stream
            .read_exact(&mut body)
            .expect("failed to read body");

        let unmarshal_event = match opcode {
            0 => events::unmarshal_advertisement_packet,
            1 => events::unmarshal_create_connection_channel_response,
            2 => events::unmarshal_connection_status_changed,
            3 => events::unmarshal_connection_channel_removed,
            4..=7 => events::unmarshal_button_event,
            8 => events::unmarshal_new_verified_button,
            9 => events::unmarshal_get_info_response,
            10 => events::unmarshal_no_space_for_new_connection,
            11 => events::unmarshal_got_space_for_new_connection,
            12 => events::unmarshal_bluetooth_controller_state_change,
            13 => events::unmarshal_ping_response,
            14 => events::unmarshal_get_button_info_response,
            15 => events::unmarshal_scan_wizard_found_private_button,
            16 => events::unmarshal_scan_wizard_found_public_button,
            17 => events::unmarshal_scan_wizard_button_connected,
            18 => events::unmarshal_scan_wizard_completed,
            19 => events::unmarshal_button_deleted,
            20 => events::unmarshal_battery_status,
            _ => {
                return Err(events::UnmarshalError::BadOpcode(opcode));
            }
        };

        unmarshal_event(&body)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

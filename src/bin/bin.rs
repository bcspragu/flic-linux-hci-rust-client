use flic::commands::{GetInfo};
use flic::events::{self,Opcode};
use flic::Result;
use std::thread;
use std::sync::Arc;

fn main() -> Result<()> {
    let client = Arc::new(flic::Client::new());

    client.register_handler(
        Opcode::GetInfoResponse,
        handle_event,
    );

    let c = Arc::clone(&client);
    thread::spawn(move || {
        c.listen("localhost:5551").unwrap();
    });

    client.send_command(Box::new(GetInfo{}))?;

    Ok(())
}

fn handle_event(evt: &events::Event) {
        println!("Event: {:?}", evt);
}

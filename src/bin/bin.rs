use flic::commands::GetInfo;
use flic::events::Opcode;
use flic::Result;
use std::sync::Arc;
use std::{thread, time};

fn main() -> Result<()> {
    let manager = Arc::new(flic::Manager::new("localhost:5551")?);

    manager.register_handler(Opcode::GetInfoResponse, |evt| {
        println!("Event: {:?}", evt);
    });

    let m = Arc::clone(&manager);
    thread::spawn(move || {
        m.start().unwrap();
    });

    thread::sleep(time::Duration::from_millis(1000));
    manager.client.send_command(GetInfo {})?;
    thread::sleep(time::Duration::from_millis(1000));

    Ok(())
}

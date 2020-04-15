use flic::commands::GetInfo;
use flic::events::Opcode;
use flic::Result;
use std::sync::Arc;
use std::{thread, time};

fn main() -> Result<()> {
    let client = Arc::new(flic::Client::new("localhost:5551")?);

    client.register_handler(Opcode::GetInfoResponse, |evt| {
        println!("Event: {:?}", evt);
    });

    let c = Arc::clone(&client);
    thread::spawn(move || {
        c.listen().unwrap();
    });

    thread::sleep(time::Duration::from_millis(1000));
    client.send_command(GetInfo {})?;
    thread::sleep(time::Duration::from_millis(1000));

    Ok(())
}

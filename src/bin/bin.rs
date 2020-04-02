use flic::events::Opcode;
use flic::Result;

fn main() -> Result<()> {
    let mut client = flic::Client::new();

    client.register_handler(
        Opcode::GetInfoResponse,
        Box::new(|evt| {
            println!("Event: {:?}", evt);
        }),
    );

    // TODO: Figure out how to safely listen while also allowing sending commands.
    client.listen("localhost:5551")?;

    //client.send_command(Box::new(GetInfo{}))?;

    Ok(())
}

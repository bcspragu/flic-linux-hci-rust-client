use flic::commands::Ping;

fn main() -> Result<(), std::io::Error> {
    let mut client = flic::Client::new("localhost:5551")?;

    let msg = Ping { ping_id: 1234567 };

    client.send_command(Box::new(msg))?;

    Ok(())
}

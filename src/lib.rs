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
        Ok(Client { stream: stream })
    }

    pub fn send_command(&self, cmd: Box<dyn commands::Command>) -> std::io::Result<()> {
        println!("{:?}", cmd);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

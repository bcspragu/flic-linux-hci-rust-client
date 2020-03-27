use std::net::TcpStream;

mod cmd;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(host: String) -> std::io::Result<Client> {
        let stream = TcpStream::connect(host)?;
        Ok(Client { stream: stream })
    }

    pub fn close(self) -> std::io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }

    pub fn send_command(command: &cmd::Command) -> std::io::Result<()> {
        print!("{:?}", command);
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

pub mod parser;
pub mod store;

use mio::net::TcpStream;

pub struct TcpServer {}

pub fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    Ok(())
}

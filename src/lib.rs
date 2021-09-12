//! This library is designed to simplify the communication between a server agent, and a 
//! central api server.

mod error;
mod websocket_message;

pub use error::WebsocketMessageError;
pub use websocket_message::{WebsocketMessage, File, FileRequest};

//TODO tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

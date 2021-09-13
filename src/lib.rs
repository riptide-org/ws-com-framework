//! This library is designed to simplify the communication between a server agent, and a
//! central api server.

mod server;
mod client;

mod error;
mod websocket_message;

pub use error::WebsocketMessageError;
pub use websocket_message::{File, FileRequest, FileUploadRequest, WebsocketMessage};

// fn test() {
//     //lets think about how we might want this api to work

//     // enum MessageTypes {
//     //     Valid() //Contains things
//     //     Close() //Closing Message
//     //     Error() //Unknown
//     // }

//     // std::result::Result<warp::ws::Message, warp::Error>
    
    
//     use ws-com-framework::server::*; //import the server if we are server, client if we are client

//     let s = Sender::new(); //Create a new sender over the sending stream of the websocket.

//     //Same syntax, except message is now of our custom type, in this way we can limit what can be
//     //sent down the websockets - which should help to reduce errors.
//     s.send(message).await.unwrap();

//     //Close a websocket
//     s.close().await.unwrap();

//     let r = Receiver::new(rx); //Create a new reciever, which wraps over the sink of the websocket.

//     while let Some(v) = r.next().await {
//         //Very similar syntax to current solution
//         //except that v is a custom type which we can then
//         //easily match over
//     }

// }

//TODO tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

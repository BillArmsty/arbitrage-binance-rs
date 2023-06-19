use crate::{ Client, Clients };
use futures::{ FutureExt, StreamExt };
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{ Message, WebSocket };
use uuid::Uuid;

pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("Establishing client connection...{:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);


    //Spawn a new task that keeps the client_ws_sender stream open until the client has disconnected.
    tokio::task::spawn(
        client_rcv.forward(client_ws_sender).map(|result| {
            if let Err(e) = result {
                println!("error sending websocket msg: {}", e);
            }
        })
    );

    let uuid = Uuid::new_v4().to_simple().to_string();

    let new_client = Client {
        client_id: uuid.clone(),
        sender: client_sender,
    };

    clients.lock().await.insert(uuid.clone(), new_client);

    //While loop handles receiving messages from the client:
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving ws message: {}", e);
                break;
            }
        };


        //If message is received, call client_msg function to handle the message
        client_msg(&uuid, msg, &clients).await;
    }

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(client_id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.str() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    //Convert msg to a string, and if successful, responds to it if the message is “ping”
    if message == "ping" || message == "ping\n" {
        let locked = clients.lock().await;
        match locked.get(client_id) {
            Some(v) => {
                if let Some(sender) = &v.sender {
                    let _ = sender.send(Ok(Message::text("pong"))).await;
                }
            }
            None => {
                return;
            }
        }
        return;
    }
}

use crate::{models, Clients};
use chrono::{DateTime, Utc};
use rand::prelude::*;
use serde::Serialize;
use tokio;
use tokio::time::Duration;
use warp::ws::Message;
use tungstenite::{client::AutoStream, Websocket};

#[derive(Serialize)]
struct TestData {
    widget_type: String,
    widget_count: u32,
    creation_date: DateTime<Utc>,
}

pub async fn main_worker(clients: Clients) {
    //Loop checking to see if clients are connected
    loop {
        tokio::time::sleep(Duration::from_millis(2000)).await;

        let connected_client_count = clients.lock().await.len();
        if connected_client_count == 0 {
            println!("No clients connected, skipping broadcast");
            continue;
        } 

        println!("Broadcasting to {} clients", connected_client_count);
        
        //Retrieve data from as source then send to clients
        // let test_data_batch = generate_random_data();
        clients.lock().await.iter().for_each(|(_, client)| {
            //Send data serialized to a string
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(
                    serde_json::to_string(&test_data_batch).unwrap(),
                )));
            }
        });

    }
}


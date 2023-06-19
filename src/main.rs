use std::{ collections::Hashmap, convert::Infallible, sync::Arc };
use tokio::sync::{ mpsc, RwLock };
use warp::{ ws::Message, Filter, Rejection };
use tungstenite::connect;
use url::Url;

mod handlers;
mod workers;
mod ws;
mod models;

#[derive(Debug, Clone)]
//Client represents infor. about connected client
pub struct Client {
    //client_id is unique id for each client randomly generated
    pub client_id: String,
    //sender is used to send messages to client
    pub sender: mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>,
}

//Type clients is a thread-safe hash map of client_id to client info
type Clients = Arc<Mutex<Hashmap<String, Client>>>;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

async fn main() {
    let binance_url =
        format!("{}/stream?streams=ethbtc@depth5@100ms/bnbeth@depth5@100ms", BINANCE_WS_API);

    //Use Connect from tungstenite  to connect to the Binance WebSocket URL
    let (mut socket, response) = connect(Url::parse(&binance_url).await.unwrap()).expect("Can't connect");

    println!("Connected to Binance Websocket Stream...");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    println!("Starting update loop...");
    tokio::task::spawn(async move {
        workers::main_worker(clients.clone(), socket).await;
    })

    loop {
        let msg = socket.read_message().await.expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text message");
            }
        };

        let parsed: models::DepthStreamWrapper = serde_json
            ::from_str(&msg).await
            .expect("Error parsing message");
        for i in 0..parsed.data.asks.len() {
            println!(
                "{}: {}. ask: {}, size: {}",
                parsed.stream,
                i,
                parsed.data.asks[i].price,
                parsed.data.asks[i].size
            );
        }
    }

    let clients: Clients = Arc::new(Mutex::new(Hashmap::new()));

    println!("Configuring websocket route...");
    let ws_route = warp
        ::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(ws::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());

    // println!("Starting update loop...");
    // tokio::task::spawn(async move {
    //     workers::update_loop(clients).await;
    // });

    println!("Starting server...");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

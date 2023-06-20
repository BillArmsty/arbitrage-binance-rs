use crate::{ws, Clients, Result};
use log::debug;
use warp::Reply;

//WsHandler is used in the WebSocket route to process the connection request:
pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    debug!("ws_handler");

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}
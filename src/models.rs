use serde::de;
use serde::{Deserialize, Serialize};
//This module contains the struct definitions for the data we intend to receive from the Binance WebSocket streams

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferData {
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub size: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub last_update_id: u32,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
}

pub fn de_float_from_str<'de, D>(de: D) -> Result<f32, D::Error>
where
    D: de::Deserializer<'de>,
{
    let str_val: &str = de::Deserialize::deserialize(de)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}

#[derive(Deserialize, Debug)]
pub struct DepthStreamWrapper {
    pub stream: String,
    pub data: DepthStreamData,
}
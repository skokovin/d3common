use serde::{Deserialize, Serialize};
#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct Block {
    pub oid: i64,
    pub code: String,
    pub description: Option<String>,
    pub min_x: Option<f64>,
    pub min_y: Option<f64>,
    pub min_z: Option<f64>,
    pub max_x: Option<f64>,
    pub max_y: Option<f64>,
    pub max_z: Option<f64>,
}

pub fn encoded_one (item:&Block)-> Vec<u8>{
    postcard::to_allocvec(item).unwrap()
}

pub fn decodeded_one (bin:&Vec<u8>)-> Block{
    postcard::from_bytes(bin).unwrap()
}

pub fn encoded_all(items: &[Block]) -> Vec<u8> {
    postcard::to_allocvec(items).unwrap()
}

pub fn decoded_all(bin: &[u8]) -> Vec<Block> {
    postcard::from_bytes(bin).unwrap()
}
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializableMesh {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub extrude_dir: [f32; 3],
}

pub fn unpack_mesh_map(data: &[u8])-> Result<HashMap<i64, (Option<SerializableMesh>, Option<SerializableMesh>)>, postcard::Error>{
    let result: Result<HashMap<i64, (Option<SerializableMesh>, Option<SerializableMesh>)>, postcard::Error> = postcard::from_bytes(data);
    result
}
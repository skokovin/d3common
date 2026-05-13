
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct InpSinglePartDb {
    pub oid: i64,                      // NOT NULL
    pub name: String,                  // NOT NULL
    pub description: Option<String>,
    pub prd_part_oid: i64,             // NOT NULL
    pub part_type: Option<i32>,
    pub str_group_oid: Option<i64>,
    pub block_oid: Option<i64>,
    pub surface_oid: Option<i64>,
    pub surface_name: Option<String>,
    pub facet: Option<i32>,
    pub category: Option<i32>,

    // Координаты и габариты
    pub x_min: Option<f64>,
    pub y_min: Option<f64>,
    pub z_min: Option<f64>,
    pub x_max: Option<f64>,
    pub y_max: Option<f64>,
    pub z_max: Option<f64>,
    pub x_cog: Option<f64>,
    pub y_cog: Option<f64>,
    pub z_cog: Option<f64>,

    // Матрица трансформации
    pub a11: Option<f64>, pub a21: Option<f64>, pub a31: Option<f64>,
    pub a12: Option<f64>, pub a22: Option<f64>, pub a32: Option<f64>,
    pub a13: Option<f64>, pub a23: Option<f64>, pub a33: Option<f64>,
    pub a14: Option<f64>, pub a24: Option<f64>, pub a34: Option<f64>,

    // Физические свойства
    pub weight: Option<f64>,
    pub area: Option<f64>,
    pub thick: Option<f64>,
    pub thick_orient: Option<f64>,
    pub x_ori: Option<f64>,
    pub y_ori: Option<f64>,
    pub z_ori: Option<f64>,
    pub flange_x_ori: Option<f64>,
    pub flange_y_ori: Option<f64>,
    pub flange_z_ori: Option<f64>,
    pub web_thick_orient: Option<i32>, // default 0
    pub length: Option<f64>,

    // Ссылки и профили
    pub kse_oid: Option<i64>,
    pub notch_oid: Option<i64>,
    pub landing: Option<String>,
    pub use_marks_prof_begin: Option<i32>,
    pub prof_dist_begin: Option<f64>,
    pub prof_geo_std_begin_oid: Option<i64>,
    pub use_marks_prof_end: Option<i32>,
    pub prof_dist_end: Option<f64>,
    pub prof_geo_std_end_oid: Option<i64>,
    pub master_oid: Option<i64>,
    pub surf_dist: Option<f64>,        // default 0
    pub zone_oid: Option<i64>,

    // Ассоциации
    pub associated_oid: Option<i64>,   // default 0
    pub associated_name: Option<String>,
    pub associated_oid2: Option<i64>,  // default 0
    pub associated_name2: Option<String>,

    // Матрица OCC
    pub occ_a11: Option<f64>, pub occ_a21: Option<f64>, pub occ_a31: Option<f64>,
    pub occ_a12: Option<f64>, pub occ_a22: Option<f64>, pub occ_a32: Option<f64>,
    pub occ_a13: Option<f64>, pub occ_a23: Option<f64>, pub occ_a33: Option<f64>,
    pub occ_a14: Option<f64>, pub occ_a24: Option<f64>, pub occ_a34: Option<f64>,
}

pub fn encoded_part (my_part:&InpSinglePartDb)-> Vec<u8>{
    postcard::to_allocvec(my_part).unwrap()
}

pub fn decodeded_part (bin:&Vec<u8>)-> InpSinglePartDb{
    postcard::from_bytes(bin).unwrap()
}

pub fn encoded_parts(parts: &[InpSinglePartDb]) -> Vec<u8> {
    postcard::to_allocvec(parts).unwrap()
}

pub fn decoded_parts(bin: &[u8]) -> Vec<InpSinglePartDb> {
    postcard::from_bytes(bin).unwrap()
}
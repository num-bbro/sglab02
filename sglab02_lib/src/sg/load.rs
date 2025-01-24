//use super::dcl::{BaseData, Config};
//use crate::sg::dcl;
//use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
const SUN_LIGHT: [f64; 12] = [
    -6.54_f64, 6.66, 12.60, 17.31, 8.42, 1.27, -4.18, -6.70, -4.71, -3.95, -8.48, -11.70,
];

#[allow(dead_code)]
pub fn get_sun_light() -> Vec<f64> {
    let mut ret = vec![0f64; 365];
    let mut po = 15;
    let mut x0 = 0_f64;
    for y in 0..11 {
        x0 = SUN_LIGHT[y];
        let dx = (SUN_LIGHT[y + 1] - SUN_LIGHT[y]) / 30_f64;
        for _m in 0..30 {
            x0 += dx;
            ret[po] = x0;
            //print!("{}.{}\n", po, x0);
            po += 1;
        }
    }
    let dx = (SUN_LIGHT[0] - SUN_LIGHT[11]) / 30_f64;
    for _m in 0..35 {
        x0 += dx;
        ret[po] = x0;
        //print!("{}.{}\n", po, x0);
        po += 1;
        if po >= 365 {
            po = 0;
        }
    }
    ret
}

#[allow(dead_code)]
pub fn load_rain() -> HashMap<String, Vec<f64>> {
    //if let Ok(file) = File::open("data/pvrnyr.bin") {
    if let Ok(file) = File::open(crate::sg::ldp::res("pvrnyr.bin")) {
        let rd = BufReader::new(file);
        if let Ok(pvrnyr) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, Vec<f64>>>(rd)
        {
            return pvrnyr;
        }
    }
    HashMap::new()
}

pub fn load_sbgismp() -> HashMap<String, (f32,f32,String,String,String,String,String)> {
    if let Ok(file) = File::open(crate::sg::ldp::res("sbgismp.bin")) {
        let rd = BufReader::new(file);
        if let Ok(sbgismp) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, (f32,f32,String,String,String,String,String)>>(rd)
        {
			return sbgismp;
        } else {
			print!("NG DESER SBGISMP\n");
		}
    } else {
		print!("NG OPEN SBGISMP\n");
	}
	HashMap::new()
}

// car register by province
pub fn load_pvcamp() -> HashMap<String, f64> {
    //if let Ok(file) = File::open("data/pvcamp.bin") {
    if let Ok(file) = File::open(crate::sg::ldp::res("pvcamp.bin")) {
        let rd = BufReader::new(file);
        if let Ok(pvcamp) = bincode::deserialize_from::<BufReader<File>, HashMap<String, f64>>(rd) {
            return pvcamp;
        }
    }
    HashMap::new()
}

#[allow(dead_code)]
pub fn load_gpp() -> HashMap<String, f64> {
    //if let Ok(file) = File::open("data/pvgprt.bin") {
    if let Ok(file) = File::open(crate::sg::ldp::res("pvgprt.bin")) {
        let rd = BufReader::new(file);
        if let Ok(pvgprt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, f64>>(rd) {
            return pvgprt;
        }
    }
    HashMap::new()
}

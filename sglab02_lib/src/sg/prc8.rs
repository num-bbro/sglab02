//use std::io::BufReader;
//use std::fs::File;
use crate::sg::gis1::ar_list;
use crate::sg::gis1::db2_dir;
use crate::sg::gis1::DbfVal;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub async fn prc81() -> Result<(), Box<dyn std::error::Error>> {
    let ly = "DS_GroupMeter_Detail";
    for r in ar_list() {
        let mut db = Vec::<HashMap<String, DbfVal>>::new();
        let dbf = format!("{}/{}_{}.db", db2_dir(), r, ly);
        println!("dbf {}", dbf);
        if let Ok(f) = File::open(dbf) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfVal>>>(
                BufReader::new(f),
            ) {
                db = dt;
            }
        }
        println!("red {}", db.len());
    }
    Ok(())
}

use crate::sg::dcl::LoadProfVal;
use crate::sg::prc3::ld_p3_prvs;
use crate::sg::prc5::{prv_calc, pv_sub};

pub async fn prc82() -> Result<(), Box<dyn std::error::Error>> {
    let prvs = ld_p3_prvs();
    for pv in &prvs {
        //println!("{}", pv);
        if let (Some(_sbs), Some(calc)) = (pv_sub().get(pv), prv_calc().get(pv)) {
            let mut sum = 0f32;
            for dl in &calc.year_load.loads {
                for hh in &dl.load {
                    if let LoadProfVal::Value(d) = hh {
                        sum += d;
                    }
                }
            }
            //println!("{}\t{}", pv, calc.year_load.power_quality.pos_energy);
            sum /= 2f32;
            println!("{}\t{}", pv, sum);
        }
    }
    Ok(())
}

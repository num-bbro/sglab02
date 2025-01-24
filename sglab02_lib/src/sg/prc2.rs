//use crate::sg::ldp::base;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
//use crate::sg::ldp::FeederTranx;
use std::fs::File;
use std::io::BufReader;
use crate::sg::ldp::TranxInfo;
use crate::sg::imp::CSVFile;
use crate::sg::imp::src_dir;
//use std::path::PathBuf;
use crate::sg::imp::data_dir;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Transformer {
    pub fd_id: String,
    pub tx_id: String,
    pub tx_power: f64,
    pub tx_own: String,
    pub mt_ph_a: usize,
    pub mt_ph_b: usize,
    pub mt_ph_c: usize,
    pub mt_1_ph: usize,
    pub mt_3_ph: usize,
    pub mt_else: usize,
    pub mt_cnt: usize,
    pub eg5_a: f64,
    pub eg5_b: f64,
    pub eg5_c: f64,
    pub eg5_1p: f64,
    pub eg5_3p: f64,
    pub eg5_sm: f64,
    pub eg2_a: f64,
    pub eg2_b: f64,
    pub eg2_c: f64,
    pub eg2_1p: f64,
    pub eg2_3p: f64,
    pub eg2_sm: f64
}

/*
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransEnergy {
    pub pv_id: String,
    pub sb_id: String,
    pub fd_id: String,
    pub tx_no: usize,
    pub tx_pw_no: HashMap::<usize,(usize,usize,usize)>,
    pub tx_mt_min: usize,
    pub tx_mt_max: usize,
    pub mt_ph_a: usize,
    pub mt_ph_b: usize,
    pub mt_ph_c: usize,
    pub mt_1_ph: usize,
    pub mt_3_ph: usize,
    pub mt_else: usize,
    pub mt_cnt: usize,
    pub eg5_a: f64,
    pub eg5_b: f64,
    pub eg5_c: f64,
    pub eg5_1p: f64,
    pub eg5_3p: f64,
    pub eg5_sm: f64,
    pub eg2_a: f64,
    pub eg2_b: f64,
    pub eg2_c: f64,
    pub eg2_1p: f64,
    pub eg2_3p: f64,
    pub eg2_sm: f64
}
*/

// billing to trans+mete
pub async fn prc21() -> Result<(), Box<dyn std::error::Error>> {
    //let base = base();
    let mut mt202405 = HashMap::<String,f64>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("mt-202405.bin")) {
        let rd = BufReader::new(file);
        if let Ok(mt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, f64>>(rd) {
            mt202405 = mt;
        }
    }
    let mut mt202402 = HashMap::<String,f64>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("mt-202402.bin")) {
        if let Ok(mt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, f64>>(BufReader::new(file)) {
            mt202402 = mt;
        }
    }
    let /*mut*/ txmtno = HashMap::<usize,(usize,usize,usize)>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("txmtmp.bin")) {
        //let (mut t30, mut t50, mut t100, mut t160, mut t250, mut t300, mut t500, mut t2000) = (0,0,0,0,0,0,0,0);
        let rd = BufReader::new(file);
        if let Ok(txmtmp) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, TranxInfo>>(rd) {
            let mut fd_trs = HashMap::<String,Vec<Transformer>>::new();
            //let mut fdtxmp = HashMap::<String, Vec<FeederTranx>>::new();
            for (_k, tx) in txmtmp { // for each tx
                if tx.trans_feed.len() < 5 {
                    continue;
                }
                let fd0 = &tx.trans_feed[3..5];
                let fd_id = format!("{}{}", tx.trans_sub, fd0);

                let tx_id = tx.trans_id.to_string();
                let tx_power = tx.trans_power;
                let tx_own = tx.trans_own;
                //println!("tx:{} pw:{} ow:{}", tx_id, tx_power, tx_own);

                let (mut mt_ph_a, mut mt_ph_b, mut mt_ph_c, mut mt_1_ph, mut mt_3_ph, mut mt_else, _mt_sm) =
                    (0, 0, 0, 0, 0, 0, tx.meters.len());
                let (mut eg5_a, mut eg5_b, mut eg5_c, mut eg5_1p, mut eg5_3p, mut eg5_sm) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64, );
                let (mut eg2_a, mut eg2_b, mut eg2_c, mut eg2_1p, mut eg2_3p, mut eg2_sm) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64, );
                let mt_cnt = tx.meters.len();
                for mt in &tx.meters {
                    let /*mut*/ eg05 = if let Some(eg) = mt202405.get(&mt.meter_id) { *eg } else { 0f64 };
                    let /*mut*/ eg02 = if let Some(eg) = mt202402.get(&mt.meter_id) { *eg } else { 0f64 };
                    eg5_sm += eg05;
                    eg2_sm += eg02;
                    if mt.meter_phase == "A" {
                        mt_ph_a += 1;
                        mt_1_ph += 1;
                        eg5_a += eg05;
                        eg5_1p += eg05;
                        eg2_a += eg02;
                        eg2_1p += eg02;
                    } else if mt.meter_phase == "B" {
                        mt_ph_b += 1;
                        mt_1_ph += 1;
                        eg5_b += eg05;
                        eg5_1p += eg05;
                        eg2_b += eg02;
                        eg2_1p += eg02;
                    } else if mt.meter_phase == "C" {
                        mt_ph_c += 1;
                        mt_1_ph += 1;
                        eg5_c += eg05;
                        eg5_1p += eg05;
                        eg2_c += eg02;
                        eg2_1p += eg02;
                    } else if mt.meter_phase == "ABC" {
                        mt_3_ph += 1;
                        eg5_3p += eg05;
                        eg2_3p += eg05;
                    } else {
                        mt_else += 1;
                    }
                }
                let trn = Transformer {
                    fd_id, tx_id, tx_power, tx_own, mt_ph_a, mt_ph_b, mt_ph_c, mt_1_ph, mt_3_ph, mt_else, mt_cnt,
                    eg5_a, eg5_b, eg5_c, eg5_1p, eg5_3p, eg5_sm, eg2_a, eg2_b, eg2_c, eg2_1p, eg2_3p, eg2_sm,
                };
                if let Some(/*mut*/ trnv) = fd_trs.get_mut(&trn.fd_id) {
                    trnv.push(trn);
                } else {
                    fd_trs.insert(trn.fd_id.to_string(), vec![trn]);
                }
            }
            println!("fd trs {}", fd_trs.len());
            let file = format!("{}/fd_trs.bin", crate::sg::imp::data_dir());
            if let Ok(ser) = bincode::serialize(&fd_trs) {
                std::fs::write(file, ser).unwrap();
            }
        } // read txmtmp.bin
    } // end open file
    println!("{:?}", txmtno);

    Ok(())
}

// transformer to feeder
pub async fn prc23() -> Result<(), Box<dyn std::error::Error>> {
    println!("prc23");
    let /*mut*/ _fd_trs = HashMap::<String,Vec::<Transformer>>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("fd_trs.bin")) {
        if let Ok(trs) = bincode::deserialize_from::<BufReader<File>, HashMap::<String,Vec::<Transformer>>>(BufReader::new(file)) {
            println!("trns {}", trs.len());
        }
    }
    Ok(())
}

// billing file to bin
pub async fn prc22() -> Result<(), Box<dyn std::error::Error>> {
    let flst = vec!["202402","202405"];
    let /*mut*/ _csv_v = Vec::<CSVFile>::new();
    for f in flst {
        let fln = format!("{}/20240801_กรอ/export_กรอ_bil013_{}.csv", src_dir(), f);
        println!("start {}", &fln);
        if let Ok(mut rdr) = csv::Reader::from_path(&fln) { // if read file
            let mut mtpwmp = HashMap::<String,f64>::new();
            for rs in rdr.records() { // loop all record
                let /*mut*/ _row = Vec::<String>::new();
                //let mut ary = [String; 23];
                if let Ok(rc) = rs { // if the record exist
                    if let (Some(id),Some(pw)) = (rc.get(7), rc.get(15)) {
                        if let Ok(pw) = pw.parse::<f64>() {
                            if let Some(/*mut*/ v) = mtpwmp.get_mut(id) {
                                *v += pw;
                                //println!("DBL {} {} = {}", id, pw, *v);
                            } else {
                                //println!("mtid='{}'", id);
                                mtpwmp.insert(id.to_string(), pw);
                            }
                        }
                    }
                } // fi record
            } // loop all rec
            if let Some(_p) = mtpwmp.get("5900745620") {
                println!("PPPP 5900745620");
            }
            println!("{} = {}", f, mtpwmp.len());
            if let Ok(ser) = bincode::serialize(&mtpwmp) { // if serialize
                let lpf = format!("{}/mt-{}.bin", data_dir(), f);
                std::fs::write(lpf, ser).unwrap();
            } // end serialize
        }
    }
    Ok(())
}


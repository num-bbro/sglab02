use serde::{Deserialize, Serialize};
use sglab02_lib::sg::prc2::Transformer;
use sglab02_lib::sg::prc3::ld_p3_prv_sub;
use sglab02_lib::sg::prc3::ld_p3_sub_inf;
use sglab02_lib::sg::prc5::prvs;
use sglab02_lib::sg::prc6::ld_p62_fd_trans;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubCalc {
    pub pv: String,
    pub sb: String,
    pub p_tx_cn_m: HashMap<u32, u32>,
    pub c_tx_cn_m: HashMap<u32, u32>,

    pub mt_ph_a: usize,
    pub mt_ph_b: usize,
    pub mt_ph_c: usize,
    pub mt_1_ph: usize,
    pub mt_3_ph: usize,

    pub eg_a: f64,
    pub eg_b: f64,
    pub eg_c: f64,
    pub eg_1p: f64,
    pub eg_3p: f64,
    pub eg_sm: f64,
}

pub fn sb_tr() -> Result<(), Box<dyn std::error::Error>> {
    let pv = prvs();
    let pvsb = ld_p3_prv_sub();
    let sbif = ld_p3_sub_inf();
    let mut sbtxmp = HashMap::new();
    let mut tx_cn_m = HashMap::<u32, u32>::new();
    for p in pv {
        let pp = p.to_string();
        if let Some(sbv) = pvsb.get(&pp) {
            for sb in sbv {
                let (mut mt_ph_a, mut mt_ph_b, mut mt_ph_c, mut mt_1_ph, mut mt_3_ph) =
                    (0, 0, 0, 0, 0);
                let (mut eg_a, mut eg_b, mut eg_c, mut eg_1p, mut eg_3p, mut eg_sm) =
                    (0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
                let mut p_tx_cn_m = HashMap::<u32, u32>::new();
                let mut c_tx_cn_m = HashMap::<u32, u32>::new();
                if let Some(sf) = sbif.get(sb) {
                    println!("{}-{}", pp, sb);
                    for f in &sf.feeders {
                        let txv: Vec<Transformer> = ld_p62_fd_trans(f);
                        for tr in txv {
                            let txpw = tr.tx_power as u32;
                            if let Some(cn) = tx_cn_m.get_mut(&txpw) {
                                *cn += 1;
                            } else {
                                tx_cn_m.insert(txpw, 1u32);
                            }
                            mt_ph_a += tr.mt_ph_a;
                            mt_ph_b += tr.mt_ph_b;
                            mt_ph_c += tr.mt_ph_c;
                            mt_1_ph += tr.mt_1_ph;
                            mt_3_ph += tr.mt_3_ph;

                            eg_a += (tr.eg5_a + tr.eg2_a) * 0.5f64 * 12f64 / 1000f64;
                            eg_b += (tr.eg5_b + tr.eg2_b) * 0.5f64 * 12f64 / 1000f64;
                            eg_c += (tr.eg5_c + tr.eg2_c) * 0.5f64 * 12f64 / 1000f64;
                            eg_1p += (tr.eg5_1p + tr.eg2_1p) * 0.5f64 * 12f64 / 1000f64;
                            eg_3p += (tr.eg5_3p + tr.eg2_3p) * 0.5f64 * 12f64 / 1000f64;
                            eg_sm += (tr.eg5_sm + tr.eg2_sm) * 0.5f64 * 12f64 / 1000f64;

                            match tr.tx_own.as_str() {
                                "P" => {
                                    if let Some(cn) = p_tx_cn_m.get_mut(&txpw) {
                                        *cn += 1;
                                    } else {
                                        p_tx_cn_m.insert(txpw, 1u32);
                                    }
                                }
                                "C" => {
                                    if let Some(cn) = c_tx_cn_m.get_mut(&txpw) {
                                        *cn += 1;
                                    } else {
                                        c_tx_cn_m.insert(txpw, 1u32);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                let sbtx = SubCalc {
                    pv: pp.to_string(),
                    sb: sb.to_string(),
                    p_tx_cn_m,
                    c_tx_cn_m,
                    mt_ph_a,
                    mt_ph_b,
                    mt_ph_c,
                    mt_1_ph,
                    mt_3_ph,
                    eg_a,
                    eg_b,
                    eg_c,
                    eg_1p,
                    eg_3p,
                    eg_sm,
                    ..Default::default()
                };
                sbtxmp.insert(sb.to_string(), sbtx);
            }
        }
    }
    /*
    for (k, v) in tx_cn_m {
        println!("{k} {v}");
    }
    */
    let file = "../sgdata/p41_sbtx.bin";
    if let Ok(ser) = bincode::serialize(&sbtxmp) {
        std::fs::write(file, ser).unwrap();
    }
    let sb = ld_sb_tr()?;
    println!("sub: {}", sb.len());
    Ok(())
}

pub fn ld_sb_tr() -> Result<HashMap<String, SubCalc>, Box<dyn std::error::Error>> {
    let f = File::open("../sgdata/p41_sbtx.bin")?;
    Ok(bincode::deserialize_from::<
        BufReader<File>,
        HashMap<String, SubCalc>,
    >(BufReader::new(f))?)
}

use std::sync::OnceLock;
pub static SB_TR: OnceLock<HashMap<String, SubCalc>> = OnceLock::new();
pub fn ld_sb_tr0() -> &'static HashMap<String, SubCalc> {
    SB_TR.get_or_init(sb_tr_init)
}
fn sb_tr_init() -> HashMap<String, SubCalc> {
    ld_sb_tr().unwrap()
}

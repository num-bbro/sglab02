//use std::io::BufReader;
//use std::fs::File;
use crate::sg::gis1::ar_list;
use crate::sg::gis1::db2_dir;
use crate::sg::gis1::DbfVal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use thousands::Separable;

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

//pub fn ld_p3_sub_inf() -> HashMap<String, SubstInfo> {
use crate::sg::prc3::ld_p3_sub_inf;
use crate::sg::prc3::ld_p3_treg_m;

use super::prc1::SubstInfo;

pub fn prc83() {
    let subhm: HashMap<String, SubstInfo> = ld_p3_sub_inf();
    println!("{}", subhm.len());
}

use crate::sg::prc3::TransEnergy;
pub fn prc84() {
    let treg_m: HashMap<String, TransEnergy> = ld_p3_treg_m("p3_pv_treg_m.bin");
    println!("{}", treg_m.len());
    let mut trpw_no = HashMap::<usize, usize>::new();
    for (_pv, tr) in treg_m {
        //println!("{_pv}");
        for (pw, (cn, _, _)) in &tr.txp_pw_no {
            if let Some(cnn) = trpw_no.get_mut(pw) {
                *cnn += cn;
            } else {
                trpw_no.insert(*pw, *cn);
            }
        }
    }
    println!("{:?}", trpw_no);
    let mut v: Vec<_> = trpw_no.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));
    println!("{:?}", v);
}

use crate::sg::prc2::Transformer;
use crate::sg::prc3::ld_p3_prv_sub;
use crate::sg::prc3::DataCalc;
use crate::sg::prc4::grp1;
use crate::sg::prc4::ld_pv_sbv_m;
use crate::sg::prc4::Proc41Item;
use crate::sg::prc6::ld_p61_fd_calc;
//use std::fmt::Write;

//use crate::sg::prc5::feed_calc;
use crate::sg::prc4::ld_p48_pv_eb;
use crate::sg::prc4::ld_p48_pv_et;
use crate::sg::prc4::ld_p48_pv_ev;
use crate::sg::prc4::EVCalc;
use crate::sg::prc6::ld_p62_fd_trans;
//use crate::sg::prc8::LoadProfVal::Value;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct P85Para {
    pub pv: String,
    pub sball: usize,
    pub sbcn: usize,
    pub dt: usize,
    pub m1: usize,
    pub m3: usize,
    pub bcn: i32,
    pub bmwh: f32,
    pub txpw30: i32,
    pub txpw50: i32,
    pub txpw100: i32,
    pub txpw160: i32,
    pub txpw300: i32,
    pub txpw500: i32,
    pub txpw1000: i32,
    pub txpw2000: i32,
    pub txpwex: i32,
    pub en: f32,
    pub ea: f64,
    pub eb: f64,
    pub ec: f64,
    pub ee: f64,
    pub da: f64,
    pub db: f64,
    pub dc: f64,
    pub etotal: f64,
    pub ev: Vec<EVCalc>,
    pub et: Vec<EVCalc>,
    pub ek: Vec<EVCalc>,
}

pub fn prc85() {
    // all P85Para process
    let pv = grp1();
    let pvsb = ld_p3_prv_sub();
    let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();
    let pvev = ld_p48_pv_ev();
    let pvet = ld_p48_pv_et();
    let pveb = ld_p48_pv_eb();
    println!("ev:{}  et:{}  eb:{}", pvev.len(), pvet.len(), pveb.len());

    let mut p85_para = Vec::<P85Para>::new();
    for p in &pv {
        let pp = p.to_string();
        let sbv = pvsb.get(&pp).unwrap();
        let sbs = sbsl.get(&pp).unwrap();
        println!("{} {} {}", p, sbv.len(), sbs.len());
        let mut trs = HashMap::<i32, i32>::new();
        let mut p85 = P85Para::default();
        p85.pv = p.to_string();
        p85.sball = sbv.len(); // all substations
        p85.sbcn = sbs.len(); // selected substations
        if let Some(ev) = pvev.get(&pp) {
            p85.ev = ev.clone();
        }
        if let Some(et) = pvet.get(&pp) {
            p85.et = et.clone();
        }
        if let Some(eb) = pveb.get(&pp) {
            p85.ek = eb.clone();
        }
        for sb in sbs {
            let ss: &Proc41Item = sb;
            let sf: &SubstInfo = sbif.get(&ss.sbid).unwrap();

            for f in &sf.feeders {
                let ld: DataCalc = ld_p61_fd_calc(f);
                let tx: Vec<Transformer> = ld_p62_fd_trans(f);
                p85.en += ld.year_load.power_quality.pos_energy;
                for tr in tx {
                    if tr.tx_own != "P" {
                        continue;
                    }
                    p85.ea += tr.eg5_a * 12f64 / 1000f64;
                    p85.eb += tr.eg5_b * 12f64 / 1000f64;
                    p85.ec += tr.eg5_c * 12f64 / 1000f64;
                    p85.etotal += tr.eg5_sm;
                    let txpw = tr.tx_power as i32;
                    if let Some(cn) = trs.get_mut(&txpw) {
                        *cn += 1;
                    } else {
                        trs.insert(txpw, 1);
                    }
                    if txpw <= 30 {
                        p85.txpw30 += 1;
                    } else if txpw <= 50 {
                        p85.txpw50 += 1;
                    } else if txpw <= 100 {
                        p85.txpw100 += 1;
                    } else if txpw <= 160 {
                        p85.txpw160 += 1;
                    } else if txpw <= 300 {
                        p85.txpw300 += 1;
                    } else if txpw <= 500 {
                        p85.txpw500 += 1;
                    } else if txpw <= 1000 {
                        p85.txpw1000 += 1;
                    } else if txpw <= 2000 {
                        p85.txpw2000 += 1;
                    } else {
                        p85.txpwex += 1;
                    }
                }
            }
            p85.dt += ss.dt;
            p85.m1 += ss.mt1;
            p85.m3 += ss.mt3;
            if ss.mwh > 0.0 {
                p85.bcn += 1;
                p85.bmwh += ss.mwh;
            }
        }
        p85.ee = (p85.ea + p85.eb + p85.ec) / 3f64;
        p85.da = (p85.ea - p85.ee).abs();
        p85.db = (p85.eb - p85.ee).abs();
        p85.dc = (p85.ec - p85.ee).abs();
        //println!("{:?}", p85);
        p85_para.push(p85);
    }
    let file = format!("{}/p85_para.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&p85_para) {
        std::fs::write(file, ser).unwrap();
    }
}

pub fn ld_p85_para() -> Vec<P85Para> {
    let file = format!("{}/p85_para.bin", crate::sg::imp::data_dir());
    if let Ok(f) = File::open(&file) {
        if let Ok(dt) =
            bincode::deserialize_from::<BufReader<File>, Vec<P85Para>>(BufReader::new(f))
        {
            return dt;
        }
    }
    Vec::<P85Para>::new()
}

pub fn p8_ev() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let up = 420.0;
    for pv in p85 {
        //println!("{:?}", pv.pv);
        let mut pvtot = 0.0;
        for evy in pv.ev {
            tot += up * evy.evmwh;
            pvtot += up * evy.evmwh;
            //print!(" {}:{}", evy.yr, evy.evmwh);
        }
        println!("{} {pvtot:.1}", pv.pv);
    }
    println!("EV: {}", tot.separate_with_commas());
}

pub fn p8_et() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let up = 420.0;
    for pv in p85 {
        let mut pvtot = 0.0;
        //println!("{:?}", pv.pv);
        for evy in pv.et {
            tot += up * evy.evmwh;
            pvtot += up * evy.evmwh;
            //print!(" {}:{}", evy.yr, evy.evmwh);
        }
        println!("{} {pvtot:.1}", pv.pv);
        //println!("");
    }
    println!("ET: {}", tot.separate_with_commas());
}

pub fn p8_eb() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let up = 420.0;
    for pv in p85 {
        let mut pvtot = 0.0;
        //println!("{:?}", pv.pv);
        for evy in pv.ek {
            tot += up * evy.evmwh;
            pvtot += up * evy.evmwh;
            //print!(" {}:{}", evy.yr, evy.evmwh);
        }
        //println!("");
        println!("{} {pvtot:.1}", pv.pv);
    }
    println!("ET: {}", tot.separate_with_commas());
}

pub fn p8_mv_re() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let up = 400.0;
    let rt = 0.05;
    for pv in p85 {
        let e1 = pv.en * rt * up * 12.0;
        tot += e1;
        println!("{} {}", pv.pv, e1.separate_with_commas());
    }
    println!("RE: {}", tot.separate_with_commas());
}

pub fn p8_lv_re() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let up = 400.0;
    let rt = 0.05;
    for pv in p85 {
        let e1 = pv.etotal / 1000.0 * rt * up * 12.0;
        tot += e1;
        println!("{} {}", pv.pv, e1.separate_with_commas());
    }
    println!("LV RE: {}", tot.separate_with_commas());
}

pub fn p8_sell_old_mt() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let up1 = 50.0;
    let up3 = 100.0;
    for pv in p85 {
        let e1 = pv.m1 as f32 * up1;
        let e3 = pv.m3 as f32 * up3;
        tot += e1 + e3;
        println!(
            "{} {} {}",
            pv.pv,
            e1.separate_with_commas(),
            e3.separate_with_commas()
        );
    }
    println!("sell meter: {}", tot.separate_with_commas());
}

pub fn p8_rep_cut() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let rt = 0.1;
    let yr = 12.0;
    let up1 = 525.0 + 100.0;
    let up3 = 1285.0 + 200.0;
    for pv in p85 {
        let e1 = pv.m1 as f32 * rt * up1 * yr;
        let e3 = pv.m3 as f32 * rt * up3 * yr;
        tot += e1 + e3;
        println!(
            "{} {} {}",
            pv.pv,
            e1.separate_with_commas(),
            e3.separate_with_commas()
        );
    }
    println!("replace cut: {}", tot.separate_with_commas());
}

pub fn p8_end_cut() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let rt = 0.02;
    let yr = 12.0;
    let up1 = 525.0 + 250.0;
    let up3 = 1285.0 + 400.0;
    for pv in p85 {
        let e1 = pv.m1 as f32 * rt * up1 * yr;
        let e3 = pv.m3 as f32 * rt * up3 * yr;
        tot += e1 + e3;
        println!(
            "{} {} {}",
            pv.pv,
            e1.separate_with_commas(),
            e3.separate_with_commas()
        );
    }
    println!("old replace cut: {}", tot.separate_with_commas());
}

pub fn p8_read_cut() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let rt = 0.5417;
    let yr = 12.0;
    let up = 6.2;
    for pv in p85 {
        let em = pv.m1 + pv.m3;
        let em = em as f32;
        let ex = em * up * rt * 12.0 * yr;
        tot += ex;
        println!("{} {}", pv.pv, ex.separate_with_commas(),);
    }
    println!("read cut: {}", tot.separate_with_commas());
}

pub fn p8_conn_cut() {
    let p85 = ld_p85_para();
    let mut tot = 0.0;
    let yr = 12.0;
    let c1 = 130.0;
    let c3 = 190.0;
    let r1 = 0.004;
    let r3 = 0.001;
    let dy = 260.0;
    for pv in p85 {
        let m1 = pv.m1 as f32;
        let m3 = pv.m3 as f32;
        let xm1 = m1 * r1 * dy * c1 * yr;
        let xm3 = m3 * r3 * dy * c3 * yr;
        let ex = xm1 + xm3;
        tot += ex;
        println!("{} {}", pv.pv, ex.separate_with_commas(),);
    }
    println!("conn cut: {}", tot.separate_with_commas());
}
//sub_calc() -> &'static HashMap<String, DataCalc>

//use crate::sg::prc3::ld_p3_calc;
//use crate::sg::prc5::sub_calc;
use crate::sg::wk4::DayLoad;
use crate::sg::wk4::YearLoad;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoadProYr {
    pub prov: String,
    pub ssid: String,
    pub fdid: String,
    pub year_load: YearLoad,
}

impl LoadProYr {
    pub fn new() -> Self {
        let mut n = Self::default();
        for di in 0..365 {
            // for day
            let mut day_load = DayLoad::default();
            day_load.day = di + 1;
            for _hi in 0..48 {
                // for hour
                day_load.load.push(LoadProfVal::Value(0f32));
            } // for hour
            n.year_load.loads.push(day_load);
        } // for day
        n
    }
}

pub fn p8_bess_1() {
    let pv = grp1();
    let pvsb = ld_p3_prv_sub();
    let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();
    for p in &pv {
        let pp = p.to_string();
        let sbv = pvsb.get(&pp).unwrap();
        let sbs = sbsl.get(&pp).unwrap();
        println!("{} {} {}", p, sbv.len(), sbs.len());
        for sb in sbs {
            if let Some(sf) = sbif.get(&sb.sbid) {
                let mut sld = LoadProYr::new();
                sld.prov = pp.to_string();
                sld.ssid = sb.sbid.to_string();
                let mut vals = vec![0f32; 365 * 48];
                let mut cn = 0;
                for f in &sf.feeders {
                    let ld: DataCalc = ld_p61_fd_calc(f);
                    if ld.year_load.loads.len() < 365 {
                        continue;
                    }
                    for di in 0..365 {
                        let dl = &ld.year_load.loads[di];
                        for ti in 0..48 {
                            let tl = &dl.load[ti];
                            let ii = di * 48 + ti;
                            match tl {
                                LoadProfVal::Value(v) => {
                                    vals[ii] += v;
                                    cn += 1;
                                }
                                LoadProfVal::Null | LoadProfVal::None => {}
                            }
                        }
                    }
                }
                println!("sb:{} v:{}", sb.sbid, cn);
            } else {
                continue;
            }
        }
    }
}

pub fn p8_tou() {
    let p85 = ld_p85_para();
    /*
    let tot = 18_700_000.0 * 9.0;
    let tot = 491_300_000.0 * 9.0;
    let tot = 367_140_000.0 * 9.0;
    let tot = 39_720_000.0 * 9.0;
    let tot = 12_150_000.0 * 9.0;
    let tot = 1_094_360_000.0 * 9.0;
    let tot = 1_045_990_000.0 * 9.0;
    let tot = 13_700_000.0 * 9.0;
    let tot = 827_500_000.0;
    let tot = 1_275_000_000.0;
    let tot = 2_721_000_000.0;
    let tot = 42_241_300_000.0;
    let tot = 152_440_000.0 * 9.0;
    let tot = 143_990_000.0 * 9.0;
    let tot = 209_950_000.0 * 9.0;
    let tot = 264_210_000.0 * 9.0;
    */
    let tot = 9_500_380_000.0 * 9.0;
    let mut cn = 0;
    for pv in &p85 {
        cn += pv.m1 + pv.m3;
    }
    for pv in &p85 {
        let pc = (pv.m1 + pv.m3) as f32 / cn as f32;
        println!("{} {:.1}", pv.pv, pc * tot);
    }
}

use crate::sg::prc5::feed_calc;
use crate::sg::prc5::fd_trs;
use serde::Serialize;
use serde::Deserialize;
use crate::sg::prc3::DataCalc;
use crate::sg::prc2::Transformer;
use std::io::BufReader;
use std::fs::File;


pub async fn prc61() -> Result<(), Box<dyn std::error::Error>> {
    let fdcalc = feed_calc();
    println!("fd: {}", fdcalc.len());
    //pub fn feed_calc() -> &'static HashMap<String, DataCalc> {    FEED_CALC.get_or_init(feed_calc_init) }
    for (fd,calc) in fdcalc {
        let fd_dir = format!("{}/mvfd/{}", crate::sg::imp::data_dir(), fd);
        let _ = std::fs::create_dir_all(&fd_dir);
        let fd_file = format!("{}/p61_{}.bin", fd_dir, fd);
        if let Ok(ser) = bincode::serialize(&calc) {
            std::fs::write(fd_file.clone(), ser).unwrap();
            println!("write {}", fd_file);
            let _ld = ld_p61_fd_calc(fd);
        }
    }
    Ok(())
}

pub fn ld_p61_fd_calc(fd: &String) -> DataCalc {
    let fd_file = format!("{}/mvfd/{}/p61_{}.bin", crate::sg::imp::data_dir(), fd, fd);
    if let Ok(f) = File::open(crate::sg::ldp::res(&fd_file)) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, DataCalc>(BufReader::new(f)) {
            return dt;
        }
    }
    DataCalc::default()
}

pub async fn prc62() -> Result<(), Box<dyn std::error::Error>> {
    let fdtrs = fd_trs();
    println!("fd: {}", fdtrs.len());
    for (fd, trans) in fdtrs {
        let fd_dir = format!("{}/mvfd/{}", crate::sg::imp::data_dir(), fd);
        let _ = std::fs::create_dir_all(&fd_dir);
        let fd_file = format!("{}/p62_{}.bin", fd_dir, fd);
        if let Ok(ser) = bincode::serialize(&trans) {
            std::fs::write(fd_file.clone(), ser).unwrap();
            println!("write {}", fd_file);
            let _ld = ld_p62_fd_trans(fd);
        }
    }
    Ok(())
}

pub fn ld_p62_fd_trans(fd: &String) -> Vec<Transformer> {
    let fd_file = format!("{}/mvfd/{}/p62_{}.bin", crate::sg::imp::data_dir(), fd, fd);
    if let Ok(f) = File::open(crate::sg::ldp::res(&fd_file)) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, Vec<Transformer>>(BufReader::new(f)) {
            return dt;
        }
    }
    Vec::<Transformer>::new()
}

use crate::sg::gis1::db2_dir;
use crate::sg::gis1::DbfVal;
use std::collections::HashMap;
use crate::sg::gis1::ar_list;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Prc6TranxInfo {
    pub feeder_id: String,
    pub facility_id: String,
    pub x: f64,
    pub y: f64,
}

use crate::sg::prc6::DbfVal::Character;
pub async fn prc63() -> Result<(), Box<dyn std::error::Error>> {
    let ly = "DS_Transformer";
    //let mut fd_trs = HashMap::<String,Vec::<Prc6TranxInfo>>::new();
    let mut fd_tr_lo = HashMap::<String,HashMap::<String,Prc6TranxInfo>>::new();
    for r in ar_list() {
        let dbf = format!("{}/{}_{}.db", db2_dir(), r, ly);
        let pnf = format!("{}/{}_{}.pn", db2_dir(), r, ly);
        //println!("{}, {}", dbf, rgf);
        let mut dbs = Vec::<HashMap::<String, DbfVal>>::new();
        let mut pns = Vec::<(f64,f64)>::new();
        if let Ok(f) = File::open(dbf) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfVal>>>(BufReader::new(f)) {
                dbs = dt;
            }
        }
        if let Ok(f) = File::open(pnf) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<(f64,f64)>>(BufReader::new(f)) {
                pns = dt;
                println!("pns: {}", pns.len());
            }
        }
        if dbs.len()!=pns.len() { println!("ERROR {} {}", dbs.len(), pns.len()); continue; }
        println!("{} {}", dbs.len(), pns.len());
        for i in 0..dbs.len() {
            let db = &dbs[i];
            let pn = &pns[i];
            if let (Some(Character(Some(faid))), Some(Character(Some(fdid)))) = (db.get("FACILITYID"), db.get("FEEDERID")) {
                let facility_id = faid.to_string();
                let feeder_id = fdid.to_string();
                let x = pn.0;
                let y = pn.1;
                let trx = Prc6TranxInfo { feeder_id, facility_id, x, y };
                /*
                if let Some(trxv) = fd_trs.get_mut(fdid) {
                    trxv.push(trx);
                } else {
                    fd_trs.insert(fdid.to_string(), vec![trx]);
                }
                */
                if let Some(tr_lo) = fd_tr_lo.get_mut(fdid) {
                    tr_lo.insert(faid.to_string(), trx);
                } else {
                    let mut tr_lo = HashMap::<String,Prc6TranxInfo>::new();
                    tr_lo.insert(faid.to_string(), trx);
                    fd_tr_lo.insert(fdid.to_string(), tr_lo);
                }
            }
        }
    }
    for (fd, fd_tr_lo) in fd_tr_lo {
        let fd_dir = format!("{}/mvfd/{}", crate::sg::imp::data_dir(), fd);
        let _ = std::fs::create_dir_all(&fd_dir);
        let fd_file = format!("{}/p63_lo_{}.bin", fd_dir, fd);
        if let Ok(ser) = bincode::serialize(&fd_tr_lo) {
            std::fs::write(fd_file.clone(), ser).unwrap();
            //let ld = ld_p62_fd_trans(fd);
            println!("write {}", fd_file);
        }
    }
    Ok(())
}

pub fn ld_p63_fd_tr_lo(fd: &String) -> HashMap::<String,Prc6TranxInfo> {
    let fd_file = format!("{}/mvfd/{}/p63_lo_{}.bin", crate::sg::imp::data_dir(), fd, fd);
    if let Ok(f) = File::open(crate::sg::ldp::res(&fd_file)) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, HashMap::<String,Prc6TranxInfo>>(BufReader::new(f)) {
            return dt;
        }
    }
    HashMap::<String,Prc6TranxInfo>::new()
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Prc6MeterInfo {
    pub feeder_id: String,
    pub pea_no: String,
    pub account_no: String,
    pub install_no: String,
    pub amp: f32,
    pub owner: String,
    pub x: f64,
    pub y: f64,
}

use crate::sg::prc6::DbfVal::Float;
pub async fn prc64() -> Result<(), Box<dyn std::error::Error>> {
    let ly = "DS_LowVoltageMeter";
    //let mut fd_trs = HashMap::<String,Vec::<Prc6TranxInfo>>::new();
    let mut fd_mt_mp = HashMap::<String, HashMap::<String, Prc6MeterInfo>>::new();
    for r in ar_list() {
        let dbf = format!("{}/{}_{}.db", db2_dir(), r, ly);
        let pnf = format!("{}/{}_{}.pn", db2_dir(), r, ly);
        //println!("{}, {}", dbf, rgf);
        let mut dbs = Vec::<HashMap::<String, DbfVal>>::new();
        let mut pns = Vec::<(f64,f64)>::new();
        if let Ok(f) = File::open(dbf) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfVal>>>(BufReader::new(f)) {
                dbs = dt;
            }
        }
        if let Ok(f) = File::open(pnf) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<(f64,f64)>>(BufReader::new(f)) {
                pns = dt;
            }
        }
        if dbs.len()!=pns.len() { println!("ERROR {} {}", dbs.len(), pns.len()); continue; }
        println!("{} {}", dbs.len(), pns.len());
        for i in 0..dbs.len() {
            let db = &dbs[i];
            if let ( Some(Character(Some(fdid))), Some(Character(Some(mtid))), Some(Character(Some(acid))), 
                Some(Character(Some(inst))), Some(Character(Some(own))), Some(Float(Some(amp))) ) = 
                ( db.get("FEEDERID"), db.get("PEANO"), db.get("ACCOUNTNUM"), db.get("INSTALLATI"), db.get("OWNER"), db.get("AMP") ) {
                if fdid.len()<5 { continue; }
                let fdid = fdid[0..5].to_string();
                let feeder_id = fdid.to_string();
                let pea_no = mtid.to_string();
                let account_no = acid.to_string();
                let install_no = inst.to_string();
                let amp = *amp;
                let owner = own.to_string();
                let x = pns[i].0;
                let y = pns[i].1;
                let mt = Prc6MeterInfo { feeder_id, pea_no, account_no, install_no, amp, owner, x, y };
                if let Some(mt_mp) = fd_mt_mp.get_mut(&fdid) {
                    mt_mp.insert(mtid.to_string(), mt);
                } else {
                    println!("fd: {}", fdid);
                    let mut mt_mp = HashMap::<String, Prc6MeterInfo>::new();
                    mt_mp.insert(mtid.to_string(), mt);
                    fd_mt_mp.insert(fdid.to_string(), mt_mp);
                }
            }
        }
    }
    for (fd, mt_mp) in fd_mt_mp {
        if fd.len()<5 { continue;}
        let fd = fd[0..5].to_string();
        let fd_dir = format!("{}/mvfd/{}", crate::sg::imp::data_dir(), fd);
        let _ = std::fs::create_dir_all(&fd_dir);
        let fd_file = format!("{}/p64_mt_mp_{}.bin", fd_dir, fd);
        if let Ok(ser) = bincode::serialize(&mt_mp) {
            std::fs::write(fd_file.clone(), ser).unwrap();
            //let ld = ld_p62_fd_trans(fd);
            let fd = ld_p64_mt_mp(&fd);
            println!("write {} = {}", fd_file, fd.len());
        }
    }

    Ok(())
}

pub fn ld_p64_mt_mp(fd: &String) -> HashMap::<String, Prc6MeterInfo> {
    let fd_file = format!("{}/mvfd/{}/p64_mt_mp_{}.bin", crate::sg::imp::data_dir(), fd, fd);
    if let Ok(f) = File::open(crate::sg::ldp::res(&fd_file)) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, HashMap::<String, Prc6MeterInfo>>(BufReader::new(f)) {
            return dt;
        }
    }
    HashMap::<String, Prc6MeterInfo>::new()
}

use crate::sg::ldp::TranxInfo;
use crate::sg::ldp::MeterInfo;

pub async fn prc65() -> Result<(), Box<dyn std::error::Error>> {
    let mut txmtmp = HashMap::<String, TranxInfo>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("txmtmp.bin")) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, HashMap<String, TranxInfo>>(BufReader::new(file)) {
            txmtmp = dt;
        }
    }
    let mut fd_tx_mt = HashMap::<String, HashMap<String, Vec<MeterInfo>>>::new();
    for (_, trx) in &txmtmp {
        if trx.trans_feed.len()<5 { continue; }
        let fdid = trx.trans_feed[0..5].to_string();
        if let Some(fd_tx) = fd_tx_mt.get_mut(&fdid) {
            fd_tx.insert(trx.trans_id.to_string(), trx.meters.clone());
        } else {
            let mut tx_mt = HashMap::<String,Vec<MeterInfo>>::new();
            tx_mt.insert(trx.trans_id.to_string(), trx.meters.clone());
            fd_tx_mt.insert(fdid.to_string(), tx_mt);
            println!("fd: {}", trx.trans_feed);
        }
    }
    println!("TX: {}", txmtmp.len());
    for (fd, tx_mt) in fd_tx_mt {
        if fd.len()<5 { continue;}
        let fd = fd[0..5].to_string();
        let fd_dir = format!("{}/mvfd/{}", crate::sg::imp::data_dir(), fd);
        let _ = std::fs::create_dir_all(&fd_dir);
        let fd_file = format!("{}/p65_tx_mt_{}.bin", fd_dir, fd);
        if let Ok(ser) = bincode::serialize(&tx_mt) {
            std::fs::write(fd_file.clone(), ser).unwrap();
            //let ld = ld_p62_fd_trans(fd);
            let fd = ld_p65_fd_tr_mt(&fd);
            println!("write {} = {}", fd_file, fd.len());
        }
    }
    Ok(())
}

pub fn ld_p65_fd_tr_mt(fd: &String) -> HashMap<String, Vec<MeterInfo>> {
    let fd_file = format!("{}/mvfd/{}/p65_tx_mt_{}.bin", crate::sg::imp::data_dir(), fd, fd);
    if let Ok(f) = File::open(crate::sg::ldp::res(&fd_file)) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, HashMap<String, Vec<MeterInfo>>>(BufReader::new(f)) {
            return dt;
        }
    }
    HashMap::<String, Vec<MeterInfo>>::new()
}


//pub fn sub_inf() -> &'static HashMap<String, SubstInfo> {    SUB_INF.get_or_init(sub_inf_init) }
//fn sub_inf_init() -> HashMap<String, SubstInfo> {    ld_p3_sub_inf() }

use crate::sg::prc3::ld_p3_sub_inf;
pub async fn prc66() -> Result<(), Box<dyn std::error::Error>> {
    let subinf = ld_p3_sub_inf();
    let mut trn = 0;
    let mut mtn = 0;
    for (sbid, sbin) in subinf {
        println!("{}", sbid);
        for fdid in &sbin.feeders {
            let tr = ld_p65_fd_tr_mt(fdid);
            let mt = ld_p64_mt_mp(fdid);
            trn += tr.len();
            mtn += mt.len();
            println!("{} tr:{} mt:{}", fdid, tr.len(), mt.len());
        }
    }
    println!("{} {}", trn, mtn);
    Ok(())
}

#[allow(dead_code)]
pub async fn prc66a() -> Result<(), Box<dyn std::error::Error>> {
    let subinf = ld_p3_sub_inf();
    let mut trn = 0;
    let mut mtn = 0;
    for (sbid, sbin) in subinf {
        println!("{}", sbid);
        for fdid in &sbin.feeders {
            let tr = ld_p65_fd_tr_mt(fdid);
            let mt = ld_p64_mt_mp(fdid);
            trn += tr.len();
            mtn += mt.len();
            println!("{} tr:{} mt:{}", fdid, tr.len(), mt.len());
        }
    }
    println!("{} {}", trn, mtn);
    Ok(())
}

pub async fn prc67() -> Result<(), Box<dyn std::error::Error>> {
    let ly = "DS_LowVoltageMeter";
    let mut mt_lo_mp = HashMap::<String, (f64,f64)>::new();
    for r in ar_list() {
        let dbf = format!("{}/{}_{}.db", db2_dir(), r, ly);
        let pnf = format!("{}/{}_{}.pn", db2_dir(), r, ly);
        //println!("{}, {}", dbf, rgf);
        let mut dbs = Vec::<HashMap::<String, DbfVal>>::new();
        let mut pns = Vec::<(f64,f64)>::new();
        if let Ok(f) = File::open(dbf) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfVal>>>(BufReader::new(f)) {
                dbs = dt;
            }
        }
        if let Ok(f) = File::open(pnf) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<(f64,f64)>>(BufReader::new(f)) {
                pns = dt;
            }
        }
        if dbs.len()!=pns.len() { println!("ERROR {} {}", dbs.len(), pns.len()); continue; }
        println!("{} {}", dbs.len(), pns.len());
        for i in 0..dbs.len() {
            let db = &dbs[i];
            if let Some(Character(Some(mtid))) = db.get("PEANO") {
                mt_lo_mp.insert(mtid.to_string(), (pns[i].0, pns[i].1));
            }
        }
    }
    let fd_file = format!("{}/p67_mt_lo_mp.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&mt_lo_mp) {
        std::fs::write(fd_file.clone(), ser).unwrap();
        let fd = ld_p67_mt_lo_mp();
        println!("write {} = {}", fd_file, fd.len());
    }

    Ok(())
}

pub fn ld_p67_mt_lo_mp() -> HashMap::<String, (f64,f64)> {
    let fd_file = format!("{}/p67_mt_lo_mp.bin", crate::sg::imp::data_dir());
    if let Ok(f) = File::open(crate::sg::ldp::res(&fd_file)) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, HashMap::<String, (f64,f64)>>(BufReader::new(f)) {
            return dt;
        }
    }
    HashMap::<String, (f64,f64)>::new()
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Prc6MeterInfo2 {
    pub meter_id: String,
    pub meter_phase: String,
    pub meter_office: String,
    pub e5: f32,
    pub e2: f32,
    pub x: f32,
    pub y: f32,
}
    
pub async fn prc68() -> Result<(), Box<dyn std::error::Error>> {
    let mut txmtmp = HashMap::<String, TranxInfo>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("txmtmp.bin")) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, HashMap<String, TranxInfo>>(BufReader::new(file)) {
            txmtmp = dt;
        }
    }
    println!("LOAD TXMT");
    let mt_lo_mp = ld_p67_mt_lo_mp();
    let mut mt202405 = HashMap::<String,f64>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("mt-202405.bin")) {
        let rd = BufReader::new(file);
        if let Ok(mt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, f64>>(rd) {
            mt202405 = mt;
        }
    }
    println!("LOAD E5");
    let mut mt202402 = HashMap::<String,f64>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("mt-202402.bin")) {
        if let Ok(mt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, f64>>(BufReader::new(file)) {
            mt202402 = mt;
        }
    }
    println!("LOAD E2");

    let subinf = ld_p3_sub_inf();
    let mut trn = 0;
    let mut mtn = 0;
    let /*mut*/ _mtl = 0;
    for (sbid, sbin) in subinf {
        println!("{}", sbid);
        for fdid in &sbin.feeders {
            let trs = ld_p62_fd_trans(&fdid);
            if trs.len()==0 { continue; }
            trn += trs.len();
            let mut fd_mt_mp = HashMap::<String,Prc6MeterInfo2>::new();
            for tr in &trs {
                if let Some(txif) = txmtmp.get(&tr.tx_id) {
                    mtn += txif.meters.len();
                    for mt in &txif.meters {
                        let id = &mt.meter_id;
                        let mut mtif = Prc6MeterInfo2::default();
                        mtif.meter_id = id.to_string();
                        mtif.meter_phase = mt.meter_phase.to_string();
                        mtif.meter_office = mt.meter_office.to_string();
                        if let Some(lo) = mt_lo_mp.get(id) {
                            mtif.x = lo.0 as f32;
                            mtif.y = lo.1 as f32;
                        }
                        if let Some(e5) = mt202405.get(id) {
                            mtif.e5 = *e5 as f32;
                        }
                        if let Some(e2) = mt202402.get(id) {
                            mtif.e2 = *e2 as f32;
                        }
                        fd_mt_mp.insert(id.to_string(), mtif);
                    }
                }
            }
            println!("{} tr:{} mt:{}", fdid, trs.len(), fd_mt_mp.len());
            let fd = fdid.to_string();
            let fd_dir = format!("{}/mvfd/{}", crate::sg::imp::data_dir(), fd);
            let _ = std::fs::create_dir_all(&fd_dir);
            let fd_file = format!("{}/p68_mt_mp_{}.bin", fd_dir, fd);
            if let Ok(ser) = bincode::serialize(&fd_mt_mp) {
                std::fs::write(fd_file.clone(), ser).unwrap();
                //let ld = ld_p62_fd_trans(fd);
                let fd = ld_p68_mt_mp(&fd);
                println!("write {} = {}", fd_file, fd.len());
            }
        }
    }
    println!("{} {}", trn, mtn);
    Ok(())
}

pub fn ld_p68_mt_mp(fd: &String) -> HashMap::<String,Prc6MeterInfo2> {
    let fd_file = format!("{}/mvfd/{}/p68_mt_mp_{}.bin", crate::sg::imp::data_dir(), fd, fd);
    if let Ok(f) = File::open(crate::sg::ldp::res(&fd_file)) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, HashMap::<String,Prc6MeterInfo2>>(BufReader::new(f)) {
            return dt;
        }
    }
    HashMap::<String,Prc6MeterInfo2>::new()
}

pub async fn prc69() -> Result<(), Box<dyn std::error::Error>> {
    let mt_lo_mp = ld_p67_mt_lo_mp();
    println!("lo {}", mt_lo_mp.len());
    for (id,_lo) in mt_lo_mp {
        println!("{}", id);
        break;
    }
    Ok(())
}

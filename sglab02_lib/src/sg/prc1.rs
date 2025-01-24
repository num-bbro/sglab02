use crate::sg::ldp::base;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use crate::sg::imp::SubInfo;
use crate::sg::wk4::Wk4Proc;
use crate::sg::wk4::Substation as Wk4Substation;
use crate::sg::wk5::Substation as Wk5Substation;
use crate::sg::wk5::FeederLoad;
use crate::sg::dcl::LoadProfVal;
use regex::Regex;
use crate::sg::wk4::YearLoad;
use crate::sg::wk4::DayLoad;
//use crate::sg::ldp::FeederTranxInfo;
use crate::sg::ldp::FeederTranx;
use crate::sg::ldp::TranxInfo;
use std::collections::HashSet;
use chrono::NaiveDateTime;
use crate::sg::imp::SPPInfo;
use crate::sg::imp::VSPPInfo;
use crate::sg::imp::SPPLoadProfile;
use crate::sg::imp::VSPPLoadProfile;

pub async fn proc1() -> Result<(), Box<dyn std::error::Error>> {
    let mut sbpvmp = HashMap::<String, String>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("sbpvmp.bin")) {
        let rd = BufReader::new(file);
        if let Ok(sbpv) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, String>>(rd) {
            sbpvmp = sbpv;
        }
    }
    let mut subxls = HashMap::<String,SubInfo>::new();
    if let Ok(fsbinfo) = File::open(format!("{}/sub_xlsx.bin", crate::sg::imp::data_dir())) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(sub) = bincode::deserialize_from::<BufReader<File>,HashMap::<String,SubInfo>>(rsbinfo) {
            subxls = sub;
        }
    }
    //let mut cn = 0;
    for (k,v) in subxls.into_iter() {
        //cn += 1;
        if let Some(_xx) = sbpvmp.get(&k) {
        } else {
            println!("'{}' == [name={} volt-{} cate-{} egat-{}]", k, v.name, v.volt, v.cate, v.egat);
        }
    }
    Ok(())
}

pub async fn proc2() -> Result<(), Box<dyn std::error::Error>> {
    let mut sbpvmp = HashMap::<String, String>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("sbpvmp.bin")) {
        let rd = BufReader::new(file);
        if let Ok(sbpv) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, String>>(rd) {
            sbpvmp = sbpv;
        }
    }
    if let Some(x) = sbpvmp.get("PGA") {
        println!("PGA: {}", x);
    } else {
        println!("NO PGA");
    }
    if let Some(x) = sbpvmp.get("PGU") {
        println!("PGU: {}", x);
    } else {
        println!("NO PGU");
    }
    if let Some(x) = sbpvmp.get("KNA") {
        println!("KNA: {}", x);
    } else {
        println!("NO KNA");
    }
    if let Some(x) = sbpvmp.get("KNU") {
        println!("KNU: {}", x);
    } else {
        println!("NO KNU");
    }
    Ok(())
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SubstInfo {
    pub sbid: String,
    pub name: String,
    pub enam: String,
    pub area: String,
    pub arid: String,
    pub volt: String,
    pub cate: String,
    pub egat: String,
    pub state: String,
    pub conf: String,
    pub trax: String,
    pub mvax: String,
    pub feed: String,
    pub feno: usize,
    pub feeders: Vec<String>,
    pub trxn: usize,
    pub mvxn: i32,
    pub prov: String,
}

pub async fn proc3() -> Result<(), Box<dyn std::error::Error>> {
    let mut sbpvmp = HashMap::<String, String>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("sbpvmp.bin")) {
        let rd = BufReader::new(file);
        if let Ok(sbpv) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, String>>(rd) {
            sbpvmp = sbpv;
        }
    }
    let pvad = [
        ("MKU","มหาสารคาม"),
        ("SMZ","สมุทรสาคร"),
        ("TPV","สระแก้ว"),
        ("NPW","นครปฐม"),
        ("UBV","อุบลราชธานี"),
        ("MTU","เชียงใหม่"),
        ("BLU","สงขลา"),
        ("RIV","ตราด"),
        ("RSB","ปทุมธานี"),
        ("KSU","สุรินทร์"),
        ("QSA","สุราษฎร์ธานี"),
        ("HPA","พิจิตร"),
        ("KNU","นครศรีธรรมราช"),
    ];
    for (s,p) in pvad {
        sbpvmp.insert(s.to_string(), p.to_string());
    }
    let mut arabmp = HashMap::<String,String>::new();
    let armp = [
        ("กฟก.1", "C1"),
        ("กฟก.2", "C2"),
        ("กฟก.3", "C3"),
        ("กฟน.1", "N1"),
        ("กฟน.2", "N2"),
        ("กฟน.3", "N3"),
        ("กฟฉ.1", "NE1"),
        ("กฟฉ.2", "NE2"),
        ("กฟฉ.3", "NE3"),
        ("กฟต.1", "S1"),
        ("กฟต.2", "S2"),
        ("กฟต.3", "S3"),
    ];
    for (n,a) in armp {
        arabmp.insert(n.to_string(), a.to_string());
    }
    let mut subxls = HashMap::<String,SubInfo>::new();
    if let Ok(fsbinfo) = File::open(format!("{}/sub_xlsx.bin", crate::sg::imp::data_dir())) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(sub) = bincode::deserialize_from::<BufReader<File>,HashMap::<String,SubInfo>>(rsbinfo) {
            subxls = sub;
        }
    }

    let mut sbenm = HashMap::<String,String>::new();
    if let Ok(fsbinfo) = File::open(format!("{}/sub_xml.bin", crate::sg::imp::data_dir())) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(sub) = bincode::deserialize_from::<BufReader<File>,Vec::<HashMap::<String,String>>>(rsbinfo) {
            let tgid = "cim:IdentifiedObject.description".to_string();
            let tgnm = "cim:IdentifiedObject.name".to_string();
            for sb in sub {
                if let (Some(sbid),Some(sbnm)) = (sb.get(&tgid),sb.get(&tgnm)) {
                    sbenm.insert(sbid.to_string(), sbnm.to_string());
                }
            }
        }
    }

    let mut subinfo = Vec::<SubstInfo>::new();
    for (_k,sb) in subxls {
        let sbid = sb.sbid.to_string();
        if sbid.len()!=3 { continue; }
        let name = sb.name.to_string();
        let area = sb.area.to_string();
        let volt = sb.volt.to_string();
        let cate = sb.cate.to_string();
        let egat = sb.egat.to_string();
        let state = sb.state.to_string();
        let conf = sb.conf.to_string();
        let trax = sb.trax.to_string();
        let mvax = sb.mvax.to_string();
        let feed = sb.feed.to_string();
        let mut enam = "?".to_string();
        let mut arid = "?".to_string();
        let mut feno = 0usize;
        let mut trxn = 0usize;
        let mut mvxn = 0i32;
        let mut prov = "?".to_string();
        if let Some(pv) = sbpvmp.get(&sbid) {
            prov = pv.to_string();
        }
        if let Some(en) = sbenm.get(&sbid) {
            enam = en.to_string();
        }
        if let Ok(fd) = sb.feed.to_string().parse::<usize>() {
            feno = fd;
        }
        //if feno==0 { continue; }
        if let Ok(tr) = trax.parse::<usize>() {
            trxn = tr;
        }
        //if trxn==0 { continue; }
        if let Ok(mv) = mvax.parse::<i32>() {
            mvxn = mv;
        }
        //if mvxn==0 { continue; }
        if let Some(ab) = arabmp.get(&sb.area) {
            arid = ab.to_string();
        }
        //if arid=="?" { continue; }
        if enam=="?" {
            println!("sbid: {} {} - {}", sbid, name, enam);
        }
        let mut feeders = Vec::<String>::new();
        for i in 1..=feno {
            let fdid = format!("{}{:02?}",sbid,i);
            feeders.push(fdid);
        }
        let sbif = SubstInfo {sbid,name,area,volt,cate,egat,state,conf,trax,mvax,feed,enam,arid,feno,trxn,mvxn,feeders,prov};
        subinfo.push(sbif)        
    }
    let sbinfo = format!("{}/subinfo.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&subinfo) {
        std::fs::write(sbinfo, ser).unwrap();
    }
    Ok(())
}

#[derive(Debug,Serialize,Deserialize,Default)]
pub struct PEAGrid {
    pub prvs: Vec<String>,
    pub prv_sub: HashMap<String,Vec<String>>,
    pub sub_inf: HashMap<String, SubstInfo>,
    pub sub_ldp: HashMap<String, Wk5Substation>,
    //feed_ldp: HashMap::<String, FeederLoad>,
}

// PEA Grid
pub async fn proc4() -> Result<(), Box<dyn std::error::Error>> {

    let mut subinf = Vec::<SubstInfo>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("subinfo.bin")) {
        let rd = BufReader::new(file);
        if let Ok(sbif)=bincode::deserialize_from::<BufReader<File>, Vec::<SubstInfo>>(rd) {
            subinf = sbif;
        }
    }
    println!("read sub info");

    let mut prvs = Vec::<String>::new();
    let mut prv_sub = HashMap::<String,Vec<String>>::new();
    let mut sub_inf = HashMap::<String, SubstInfo>::new();
    let mut sub_ldp = HashMap::<String, Wk5Substation>::new();
    let mut sbids = HashMap::<String,i32>::new();
    //let mut cn = 0;
    while let Some(sb) = subinf.pop() {
        sbids.insert(sb.sbid.to_string(), 1);
        if let Some(sbv) = prv_sub.get_mut(&sb.prov) {
            sbv.push(sb.sbid.to_string());
            sbv.sort();
        } else {
            //cn += 1;
            //println!("{}.{}-{}", cn, sb.prov, sb.sbid);
            let sbv = vec![sb.sbid.to_string()];
            prv_sub.insert(sb.prov.to_string(), sbv);
            prvs.push(sb.prov.to_string());
        }
        if let Some(si) = sub_inf.get(&sb.sbid) {
            println!("repeted {}", si.sbid);
        } else {
            sub_inf.insert(sb.sbid.to_string(), sb);
        }
    }
    prvs.sort();

    let mut subldp = Vec::<Wk4Substation>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("wk4prc.bin")) {
        let rd = BufReader::new(file);
        if let Ok(wk4prc) = bincode::deserialize_from::<BufReader<File>, Wk4Proc>(rd) {
            subldp = wk4prc.ssv;
        }
    }
    println!("read load profile");
    
    let mut sbldp = HashMap::<String,i32>::new();
    for sb in &subldp { sbldp.insert(sb.sbst.to_string(), 1); }

    let mut cn = 0;
    println!("==== NO IN LOAD PROFILE ====");
    for (s,_) in &sub_inf {
        if let Some(_s) = sbldp.get(s) {
        } else {
            cn += 1;
            println!("{}. {}", cn, s);
        }
    }

    let mut cn = 0;
    println!("==== NO IN SUBST LIST ====");
    for sb in &subldp {
        if let Some(_s) = sbids.get(&sb.sbst) {
        } else {
            cn += 1;
            println!("{}. {}", cn, sb.sbst);
        }
    }

    let cfg = base().config.clone();
    let cfg = cfg.read().await;
    let stw = cfg.criteria.solar_time_window;

    //let mut txno = 0;
    //let mut subldp1 = Vec::<Wk5Substation>::new();
    //let (mut sum1, mut _sum2) = (0.0, 0.0);
    for ss in &mut subldp {
        //sum1 += ss.year_load.power_quality.pos_energy;
        //sum2 += ss.last_year_load.power_quality.pos_energy;

        let mut ss2 = Wk5Substation::default();
        ss2.ssid = ss.sbst.to_string();
        ss2.prov = ss.prov.to_string();
        ss2.name = ss.name.to_string();
        ss2.last_year = ss.last_year;
        ss2.year_load = ss.year_load.clone();

        //let mut feed_ldp = HashMap::<String, FeederLoad>::new();
        let mut fdmp = HashMap::<String, FeederLoad>::new();

        //let re = Regex::new(r"..._[0-9][0-9][VW]B01.+").unwrap();
        let re = Regex::new(r"..._[0-9][0-9][VW]B01.*").unwrap();
        //let mut fdmp = HashMap::<String, FeederLoad>::new();

        //println!("{}: {}", ss.sbst, ss.feeders.len());
        let mut cn = 0;
        for fd in &ss.feeders {
            cn += 1;
            let fdid2 = fd.feed[4..6].to_string();
            /*
            let mut fdno = 0;
            if let Ok(no) = fdid2.parse::<i32>() {
                fdno = no;
            }
            */
            let fdid = format!("{}{}", ss2.ssid, fdid2);
            if re.is_match(&fd.feed) == false { continue }
            //println!("  {}.{}->{} : {}", cn, fd.feed, fdid, re.is_match(&fd.feed));
            if let Some(_fd2) = fdmp.get_mut(&fdid) {
                println!("======================  {}.{}->{} : {}", cn, fd.feed, fdid, re.is_match(&fd.feed));
            } else {
                let mut fd2 = FeederLoad::default();
                fd2.ssid = ss.sbst.to_string();
                fd2.fdid = fd.feed.to_string();
                fd2.fdid5 = fdid.to_string();
                fd2.prov = ss.prov.to_string();
                //fd2.trans = fd.trans.clone();
                for tx in &fd.trans {
                    fd2.tx.tx_no += 1;
                    //txno += 1;
                    if tx.tx_own == "P" {
                        fd2.tx.tx_pea += 1;
                    } else {
                        fd2.tx.tx_cus += 1;
                    }
                    fd2.tx.mt1_no += tx.mt_1_ph;
                    fd2.tx.mt3_no += tx.mt_3_ph;
                }
                fd2.year_load = fd.year_load.clone();
                fd2.last_year_load = fd.last_year_load.clone();
                for di in 0..fd.year_load.loads.len() {
                    for hi in 0..fd.year_load.loads[di].load.len() {
                        match fd.year_load.loads[di].load[hi] {
                            LoadProfVal::Value(_y) => {}
                            _ => {
                                fd2.year_load.loads[di].load[hi] = LoadProfVal::Value(0.0)
                            }
                        }
                    }
                }
                fdmp.insert(fdid, fd2);
            };
        }
        let mut fds: Vec<Box<FeederLoad>> = fdmp.into_iter().map(|(_k, v)| Box::new(v)).collect();
        fds.sort_by(|a, b| a.fdid5.partial_cmp(&b.fdid5).unwrap());
        ss2.feeders = fds;
        //ss2.a();

        sub_ldp.insert(ss2.ssid.to_string(), ss2);
        //subldp1.push(ss2);
    }

    sub_ldp_calc(&mut sub_ldp).await;
    sub_ldp_calc_last(&mut sub_ldp).await;
    power_quality(&mut sub_ldp, stw).await;
    let _ = calc_trans(&mut sub_ldp).await;

    let prvs0 = prvs.clone();
    let prv_sub0 = prv_sub.clone();
    let sub_inf0 = sub_inf.clone();

    let peagrd = PEAGrid { prvs, prv_sub, sub_inf, sub_ldp, };
    let file = format!("{}/peagrd4.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&peagrd) {
        std::fs::write(file, ser).unwrap();
    }

    let prvs = prvs0;
    let prv_sub = prv_sub0;
    let sub_inf = sub_inf0;
    let sub_ldp = HashMap::<String, Wk5Substation>::new();
    let peagrd = PEAGrid { prvs, prv_sub, sub_inf, sub_ldp, };
    let file = format!("{}/peagrd4a.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&peagrd) {
        std::fs::write(file, ser).unwrap();
    }

    Ok(())
}

pub async fn calc_trans(sub_ldp: &mut HashMap<String,Wk5Substation>) -> Result<(), Box<dyn std::error::Error>> {

    let mut fdtxmp = HashMap::<String, Vec<FeederTranx>>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("fdtxmp.bin")) {
        let rd = BufReader::new(file);
        if let Ok(fdtxmp0) = bincode::deserialize_from::<BufReader<File>, HashMap::<String, Vec<FeederTranx>>>(rd) {
            fdtxmp = fdtxmp0;
        }
    }
    //let (mut txn, mut mtn, mut fdn, mut efn, mut txo, mut mto, mut _fdo) = (0, 0, 0, 0, 0, 0, 0);
    //let re = Regex::new(r"..._[0-9][0-9][VWB].+").unwrap();
    let mut fdcnt: HashMap<String, i32> = fdtxmp.iter().map(|(k, _v)| (k.to_string(), 0)).collect();
    let mut fd_keys = HashSet::<String>::new();
    for (k, ss) in sub_ldp {
        for fd in &mut ss.feeders {
            let pw1 = fd.year_load.power_quality.pos_energy;
            let pw0 = fd.last_year_load.power_quality.pos_energy;
            /*
            let fd0 = &fd.feed[0..6];
            let fd0 = fd0.to_string();
            if pw == 0f32 {
                continue;
            }
            */
            fd.trans.clear();
            if let Some(txfv) = fdtxmp.get(&fd.fdid5) {
                if let Some(c) = fdcnt.get_mut(&fd.fdid5) {
                    *c += 1;
                }
                if let Some(_fd) = fd_keys.get(&fd.fdid5) {
                    //txo += txfv.len();
                    for _tx in txfv {
                        //mto += (tx.mt_1_ph + tx.mt_3_ph);
                    }
                    //fdo += 1;
                } else {
                    //txn += txfv.len();
                    for _tx in txfv {
                        //mtn += (tx.mt_1_ph + tx.mt_3_ph);
                    }
                    //fdn += 1;
                    fd_keys.insert(fd.fdid5.to_string());
                }
                println!("{} {} {} en:{} {}", k, fd.fdid5, txfv.len(), pw1, pw0);
                fd.trans.append(&mut txfv.clone());
            } else {
                //efn += 1;
            }
        }
    }
    /*
    print!(
        "tx:{} mt:{} fd:{}, to: tx:{} mt:{} fd:{} ef:{}\n",
        txn, mtn, fdn, txo, mto, fdo, efn
    );
    let mut bnk = vec![];
    for (k, v) in fdcnt {
        if v == 0 {
            if let Some(txfv) = fd_tx_info.fdtxmp.get(&k) {
                bnk.push((k.to_string(), txfv.clone()));
            }
        }
    }
    bnk.sort_by(|a, b| a.0.cmp(&b.0));
    for (i, v) in bnk.iter().enumerate() {
        print!(" {}.{} - {}\n", i, v.0, v.1.len());
    }
    */
    Ok(())
}

pub async fn power_quality(sub_ldp: &mut HashMap<String, Wk5Substation>, stw: f32) {
    for (_k, ss) in sub_ldp {
        for fd in &mut ss.feeders {
            fd.year_load.power(stw).await;
            fd.last_year_load.power(stw).await;
        }
        ss.year_load.power(stw).await;
        ss.last_year_load.power(stw).await;
    }
}

pub async fn sub_ldp_calc(sub_ldp: &mut HashMap<String, Wk5Substation>) {
    for (_k, ss) in sub_ldp {
        let mut ss_val = [0f32; 365 * 48];
        for fd in &mut ss.feeders {
            for (di, dl) in fd.year_load.loads.iter().enumerate() {
                for (hi, hl) in dl.load.iter().enumerate() {
                    let ii = di * 48 + hi;
                    if let LoadProfVal::Value(v) = hl {
                        ss_val[ii] += v;
                    } else {
                        print!("ERR {} {} {}\n", fd.fdid, di, hi);
                    }
                }
            }
        }
        ss.year_load = YearLoad::default();
        for di in 0..365 {
            let mut day_load = DayLoad::default();
            day_load.day = di + 1;
            for hi in 0..48 {
                let ii = di * 48 + hi;
                day_load.load.push(LoadProfVal::Value(ss_val[ii]));
            }
            ss.year_load.loads.push(day_load);
        }
    }
}

pub async fn sub_ldp_calc_last(sub_ldp: &mut HashMap<String, Wk5Substation>) {
    for (_k, ss) in sub_ldp {
        let mut ss_val = [0f32; 365 * 48];
        for fd in &mut ss.feeders {
            for (di, dl) in fd.last_year_load.loads.iter().enumerate() {
                for (hi, hl) in dl.load.iter().enumerate() {
                    let ii = di * 48 + hi;
                    if let LoadProfVal::Value(v) = hl {
                        ss_val[ii] += v;
                    } else {
                        print!("ERR {} {} {}\n", fd.fdid, di, hi);
                    }
                }
            }
        }
        ss.last_year_load = YearLoad::default();
        for di in 0..365 {
            let mut day_load = DayLoad::default();
            day_load.day = di + 1;
            for hi in 0..48 {
                let ii = di * 48 + hi;
                day_load.load.push(LoadProfVal::Value(ss_val[ii]));
            }
            ss.last_year_load.loads.push(day_load);
        }
    }
}

// meter
pub async fn proc5() -> Result<(), Box<dyn std::error::Error>> {
    //let base = base();
    let /*mut*/ _cn = 0;
    if let Ok(file) = File::open(crate::sg::ldp::res("txmtmp.bin")) {
        let rd = BufReader::new(file);
        if let Ok(txmtmp) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, TranxInfo>>(rd) {
            let mut fdtxmp = HashMap::<String, Vec<FeederTranx>>::new();
            for (_k, tx) in txmtmp {            
                if tx.trans_feed.len() < 5 {
                    continue;
                }
                let fd0 = &tx.trans_feed[3..5];
                let fdid = format!("{}{}", tx.trans_sub, fd0);

                let tx_id = tx.trans_id.to_string();
                let tx_power = tx.trans_power;
                let tx_own = tx.trans_own;
                let (mut mt_ph_a, mut mt_ph_b, mut mt_ph_c, mut mt_1_ph, mut mt_3_ph, mut mt_else) =
                    (0, 0, 0, 0, 0, 0);
                for mt in &tx.meters {
                    if mt.meter_phase == "A" {
                        mt_ph_a += 1;
                        mt_1_ph += 1;
                    } else if mt.meter_phase == "B" {
                        mt_ph_b += 1;
                        mt_1_ph += 1;
                    } else if mt.meter_phase == "C" {
                        mt_ph_c += 1;
                        mt_1_ph += 1;
                    } else if mt.meter_phase == "ABC" {
                        mt_3_ph += 1;
                    } else {
                        mt_else += 1;
                    }
                }
                let fdtx = FeederTranx {
                    tx_id,
                    fd_id: tx.trans_feed.to_string(),
                    tx_power,
                    tx_own,
                    mt_ph_a,
                    mt_ph_b,
                    mt_ph_c,
                    mt_1_ph,
                    mt_3_ph,
                    mt_else,
                };
                if let Some(v) = fdtxmp.get_mut(&fdid) {
                    v.push(fdtx);
                } else {
                    //print!("FD {} {} {}\n", tx.trans_sub, &tx.trans_feed, fd0);
                    fdtxmp.insert(fdid, vec![fdtx]);
                }
            }
            let file = format!("{}/fdtxmp.bin", crate::sg::imp::data_dir());
            if let Ok(ser) = bincode::serialize(&fdtxmp) {
                std::fs::write(file, ser).unwrap();
            }
        } // read txmtmp.bin
    } // end open file
    Ok(())
}

pub fn grp1() -> [&'static str;25] {
[
"ระยอง",
"ชลบุรี",
"กระบี่",
"สระแก้ว",
"พระนครศรีอยุธยา",
"ฉะเชิงเทรา",
"สมุทรสาคร",
"ปทุมธานี",
"บุรีรัมย์",
"ปราจีนบุรี",
"เพชรบุรี",
"เชียงใหม่",
"สระบุรี",
"พิษณุโลก",
"ราชบุรี",
"ขอนแก่น",
"นครปฐม",
"สงขลา",
"สุราษฎร์ธานี",
"นครสวรรค์",
"นครราชสีมา",
"ลพบุรี",
"ภูเก็ต",
"ระนอง",
"สมุทรสงคราม",]
}

// outage
pub async fn proc6() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = base().config.clone();
    let cfg = cfg.read().await;
    let _stw = cfg.criteria.solar_time_window;
    let _prv: HashSet<&str> = grp1().into_iter().collect();
    if let Ok(file) = File::open(crate::sg::ldp::res("peagrd4.bin")) {
        let rd = BufReader::new(file);
        if let Ok(mut grd)=bincode::deserialize_from::<BufReader<File>, PEAGrid>(rd) {
            println!("prv:{}", grd.prvs.len());
            /*
            for (k,s) in &grd.prv_sub {
                let pp = k.as_str();
                if let Some(sx) = prv.get(pp) { } else { continue; }
                let (mut pw1, mut pw0) = (0f32,0f32);
                for si in s {
                    if let Some(ss) = grd.sub_ldp.get(si) {
                        pw1 += ss.year_load.power_quality.pos_energy;
                        pw0 += ss.last_year_load.power_quality.pos_energy;
                    }
                }
                println!("{}|{}|{}", k, pw1, pw0);
            }
            */
            println!("goto outage");
            outage(&mut grd).await?;
            /*
            println!("prv:{}", grd.prvs.len());
            println!("inf:{}", grd.sub_inf.len());
            println!("ldp:{}", grd.sub_ldp.len());
            for (k,ss) in &mut grd.sub_ldp {
                println!("sb:{}-{}", k, ss.feeders.len());
            }
            */
            /*
            println!("fdl:{}", grd.feed_ldp.len());
            for (k,fd) in &mut grd.feed_ldp {
                println!("fd:{}", k);
                fd.year_load.power(stw).await;
            }
                */
        }
    }
    Ok(())
}

pub async fn outage(grd: &mut PEAGrid) -> Result<(), Box<dyn std::error::Error>> {
    let mut ssfdot = HashMap::<String, HashMap<String, Vec<(String, String, String)>>>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("sbfdot.bin")) {
        let rd = BufReader::new(file);
        if let Ok(ssfd) = bincode::deserialize_from::<
            BufReader<File>,
            HashMap<String, HashMap<String, Vec<(String, String, String)>>>, >(rd) {
            ssfdot = ssfd;
        }
    }
    let tp0 = "ไฟฟ้าขัดข้อง";
    let fm0 = "%d-%m-%Y %H:%M:%S";
    for pv in grp1() {
        let mut ot00 = 0;
        let mut otcn = 0;
        if let Some(sbv) = grd.prv_sub.get(pv) {
            for sb in sbv {
                if let Some(ss) = grd.sub_ldp.get(sb) {
                    for fd in &ss.feeders {
                        if let Some(ssot) = ssfdot.get(&ss.ssid) {
                            if let Some(fdot) = ssot.get(&fd.fdid5) {
                                otcn += fdot.len();
                                for (st, ed, tp) in fdot {
                                    if tp == tp0 {
                                        let dttm1 = NaiveDateTime::parse_from_str(st.as_str(), fm0).unwrap();
                                        let dttm2 = NaiveDateTime::parse_from_str(ed.as_str(), fm0).unwrap();
                                        //ot00 += dttm2.timestamp_millis() - dttm1.timestamp_millis();
                                        ot00 += dttm2.and_utc().timestamp_millis() - dttm1.and_utc().timestamp_millis();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let otd = ot00 as f64;
        let min = otd / (otcn as f64 * 1000f64 * 60f64);
        println!("{}|{}|{}", pv, otcn, min);
    }
    Ok(())
}

pub async fn proc7() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = base().config.clone();
    let cfg = cfg.read().await;
    let _stw = cfg.criteria.solar_time_window;
    let _prv: HashSet<&str> = grp1().into_iter().collect();
    if let Ok(file) = File::open(crate::sg::ldp::res("peagrd4.bin")) {
        let rd = BufReader::new(file);
        if let Ok(/*mut*/ grd)=bincode::deserialize_from::<BufReader<File>, PEAGrid>(rd) {
            println!("prv:{}", grd.prvs.len());
            for (k,s) in &grd.prv_sub {
                let _pp = k.as_str();
                let (mut pw1, mut pw0) = (0f32,0f32);
                let (mut nw1, mut nw0) = (0f32,0f32);
                for si in s {
                    if let Some(ss) = grd.sub_ldp.get(si) {
                        nw1 += ss.year_load.power_quality.neg_energy;
                        nw0 += ss.last_year_load.power_quality.neg_energy;
                        pw1 += ss.year_load.power_quality.pos_energy;
                        pw0 += ss.last_year_load.power_quality.pos_energy;
                    }
                }
                println!("{}|{}|{}|{}|{}", k, pw1, pw0, nw1, nw0);
            }
        }
    }
    Ok(())
}

#[derive(Debug,Serialize,Deserialize)]
pub struct GridState {
    prov: String,
    ssid: String,
    fdid: String,
    pe22: f64,
    pe21: f64,
    ne22: f64,
    ne21: f64,
}

#[derive(Debug,Serialize,Deserialize,Clone,Default)]
pub struct SPPConn {
    pub ppid: String,
    pub sbid: String,
    pub sbi2: String,
    pub fdid: String,
    pub ppif: SPPInfo,
    pub lpcn: usize,
    pub lpsm: f64,
    pub lpav: f64,
}

#[derive(Debug,Serialize,Deserialize,Clone,Default)]
pub struct VSPPConn {
    pub ppid: String,
    pub sbid: String,
    pub fdid: String,
    pub ppif: VSPPInfo,
    pub lpcn: usize,
    pub lpsm: f64,
    pub lpav: f64,
}

pub async fn proc8() -> Result<(), Box<dyn std::error::Error>> {
    let /*mut*/ _grd = PEAGrid::default();
    let mut sub_thnm = HashMap::<String,String>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("peagrd4a.bin")) {
        let rd = BufReader::new(file);
        if let Ok(/*mut*/ g)=bincode::deserialize_from::<BufReader<File>, PEAGrid>(rd) {
            for (_k,sb) in &g.sub_inf {
                sub_thnm.insert(sb.name.to_string(), sb.sbid.to_string());
            }
            //println!("{} {} {}", g.prvs.len(), g.prv_sub.len(), g.sub_inf.len());
        }
    }

    let mut sppv = Vec::<SPPConn>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("spp_info.bin")) {
        let rd = BufReader::new(file);
        if let Ok(/*mut*/ sppinf)=bincode::deserialize_from::<BufReader<File>, Vec<SPPInfo>>(rd) {
            println!("all {}", sppinf.len());
            for spp in &sppinf {
                if spp.abbr.len()!=6 { continue; }
                let /*mut*/ _s2 = String::new();
                let mut idv = Vec::<String>::new();
                let pts = spp.conn.split("-");
                for pt in pts {
                    let p1 = pt;
                    let p1 = p1.replace("Add bay","");
                    let p1 = p1.replace("(Add bay)","");
                    let p1 = p1.trim();
                    if let Some(si) = sub_thnm.get(p1) {
                        idv.push(si.to_string());
                    }
                }
                if idv.len()==0 { continue; }
                let ppid = spp.abbr.to_string();
                let sbid = idv[0].to_string();
                let mut sbi2 = "".to_string();
                if idv.len()>1 { sbi2 = idv[1].to_string(); }
                let ppif = spp.clone();
                let fdid = "".to_string();
                let spp = SPPConn { ppid, sbid, sbi2, fdid, ppif, ..Default::default()};
                sppv.push(spp);
            }
            let mut cn = 0;
            for spp in &sppv {
                cn += 1;
                println!("{}.{} {} {}", cn, spp.ppid, spp.sbid, spp.sbi2);
            }
        }
        let file = format!("{}/spp_conn.bin", crate::sg::imp::data_dir());
        if let Ok(ser) = bincode::serialize(&sppv) {
            std::fs::write(file, ser).unwrap();
        }
    }

    let mut spphs = HashMap::<String,SPPConn>::new();
    for spp in &sppv {
        spphs.insert(spp.ppid.to_string(), spp.clone());
    }
    let /*mut*/ _lpv = Vec::<SPPLoadProfile>::new();
    let yrs = ["2022", "2023"];
    let _yr = "2022";
    for yr in yrs {
        let lpf = format!("spp-{}.bin", yr);
        if let Ok(file) = File::open(crate::sg::ldp::res(&lpf)) {
            let rd = BufReader::new(file);
            if let Ok(mut lp)=bincode::deserialize_from::<BufReader<File>, Vec::<SPPLoadProfile>>(rd) {
                println!("profile {} - {}", lpf, lp.len());
                let mut en = 0;
                for vs in &mut lp {
                    let sbky = vs.substation.trim();
                    if let (Some(/*mut*/ spp),Ok(mw)) = (spphs.get_mut(sbky),vs.mw.parse::<f64>()) {
                        spp.lpcn += 1;
                        spp.lpsm += mw;
                    } else {
                        en += 1;
                    }
                }
                println!("  {}", en);
            }
        }
    }
    let mut spp_ldp: Vec<SPPConn> = spphs.into_iter().map(|(_k, v)| v).collect();
    for ssp in &mut spp_ldp { if ssp.lpcn>0 { ssp.lpav = ssp.lpsm / ssp.lpcn as f64; } }
    let file = format!("{}/spp_ldp.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&spp_ldp) {
        std::fs::write(file, ser).unwrap();
    }
    println!("finish SPP");

    let mut vsppv = Vec::<VSPPConn>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("vspp_info.bin")) {
        let rd = BufReader::new(file);
        if let Ok(/*mut*/ vsppi)=bincode::deserialize_from::<BufReader<File>, Vec<VSPPInfo>>(rd) {
            println!("all={}", vsppi.len());
            //let mut _cn = 0;
            for vsp in &vsppi {
                if vsp.ppid.len()!=6 { continue; }
                if vsp.circ.len()==0 { continue; }
                let ppid = vsp.ppid.to_string();
                if vsp.circ.len()<=3 { continue; }
                let mut fdid = vsp.circ.to_string();
                let sbid = fdid[0..3].to_string();
                if fdid.len()==4 {
                    let no = fdid[3..4].to_string();
                    if let Ok(ni) = no.parse::<usize>() {
                        fdid = format!("{}{:02}", sbid, ni);
                    } else {
                        continue;
                    }
                }
                let ppif = vsp.clone();
                let vspp = VSPPConn { ppid, sbid, fdid, ppif, ..Default::default()};
                vsppv.push(vspp);
                //cn += 1;
            }
        }
        let mut cn = 0;
        for spp in &vsppv {
            cn += 1;
            println!("{}.{} {} {}", cn, spp.ppid, spp.sbid, spp.fdid);
        }
        let file = format!("{}/vspp_conn.bin", crate::sg::imp::data_dir());
        if let Ok(ser) = bincode::serialize(&vsppv) {
            std::fs::write(file, ser).unwrap();
        }
    }
    let mut vspphs = HashMap::<String,VSPPConn>::new();
    for vspp in &vsppv {
        vspphs.insert(vspp.ppid.to_string(), vspp.clone());
    }
    let /*mut*/ _lpv = Vec::<VSPPLoadProfile>::new();
    let yrs = ["2022", "2023"];
    for yr in yrs {
        let lpf = format!("vspp-{}.bin", yr);
        println!("file: {}",lpf);
        if let Ok(file) = File::open(crate::sg::ldp::res(&lpf)) {
            let rd = BufReader::new(file);
            if let Ok(mut lp)=bincode::deserialize_from::<BufReader<File>, Vec::<VSPPLoadProfile>>(rd) {
                println!("profile {} - {}", lpf, lp.len());
                let mut en = 0;
                for vs in &mut lp {
                    let sbky = vs.plant_code.trim();
                    if let (Some(/*mut*/ vspp),Ok(mw)) = (vspphs.get_mut(sbky),vs.mw.parse::<f64>()) {
                        vspp.lpcn += 1;
                        vspp.lpsm += mw;
                    } else {
                        en += 1;
                    }
                }
                println!("  {}", en);
            }
        }
    }
    let mut vspp_ldp: Vec<VSPPConn> = vspphs.into_iter().map(|(_k, v)| v).collect();
    for ssp in &mut vspp_ldp { if ssp.lpcn>0 { ssp.lpav = ssp.lpsm / ssp.lpcn as f64; } }
    let file = format!("{}/vspp_ldp.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&vspp_ldp) {
        std::fs::write(file, ser).unwrap();
    }
    println!("finish VSPP");

    Ok(())
}

pub fn p1_spp_conn() -> Vec::<SPPConn> {
    if let Ok(f) = File::open(crate::sg::ldp::res("spp_conn.bin")) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, Vec::<SPPConn>>(BufReader::new(f)) {
            return dt;
        }
    }
    Vec::<SPPConn>::new()
}

pub fn p1_vspp_conn() -> Vec::<VSPPConn> {
    if let Ok(f) = File::open(crate::sg::ldp::res("vspp_conn.bin")) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, Vec::<VSPPConn>>(BufReader::new(f)) {
            return dt;
        }
    }
    Vec::<VSPPConn>::new()
}

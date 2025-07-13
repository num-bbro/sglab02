use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sglab02_lib::sg::dcl;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoadProfVal {
    None,
    Null,
    Value(f32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeederLoadRaw {
    pub sbst: String,
    pub name: String,
    pub feed: String,
    pub time_r: Vec<LoadProfVal>,
}

impl FeederLoadRaw {
    fn new_num_of_day(nd: usize) -> Self {
        FeederLoadRaw {
            sbst: "".to_string(),
            name: "".to_string(),
            feed: "".to_string(),
            time_r: vec![LoadProfVal::None; nd * 48],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FeederLoad {
    pub sbst: String,
    pub name: String,
    pub feed: String,
    pub time_r: Vec<LoadProfVal>,
    pub time_v: Vec<f32>,
    pub adj: DataAdjust,
    pub chk0: DataCheck,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataCheck {
    pub good: usize,
    pub null: usize,
    pub none: usize,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataAdjust {
    pub fstday: Option<usize>,
    pub adj_lead: usize,
    pub adj_one: usize,
    pub adj_fill: usize,
    pub adj_last: usize,
}

pub fn rd_lp23() {
    println!("lp 2023");
    if let Ok(file) = File::open("../sgdata/lp23.bin") {
        let sbdir = "../sgdata/lp2023";
        let _ = fs::create_dir_all(sbdir);
        let rd = BufReader::new(file);
        if let Ok((vsb, vfl)) = bincode::deserialize_from::<
            BufReader<File>,
            (Vec<String>, HashMap<String, Vec<Box<FeederLoadRaw>>>),
        >(rd)
        {
            println!("vs:{} vf:{}", vsb.len(), vfl.len());
            for (k, v) in vfl {
                let sbf = format!("{}/{}.bin", sbdir, k);
                println!("{}", sbf);
                if let Ok(ser) = bincode::serialize(&v) {
                    std::fs::write(sbf, ser).unwrap();
                }
            }
        }
    }
}

use sglab02_lib::sg::prc3::ld_p3_prv_sub;
use sglab02_lib::sg::prc3::ld_p3_sub_inf;
use sglab02_lib::sg::prc4::grp1;
use sglab02_lib::sg::prc4::ld_pv_sbv_m;

pub fn ld_lpsb(sb: &str) -> Vec<Box<FeederLoadRaw>> {
    let sbdir = "../sgdata/lp2023";
    let file = format!("{}/{}.bin", sbdir, sb);
    if let Ok(f) = File::open(&file) {
        if let Ok(dt) =
            bincode::deserialize_from::<BufReader<File>, Vec<Box<FeederLoadRaw>>>(BufReader::new(f))
        {
            return dt;
        }
    }
    Vec::<Box<FeederLoadRaw>>::new()
}

use regex::Regex;

pub fn lp23_mo1() {
    let pv = grp1();
    //let pvsb = ld_p3_prv_sub();
    let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();
    //let sbdir = "../sgdata/lp2023a";
    let re = Regex::new(r"..._[0-9][0-9][VW]B01").unwrap();
    for p in &pv {
        let pp = p.to_string();
        println!("pv:{}", p);
        //if let Some(sbv) = pvsb.get(&pp) {
        if let Some(sbv) = sbsl.get(&pp) {
            for sb in sbv {
                let lp23 = ld_lpsb(sb.sbid.as_str());
                let mut sblp = FeederLoad::default();
                let mut sbpf = Vec::<FeederLoad>::new();
                sblp.sbst = sb.sbid.to_string();
                sblp.time_v = vec![0f32; 365 * 48];
                //println!("  sb:{}", sb);
                if let Some(sbif) = sbif.get(&sb.sbid) {
                    for fd in lp23 {
                        if !re.is_match(&fd.feed) {
                            continue;
                        }
                        let mut fdpf = FeederLoad::default();
                        let ff = format!("{}{}", &fd.feed[0..3], &fd.feed[4..6]);
                        fdpf.sbst = fd.sbst;
                        fdpf.name = fd.name;
                        fdpf.feed = ff;
                        fdpf.time_r = fd.time_r;
                        let mut max = 0f32;
                        for (i, lpv) in fdpf.time_r.iter().enumerate() {
                            match &lpv {
                                LoadProfVal::Value(_) => fdpf.chk0.good += 1,
                                LoadProfVal::Null => fdpf.chk0.null += 1,
                                LoadProfVal::None => fdpf.chk0.none += 1,
                            }
                            if let LoadProfVal::Value(v) = &lpv {
                                sblp.time_v[i] += *v;
                                if *v > max {
                                    max = *v;
                                }
                            }
                        }
                        /*
                        println!(
                            "     flp: '{}' {} d:{} u:{} n:{}",
                            fdpf.feed, max, fdpf.chk0.good, fdpf.chk0.null, fdpf.chk0.none
                        );
                        */
                        sbpf.push(fdpf);
                    }
                    sbpf.sort_by(|a, b| a.feed.cmp(&b.feed));
                    let (mut sbmin, mut sbmax, mut sbsum, mut sbcnt) = (0f32, 0f32, 0f32, 0f32);
                    for sv in &sblp.time_v {
                        if *sv < sbmin {
                            sbmin = *sv;
                        }
                        if *sv > sbmax {
                            sbmax = *sv;
                        }
                        sbsum += *sv;
                        sbcnt += 1f32;
                    }
                    let sbavg = if sbcnt == 0f32 { 0f32 } else { sbsum / sbcnt };
                    let mvxn = sbif.mvxn as f32 * 0.7f32;
                    let lvx = (sbavg + sbmax) * 0.5f32;
                    let per = lvx / mvxn;
                    println!("{}\t{}\t{}", sb.sbid, per, sb.mwh);
                    /*
                    println!(
                        "  sb:{} [{}] {} {} {} {} {} - {} {}",
                        sb, per, lvx, sbmax, sbsum, sbcnt, sbavg, sbif.trxn, sbif.mvxn
                    );
                    */
                }
            }
        }
    }
}

pub fn read_lp24_csv() -> Result<(), Box<dyn std::error::Error>> {
    let dtt0 = NaiveDateTime::parse_from_str("2024-01-01 00:00", "%Y-%m-%d %H:%M")?;
    let t0 = dtt0.and_utc().timestamp_millis();
    let mut fdmp = HashMap::<String, Box<FeederLoadRaw>>::new();
    let mut sbmp = HashMap::<String, i32>::new();
    let lp_dr = "/mnt/d/CHMBACK/pea-data/loadprofile2024";
    for m in 1..=12 {
        let mn = format!("{}/Load Profile 2024{:02}.csv", lp_dr, m);
        println!("m:{}", mn);
        if let Ok(mut rdr) = csv::Reader::from_path(&mn) {
            for rc in rdr.records().flatten() {
                if let (Some(sb), Some(nm), Some(fd), Some(dt), Some(tm), Some(va)) = (
                    rc.get(0),
                    rc.get(1),
                    rc.get(2),
                    rc.get(3),
                    rc.get(4),
                    rc.get(5),
                ) {
                    let sb = sb.trim();
                    let fd = format!("{}_{}", &sb, &fd);
                    let dtf = format!("{} {}", dt, tm);
                    //println!("dtf: {}", dtf);
                    if let Ok(dttm) = NaiveDateTime::parse_from_str(dtf.as_str(), "%Y-%m-%d %H:%M")
                    {
                        //println!("  dt:'{}' tm:'{}' dttm:'{}' va:{}", dt, tm, dttm, va);
                        let t1 = dttm.and_utc().timestamp_millis();
                        let dtsec = (t1 - t0) / 1000;
                        let dtsec = dtsec as usize;
                        let dthlf = dtsec / 1800;
                        let vv = if let Ok(v) = va.parse::<f32>() {
                            //println!("    {}", v);
                            LoadProfVal::Value(v)
                        } else {
                            LoadProfVal::Null
                        };
                        if let Some(fl) = fdmp.get_mut(&fd) {
                            if dthlf < fl.time_r.len() {
                                fl.time_r[dthlf] = vv;
                            }
                        } else {
                            let mut fl = Box::new(FeederLoadRaw::new_num_of_day(365));
                            fl.sbst = sb.to_string();
                            fl.name = nm.to_string();
                            fl.feed = fd.to_string();
                            if dthlf < fl.time_r.len() {
                                fl.time_r[dthlf] = vv;
                            }
                            fdmp.insert(fd, fl);
                        }
                        if let Some(sbcn) = sbmp.get_mut(sb) {
                            *sbcn += 1;
                        } else {
                            sbmp.insert(sb.to_string(), 1);
                        }
                    } // end of date time calculation
                } else {
                    println!("error: {:?}", rc);
                }
            } // end loop month
            let (mut no, mut nu, mut va) = (0, 0, 0);
            for (_k, ff) in &fdmp {
                for dd in &ff.time_r {
                    match dd {
                        LoadProfVal::None => no += 1,
                        LoadProfVal::Null => nu += 1,
                        LoadProfVal::Value(_) => va += 1,
                    }
                }
            }
            println!("  va:{} nu:{} no:{}", va, nu, no);
        } else {
            println!("could not open {}", mn);
        } // end if file exists
    }

    let mut sbmp = HashMap::<String, Vec<Box<FeederLoadRaw>>>::new();
    for (_fd, rc) in fdmp {
        //println!("{} {}", rc.sbst, rc.feed);
        if let Some(fdv) = sbmp.get_mut(&rc.sbst) {
            fdv.push(rc);
        } else {
            sbmp.insert(rc.sbst.to_string(), vec![rc]);
        }
    }

    let sbdir = "../sgdata/lp2024";
    let _ = fs::create_dir_all(sbdir);
    for (k, v) in sbmp {
        let sbf = format!("{}/{}.bin", sbdir, k);
        //println!("{}", sbf);
        if let Ok(ser) = bincode::serialize(&v) {
            std::fs::write(sbf, ser).unwrap();
        }
    }

    Ok(())
}

pub fn ld_lp_yr_sb(yr: &str, sb: &str) -> Vec<Box<FeederLoadRaw>> {
    let sbdir = format!("../sgdata/lp{}", yr);
    let file = format!("{}/{}.bin", sbdir, sb);
    if let Ok(f) = File::open(&file) {
        if let Ok(dt) =
            bincode::deserialize_from::<BufReader<File>, Vec<Box<FeederLoadRaw>>>(BufReader::new(f))
        {
            return dt;
        }
    }
    Vec::<Box<FeederLoadRaw>>::new()
}

pub fn lp_ana1(yr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pv = grp1();
    let _pvsb = ld_p3_prv_sub();
    let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();
    //let sbdir = "../sgdata/lp2023a";
    let re = Regex::new(r"..._[0-9][0-9][VW]B01").unwrap();
    for p in &pv {
        let pp = p.to_string();
        println!("pv:{}", p);
        //if let Some(sbv) = pvsb.get(&pp) {
        if let Some(sbv) = sbsl.get(&pp) {
            for sb in sbv {
                let lp23 = ld_lp_yr_sb(yr, sb.sbid.as_str());
                let mut sblp = FeederLoad::default();
                let mut sbpf = Vec::<FeederLoad>::new();
                sblp.sbst = sb.sbid.to_string();
                sblp.time_v = vec![0f32; 365 * 48];
                //println!("  sb:{}", sb);
                if let Some(sbif) = sbif.get(&sb.sbid) {
                    for fd in lp23 {
                        if !re.is_match(&fd.feed) {
                            continue;
                        }
                        let mut fdpf = FeederLoad::default();
                        let ff = format!("{}{}", &fd.feed[0..3], &fd.feed[4..6]);
                        fdpf.sbst = fd.sbst;
                        fdpf.name = fd.name;
                        fdpf.feed = ff;
                        fdpf.time_r = fd.time_r;
                        /*
                        let (mut no, mut nu, mut va) = (0, 0, 0);
                        for td in &fdpf.time_r {
                            match td {
                                LoadProfVal::None => no += 1,
                                LoadProfVal::Null => nu += 1,
                                LoadProfVal::Value(_) => va += 1,
                            }
                        }
                        */
                        //println!("sb:{} fd:{} tm:{} {} {}", sb.sbid, fdpf.feed, va, nu, no);
                        let mut max = 0f32;
                        for (i, lpv) in fdpf.time_r.iter().enumerate() {
                            match &lpv {
                                LoadProfVal::Value(_) => fdpf.chk0.good += 1,
                                LoadProfVal::Null => fdpf.chk0.null += 1,
                                LoadProfVal::None => fdpf.chk0.none += 1,
                            }
                            if let LoadProfVal::Value(v) = &lpv {
                                sblp.time_v[i] += *v;
                                if *v > max {
                                    max = *v;
                                }
                            }
                        }
                        /*
                        println!(
                            "     flp: '{}' {} d:{} u:{} n:{}",
                            fdpf.feed, max, fdpf.chk0.good, fdpf.chk0.null, fdpf.chk0.none
                        );
                        */
                        sbpf.push(fdpf);
                    }
                    sbpf.sort_by(|a, b| a.feed.cmp(&b.feed));
                    let (mut sbmin, mut sbmax, mut sbsum, mut sbcnt) = (0f32, 0f32, 0f32, 0f32);
                    for sv in &sblp.time_v {
                        if *sv < sbmin {
                            sbmin = *sv;
                        }
                        if *sv > sbmax {
                            sbmax = *sv;
                        }
                        sbsum += *sv;
                        sbcnt += 1f32;
                    }
                    let sbavg = if sbcnt == 0f32 { 0f32 } else { sbsum / sbcnt };
                    let mvxn = sbif.mvxn as f32 * 0.7f32;
                    let lvx = (sbavg + sbmax) * 0.5f32;
                    let per = lvx / mvxn;
                    println!("{}\t{}\t{}", sb.sbid, per, sb.mwh);
                    /*
                    println!(
                        "  sb:{} [{}] {} {} {} {} {} - {} {}",
                        sb, per, lvx, sbmax, sbsum, sbcnt, sbavg, sbif.trxn, sbif.mvxn
                    );
                    */
                }
            }
        }
    }
    Ok(())
}

//use sglab02_lib::sg::ldp;

pub fn load_lpyd() -> Vec<(Vec<String>, HashMap<String, Vec<Box<dcl::FeederLoad>>>)> {
    //if let Ok(f) = File::open("data/lpyd.bin") {
    //if let Ok(f) = File::open(ldp::res("lpyd.bin")) {
    let lpyd = "../sgdata/lpyd.bin";
    if let Ok(f) = File::open(lpyd) {
        let r = BufReader::new(f);
        if let Ok(/*mut*/ lpyd) = bincode::deserialize_from::<
            BufReader<File>,
            Vec<(Vec<String>, HashMap<String, Vec<Box<dcl::FeederLoad>>>)>,
        >(r)
        {
            println!("..1");
            lpyd
        } else {
            println!("..2");
            Vec::new()
        }
    } else {
        println!("..3");
        Vec::new()
    }
}

pub fn lp_ana2() -> Result<(), Box<dyn std::error::Error>> {
    let mut lpyd = load_lpyd();
    let (_, lp23) = lpyd.pop().unwrap();
    let (_, lp22) = lpyd.pop().unwrap();
    let (_, lp21) = lpyd.pop().unwrap();
    println!("23:{} 22:{} 21:{}", lp23.len(), lp22.len(), lp21.len());

    let sbdir = "../sgdata/lpraw/lp2022";
    let _ = fs::create_dir_all(sbdir);
    for (s, v) in &lp22 {
        let mut fdv = Vec::<FeederLoadRaw>::new();
        println!("sb: {}", s);
        for fd in v {
            let mut fd2 = FeederLoadRaw::new_num_of_day(365usize);
            fd2.sbst = fd.sbst.to_string();
            fd2.name = fd.name.to_string();
            fd2.feed = fd.feed.to_string();
            //println!("  fd:{}", fd2.feed);
            for (i, td) in fd.time_r.iter().enumerate() {
                fd2.time_r[i] = match td {
                    dcl::LoadProfVal::None => LoadProfVal::None,
                    dcl::LoadProfVal::Null => LoadProfVal::Null,
                    dcl::LoadProfVal::Value(v) => LoadProfVal::Value(*v),
                }
            }
            fdv.push(fd2);
        }
        println!("22 s:{} v:{} = {}", s, v.len(), fdv.len());
        let sbf = format!("{}/{}.bin", sbdir, s);
        println!("{}", sbf);
        if let Ok(ser) = bincode::serialize(&fdv) {
            std::fs::write(sbf, ser).unwrap();
        }
    }

    let sbdir = "../sgdata/lpraw/lp2021";
    let _ = fs::create_dir_all(sbdir);
    for (s, v) in &lp21 {
        println!("21 s:{} v:{}", s, v.len());
        let mut fdv = Vec::<FeederLoadRaw>::new();
        for fd in v {
            let mut fd2 = FeederLoadRaw::new_num_of_day(365usize);
            fd2.sbst = fd.sbst.to_string();
            fd2.name = fd.name.to_string();
            fd2.feed = fd.feed.to_string();
            for (i, td) in fd.time_r.iter().enumerate() {
                fd2.time_r[i] = match td {
                    dcl::LoadProfVal::None => LoadProfVal::None,
                    dcl::LoadProfVal::Null => LoadProfVal::Null,
                    dcl::LoadProfVal::Value(v) => LoadProfVal::Value(*v),
                }
            }
            fdv.push(fd2);
        }
        let sbf = format!("{}/{}.bin", sbdir, s);
        println!("{}", sbf);
        if let Ok(ser) = bincode::serialize(&fdv) {
            std::fs::write(sbf, ser).unwrap();
        }
    }
    Ok(())
}

pub fn read_lp_csv(yr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fst = format!("{}-01-01 00:00", yr);
    let dtt0 = NaiveDateTime::parse_from_str(fst.as_str(), "%Y-%m-%d %H:%M")?;
    let t0 = dtt0.and_utc().timestamp_millis();
    let mut fdmp = HashMap::<String, Box<FeederLoadRaw>>::new();
    let mut sbmp = HashMap::<String, i32>::new();
    let lp_dr = format!("/mnt/d/CHMBACK/pea-data/loadprofile{}", yr);
    for m in 1..=12 {
        let mn = format!("{}/Load Profile {}{:02}.csv", lp_dr, yr, m);
        println!("m:{}", mn);
        if let Ok(mut rdr) = csv::Reader::from_path(&mn) {
            for rc in rdr.records().flatten() {
                if let (Some(sb), Some(nm), Some(fd), Some(dt), Some(tm), Some(va)) = (
                    rc.get(0),
                    rc.get(1),
                    rc.get(2),
                    rc.get(3),
                    rc.get(4),
                    rc.get(5),
                ) {
                    let sb = sb.trim();
                    let fd = format!("{}_{}", &sb, &fd);
                    let dtf = format!("{} {}", dt, tm);
                    //println!("dtf: {}", dtf);
                    if let Ok(dttm) = NaiveDateTime::parse_from_str(dtf.as_str(), "%Y-%m-%d %H:%M")
                    {
                        //println!("  dt:'{}' tm:'{}' dttm:'{}' va:{}", dt, tm, dttm, va);
                        let t1 = dttm.and_utc().timestamp_millis();
                        let dtsec = (t1 - t0) / 1000;
                        let dtsec = dtsec as usize;
                        let dthlf = dtsec / 1800;
                        let vv = if let Ok(v) = va.parse::<f32>() {
                            //println!("    {}", v);
                            LoadProfVal::Value(v)
                        } else {
                            LoadProfVal::Null
                        };
                        if let Some(fl) = fdmp.get_mut(&fd) {
                            if dthlf < fl.time_r.len() {
                                fl.time_r[dthlf] = vv;
                            }
                        } else {
                            let mut fl = Box::new(FeederLoadRaw::new_num_of_day(365));
                            fl.sbst = sb.to_string();
                            fl.name = nm.to_string();
                            fl.feed = fd.to_string();
                            if dthlf < fl.time_r.len() {
                                fl.time_r[dthlf] = vv;
                            }
                            fdmp.insert(fd, fl);
                        }
                        if let Some(sbcn) = sbmp.get_mut(sb) {
                            *sbcn += 1;
                        } else {
                            sbmp.insert(sb.to_string(), 1);
                        }
                    } // end of date time calculation
                } else {
                    println!("error: {:?}", rc);
                }
            } // end loop month
            let (mut no, mut nu, mut va) = (0, 0, 0);
            for (_k, ff) in &fdmp {
                for dd in &ff.time_r {
                    match dd {
                        LoadProfVal::None => no += 1,
                        LoadProfVal::Null => nu += 1,
                        LoadProfVal::Value(_) => va += 1,
                    }
                }
            }
            println!("  va:{} nu:{} no:{}", va, nu, no);
        } else {
            println!("could not open {}", mn);
        } // end if file exists
    }

    let mut sbmp = HashMap::<String, Vec<Box<FeederLoadRaw>>>::new();
    for (_fd, rc) in fdmp {
        //println!("{} {}", rc.sbst, rc.feed);
        if let Some(fdv) = sbmp.get_mut(&rc.sbst) {
            fdv.push(rc);
        } else {
            sbmp.insert(rc.sbst.to_string(), vec![rc]);
        }
    }

    let sbdir = format!("../sgdata/lp{}", yr);
    let _ = fs::create_dir_all(&sbdir);
    for (k, v) in sbmp {
        let sbf = format!("{}/{}.bin", sbdir, k);
        //println!("{}", sbf);
        if let Ok(ser) = bincode::serialize(&v) {
            std::fs::write(sbf, ser).unwrap();
        }
    }

    Ok(())
}

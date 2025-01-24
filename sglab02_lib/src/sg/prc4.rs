use crate::sg::ldp::base;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
//use crate::sg::ldp::FeederTranx;
use std::fs::File;
use std::fs;
use std::io::BufReader;
//use crate::sg::ldp::TranxInfo;
////use crate::sg::imp::CSVFile;
//use crate::sg::imp::src_dir;
//use std::path::PathBuf;
//use crate::sg::imp::data_dir;
//use crate::sg::wk4::YearLoad;
//use crate::sg::ldp;
//use crate::sg::wk5::Tranx;
//use crate::sg::wk5::EvalPara1;
use crate::sg::wk5::EvDistCalc;
//use crate::sg::prc1::SubstInfo;
//use crate::sg::wk4::Substation as Wk4Substation;
//use crate::sg::wk5::Substation as Wk5Substation;
//use crate::sg::wk4::Wk4Proc;
//use regex::Regex;
//use crate::sg::dcl::LoadProfVal;
//use crate::sg::wk4::DayLoad;
//use crate::sg::prc2::Transformer;
use crate::sg::prc1::p1_spp_conn;
use crate::sg::prc1::p1_vspp_conn;
use crate::sg::prc1::SPPConn;
use crate::sg::prc1::VSPPConn;
use crate::sg::imp::ld_replan;
use crate::sg::imp::REPlan;
//use crate::sg::imp::ld_bisze;
//use crate::sg::gis1::ar_list;
//use crate::sg::gis1::DbfVal;
use crate::sg::mvline::utm_latlong;
use crate::sg::wk5::ld_fd_es_m;
//use crate::sg::mvline::latlong_utm;
use crate::sg::prc3::year_load_power;
use crate::sg::prc3::ld_p3_calc;
use crate::sg::prc3::ld_sub_loc;
use crate::sg::prc3::ld_p3_treg_m;
use crate::sg::prc3::ld_p3_sub_inf;
use crate::sg::prc3::ld_p3_prv_sub;
use crate::sg::load::load_pvcamp;

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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Proc41Item {
    pv: String,
    sbid: String,
    name: String,
    sbnm: String,
    pos_peak: f32,
    pos_avg: f32,
    pos_ngy: f32,
    neg_peak: f32,
    neg_avg: f32,
    neg_ngy: f32,
    neg_cnt: usize,
    mwh: f32,
    mw: f32,
    max_pw: i32,
    trxno: usize,
    mt_tx: f32,
    latlon: String,
    dt: usize,
    mt: usize,
    mt1: usize,
    mt3: usize,
    e5: f64,
    e2: f64,
    spp: usize,
    vspp: usize,
    repl: usize,
}

pub async fn prc41() -> Result<(), Box<dyn std::error::Error>> {
    //let prvs = ld_p3_prvs();

    let prvs = grp1();
    let prv_sub = ld_p3_prv_sub();
    let sub_inf = ld_p3_sub_inf();
    let mut pv_calc = ld_p3_calc("p3_feed_calc.bin");
    let treg_m = ld_p3_treg_m("p3_pv_treg_m.bin");
    let treg_sb_m = ld_p3_treg_m("p3_sb_treg_m.bin");

    // SPP
    let sppv = p1_spp_conn();
    let mut spp_sb_m1 = HashMap::<String,Vec::<SPPConn>>::new();
    let mut spp_sb_m2 = HashMap::<String,Vec::<SPPConn>>::new();
    for spp in &sppv {
        if let Some(sppv) = spp_sb_m1.get_mut(&spp.sbid) {
            sppv.push(spp.clone());
        } else {
            spp_sb_m1.insert(spp.sbid.to_string(), vec![spp.clone()]);
        }
        if let Some(sppv) = spp_sb_m2.get_mut(&spp.sbi2) {
            sppv.push(spp.clone());
        } else {
            spp_sb_m2.insert(spp.sbi2.to_string(), vec![spp.clone()]);
        }
    }

    // VSPP
    let vsppv = p1_vspp_conn();
    let mut vspp_sb_m = HashMap::<String,Vec::<VSPPConn>>::new();
    for vspp in &vsppv {
        if let Some(vsppv) = vspp_sb_m.get_mut(&vspp.sbid) {
            vsppv.push(vspp.clone());
        } else {
            vspp_sb_m.insert(vspp.sbid.to_string(), vec![vspp.clone()]);
        }
    }

    // RE Plan
    let newre = ld_replan();
    let mut re_sb_m = HashMap::<String,Vec::<REPlan>>::new();
    for rep in &newre {
        if let Some(/*mut*/ rev) = re_sb_m.get_mut(&rep.sbid) {
            rev.push(rep.clone());
        } else {
            re_sb_m.insert(rep.sbid.clone(), vec![rep.clone()]);
        }
    }

    let sb_loc_m = ld_sub_loc();
    let fd_es_m = ld_fd_es_m();

    use std::fmt::Write;
    let mut pv_req = String::new();
    let mut sb_req = String::new();
    let /*mut*/ _fd_req = String::new();
    let /*mut*/ _pv35 = String::new();
    let mut sb_inf_v = Vec::<Proc41Item>::new();
    write!(sb_req, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", 
        "sb", "name", "pv", "pos_peak", "pos_avg", "pos_ngy", "neg_peak", "neg_avg", "neg_cnt", "neg_ngy",
         "mwh ", "mw1", "max pw", "trxno", "lat+long", "DT","MT","E5:GWh","E2:GWh", "SPP", "VSPP", "REPL").unwrap();
    for pv in prvs { // for pv
        let (mut pv1, mut pv2) = (String::new(),String::new());
        let (mut pv_pos_peak, mut pv_pos_avg, mut pv_neg_peak, mut pv_neg_avg, mut pvmw, mut pvtr) = (0f32,0f32,0f32,0f32,0i32,0usize);
        let mut spp_cn = 0;
        let mut vspp_cn = 0;
        let mut rep_cn = 0;
        let /*mut*/ _rep_pw = 0;
        let mut pv_es = 0f32;
        let /*mut*/ _pv_inf = Proc41Item::default();
        if let Some(sbs) = prv_sub.get(pv) { // if pvca
            for sb in sbs { // for sb
                //println!("  sb: {} >>>>>", sb);
                let (mut pos_peak, mut pos_avg, mut neg_peak, mut neg_avg) = (0f32,0f32,0f32,0f32);
                //let (mut ssmw, mut sstr) = (0i32,0usize);
                //let mut ssmw = 0i32;
                let (mut neg_cnt, mut pos_engy, mut neg_engy) = (0usize, 0f32, 0f32);
                let (mut ss1, mut ss2) = (String::new(),String::new());
                let mut sb_es = 0f32;
                let mut sb_inf = Proc41Item::default();
                sb_inf.sbid = sb.to_string();
                sb_inf.pv = pv.to_string();
                if let Some(sbif) = sub_inf.get(sb.as_str()) { // if sub
                    //sbif.a();
                    for fd in &sbif.feeders { // for feeders
                        if let Some(/*mut*/ ca) = pv_calc.get_mut(fd) {
                            if ca.year_load.loads.len()<365 { continue; }
                            if ca.year_load.loads[100].load.len()<48 { continue; }
                            let mut fd_es = 0f32;
                            if let Some(es) = fd_es_m.get(fd) {
                                fd_es = *es;
                            }
                            year_load_power(&mut ca.year_load);
                            year_load_power(&mut ca.last_year_load);
                            write!(ss2, "===|===|{}|{}|{}|{}|{}\t{}\n", fd, 
                                ca.year_load.power_quality.pos_peak, ca.year_load.power_quality.pos_avg,
                                ca.year_load.power_quality.neg_peak, ca.year_load.power_quality.neg_avg,
                                fd_es,
                            ).unwrap();
                            pos_peak += ca.year_load.power_quality.pos_peak;
                            pos_avg += ca.year_load.power_quality.pos_avg;
                            neg_peak += ca.year_load.power_quality.neg_peak;
                            neg_avg += ca.year_load.power_quality.neg_avg;
                            //if ca.year_load.power_quality.neg_avg>0.0 {
                            neg_cnt += ca.year_load.power_quality.neg_cnt;
                            neg_engy += ca.year_load.power_quality.neg_energy;
                            pos_engy += ca.year_load.power_quality.pos_energy;
                            //}

                            sb_inf.pos_peak += ca.year_load.power_quality.pos_peak;
                            sb_inf.pos_avg += ca.year_load.power_quality.pos_avg;
                            sb_inf.neg_peak += ca.year_load.power_quality.neg_peak;
                            sb_inf.neg_avg += ca.year_load.power_quality.neg_avg;
                            sb_inf.neg_cnt += ca.year_load.power_quality.neg_cnt;
                            sb_inf.neg_ngy += ca.year_load.power_quality.neg_energy;
                            sb_inf.pos_ngy += ca.year_load.power_quality.pos_energy;

                            sb_es += fd_es;
                        }
                    }
                    if let Some(sppv) = spp_sb_m1.get(sb) {
                        spp_cn += sppv.len();
                    }
                    if let Some(sppv) = spp_sb_m2.get(sb) {
                        spp_cn += sppv.len();
                    }
                    if let Some(vsppv) = vspp_sb_m.get(sb) {
                        vspp_cn += vsppv.len();
                    }
                    if let Some(repv) = re_sb_m.get(sb) {
                        rep_cn += repv.len();
                    }
                    let (mut x, mut y) = (0f64,0f64);
                    let mut ldln = String::new();
                    if let Some((a,b)) = sb_loc_m.get(sb) {
                        x = *a;
                        y = *b;
                        let (xx, yy) = utm_latlong(x as f32, y as f32);
                        ldln = format!("{:.4},{:.4}", xx, yy);
                    }
                    if ldln.len()==0 { continue; }
                    ldln = format!("https://maps.google.com/?q={}", ldln);
                    let sb_es0 = (sb_es + 0.5) as i32;
                    let /*mut*/ sb_es0 = sb_es0 as f32;
                    let /*mut*/ _sb_pw0 = sb_es0 * 0.5;
                    write!(ss1, "===|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n", sb, sbif.name, pv, pos_peak, pos_avg, neg_peak,
                        neg_avg, neg_cnt, sbif.mvxn, sbif.trxn, x, y, sb_es).unwrap();
                    let (mut sb_tx_no, mut sb_mt_cnt, mut sb_eg5_sm, mut sb_eg2_sm) = (0usize,0usize,0f64,0f64);
                    let (mut mt_1p, mut mt_3p) = (0usize, 0usize);
                    if let Some(treg) = treg_sb_m.get(sb) {
                        sb_tx_no = treg.tx_no;
                        sb_mt_cnt = treg.mt_cnt;
                        mt_1p = treg.mt_1_ph;
                        mt_3p = treg.mt_3_ph;
                        sb_eg5_sm = treg.eg5_sm/1000000.0;
                        sb_eg2_sm = treg.eg2_sm/1000000.0;
                    }
                    
                    let mut mwh = pos_engy / 365.0 / 10.0;
                    //let mut mw1 = (neg_peak+neg_avg)*0.5;
                    let /*mut*/ mw1;

                    // spp count
                    let mut spp = 0;
                    if let Some(sp) = spp_sb_m1.get(sb) {
                        spp += sp.len();
                    }
                    if let Some(sp) = spp_sb_m2.get(sb) {
                        spp += sp.len();
                    }

                    // vspp count
                    let mut vspp = 0;
                    if let Some(sp) = vspp_sb_m.get(sb) {
                        vspp += sp.len();
                    }

                    // re plan count
                    let mut repl = 0;
                    if let Some(re) = re_sb_m.get(sb) {
                        repl += re.len();
                    }
                    
                    let mt_tx = sb_mt_cnt as f32 / sb_tx_no as f32;

                    if pos_peak>1.0 && neg_cnt>10000 && neg_engy>5000.0 {
                        //let mut mwh = pos_engy / 365.0 / 25.0;
                        //let mw2 = neg_avg * 1.5;
                        //if mw1>mw2 { mw1 = mw2; }
                        mwh = mwh.ceil();
                        //if neg_cnt<200 { mw1 = 0.0; mwh = 0.0; };
                        mw1 = mwh * 0.5;

                    } else {
                        mwh = 0.0;
                        mw1 = 0.0;
                    }

                    sb_inf.name = sbif.name.to_string();
                    sb_inf.mwh = mwh;
                    sb_inf.mw = mw1;
                    sb_inf.max_pw = sbif.mvxn;
                    sb_inf.trxno = sbif.trxn;
                    sb_inf.latlon = ldln.to_string();
                    sb_inf.dt = sb_tx_no;
                    sb_inf.mt = sb_mt_cnt;
                    sb_inf.mt1 = mt_1p;
                    sb_inf.mt3 = mt_3p;
                    sb_inf.e5 = sb_eg5_sm.ceil();
                    sb_inf.e2 = sb_eg2_sm.ceil();
                    sb_inf.spp = spp;
                    sb_inf.vspp = vspp;
                    sb_inf.repl = repl;
                    sb_inf.mt_tx = mt_tx;
                
                    if mt_tx>10.0 {
                        write!(sb_req, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", 
                        sb, sbif.name, pv, pos_peak, pos_avg, pos_engy, neg_peak, neg_avg, neg_cnt, neg_engy,
                        mwh, mw1, sbif.mvxn, sbif.trxn, ldln, 
                        sb_tx_no, sb_mt_cnt, sb_eg5_sm.ceil(), sb_eg2_sm.ceil(),
                        spp, vspp, repl).unwrap();
                    }
                    pv_pos_peak += pos_peak;
                    pv_pos_avg += pos_avg;
                    pv_neg_peak += neg_peak;
                    pv_neg_avg += neg_avg;
                    pv_es += sb_es;
                    let ssmw = sbif.mvxn;
                    let sstr = sbif.trxn;
                    pvmw += ssmw;
                    pvtr += sstr;

                }
                if ss2.len()>2 {
                    write!(pv2,"{}", ss1).unwrap();
                    //write!(pv2,"{}", ss2);
                }
                sb_inf_v.push(sb_inf);
            }
            let (mut tx_no, mut mt_cnt, mut eg5_sm, mut eg2_sm) = (0usize,0usize,0f64,0f64);
            if let Some(treg) = treg_m.get(pv) {
                tx_no = treg.tx_no;
                mt_cnt = treg.mt_cnt;
                eg5_sm = treg.eg5_sm/1000000.0;
                eg2_sm = treg.eg2_sm/1000000.0;
            }
            write!(pv_req, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", pv, pv_pos_peak, pv_pos_avg, pv_neg_peak, pv_neg_avg, pvmw, pvtr, tx_no, mt_cnt, eg5_sm, eg2_sm, spp_cn, vspp_cn, rep_cn, pv_es).unwrap();
            //write!(pv_req, "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n"
            //    , pv, pv_pos_peak, pv_pos_avg, pv_neg_peak, pv_neg_avg, pvmw, pvtr, tx_no, mt_cnt, eg5_sm, eg2_sm, spp_cn, vspp_cn, rep_cn, pv_es);
            write!(pv1, "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n"
                , pv, pv_pos_peak, pv_pos_avg, pv_neg_peak, pv_neg_avg, pvmw, pvtr, tx_no, mt_cnt, eg5_sm, eg2_sm, spp_cn, vspp_cn, rep_cn, pv_es).unwrap();
        }
        print!("{}", pv1);

        print!("{}", pv2);
    }
    let file = format!("{}/p41_sb_inf_v.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&sb_inf_v) { std::fs::write(file, ser).unwrap(); }
    let vv = ld_sb_inf_v()?;
    println!("{}", vv.len());

    if let Ok(_) = fs::write("prj1/p41_pv_req.txt", pv_req) { }
    if let Ok(_) = fs::write("prj1/p41_sb_req.txt", sb_req) { println!("WRITE FILE P41"); }
    Ok(())
}

pub fn ld_sb_inf_v() -> Result<Vec::<Proc41Item>, Box<dyn std::error::Error>> {
    if let Ok(f) = File::open(crate::sg::ldp::res("p41_sb_inf_v.bin")) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, Vec::<Proc41Item>>(BufReader::new(f)) {
            return Ok(dt);
        }
    }
    Ok(Vec::<Proc41Item>::new())
}    

pub async fn prc42() -> Result<(), Box<dyn std::error::Error>> {
    let vv = ld_sb_inf_v()?;
    use std::fmt::Write;

    let mut pv_sbv_m = HashMap::<String,Vec::<Proc41Item>>::new();
    for v in &vv {
        if let Some(vec) = pv_sbv_m.get_mut(&v.pv) {
            vec.push(v.clone());
        } else {
            pv_sbv_m.insert(v.pv.to_string(), vec![v.clone()]);
        }
    }
    let mut pv_sbv_mt = pv_sbv_m.clone();
    let mut pv_sbv_ne = pv_sbv_m.clone();
    let mut pv_sbv_eg = pv_sbv_m.clone();
    for (_k,v) in &mut pv_sbv_mt {
        v.sort_by(|a, b| b.mt.cmp(&a.mt));
    }
    for (_k,v) in &mut pv_sbv_ne {
        v.sort_by(|a, b| b.neg_cnt.cmp(&a.neg_cnt));
    }
    for (_k,v) in &mut pv_sbv_eg {
        v.sort_by(|a, b| b.pos_ngy.partial_cmp(&a.pos_ngy).unwrap());
    }

    let mut ss_r1 = String::new();
    let mut ss_r2 = String::new();
    let mut ss_r3 = String::new();
    let mut ss_mt = String::new();
    let mut ss_ne = String::new();
    let mut ss_eg = String::new();
    let prvs = grp1();
//    let fc = 1.0/365.0/20.0;
    let fc0 = 1.0/365.0/20.0;
    let fc1 = 1.0/365.0/10.0;
    let fc2 = 1.0/365.0/40.0;
    let fc3 = 1.0/365.0/100.0;
    let fc4 = 1.0/365.0/15.0;

    write!(ss_r2, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", 
        "sb", "name", "pv", "pos_peak", "pos_avg", "pos_ngy", "neg_peak", "neg_avg", "neg_cnt", "neg_ngy",
         "mwh ", "mw1", "max pw", "trxno", "lat+long", "DT","MT1","MT3","E5:GWh","E2:GWh", "SPP", "VSPP", "REPL").unwrap();

    let _set1 = vec!["สงขลา",];
    let _set1 = vec!["สระบุรี","ขอนแก่น","สุราษฏร์ธานี","นครสวรรค์",];
    let _set1 = vec!["สระแก้ว","เพชรบุรี",];
    let _set1 = vec!["ฉะเชิงเทรา","สมุทรสาคร","นครปฐม",];

         
    for pv in prvs {
        let mut mt_del = Vec::<String>::new();
        write!(ss_mt, "\n{}\n", pv).unwrap();

        let mut fc = fc0;

        if vec!["พระนครศรีอยุธยา","บุรีรัมย์","ปราจีนบุรี","เชียงใหม่","ลพบุรี",].contains(&&pv) {
            fc = fc1;
            println!("fc1: {} :{}", pv, fc);
        }
        if vec!["ระนอง","สระแก้ว","นครสวรรค์",].contains(&&pv) {
            fc = fc2;
            println!("fc2: {} :{}", pv, fc);
        }
        if vec!["สมุทรสงคราม","สุราษฏร์ธานี",].contains(&&pv) {
            fc = fc3;
            println!("fc3: {} :{}", pv, fc);
        }
        if vec!["ชลบุรี","ระยอง","ฉะเชิงเทรา","สมุทรสาคร",].contains(&&pv) {
            fc = fc4;
            println!("fc4: {} :{}", pv, fc);
        }

        let mut mxbt = 1;
        if vec!["ชลบุรี","ระยอง","ฉะเชิงเทรา","สมุทรสาคร",].contains(&&pv) {
            mxbt = 2;
            println!("mxbt:2.{} mxbt:{}", pv, mxbt);
        }

        if let Some(sbv) = pv_sbv_mt.get(pv) {
            let cn = if sbv.len()>=mxbt { mxbt } else { sbv.len() };
            for c in 0..cn {
                let v = &sbv[c];
                if v.mt==0 { continue; }
                if v.pos_avg==0.0 { continue; }
                let bc = v.pos_ngy * fc;
                write!(ss_mt,"{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", v.pv, v.sbid, v.mwh, v.pos_ngy, v.neg_cnt, v.dt, v.mt, bc).unwrap();
            }
            if pv=="นครราชสีมา" {
                let mut mt_cnt = 0;
                for v in sbv {
                    mt_cnt += v.mt;
                    if mt_cnt>640000 {
                        mt_del.push(v.sbid.to_string());
                    }
                }
            }
        }
        write!(ss_ne, "\n{}\n", pv).unwrap();
        if let Some(sbv) = pv_sbv_ne.get(pv) {
            let cn = if sbv.len()>=mxbt { mxbt } else { sbv.len() };
            for c in 0..cn {
                let v = &sbv[c];
                if v.mt==0 { continue; }
                if v.pos_avg==0.0 { continue; }
                let bc = v.pos_ngy * fc;
                write!(ss_ne,"{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", v.pv, v.sbid, v.mwh, v.pos_ngy, v.neg_cnt, v.dt, v.mt, bc).unwrap();
            }
        }
        write!(ss_eg, "\n{}\n", pv).unwrap();
        if let Some(sbv) = pv_sbv_eg.get(pv) {
            let cn = if sbv.len()>=mxbt { mxbt } else { sbv.len() };
            for c in 0..cn {
                let v = &sbv[c];
                if v.mt==0 { continue; }
                if v.pos_avg==0.0 { continue; }
                let bc = v.pos_ngy * fc;
                write!(ss_eg,"{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", v.pv, v.sbid, v.mwh, v.pos_ngy, v.neg_cnt, v.dt, v.mt, bc).unwrap();
            }
        }

        let mut sbv0 = Vec::<String>::new();
        write!(ss_r1, "\n{}\n", pv).unwrap();
        if let (Some(sbv1),Some(sbv2)) = (pv_sbv_mt.get(pv), pv_sbv_eg.get(pv)) {
            let cn1 = if sbv1.len()>=mxbt { mxbt } else { sbv1.len() };
            let cn2 = if sbv2.len()>=mxbt { mxbt } else { sbv2.len() };
            for c in 0..cn1 {
                sbv0.push(sbv1[c].sbid.to_string());
            }
            for c in 0..cn2 {
                if sbv0.contains(&sbv2[c].sbid) { continue; }
                sbv0.push(sbv2[c].sbid.to_string());
            }
            for v in sbv1 {
                if sbv0.contains(&v.sbid) {
                    let bc = v.pos_ngy * fc;
                    write!(ss_r1,"{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", v.pv, v.sbid, v.mwh, v.pos_ngy, v.neg_cnt, v.dt, v.mt, bc).unwrap();
                }
            }
        }
        write!(ss_r2, "\n{}\n", pv).unwrap();
        let mut pv_dt = 0usize;
        let mut pv_m1 = 0usize;
        let mut pv_m3 = 0usize;
        let mut pv_es = 0f32;
        if let Some(sbv) = pv_sbv_m.get_mut(pv) {
            for v in sbv {
                v.mwh = 0.0;
                if mt_del.contains(&v.sbid) { continue; }
                if sbv0.contains(&v.sbid) {
                    v.mwh = v.pos_ngy * fc;
                    v.mwh = v.mwh.ceil();
                }
                v.mw = v.mwh * 0.5;
                if v.mt == 0 { continue; }
                if v.mt_tx<10.0 { continue; }
                if v.pos_avg==0.0 { continue; }

                pv_dt += v.dt;
                pv_m1 += v.mt1;
                pv_m3 += v.mt3;
                pv_es += v.mwh;
            
                write!(ss_r2, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", 
                    v.sbid, v.name, v.pv, v.pos_peak, v.pos_avg, v.pos_ngy, v.neg_peak, v.neg_avg, v.neg_cnt, v.neg_ngy,
                    v.mwh, v.mw, v.max_pw, v.trxno, v.latlon, 
                    v.dt, v.mt1, v.mt3, v.e5.ceil(), v.e2.ceil(),
                    v.spp, v.vspp, v.repl).unwrap();
            }
        }
        write!(ss_r3, "{}\t{}\t{}\t{}\t{}\n", pv, pv_dt, pv_m1, pv_m3, pv_es).unwrap();
    }
    if let Ok(_) = fs::write("prj1/p42_sb_r1.txt", ss_r1) { }
    if let Ok(_) = fs::write("prj1/p42_sb_r2.txt", ss_r2) { }
    if let Ok(_) = fs::write("prj1/p42_sb_r3.txt", ss_r3) { }
    if let Ok(_) = fs::write("prj1/p42_sb_mt.txt", ss_mt) { }
    if let Ok(_) = fs::write("prj1/p42_sb_ne.txt", ss_ne) { }
    if let Ok(_) = fs::write("prj1/p42_sb_eg.txt", ss_eg) { }

    /*
    for v in &vv {
        write!(ss,"{}\t{}\t{}\t{}\t{}\n", v.pv, v.sbid, v.mwh, v.pos_ngy, v.neg_cnt);
    }
    if let Ok(_) = fs::write("prj1/p42_sb_ngy.txt", ss) { }
    */
    Ok(())
}

pub async fn prc43() -> Result<(), Box<dyn std::error::Error>> {
    car_reg_2023().await;
    Ok(())
}

async fn car_reg_2023() {
    let cfg = base().config.clone();

    let cfg = cfg.read().await;
    let bkks = cfg.criteria.car_reg_bkk_province.split(",");
    let bkks = bkks.collect::<Vec<&str>>();
    let bkkn = cfg.criteria.car_reg_bkk_percent.split(",");
    let bkkn = bkkn.collect::<Vec<&str>>();
    let bkkn = bkkn
        .iter()
        .map(|a| a.parse().unwrap())
        .collect::<Vec<f64>>();
    
        println!("{:?}", bkks);
        println!("{:?}", bkkn);

    let asss = cfg.criteria.car_reg_to_province.split(",");
    let asss = asss.collect::<Vec<&str>>();
    let assn = cfg.criteria.car_reg_to_percent.split(",");
    let assn = assn.collect::<Vec<&str>>();
    let assn = assn
        .iter()
        .map(|a| a.parse().unwrap())
        .collect::<Vec<f64>>();

        println!("{:?}", asss);
        println!("{:?}", assn);
        
    if bkkn.len() != bkks.len() {
        panic!("car_reg_bkk_province is NG\n");
    }
    if assn.len() != asss.len() {
        panic!("car_reg_to_province is NG\n");
    }

    let mut pv_ca_mp = load_pvcamp();
    let mut pv_ca_mp2 = HashMap::new();
    let mut pv_ca_cn2 = HashMap::new();
    //let mut cnt0 = 0.0;
    pv_ca_mp.insert("กรุงเทพมหานคร".to_string(), 967297.0);
    for (k, v) in &pv_ca_mp {
        //cnt0 += *v;
        let mut kk = k.to_string();
        let mut vv = *v;
        if k == "ยะลา" {
            if let Some(v2) = pv_ca_mp.get("สาขา อ.เบตง") {
                //let v1 = *v2;
                vv += *v2;
            }
        }
        if kk == " พระนครศรีอยุธยา" {
            kk = "พระนครศรีอยุธยา".to_string();
        }
        if kk == "แม่ฮองสอน" {
            kk = "แม่ฮ่องสอน".to_string();
        }
        if kk == "สาขา อ.เบตง" {
            //print!("NO BETONG\n");
        } else {
            //print!("'{}' - {}\n", kk, vv);
            pv_ca_mp2.insert(kk.clone(), vv);
            pv_ca_cn2.insert(kk, 0);
        }
    }

    //println!("pv_ca_mp2: {:?}", pv_ca_mp2);
    //println!("pv_ca_cn2: {:?}", pv_ca_cn2);
    let mut tk0 = 0.0;
    for (i, b) in bkks.iter().enumerate() {
        if let Some(nn) = pv_ca_mp2.get_mut(&b.to_string()) {
            let tk = *nn * bkkn[i] / 100.0;
            *nn -= tk;
            tk0 += tk;
            //print!("take from {} {}\n", b, tk);
        }
    }
    for (i, t) in asss.iter().enumerate() {
        let _ts = t.to_string();
        if let Some(cn) = pv_ca_mp2.get_mut(&t.to_string()) {
            let ad = tk0 * assn[i] / 100.0;
            *cn += ad;
            //print!("add to {} {}\n", t, ad);
        }
    }
    let mut pv_car_reg_mp = HashMap::new();
    let mut total = 0.0f32;
    for (k, v) in &pv_ca_mp2 {
        if bkks.contains(&k.as_str()) {
            continue;
        }
        let mut pv_ca_reg = EvDistCalc::default();
        pv_ca_reg.id = k.to_string();
        pv_ca_reg.ev_no = *v as f32;
        total += pv_ca_reg.ev_no;
        pv_car_reg_mp.insert(k.to_string(), pv_ca_reg);
    }
    let mut ev_reg_no = cfg.criteria.ev_car_reg_cnt;
    ev_reg_no += cfg.criteria.ev_car_all_reg;
    //ev_reg_no += 100000.0;
    for (_k, v) in &mut pv_car_reg_mp {
        if total > 0.0 {
            v.ev_pc = v.ev_no / total as f32;
            v.ev_ds = v.ev_pc * ev_reg_no;
        } else {
            v.ev_ds = 0.0;
        }
    }

    use std::fmt::Write;
    let mut ss = String::new();
    let rt0 = 0.1;
    //let rti = 0.01;
    let rti = 0.1/13.0;
    for pv in grp1() {
        if let Some(v) = pv_car_reg_mp.get(pv) {
            write!(ss, "{}", pv).unwrap();
            let mut mwh = v.ev_ds * 0.011 * 3.0 * 365.0;
            let mut rtc = rt0;
            for y in 2023..=2039 {
                mwh += mwh * rtc;
                if y>= 2025 {
                    write!(ss,"\t{}", mwh).unwrap();
                }
                rtc += rti;
            }
            write!(ss,"\n").unwrap();
        }
    }
    if let Ok(_) = fs::write("prj1/ev-grw-1.txt", ss) { }
    //wk5prc.ev_prov_dist = pv_car_reg_mp.clone();
}


pub async fn prc44() -> Result<(), Box<dyn std::error::Error>> {
    car_reg_2023_a().await;
    Ok(())
}

fn pv_adjust() -> Vec::<(&'static str, f64, f64)> {
    vec![
        ("กรุงเทพมหานคร", 0.0, 30.0,),
        ("นนทบุรี", 0.0, 25.0,),
        ("สมุทรปราการ", 0.0, 15.0,),
        ("ภูเก็ต", 0.0, 3.5,),
        ("ยะลา", 0.0, 80.0,),
        ("นราธิวาส", 0.0, 80.0,),
        ("ปัตตานี", 0.0, 80.0,),
        ("สกลนคร", 0.0, 80.0,),
        ("กาฬสินธุ์", 0.0, 80.0,),
        ("ตรัง", 0.0, 50.0,),
        ("มหาสารคาม", 0.0, 80.0,),
        ("มุกดาหาร", 0.0, 80.0,),
        ("อุดรธานี", 0.0, 80.0,),
        ("พัทลุง", 0.0, 80.0,),
        ("นครศรีธรรมราช", 0.0, 80.0,),
        ("ศรีสะเกษ", 0.0, 80.0,),
        ("ร้อยเอ็ด", 0.0, 80.0,),
        ("สุรินทร์", 0.0, 80.0,),
        ("กาฬสินธุ์", 0.0, 80.0,),
        ("สุโขทัย", 0.0, 80.0,),
        ("แพร่", 0.0, 80.0,),
        ("ประจวบคีรีขันธ์", 0.0, 80.0,),
        ("พะเยา", 0.0, 80.0,),
        ("ชุมพร", 0.0, 80.0,),
        ("นครพนม", 0.0, 80.0,),
        ("พิจิตร", 0.0, 80.0,),
        ("บึงกาฬ", 0.0, 80.0,),
        ("หนองบัวลำภู", 0.0, 80.0,),
        ("หนองคาย", 0.0, 80.0,),
        ("ตราด", 0.0, 80.0,),
        ("สตูล", 0.0, 80.0,),
        ("ชัยนาท", 0.0, 80.0,),
        ("สิงห์บุรี", 0.0, 80.0,),
        ("ราชบุรี", 5.0, 0.0,),
        ("นครสวรรค์", 3.2, 0.0,),
        ("ระนอง", 0.4, 0.0,),
        ("สมุทรสงคราม", 0.2, 0.0,),
        ("สมุทรสาคร", 4.7, 0.0,),
        ("ปทุมธานี", 13.8, 0.0,),
        ("กระบี่", 1.3, 0.0,),
        ("สงขลา", 4.9, 0.0,),
        ("เพชรบุรี", 2.4, 0.0,),
        ("นครปฐม", 6.0, 0.0,),
        ("ฉะเชิงเทรา", 6.0, 0.0,),
        ("ระยอง", 5.0, 0.0,),
        ("ปราจีนบุรี", 7.4, 0.0,),
        ("สุราษฎร์ธานี", 4.0, 0.0,),
        ("สระบุรี", 2.7, 0.0,),
        ("สระแก้ว", 1.8, 0.0,),
        ("นครราชสีมา", 4.0, 0.0,),
        ("เชียงใหม่", 3.8, 0.0,),
        ("บุรีรัมย์", 2.1, 0.0,),
        ("พิษณุโลก", 1.8, 0.0,),
        ("ชลบุรี", 1.4, 0.0,),
        ("พระนครศรีอยุธยา", 5.8, 0.0,),
        ("ขอนแก่น", 5.8, 0.0,),
        ("ลพบุรี", 1.4, 0.0,),
    ]
}

async fn car_reg_2023_a() {
    let cfg = base().config.clone();

    let _cfg = cfg.read().await;

    let mut pv_ca_mp = load_pvcamp();
    let mut pv_ca_mp2 = HashMap::new();
    //let mut cnt0 = 0.0;
    pv_ca_mp.insert("กรุงเทพมหานคร".to_string(), 967297.0);
    for (k, v) in &pv_ca_mp {
        //cnt0 += *v;
        let mut kk = k.to_string();
        let mut vv = *v;
        if k == "ยะลา" {
            if let Some(v2) = pv_ca_mp.get("สาขา อ.เบตง") {
                //let v1 = *v2;
                vv += *v2;
            }
        }
        if kk == " พระนครศรีอยุธยา" {
            kk = "พระนครศรีอยุธยา".to_string();
        }
        if kk == "แม่ฮองสอน" {
            kk = "แม่ฮ่องสอน".to_string();
        }
        if kk == "สาขา อ.เบตง" {
            //print!("NO BETONG\n");
        } else {
            //print!("'{}' - {}\n", kk, vv);
            pv_ca_mp2.insert(kk.clone(), vv);
        }
    }

    //println!("pv_ca_mp2: {:?}", pv_ca_mp2);
    //println!("pv_ca_cn2: {:?}", pv_ca_cn2);

    let ev_adx = pv_adjust();
    let mut tk0 = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let ts = adx.0.to_string();
        if let Some(nn) = pv_ca_mp2.get_mut(&ts) {
            let tk = *nn * ev_adx[i].2 / 100.0;
            *nn -= tk;
            tk0 += tk;
        }
    }
    let mut ass_sm = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let _ts = adx.0.to_string();
        if let Some(cn) = pv_ca_mp2.get_mut(&adx.0.to_string()) {
            let ad = tk0 * ev_adx[i].1 / 100.0;
            ass_sm += ev_adx[i].1;
            *cn += ad;
        }
    }
    println!("assign %{}", ass_sm);

    let mut pv_car_reg_mp = HashMap::new();
    let mut total = 0.0f32;
    for (k, v) in &pv_ca_mp2 {
        //if ["กรุงเทพมหานคร","นนทบุรี","สมุทรปราการ"].contains(&k.as_str()) {
        if ["กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ"].contains(&k.as_str()) {
            continue;
        }
        let mut pv_ca_reg = EvDistCalc::default();
        pv_ca_reg.id = k.to_string();
        pv_ca_reg.ev_no = *v as f32;
        total += pv_ca_reg.ev_no;
        pv_car_reg_mp.insert(k.to_string(), pv_ca_reg);
    }

    //ev_car_all_reg = 500219.0
    //ev_car_reg_cnt = 76366.0 # https://www.posttoday.com/smart-city/704704

    let mut ev_reg_no = 500219.0;
    ev_reg_no += 76366.0;
    
    //ev_reg_no += 100000.0;
    for (_k, v) in &mut pv_car_reg_mp {
        if total > 0.0 {
            v.ev_pc = v.ev_no / total as f32;
            v.ev_ds = v.ev_pc * ev_reg_no;
        } else {
            v.ev_ds = 0.0;
        }
    }

    use std::fmt::Write;
    let mut ss = String::new();
    let rt0 = 0.1;
    //let rti = 0.01;
    let rti = 0.1/14.0;
    let mut ev_sm = 0.0;
    for pv in grp1() {
        if let Some(v) = pv_car_reg_mp.get(pv) {
            write!(ss, "{}", pv).unwrap();
            ev_sm += v.ev_ds;
            let mut mwh = v.ev_ds * 0.011 * 3.0 * 365.0;
            let mut rtc = rt0;
            for y in 2023..=2039 {
                mwh += mwh * rtc;
                if y>= 2025 {
                    write!(ss,"\t{}", mwh).unwrap();
                }
                rtc += rti;
            }
            write!(ss,"\n").unwrap();
        }
    }
    println!("ev sum: {}", ev_sm);
    if let Ok(_) = fs::write("prj1/ev-grw-1.txt", ss) { }
    //wk5prc.ev_prov_dist = pv_car_reg_mp.clone();
}

pub async fn prc45() -> Result<(), Box<dyn std::error::Error>> {
    car_reg_2023_b().await;
    Ok(())
}


async fn car_reg_2023_b() {
    let cfg = base().config.clone();

    /*
    let car_reg_bkk_province = "กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ,ยะลา,นราธิวาส,ปัตตานี,สกลนคร,กาฬสินธุ์,ตรัง,มหาสารคาม,มุกดาหาร,อุดรธานี,พัทลุง";
    let car_reg_bkk_percent = "30.0,25.0,15.0,80.0,80.0,80.0,80.0,80.0,80.0,80.0,80.0,80.0,80.0";
    let car_reg_to_province = "พระนครศรีอยุธยา,ปทุมธานี,ชลบุรี,นครปฐม,ฉะเชิงเทรา,สมุทรสาคร,ระยอง,ราชบุรี,ปราจีนบุรี";
    let car_reg_to_percent = "15.0,15.0,15.0,10.0,10.0,10.0,10.0,8.0,7.0";
    */

    //let car_reg_bkk_province = "กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ,ยะลา,นราธิวาส,ปัตตานี,สกลนคร,กาฬสินธุ์,ตรัง,มหาสารคาม,มุกดาหาร,อุดรธานี,พัทลุง";
    //let car_reg_bkk_percent = "30.0,25.0,15.0,80.0,80.0,80.0,80.0,80.0,50.0,80.0,80.0,80.0,80.0";

    let _car_reg_bkk_province = "กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ,เชียงใหม่,ยะลา,นราธิวาส,ปัตตานี,สกลนคร,กาฬสินธุ์,ตรัง,มหาสารคาม,มุกดาหาร,อุดรธานี,พัทลุง,นครศรีธรรมราช,ศรีสะเกษ,ร้อยเอ็ด,สุรินทร์,กาฬสินธุ์,สุโขทัย,แพร่,ประจวบคีรีขันธ์,พะเยา,ชุมพร,นครพนม,พิจิตร,บึงกาฬ,หนองบัวลำภู,หนองคาย,ตราด,สตูล,ชัยนาท,สิงห์บุรี";
    let _car_reg_bkk_percent = "30.0,25.0,15.0,5.0,80.0,80.0,80.0,80.0,80.0,50.0,80.0,80.0,80.0,80.0,80,80,80,80,80,80,80,80,80,80,80,80,80,80,80,80,80,80,80";
    
    //let car_reg_to_province = "สระแก้ว,กระบี่,ปทุมธานี,เพชรบุรี,นครปฐม,ฉะเชิงเทรา,สมุทรสาคร,ระยอง,ราชบุรี,ปราจีนบุรี,สระบุรี,เชียงใหม่,พระนครศรีอยุธยา,บุรีรัมย์,พิษณุโลก";
    //let car_reg_to_percent = "2.0,0.5,20.0,5.0,10.0,8.0,10.0,5.0,8.0,12.0,5.0,5.0,5.0,2.5,2.0";
    
    let _ev_adx = vec![
        ("ราชบุรี",5.0),
        ("นครสวรรค์",3.5),
        ("ระนอง",0.5),
        ("สมุทรสงคราม",0.2),
        ("สมุทรสาคร",5.0),
        ("ปทุมธานี",14.0),
        ("สระบุรี",2.0),
        ("สระแก้ว",2.0),
        ("กระบี่",1.5),
        ("สงขลา",5.0),
        ("เพชรบุรี",2.5),
        ("นครปฐม",6.0),
        ("ฉะเชิงเทรา",6.0),
        ("ระยอง",5.0),
        ("ปราจีนบุรี",8.0),
        ("สุราษฎร์ธานี",4.0),
        ("สระบุรี",1.0),
        ("นครราชสีมา",4.0),
        ("เชียงใหม่",5.0),
        ("บุรีรัมย์",2.5),
        ("พิษณุโลก",2.0),
        ("ชลบุรี",1.4),
        ("พระนครศรีอยุธยา",5.8),
        ("ขอนแก่น",5.8),
        ("ลพบุรี",1.4),
    ];

    let _cfg = cfg.read().await;

    let mut pv_ca_mp = load_pvcamp();
    let mut pv_ca_mp2 = HashMap::new();
    //let mut pv_ca_cn2 = HashMap::new();
    //let mut cnt0 = 0.0;
    pv_ca_mp.insert("กรุงเทพมหานคร".to_string(), 967297.0);
    for (k, v) in &pv_ca_mp {
        //cnt0 += *v;
        let mut kk = k.to_string();
        let mut vv = *v;
        if k == "ยะลา" {
            if let Some(v2) = pv_ca_mp.get("สาขา อ.เบตง") {
                //let v1 = *v2;
                vv += *v2;
            }
        }
        if kk == " พระนครศรีอยุธยา" {
            kk = "พระนครศรีอยุธยา".to_string();
        }
        if kk == "แม่ฮองสอน" {
            kk = "แม่ฮ่องสอน".to_string();
        }
        if kk == "สาขา อ.เบตง" {
            //print!("NO BETONG\n");
        } else {
            //print!("'{}' - {}\n", kk, vv);
            pv_ca_mp2.insert(kk.clone(), vv);
            //pv_ca_cn2.insert(kk, 0);
        }
    }

    /*
    //println!("pv_ca_mp2: {:?}", pv_ca_mp2);
    //println!("pv_ca_cn2: {:?}", pv_ca_cn2);
    let mut tk0 = 0.0;
    for (i, b) in bkks.iter().enumerate() {
        if let Some(nn) = pv_ca_mp2.get_mut(&b.to_string()) {
            let tk = *nn * bkkn[i] / 100.0;
            *nn -= tk;
            tk0 += tk;
            //print!("take from {} {}\n", b, tk);
        }
    }
    let mut ass_sm = 0.0;
    //for (i, t) in asss.iter().enumerate() {
    for (i, adx) in ev_adx.iter().enumerate() {
        let ts = adx.0.to_string();
        if let Some(cn) = pv_ca_mp2.get_mut(&adx.0.to_string()) {
            let ad = tk0 * ev_adx[i].1 / 100.0;
            ass_sm += ev_adx[i].1;
            *cn += ad;
            //print!("add to {} {}\n", t, ad);
        }
    }
    */

    let ev_adx = pv_adjust();
    let mut tk0 = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let ts = adx.0.to_string();
        if let Some(nn) = pv_ca_mp2.get_mut(&ts) {
            let tk = *nn * ev_adx[i].2 / 100.0;
            *nn -= tk;
            tk0 += tk;
        }
    }
    let mut ass_sm = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let _ts = adx.0.to_string();
        if let Some(cn) = pv_ca_mp2.get_mut(&adx.0.to_string()) {
            let ad = tk0 * ev_adx[i].1 / 100.0;
            ass_sm += ev_adx[i].1;
            *cn += ad;
        }
    }
    
    println!("assign %{}", ass_sm);

    let mut pv_car_reg_mp = HashMap::new();
    let mut total = 0.0f32;
    for (k, v) in &pv_ca_mp2 {
        //if ["กรุงเทพมหานคร","นนทบุรี","สมุทรปราการ"].contains(&k.as_str()) {
        if ["กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ"].contains(&k.as_str()) {
            continue;
        }
        let mut pv_ca_reg = EvDistCalc::default();
        pv_ca_reg.id = k.to_string();
        pv_ca_reg.ev_no = *v as f32;
        total += pv_ca_reg.ev_no;
        pv_car_reg_mp.insert(k.to_string(), pv_ca_reg);
    }

    //ev_car_all_reg = 500219.0
    //ev_car_reg_cnt = 76366.0 # https://www.posttoday.com/smart-city/704704

    //let mut ev_reg_no = 500219.0

    //let ev_reg_no = 91654.0 + 75690.0;
    
    //ev_reg_no += 100000.0;
    for (_k, v) in &mut pv_car_reg_mp {
        if total > 0.0 {
            v.ev_pc = v.ev_no / total as f32;
        }
    }

    let ev_ls_yr = 75690.0;
    //let ev_ac_no = 500219.0 + ev_ls_yr; // accumulated ev car numbers
    let ev_ac_no = 100000.0 + ev_ls_yr; // accumulated ev car numbers

    let et_ls_yr = 238.0;
    let et_ac_no = 2962.0 + et_ls_yr;

    use std::fmt::Write;
    let mut ss = String::new();

    let (ev_rt0,ev_gw0) = (0.1,0.007);
    let (et_rt0,et_gw0) = (0.25,0.005);

    let /*mut*/ ev_sm = 0.0;

    let ev_mw = 0.011; // mw
    let ev_dy_hr = 4.0;
    let et_mw = 0.200; // mw
    let et_dy_hr = 8.0;

    for pv in grp1() {
        if let Some(v) = pv_car_reg_mp.get(pv) {
            write!(ss, "{}", pv).unwrap();
            let mut pv_ev_ac_no = ev_ac_no * v.ev_pc;
            let mut pv_ev_la_yr = ev_ls_yr * v.ev_pc;
            let mut ev_rt = ev_rt0;

            let mut pv_et_ac_no = et_ac_no * v.ev_pc;
            let mut pv_et_la_yr = et_ls_yr * v.ev_pc;
            let mut et_rt = et_rt0;

            for y in 2024..=2039 {
                ev_rt += ev_gw0;
                et_rt += et_gw0;

                pv_ev_la_yr = pv_ev_la_yr * (1.0+ev_rt);
                pv_et_la_yr = pv_et_la_yr * (1.0+et_rt);

                pv_ev_ac_no += pv_ev_la_yr;
                pv_et_ac_no += pv_et_la_yr;

                let ev_mwh = pv_ev_ac_no * ev_mw * ev_dy_hr * 360.0;
                let et_mwh = pv_et_ac_no * et_mw * et_dy_hr * 360.0;

                if y>= 2025 {
                    //write!(ss,"\t{}", pv_ev_la_yr); // new ev of the year
                    //write!(ss,"\t{}", pv_ev_ac_no); // evs of the year
                    //write!(ss,"\t{}", et_mwh); // evs of the year
                    //write!(ss,"\t{}", pv_et_la_yr); // new ev of the year
                    //write!(ss,"\t{}", pv_et_ac_no); // evs of the year
                    //write!(ss,"\t{}", et_mwh); // evs of the year
                    write!(ss,"\t{}", ev_mwh+et_mwh).unwrap(); // evs of the year
                }
            }
            /*
            let mut ev_yr_sa = v.ev_pc * ev_reg_no;
            ev_sm += ev_ds;
            let mut mwh = ev_ds * 0.011 * 3.0 * 365.0;
                ev_yr_sa += ev_tr_sa * ev_rt;
                ev_ds += v.ev_pc
                mwh += mwh * ev_rt;
                if y>= 2025 {
                    write!(ss,"\t{}", mwh);
                }
            }
            */
            write!(ss,"\n").unwrap();
        }
    }
    println!("ev sum: {}", ev_sm);
    if let Ok(_) = fs::write("prj1/ev-grw-1.txt", ss) { }
    //wk5prc.ev_prov_dist = pv_car_reg_mp.clone();
}


pub async fn prc46() -> Result<(), Box<dyn std::error::Error>> {
    car_reg_2023_c().await;
    Ok(())
}


fn pv_adjust_c() -> Vec::<(&'static str, f64, f64)> {
    vec![
        ("ชลบุรี", 1.4, 0.0,),
        ("ระยอง", 4.5, 0.0,),
        ("ฉะเชิงเทรา", 5.0, 0.0,),
        ("บุรีรัมย์", 0.0, 0.0,), // 1.5
        ("ปราจีนบุรี", 5.0, 0.0,), // 7.0
        ("นครปฐม", 5.0, 0.0,), // 6.0
        ("ภูเก็ต", 0.0, 3.0,),
        ("สมุทรสาคร", 4.0, 0.0,),
        ("พระนครศรีอยุธยา", 5.0, 0.0,),
        ("ปทุมธานี", 13.0, 0.0,),
        ("กรุงเทพมหานคร", 0.0, 30.0,),
        ("นนทบุรี", 0.0, 25.0,),
        ("สมุทรปราการ", 0.0, 15.0,),
        ("ยะลา", 0.0, 80.0,),
        ("นราธิวาส", 0.0, 80.0,),
        ("ปัตตานี", 0.0, 80.0,),
        ("สกลนคร", 0.0, 80.0,),
        ("กาฬสินธุ์", 0.0, 80.0,),
        ("ตรัง", 0.0, 50.0,),
        ("มหาสารคาม", 0.0, 80.0,),
        ("มุกดาหาร", 0.0, 80.0,),
        ("อุดรธานี", 0.0, 80.0,),
        ("พัทลุง", 0.0, 80.0,),
        ("นครศรีธรรมราช", 0.0, 80.0,),
        ("ศรีสะเกษ", 0.0, 80.0,),
        ("ร้อยเอ็ด", 0.0, 80.0,),
        ("สุรินทร์", 0.0, 80.0,),
        ("กาฬสินธุ์", 0.0, 80.0,),
        ("สุโขทัย", 0.0, 80.0,),
        ("แพร่", 0.0, 80.0,),
        ("ประจวบคีรีขันธ์", 0.0, 80.0,),
        ("พะเยา", 0.0, 80.0,),
        ("ชุมพร", 0.0, 80.0,),
        ("นครพนม", 0.0, 80.0,),
        ("พิจิตร", 0.0, 80.0,),
        ("บึงกาฬ", 0.0, 80.0,),
        ("หนองบัวลำภู", 0.0, 80.0,),
        ("หนองคาย", 0.0, 80.0,),
        ("ตราด", 0.0, 80.0,),
        ("สตูล", 0.0, 80.0,),
        ("ชัยนาท", 0.0, 80.0,),
        ("สิงห์บุรี", 0.0, 80.0,),
        ("ราชบุรี", 5.0, 0.0,),
        ("นครสวรรค์", 3.2, 0.0,),
        ("ระนอง", 0.4, 0.0,),
        ("สมุทรสงคราม", 0.2, 0.0,),
        ("กระบี่", 1.3, 0.0,),
        ("สงขลา", 4.9, 0.0,),
        ("เพชรบุรี", 2.4, 0.0,),
        ("สุราษฎร์ธานี", 4.0, 0.0,),
        ("สระบุรี", 2.7, 0.0,),
        ("สระแก้ว", 1.8, 0.0,),
        ("นครราชสีมา", 4.0, 0.0,),
        ("เชียงใหม่", 3.8, 0.0,),
        ("พิษณุโลก", 1.8, 0.0,),
        ("ขอนแก่น", 5.8, 0.0,),
        ("ลพบุรี", 1.4, 0.0,),
    ]
}


async fn car_reg_2023_c() {
    let mut pv_ca_mp = load_pvcamp();
    let mut pv_ca_mp2 = HashMap::new();
    //let mut cnt0 = 0.0;
    pv_ca_mp.insert("กรุงเทพมหานคร".to_string(), 967297.0);
    for (k, v) in &pv_ca_mp {
        //cnt0 += *v;
        let mut kk = k.to_string();
        let mut vv = *v;
        if k == "ยะลา" {
            if let Some(v2) = pv_ca_mp.get("สาขา อ.เบตง") {
                //let v1 = *v2;
                vv += *v2;
            }
        }
        if kk == " พระนครศรีอยุธยา" {
            kk = "พระนครศรีอยุธยา".to_string();
        }
        if kk == "แม่ฮองสอน" {
            kk = "แม่ฮ่องสอน".to_string();
        }
        if kk == "สาขา อ.เบตง" {
            //print!("NO BETONG\n");
        } else {
            //print!("'{}' - {}\n", kk, vv);
            pv_ca_mp2.insert(kk.clone(), vv);
            //pv_ca_cn2.insert(kk, 0);
        }
    }

    let ev_adx = pv_adjust_c();
    let mut tk0 = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let ts = adx.0.to_string();
        if let Some(nn) = pv_ca_mp2.get_mut(&ts) {
            let tk = *nn * ev_adx[i].2 / 100.0;
            *nn -= tk;
            tk0 += tk;
        }
    }
    let mut ass_sm = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let _ts = adx.0.to_string();
        if let Some(cn) = pv_ca_mp2.get_mut(&adx.0.to_string()) {
            let ad = tk0 * ev_adx[i].1 / 100.0;
            ass_sm += ev_adx[i].1;
            *cn += ad;
        }
    }
    
    println!("assign %{}", ass_sm);

    let mut pv_car_reg_mp = HashMap::new();
    let mut total = 0.0f32;
    for (k, v) in &pv_ca_mp2 {
        if ["กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ"].contains(&k.as_str()) {
            continue;
        }
        let mut pv_ca_reg = EvDistCalc::default();
        pv_ca_reg.id = k.to_string();
        pv_ca_reg.ev_no = *v as f32;
        total += pv_ca_reg.ev_no;
        pv_car_reg_mp.insert(k.to_string(), pv_ca_reg);
    }

    for (_k, v) in &mut pv_car_reg_mp {
        if total > 0.0 {
            v.ev_pc = v.ev_no / total as f32;
        }
    }

    let ev_ls_yr = 75690.0;
    let ev_ac_no = 89907.0 + ev_ls_yr;

    let et_ls_yr = 238.0 * 3.0;
    let et_ac_no = 2962.0 + et_ls_yr;

    use std::fmt::Write;
    let mut ss = String::new();

    let (ev_rt0,ev_gw0) = (0.1,0.007);
    let (et_rt0,et_gw0) = (0.2,0.005);

    let ev_mw = 0.007; // mw
    let ev_dy_hr = 3.0;
    let et_mw = 0.150; // mw
    let et_dy_hr = 6.0;

    let (mut s_ev_y, mut s_ev_a, mut s_ev_mw, mut s_ev_wh) = (ss.clone(), ss.clone(), ss.clone(), ss.clone(), );
    let (mut s_et_y, mut s_et_a, mut s_et_mw, mut s_et_wh) = (ss.clone(), ss.clone(), ss.clone(), ss.clone(), );

    for pv in grp1() {
        if let Some(v) = pv_car_reg_mp.get(pv) {
            write!(ss, "{}", pv).unwrap();
            
            write!(s_ev_y, "{}", pv).unwrap();
            write!(s_ev_a, "{}", pv).unwrap();
            write!(s_ev_mw, "{}", pv).unwrap();
            write!(s_ev_wh, "{}", pv).unwrap();
            write!(s_et_y, "{}", pv).unwrap();
            write!(s_et_a, "{}", pv).unwrap();
            write!(s_et_mw, "{}", pv).unwrap();
            write!(s_et_wh, "{}", pv).unwrap();

            let mut pv_ev_ac_no = ev_ac_no * v.ev_pc;
            let mut pv_ev_la_yr = ev_ls_yr * v.ev_pc;
            let mut ev_rt = ev_rt0;

            let mut pv_et_ac_no = et_ac_no * v.ev_pc;
            let mut pv_et_la_yr = et_ls_yr * v.ev_pc;
            let mut et_rt = et_rt0;

            for y in 2024..=2039 {
                ev_rt += ev_gw0;
                et_rt += et_gw0;

                pv_ev_la_yr = pv_ev_la_yr * (1.0+ev_rt);
                pv_et_la_yr = pv_et_la_yr * (1.0+et_rt);

                pv_ev_ac_no += pv_ev_la_yr;
                pv_et_ac_no += pv_et_la_yr;

                let pv_ev_mw = pv_ev_ac_no * ev_mw * ev_dy_hr;
                let pv_et_mw = pv_et_ac_no * et_mw * et_dy_hr;

                let pv_ev_mwh = pv_ev_mw * 360.0;
                let pv_et_mwh = pv_et_mw * 360.0;

                if y>= 2025 {
                    //write!(ss,"\t{}", pv_ev_la_yr); // new ev of the year
                    //write!(ss,"\t{}", pv_ev_ac_no); // evs of the year
                    //write!(ss,"\t{}", et_mwh); // evs of the year
                    //write!(ss,"\t{}", pv_et_la_yr); // new ev of the year
                    //write!(ss,"\t{}", pv_et_ac_no); // evs of the year
                    //write!(ss,"\t{}", et_mwh); // evs of the year
                    write!(ss,"\t{}", pv_ev_mwh+pv_et_mwh).unwrap(); // evs of the year

                    write!(s_ev_y, "\t{}", pv_ev_la_yr).unwrap();
                    write!(s_ev_a, "\t{}", pv_ev_ac_no).unwrap();
                    write!(s_ev_mw, "\t{}", pv_ev_mw).unwrap();
                    write!(s_ev_wh, "\t{}", pv_ev_mwh).unwrap();

                    write!(s_et_y, "\t{}", pv_et_la_yr).unwrap();
                    write!(s_et_a, "\t{}", pv_et_ac_no).unwrap();
                    write!(s_et_mw, "\t{}", pv_et_mw).unwrap();
                    write!(s_et_wh, "\t{}", pv_et_mwh).unwrap();
                }
            }
            write!(ss,"\n").unwrap();
            write!(s_ev_y, "\n").unwrap();
            write!(s_ev_a, "\n").unwrap();
            write!(s_ev_mw, "\n").unwrap();
            write!(s_ev_wh, "\n").unwrap();
            write!(s_et_y, "\n").unwrap();
            write!(s_et_a, "\n").unwrap();
            write!(s_et_mw, "\n").unwrap();
            write!(s_et_wh, "\n").unwrap();
        }
    }
    if let Ok(_) = fs::write("prj1/ev-grw-1.txt", ss) { }

    if let Ok(_) = fs::write("prj1/s_ev_y.txt", s_ev_y) { }
    if let Ok(_) = fs::write("prj1/s_ev_a.txt", s_ev_a) { }
    if let Ok(_) = fs::write("prj1/s_ev_mw.txt", s_ev_mw) { }
    if let Ok(_) = fs::write("prj1/s_ev_wh.txt", s_ev_wh) { }

    if let Ok(_) = fs::write("prj1/s_et_y.txt", s_et_y) { }
    if let Ok(_) = fs::write("prj1/s_et_a.txt", s_et_a) { }
    if let Ok(_) = fs::write("prj1/s_et_mw.txt", s_et_mw) { }
    if let Ok(_) = fs::write("prj1/s_et_wh.txt", s_et_wh) { }
}

pub async fn prc47() -> Result<(), Box<dyn std::error::Error>> {
   car_reg_2023_d().await;
   Ok(())
}

use crate::sg::prc3::ld_p3_prvs;
async fn car_reg_2023_d() {
    let prvs = ld_p3_prvs();
    let mut pv_ca_mp = load_pvcamp();
    let mut pv_ca_mp2 = HashMap::new();
    //let mut cnt0 = 0.0;
    pv_ca_mp.insert("กรุงเทพมหานคร".to_string(), 967297.0);
    for (k, v) in &pv_ca_mp {
        //cnt0 += *v;
        let mut kk = k.to_string();
        let mut vv = *v;
        if k == "ยะลา" {
            if let Some(v2) = pv_ca_mp.get("สาขา อ.เบตง") {
                //let v1 = *v2;
                vv += *v2;
            }
        }
        if kk == " พระนครศรีอยุธยา" {
            kk = "พระนครศรีอยุธยา".to_string();
        }
        if kk == "แม่ฮองสอน" {
            kk = "แม่ฮ่องสอน".to_string();
        }
        if kk == "สาขา อ.เบตง" {
            //print!("NO BETONG\n");
        } else {
            //print!("'{}' - {}\n", kk, vv);
            pv_ca_mp2.insert(kk.clone(), vv);
            //pv_ca_cn2.insert(kk, 0);
        }
    }

    let ev_adx = pv_adjust_c();
    let mut tk0 = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let ts = adx.0.to_string();
        if let Some(nn) = pv_ca_mp2.get_mut(&ts) {
            let tk = *nn * ev_adx[i].2 / 100.0;
            *nn -= tk;
            tk0 += tk;
        }
    }
    let mut ass_sm = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let _ts = adx.0.to_string();
        if let Some(cn) = pv_ca_mp2.get_mut(&adx.0.to_string()) {
            let ad = tk0 * ev_adx[i].1 / 100.0;
            ass_sm += ev_adx[i].1;
            *cn += ad;
        }
    }
    
    println!("assign %{}", ass_sm);

    let mut pv_car_reg_mp = HashMap::new();
    let mut total = 0.0f32;
    for (k, v) in &pv_ca_mp2 {
        if ["กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ"].contains(&k.as_str()) {
            continue;
        }
        let mut pv_ca_reg = EvDistCalc::default();
        pv_ca_reg.id = k.to_string();
        pv_ca_reg.ev_no = *v as f32;
        total += pv_ca_reg.ev_no;
        pv_car_reg_mp.insert(k.to_string(), pv_ca_reg);
    }

    for (_k, v) in &mut pv_car_reg_mp {
        if total > 0.0 {
            v.ev_pc = v.ev_no / total as f32;
        }
    }

    let ev_ls_yr = 75690.0;
    let ev_ac_no = 89907.0 + ev_ls_yr;

    let et_ls_yr = 238.0 * 3.0;
    let et_ac_no = 2962.0 + et_ls_yr;

    use std::fmt::Write;
    let mut ss = String::new();

    let (ev_rt0,ev_gw0) = (0.1,0.007);
    let (et_rt0,et_gw0) = (0.2,0.005);

    let ev_mw = 0.007; // mw
    let ev_dy_hr = 3.0;
    let et_mw = 0.150; // mw
    let et_dy_hr = 6.0;

    let (mut s_ev_y, mut s_ev_a, mut s_ev_mw, mut s_ev_wh) = (ss.clone(), ss.clone(), ss.clone(), ss.clone(), );
    let (mut s_et_y, mut s_et_a, mut s_et_mw, mut s_et_wh) = (ss.clone(), ss.clone(), ss.clone(), ss.clone(), );

    for pv in &prvs {
        if let Some(v) = pv_car_reg_mp.get(pv) {
            write!(ss, "{}", pv).unwrap();
            
            write!(s_ev_y, "{}", pv).unwrap();
            write!(s_ev_a, "{}", pv).unwrap();
            write!(s_ev_mw, "{}", pv).unwrap();
            write!(s_ev_wh, "{}", pv).unwrap();
            write!(s_et_y, "{}", pv).unwrap();
            write!(s_et_a, "{}", pv).unwrap();
            write!(s_et_mw, "{}", pv).unwrap();
            write!(s_et_wh, "{}", pv).unwrap();

            let mut pv_ev_ac_no = ev_ac_no * v.ev_pc;
            let mut pv_ev_la_yr = ev_ls_yr * v.ev_pc;
            let mut ev_rt = ev_rt0;

            let mut pv_et_ac_no = et_ac_no * v.ev_pc;
            let mut pv_et_la_yr = et_ls_yr * v.ev_pc;
            let mut et_rt = et_rt0;

            for y in 2024..=2039 {
                ev_rt += ev_gw0;
                et_rt += et_gw0;

                pv_ev_la_yr = pv_ev_la_yr * (1.0+ev_rt);
                pv_et_la_yr = pv_et_la_yr * (1.0+et_rt);

                pv_ev_ac_no += pv_ev_la_yr;
                pv_et_ac_no += pv_et_la_yr;

                let pv_ev_mw = pv_ev_ac_no * ev_mw * ev_dy_hr;
                let pv_et_mw = pv_et_ac_no * et_mw * et_dy_hr;

                let pv_ev_mwh = pv_ev_mw * 360.0;
                let pv_et_mwh = pv_et_mw * 360.0;

                if y>= 2025 {
                    //write!(ss,"\t{}", pv_ev_la_yr); // new ev of the year
                    //write!(ss,"\t{}", pv_ev_ac_no); // evs of the year
                    //write!(ss,"\t{}", et_mwh); // evs of the year
                    //write!(ss,"\t{}", pv_et_la_yr); // new ev of the year
                    //write!(ss,"\t{}", pv_et_ac_no); // evs of the year
                    //write!(ss,"\t{}", et_mwh); // evs of the year
                    write!(ss,"\t{}", pv_ev_mwh+pv_et_mwh).unwrap(); // evs of the year

                    write!(s_ev_y, "\t{}", pv_ev_la_yr).unwrap();
                    write!(s_ev_a, "\t{}", pv_ev_ac_no).unwrap();
                    write!(s_ev_mw, "\t{}", pv_ev_mw).unwrap();
                    write!(s_ev_wh, "\t{}", pv_ev_mwh).unwrap();

                    write!(s_et_y, "\t{}", pv_et_la_yr).unwrap();
                    write!(s_et_a, "\t{}", pv_et_ac_no).unwrap();
                    write!(s_et_mw, "\t{}", pv_et_mw).unwrap();
                    write!(s_et_wh, "\t{}", pv_et_mwh).unwrap();
                }
            }
            write!(ss,"\n").unwrap();
            write!(s_ev_y, "\n").unwrap();
            write!(s_ev_a, "\n").unwrap();
            write!(s_ev_mw, "\n").unwrap();
            write!(s_ev_wh, "\n").unwrap();
            write!(s_et_y, "\n").unwrap();
            write!(s_et_a, "\n").unwrap();
            write!(s_et_mw, "\n").unwrap();
            write!(s_et_wh, "\n").unwrap();
        }
    }
    if let Ok(_) = fs::write("prj1/a_ev-grw-1.txt", ss) { }

    if let Ok(_) = fs::write("prj1/a_s_ev_y.txt", s_ev_y) { }
    if let Ok(_) = fs::write("prj1/a_s_ev_a.txt", s_ev_a) { }
    if let Ok(_) = fs::write("prj1/a_s_ev_mw.txt", s_ev_mw) { }
    if let Ok(_) = fs::write("prj1/a_s_ev_wh.txt", s_ev_wh) { }

    if let Ok(_) = fs::write("prj1/a_s_et_y.txt", s_et_y) { }
    if let Ok(_) = fs::write("prj1/a_s_et_a.txt", s_et_a) { }
    if let Ok(_) = fs::write("prj1/a_s_et_mw.txt", s_et_mw) { }
    if let Ok(_) = fs::write("prj1/a_s_et_wh.txt", s_et_wh) { }
}



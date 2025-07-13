use crate::p_31::AreaRatio;
use crate::prc1::FeederLoadRaw;
use crate::prc1::LoadProfVal;
use crate::prc4::BC_ON_PEAK_BEGIN;
use crate::prc4::BC_ON_PEAK_END;
use askama::Template;
use axum;
use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sglab02_lib::sg::prc3::ld_p3_prv_sub;
use sglab02_lib::sg::prc3::ld_p3_sub_inf;
use sglab02_lib::sg::prc3::ld_sub_loc;
use sglab02_lib::sg::prc4::grp1;
use sglab02_lib::sg::prc4::ld_pv_sbv_m;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;

pub fn lp_ana3() -> Result<(), Box<dyn std::error::Error>> {
    let pv = grp1();
    //let pvsb = ld_p3_prv_sub();
    let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();
    for p in &pv {
        let pp = p.to_string();
        println!("pv:{}", p);
        if let Some(sbv) = sbsl.get(&pp) {
            for sb in sbv {
                if let Some(_sbif) = sbif.get(&sb.sbid) {
                    println!("  sb:{}", sb.sbid);
                }
            }
        }
    }
    Ok(())
}

pub fn lp_ana4() -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir(LP_RAW_DIR)?;
    let mut fipav = Vec::<(String, String, String, String)>::new();
    for path in paths.flatten() {
        let pt = path.path();
        if let Some(fnm) = pt.file_stem() {
            if let Some(fnm) = fnm.to_str() {
                if !pt.is_file() {
                    let pt2 = format!("{}/{}", LP_RAW_DIR, fnm);
                    let cald = format!("{}/{}", LP_CAL_DIR, fnm);
                    let _ = fs::create_dir_all(cald);
                    //println!("dir: {} -> pt2:{}", fnm, pt2);
                    let pt2 = fs::read_dir(pt2)?;
                    for fl in pt2.flatten() {
                        let fl = fl.path();
                        if let Some(fnm2) = fl.file_stem() {
                            if let Some(fnm2) = fnm2.to_str() {
                                let flnm = format!("{}/{}/{}.bin", LP_RAW_DIR, fnm, fnm2);
                                let flto = format!("{}/{}/{}.bin", LP_CAL_DIR, fnm, fnm2);
                                fipav.push((flnm, flto, fnm.to_string(), fnm2.to_string()));
                                //println!("  {} {} {} {}", fnm, fnm2, fl.is_file(), flnm);
                            }
                        }
                    }
                }
            }
        }
    }
    let sblo = ld_sub_loc();
    let mut all = HashMap::<String, Vec<SubLoadProf>>::new();
    for (fm, to, yr, sb) in fipav {
        if let Ok(f) = File::open(&fm) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<Box<FeederLoadRaw>>>(
                BufReader::new(f),
            ) {
                if let Some(mut ra) = all.get_mut(&yr) {
                    calc_and_save(dt, &to, &yr, &sb, &mut ra, &sblo).expect("?");
                } else {
                    let mut ra = Vec::<SubLoadProf>::new();
                    calc_and_save(dt, &to, &yr, &sb, &mut ra, &sblo).expect("?");
                    all.insert(yr.to_string(), ra);
                }
            }
        }
    }
    for (k, ys) in &all {
        let to = format!("{}/{}/{}.bin", LP_CAL_DIR, k, k);
        //println!("k:{} sb:{} f:{}", k, ys.len(), to);
        if let Ok(ser) = bincode::serialize(&ys) {
            std::fs::write(to, ser).unwrap();
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SubLoadProf {
    pub sub: String,
    pub year: String,
    pub time_r: Vec<LoadProfVal>,
    pub time_v: Vec<f32>,
    pub feed: Vec<FeedLoadProf>,
    pub valid: DataValid,
    pub calc_r: PowerCalc,
    pub calc_v: PowerCalc,
    pub calc_day: PowerCalc,
    pub calc_ngt: PowerCalc,
    pub calc_onp: PowerCalc,
    pub calc_ofp: PowerCalc,
    pub load_factor: f32,
    pub diversity_factor: f32,
    pub day_prof: Vec<f32>,
    pub maxmva: f32,
    pub latlong: Option<(f64, f64)>,
    pub ms_time_prof: Vec<TimeProf>,
    pub ds_time_prof: Vec<TimeProf>,
    pub pk30_time_prof: TimeProf,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FeedLoadProf {
    pub sub: String,
    pub feed: String,
    pub time_r: Vec<LoadProfVal>,
    pub time_v: Vec<f32>,
    pub valid: DataValid,
    pub calc_r: PowerCalc,
    pub calc_v: PowerCalc,
    pub calc_day: PowerCalc,
    pub calc_ngt: PowerCalc,
    pub calc_onp: PowerCalc,
    pub calc_ofp: PowerCalc,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerCalc {
    pub p_pk: f32,
    pub p_pk2: f32,
    pub p_cnt: usize,
    pub p_sum: f32,
    pub p_avg: f32,
    pub p_en: f32,

    pub n_pk: f32,
    pub n_pk2: f32,
    pub n_cnt: usize,
    pub n_sum: f32,
    pub n_avg: f32,
    pub n_en: f32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataValid {
    pub good: usize,
    pub null: usize,
    pub none: usize,
}

use sglab02_lib::sg::prc1::SubstInfo;

pub fn calc_and_save(
    dt: Vec<Box<FeederLoadRaw>>,
    to: &str,
    yr: &str,
    sb: &str,
    all: &mut Vec<SubLoadProf>,
    sblo: &HashMap<String, (f64, f64)>,
) -> Result<(), Box<dyn std::error::Error>> {
    //println!("calc to {} {}", fnm, dt.len());
    let mut sbif = SubstInfo::default();
    if let Some(sf) = ld_p3_sub_inf().get(sb) {
        sbif = sf.clone();
    }
    let re = Regex::new(r"..._[0-9][0-9][VW]B01").unwrap();
    let sub = sb.to_string();
    let time_r = Vec::<LoadProfVal>::new();
    let time_v = vec![0f32; 365];
    let feed = Vec::<FeedLoadProf>::new();
    let latlong = sblo.get(sb).copied();
    let year = yr.to_string();
    let mut sublp = SubLoadProf {
        sub,
        year,
        time_r,
        time_v,
        feed,
        latlong,
        ..Default::default()
    };
    for lprw in dt {
        if !re.is_match(&lprw.feed) {
            continue;
        }
        let ff = format!("{}{}", &lprw.feed[0..3], &lprw.feed[4..6]);
        let sub = sb.to_string();
        let feed = ff;
        let time_r = lprw.time_r.clone();
        let time_v = vec![0f32; 365];
        let flp = FeedLoadProf {
            sub,
            feed,
            time_r,
            time_v,
            ..Default::default()
        };
        sublp.feed.push(flp);
    }
    sublp.feed.sort_by(|a, b| a.feed.cmp(&b.feed));
    for i in 0..(365 * 48) {
        let (mut _va, mut nu, mut no, mut val) = (0, 0, 0, 0f32);
        for j in 0..sublp.feed.len() {
            match sublp.feed[j].time_r[i] {
                LoadProfVal::Value(v) => {
                    _va += 1;
                    val += v;
                }
                LoadProfVal::Null => nu += 1,
                LoadProfVal::None => no += 1,
            }
        }
        let vv = if nu + no == 0 {
            LoadProfVal::Value(val)
        } else {
            LoadProfVal::None
        };
        sublp.time_r.push(vv);
    }
    sublp.time_v = val_er_cor(&sublp.time_r);
    sublp.valid = val_er_cnt(&sublp.time_r);
    sublp.calc_r = pow_calc_real(&sublp.time_r);
    sublp.calc_v = pow_calc_val(&sublp.time_v);
    sublp.calc_day = pow_calc_day(&sublp.time_v);
    sublp.calc_ngt = pow_calc_ngt(&sublp.time_v);
    pow_calc_factor(&mut sublp);
    calc_time_prof(&mut sublp);
    //sublp.valid = DataValid { good, null, none };
    for fd in &mut sublp.feed {
        fd.time_v = val_er_cor(&fd.time_r);
        fd.valid = val_er_cnt(&fd.time_r);
        fd.calc_r = pow_calc_real(&fd.time_r);
        fd.calc_v = pow_calc_val(&fd.time_v);
        fd.calc_day = pow_calc_day(&sublp.time_v);
        fd.calc_ngt = pow_calc_ngt(&sublp.time_v);
    }
    let maxmva = sbif.mvxn as f32;
    sublp.maxmva = maxmva;
    if let Ok(ser) = bincode::serialize(&sublp) {
        std::fs::write(to, ser).unwrap();
    }
    //println!("to:{} yr:{} sb:{}", to, yr, sb);

    let feed = sublp.feed;
    sublp.feed = Vec::new();
    let sublp2 = sublp.clone();
    //println!("1.{} {}", sublp.feed.len(), sublp2.feed.len());
    sublp.feed = feed;
    //println!("2.{} {}", sublp.feed.len(), sublp2.feed.len());

    all.push(sublp2);
    /*
    if sublp.maxmva > 0f32 {
        let perc = sublp.calc_r.p_pk / sublp.maxmva * 100f32;
        if perc > 90f32 {
            println!(
                "sb:{} mx:{:.2} pk:{:.2} pk2:{:.1} av:{:.2} p:{:.2}",
                sb, sublp.maxmva, sublp.calc_r.p_pk, sublp.calc_r.p_pk2, sublp.calc_r.p_avg, perc
            );
        }
    }
    */
    Ok(())
}

fn pow_calc_day(time_v: &[f32]) -> PowerCalc {
    let mut pwc = PowerCalc::default();
    for (i, v) in time_v.iter().enumerate() {
        if i < 6 * 2 || i > 18 * 2 {
            continue;
        } // day time 6 - 18
        if *v >= 0f32 {
            pwc.p_sum += *v;
            pwc.p_cnt += 1;
            if *v > pwc.p_pk {
                pwc.p_pk = *v;
            }
        } else {
            pwc.n_sum += -*v;
            pwc.n_cnt += 1;
            if -*v > pwc.n_pk {
                pwc.n_pk = -*v;
            }
        }
    }
    for (i, v) in time_v.iter().enumerate() {
        if i < 6 * 2 || i > 18 * 2 {
            continue;
        } // day time 6 - 18
        if *v >= 0f32 {
            if *v < pwc.p_pk && *v > pwc.p_pk2 {
                pwc.p_pk2 = *v;
            }
        } else {
            if -*v < pwc.n_pk && -*v > pwc.n_pk2 {
                pwc.n_pk2 = -*v;
            }
        }
    }
    if pwc.p_pk2 == 0f32 {
        pwc.p_pk2 = pwc.p_pk;
    }
    if pwc.n_pk2 == 0f32 {
        pwc.n_pk2 = pwc.n_pk;
    }

    pwc.p_en = pwc.p_sum / 2f32;
    pwc.n_en = pwc.n_sum / 2f32;
    if pwc.p_cnt > 0 {
        pwc.p_avg = pwc.p_sum / pwc.p_cnt as f32;
    }
    if pwc.n_cnt > 0 {
        pwc.n_avg = pwc.n_sum / pwc.n_cnt as f32;
    }
    pwc
}

fn pow_calc_ngt(time_v: &[f32]) -> PowerCalc {
    let mut pwc = PowerCalc::default();
    for (i, v) in time_v.iter().enumerate() {
        if i >= 6 * 2 && i <= 18 * 2 {
            continue;
        } // night time
        if *v >= 0f32 {
            pwc.p_sum += *v;
            pwc.p_cnt += 1;
            if *v > pwc.p_pk {
                pwc.p_pk = *v;
            }
        } else {
            pwc.n_sum += -*v;
            pwc.n_cnt += 1;
            if -*v > pwc.n_pk {
                pwc.n_pk = -*v;
            }
        }
    }
    for (i, v) in time_v.iter().enumerate() {
        if i >= 6 * 2 && i <= 18 * 2 {
            continue;
        } // night time
        if *v >= 0f32 {
            if *v < pwc.p_pk && *v > pwc.p_pk2 {
                pwc.p_pk2 = *v;
            }
        } else {
            if -*v < pwc.n_pk && -*v > pwc.n_pk2 {
                pwc.n_pk2 = -*v;
            }
        }
    }
    if pwc.p_pk2 == 0f32 {
        pwc.p_pk2 = pwc.p_pk;
    }
    if pwc.n_pk2 == 0f32 {
        pwc.n_pk2 = pwc.n_pk;
    }

    pwc.p_en = pwc.p_sum / 2f32;
    pwc.n_en = pwc.n_sum / 2f32;
    if pwc.p_cnt > 0 {
        pwc.p_avg = pwc.p_sum / pwc.p_cnt as f32;
    }
    if pwc.n_cnt > 0 {
        pwc.n_avg = pwc.n_sum / pwc.n_cnt as f32;
    }
    pwc
}

fn pow_calc_peak(time_v: &[f32]) -> (PowerCalc, PowerCalc) {
    let mut pwn = PowerCalc::default();
    let mut pwf = PowerCalc::default();
    for (i, v) in time_v.iter().enumerate() {
        if i >= BC_ON_PEAK_BEGIN && i <= BC_ON_PEAK_END {
            if *v >= 0f32 {
                pwn.p_sum += *v;
                pwn.p_cnt += 1;
                if *v > pwn.p_pk {
                    pwn.p_pk = *v;
                }
            } else {
                pwn.n_sum += -*v;
                pwn.n_cnt += 1;
                if -*v > pwn.n_pk {
                    pwn.n_pk = -*v;
                }
            }
        } else {
            if *v >= 0f32 {
                pwf.p_sum += *v;
                pwf.p_cnt += 1;
                if *v > pwf.p_pk {
                    pwf.p_pk = *v;
                }
            } else {
                pwf.n_sum += -*v;
                pwf.n_cnt += 1;
                if -*v > pwf.n_pk {
                    pwf.n_pk = -*v;
                }
            }
        }
    }
    pwn.p_en = pwn.p_sum / 2f32;
    pwn.n_en = pwn.n_sum / 2f32;
    if pwn.p_cnt > 0 {
        pwn.p_avg = pwn.p_sum / pwn.p_cnt as f32;
    }
    if pwn.n_cnt > 0 {
        pwn.n_avg = pwn.n_sum / pwn.n_cnt as f32;
    }
    pwf.p_en = pwf.p_sum / 2f32;
    pwf.n_en = pwf.n_sum / 2f32;
    if pwf.p_cnt > 0 {
        pwf.p_avg = pwf.p_sum / pwf.p_cnt as f32;
    }
    if pwf.n_cnt > 0 {
        pwf.n_avg = pwf.n_sum / pwf.n_cnt as f32;
    }
    (pwn, pwf)
}

fn pow_calc_val(time_v: &[f32]) -> PowerCalc {
    let mut pwc = PowerCalc::default();
    for v in time_v {
        if *v >= 0f32 {
            pwc.p_sum += *v;
            pwc.p_cnt += 1;
            if *v > pwc.p_pk {
                pwc.p_pk = *v;
            }
        } else {
            pwc.n_sum += -*v;
            pwc.n_cnt += 1;
            if -*v > pwc.n_pk {
                pwc.n_pk = -*v;
            }
        }
    }
    for v in time_v {
        if *v >= 0f32 {
            if *v < pwc.p_pk && *v > pwc.p_pk2 {
                pwc.p_pk2 = *v;
            }
        } else {
            if -*v < pwc.n_pk && -*v > pwc.n_pk2 {
                pwc.n_pk2 = -*v;
            }
        }
    }
    if pwc.p_pk2 == 0f32 {
        pwc.p_pk2 = pwc.p_pk;
    }
    if pwc.n_pk2 == 0f32 {
        pwc.n_pk2 = pwc.n_pk;
    }

    pwc.p_en = pwc.p_sum / 2f32;
    pwc.n_en = pwc.n_sum / 2f32;
    if pwc.p_cnt > 0 {
        pwc.p_avg = pwc.p_sum / pwc.p_cnt as f32;
    }
    if pwc.n_cnt > 0 {
        pwc.n_avg = pwc.n_sum / pwc.n_cnt as f32;
    }
    pwc
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TimeProf {
    pub day: usize,
    pub month: usize,
    pub time_v: Vec<f32>,
    pub calc_v: PowerCalc,
    pub calc_onp: PowerCalc,
    pub calc_ofp: PowerCalc,
}
fn calc_time_prof(sblp: &mut SubLoadProf) {
    let mut dypk = 0f32;
    let mut dlp = [0f32; 48];
    for (ih, dd) in dlp.iter_mut().enumerate().take(48) {
        let (mut vd, mut cn) = (0f32, 0);
        for id in 0..365 {
            //for (id, rd) in sblp.time_r.iter().enumerate() {
            let ii = id * 48 + ih;
            if let LoadProfVal::Value(v) = sblp.time_r[ii] {
                vd += v;
                cn += 1;
                dypk = if v > dypk { v } else { dypk };
            }
        }
        *dd = vd / cn as f32;
    }
    sblp.day_prof = dlp.to_vec();
    /*
    if dypk > 5f32 {
        println!("dypf:{} {}", sblp.sub, dypk);
    }
    */

    // months time day_prof
    let mut ms_time_prof = Vec::<TimeProf>::new();
    for mi in 0..12 {
        let mut time_v = vec![0f32; 48];
        let mut pk = 0f32;
        for hi in 0..48 {
            let mut cn = 0;
            for di in 0..30 {
                let ii = (mi * 30 + di) * 48 + hi;
                if let LoadProfVal::Value(v) = sblp.time_r[ii] {
                    time_v[hi] += v;
                    cn += 1;
                    pk = if v > pk { v } else { pk };
                }
            }
            time_v[hi] /= cn as f32;
        }
        let mut m_time_prof = TimeProf::default();
        m_time_prof.month = mi;
        m_time_prof.calc_v = pow_calc_val(&time_v);
        m_time_prof.time_v = time_v;
        /*
        if pk > 5f32 {
            println!(
                "mpf sb:{} m:{} pk:{} pk0:{}",
                sblp.sub, mi, pk, m_time_prof.calc_v.p_pk
            );
        }
        */
        ms_time_prof.push(m_time_prof);
    }
    sblp.ms_time_prof = ms_time_prof;

    // day time profile
    let mut ds_time_prof = Vec::<TimeProf>::new();
    for di in 0..365 {
        let mut time_v = vec![0f32; 48];
        for hi in 0..48 {
            let ii = di * 48 + hi;
            //println!("{di},{hi},{ii}");
            if let LoadProfVal::Value(v) = sblp.time_r[ii] {
                time_v[hi] += v;
            }
        }
        let mut d_time_prof = TimeProf::default();
        d_time_prof.day = di;
        d_time_prof.calc_v = pow_calc_val(&time_v);
        d_time_prof.time_v = time_v;
        ds_time_prof.push(d_time_prof);
    }
    sblp.ds_time_prof = ds_time_prof;
    let mut ds_time_prof = sblp.ds_time_prof.clone();

    ds_time_prof.sort_by(|b, a| a.calc_v.p_pk.partial_cmp(&b.calc_v.p_pk).unwrap());
    let time_vs = Vec::from_iter(ds_time_prof[0..30].iter().cloned());

    let mut time_v = vec![0f32; 48];
    for hi in 0..48 {
        for (_i, prf) in time_vs.iter().enumerate() {
            time_v[hi] += prf.time_v[hi] / 30f32;
        }
    }
    let mut pk30_time_prof = TimeProf::default();
    pk30_time_prof.calc_v = pow_calc_val(&time_v);
    let (calc_onp, calc_ofp) = pow_calc_peak(&time_v);
    pk30_time_prof.calc_onp = calc_onp;
    pk30_time_prof.calc_ofp = calc_ofp;
    pk30_time_prof.time_v = time_v;
    sblp.pk30_time_prof = pk30_time_prof;

    /*
    if sblp.pk30_time_prof.calc_v.p_pk > 5f32 {
        println!("pk30:{} {}", sblp.sub, sblp.pk30_time_prof.calc_v.p_pk);
    }
    for (i, dlp) in pk30_time_prof.iter().enumerate() {
        println!(
            "  {i}. pk:{:.1} av:{:.1}",
            dlp.calc_v.p_pk, dlp.calc_v.p_avg
        );
    }
    */
}

//fn pow_calc_val(time_v: &[f32]) -> PowerCalc {}

fn pow_calc_factor(sblp: &mut SubLoadProf) {
    if sblp.calc_r.p_pk > 0f32 {
        sblp.load_factor = sblp.calc_r.p_avg / sblp.calc_r.p_pk;
    }
    let mut sumpk = 0f32;
    for (_i, fdlp) in sblp.feed.iter().enumerate() {
        sumpk += fdlp.calc_r.p_pk;
    }
    if sblp.calc_r.p_avg > 0f32 {
        sblp.diversity_factor = sumpk / sblp.calc_r.p_avg;
    }
}
use num_traits::Pow;

fn pow_calc_real(time_r: &[LoadProfVal]) -> PowerCalc {
    let mut pwc = PowerCalc::default();
    for dt in time_r {
        if let LoadProfVal::Value(v) = dt {
            if *v >= 0f32 {
                pwc.p_sum += *v;
                pwc.p_cnt += 1;
                if *v > pwc.p_pk {
                    pwc.p_pk = *v;
                }
            } else {
                pwc.n_sum += -*v;
                pwc.n_cnt += 1;
                if -*v > pwc.n_pk {
                    pwc.n_pk = -*v;
                }
            }
        }
    }
    for dt in time_r {
        if let LoadProfVal::Value(v) = dt {
            if *v >= 0f32 {
                if *v < pwc.p_pk && *v > pwc.p_pk2 {
                    pwc.p_pk2 = *v;
                }
            } else {
                if -*v < pwc.n_pk && -*v > pwc.n_pk2 {
                    pwc.n_pk2 = -*v;
                }
            }
        }
    }
    if pwc.p_pk2 == 0f32 {
        pwc.p_pk2 = pwc.p_pk;
    }
    if pwc.n_pk2 == 0f32 {
        pwc.n_pk2 = pwc.n_pk;
    }
    pwc.p_en = pwc.p_sum / 2f32;
    pwc.n_en = pwc.n_sum / 2f32;
    if pwc.p_cnt > 0 {
        pwc.p_avg = pwc.p_sum / pwc.p_cnt as f32;
    }
    if pwc.n_cnt > 0 {
        pwc.n_avg = pwc.n_sum / pwc.n_cnt as f32;
    }
    pwc
}

fn val_er_cnt(time_r: &[LoadProfVal]) -> DataValid {
    let (mut va, mut nu, mut no) = (0, 0, 0);
    for rd in time_r {
        match rd {
            LoadProfVal::Value(_) => va += 1,
            LoadProfVal::Null => nu += 1,
            LoadProfVal::None => no += 1,
        }
    }
    DataValid {
        good: va,
        null: nu,
        none: no,
    }
}

fn val_er_cor(time_r: &[LoadProfVal]) -> Vec<f32> {
    let mut lst = 0;
    let mut val = vec![0f32; time_r.len()];
    for (i, v) in time_r.iter().enumerate() {
        if let LoadProfVal::Value(va) = v {
            val[i] = *va;
            lst = i;
        }
    }
    for i in (lst + 1)..val.len() {
        val[i] = val[lst];
    }
    for i in (0..lst).rev() {
        match time_r[i] {
            LoadProfVal::Null | LoadProfVal::None => val[i] = val[i + 1],
            _ => {}
        }
    }
    val
}

pub fn get_raw_lp(yr: &str, sb: &str) -> Option<Vec<Box<FeederLoadRaw>>> {
    let fnm = &format!("{}/lp{}/{}.bin", LP_RAW_DIR, yr, sb);
    //println!("fn: {fnm}");
    if let Ok(f) = File::open(fnm) {
        if let Ok(dt) =
            bincode::deserialize_from::<BufReader<File>, Vec<Box<FeederLoadRaw>>>(BufReader::new(f))
        {
            //println!("  dt:{}", dt.len());
            return Some(dt);
        }
    }
    None
}

pub fn get_cal_lp(yr: &str, sb: &str) -> Option<SubLoadProf> {
    let fnm = &format!("{}/lp{}/{}.bin", LP_CAL_DIR, yr, sb);
    //println!("fn: {fnm}");
    if let Ok(f) = File::open(fnm) {
        if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, SubLoadProf>(BufReader::new(f))
        {
            //println!("time_v {}", dt.time_v.len());
            //println!("  dt:{}", dt.len());
            return Some(dt);
        }
    }
    None
}

pub fn get_all_lp(yr: &str) -> Option<Vec<SubLoadProf>> {
    let fnm = &format!("{}/lp{}/lp{}.bin", LP_CAL_DIR, yr, yr);
    //println!("fn: {fnm}");
    if let Ok(f) = File::open(fnm) {
        if let Ok(dt) =
            bincode::deserialize_from::<BufReader<File>, Vec<SubLoadProf>>(BufReader::new(f))
        {
            //println!("time_v {}", dt.len());
            //println!("  dt:{}", dt.len());
            return Some(dt);
        }
    }
    None
}

pub const WEBROOT: &str = "";

#[derive(Template, Debug, Default)]
#[template(path = "prc2/p2_web1.html", escape = "none")]
pub struct P2Web1 {
    pub prvs: Vec<(String, Vec<String>)>,
}
pub async fn lp_web1_pv() -> impl axum::response::IntoResponse {
    let pv = grp1();
    //let pvsb = ld_p3_prv_sub();
    let mut prvs = Vec::<(String, Vec<String>)>::new();
    let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();
    for p in &pv {
        let pp = p.to_string();
        //println!("pv:{}", p);
        if let Some(sbv) = sbsl.get(&pp) {
            let mut psbv = Vec::<String>::new();
            for sb in sbv {
                if let Some(_sbif) = sbif.get(&sb.sbid) {
                    //println!("  sb:{}", sb.sbid);
                    psbv.push(sb.sbid.to_string());
                }
            }
            prvs.push((pp, psbv));
        }
    }
    P2Web1 { prvs }
}

pub const LP_RAW_DIR: &str = "../sgdata/lpraw";
pub const LP_CAL_DIR: &str = "../sgdata/lpcal";
pub const LP_PNG_DIR: &str = "../sgdata/sbpng";

use axum::extract::Path;

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_sub.html", escape = "none")]
pub struct LPSub {
    pub sub: String,
    pub feeds: Vec<(String, Vec<i32>)>,
}
pub async fn lp_sub(Path(sb): Path<String>) -> impl axum::response::IntoResponse {
    let sbif = ld_p3_sub_inf();
    let mut feeds = Vec::<(String, Vec<i32>)>::new();
    let lp24 = get_raw_lp("2024", &sb);
    let lp23 = get_raw_lp("2023", &sb);
    let lp22 = get_raw_lp("2022", &sb);
    let lp21 = get_raw_lp("2021", &sb);
    //let re = Regex::new(r"..._[0-9][0-9][VW]B01").unwrap();
    if let Some(sf) = sbif.get(&sb) {
        //println!("feed {}", sf.feeders.len());
        for fd in &sf.feeders {
            let (v24, u24, n24) = lp_chk1(&lp24, fd);
            let (v23, u23, n23) = lp_chk1(&lp23, fd);
            let (v22, u22, n22) = lp_chk1(&lp22, fd);
            let (v21, u21, n21) = lp_chk1(&lp21, fd);
            let dt = vec![v24, u24, n24, v23, u23, n23, v22, u22, n22, v21, u21, n21];

            feeds.push((fd.to_string(), dt));
        }
    }
    let sub = sb.to_string();
    LPSub { sub, feeds }
}

fn lp_chk1(lp: &Option<Vec<Box<FeederLoadRaw>>>, fd: &str) -> (i32, i32, i32) {
    if let Some(lp) = lp {
        let mut fdlp = Option::None;
        let re = Regex::new(r"..._[0-9][0-9].B01").unwrap();
        for flp in lp {
            if re.is_match(&flp.feed) {
                let fd0 = format!("{}{}", &flp.feed[0..3], &flp.feed[4..6]);
                if fd0 == *fd {
                    fdlp = Some(flp.clone());
                }
            }
        }
        if let Some(fdlp) = fdlp {
            let (mut va, mut nu, mut no) = (0, 0, 0);
            for td in fdlp.time_r {
                match td {
                    LoadProfVal::Value(_) => va += 1,
                    LoadProfVal::Null => nu += 1,
                    LoadProfVal::None => no += 1,
                }
            }
            return (va, nu, no);
        }
        (0, 0, 0)
    } else {
        (0, 0, 0)
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_feed_meas.html", escape = "none")]
pub struct LPFeedMeas {
    pub feed: String,
    pub year: String,
    pub meas: Vec<(String, Vec<String>)>,
}
pub async fn lp_feed_meas(
    Path((fd, yr)): Path<(String, String)>,
) -> impl axum::response::IntoResponse {
    let feed = fd.to_string();
    let year = yr.to_string();
    let lp = get_raw_lp(&yr, &fd[0..3]);
    let mut meas = Vec::<(String, Vec<String>)>::new();
    if let Some(lp) = lp {
        let re = Regex::new(r"..._[0-9][0-9][VW]B01").unwrap();
        for flp in lp {
            if re.is_match(&flp.feed) {
                let fd0 = format!("{}{}", &flp.feed[0..3], &flp.feed[4..6]);
                if fd0 == *fd {
                    for i in 0..365 {
                        let mut dd = Vec::<String>::new();
                        for j in 0..48 {
                            let ij = i * 48 + j;
                            let dt = match flp.time_r[ij] {
                                LoadProfVal::Value(v) => format!("{:.1}", v),
                                LoadProfVal::Null => "--".to_string(),
                                LoadProfVal::None => "__".to_string(),
                            };
                            dd.push(dt);
                        }
                        meas.push((format!("{:03}", i + 1), dd));
                    }
                    println!("fd:{fd} yr:{yr} lp:{} fd0:{}", flp.time_r.len(), flp.feed);
                }
            }
        }
    }
    LPFeedMeas { feed, year, meas }
}

use sglab02_lib::sg::prc5::prvs;

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_web2.html", escape = "none")]
pub struct LPWeb2 {
    pub prvs: Vec<(String, Vec<String>)>,
}
pub async fn lp_web2() -> impl axum::response::IntoResponse {
    //let pv = grp1();
    let pv = prvs();
    let pvsb = ld_p3_prv_sub();
    let mut prvs = Vec::<(String, Vec<String>)>::new();
    //let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();
    for p in pv {
        let pp = p.to_string();
        //println!("pv:{}", p);
        //if let Some(sbv) = sbsl.get(&pp) {
        if let Some(sbv) = pvsb.get(&pp) {
            let mut psbv = Vec::<String>::new();
            for sb in sbv {
                if let Some(_sbif) = sbif.get(sb) {
                    //println!("  sb:{}", sb.sbid);
                    psbv.push(sb.to_string());
                }
            }
            prvs.push((pp, psbv));
        }
    }
    LPWeb2 { prvs }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_sub2.html", escape = "none")]
pub struct LPSub2 {
    pub sub: String,
    pub calc: Vec<PowerCalc>,
    pub feeds: Vec<(String, Vec<PowerCalc>)>,
}
pub async fn lp_sub2(Path(sb): Path<String>) -> impl axum::response::IntoResponse {
    let sbif = ld_p3_sub_inf();
    let mut feeds = Vec::<(String, Vec<PowerCalc>)>::new();
    let lp24 = get_cal_lp("2024", &sb);
    let lp23 = get_cal_lp("2023", &sb);
    let lp22 = get_cal_lp("2022", &sb);
    let lp21 = get_cal_lp("2021", &sb);
    //let re = Regex::new(r"..._[0-9][0-9][VW]B01").unwrap();
    let c24 = lp_chk2(&lp24, "");
    let c23 = lp_chk2(&lp23, "");
    let c22 = lp_chk2(&lp22, "");
    let c21 = lp_chk2(&lp21, "");
    let calc = vec![c24, c23, c22, c21];
    if let Some(sf) = sbif.get(&sb) {
        println!("feed {}", sf.feeders.len());
        for fd in &sf.feeders {
            let c24 = lp_chk2(&lp24, fd);
            let c23 = lp_chk2(&lp23, fd);
            let c22 = lp_chk2(&lp22, fd);
            let c21 = lp_chk2(&lp21, fd);
            feeds.push((fd.to_string(), vec![c24, c23, c22, c21]));
        }
    }
    let sub = sb.to_string();
    LPSub2 { sub, feeds, calc }
}

fn lp_chk2(lp: &Option<SubLoadProf>, fd: &str) -> PowerCalc {
    if let Some(lp) = lp {
        for flp in &lp.feed {
            if fd == "" {
                return lp.calc_v.clone();
            }
            if flp.feed == fd {
                return flp.calc_v.clone();
            }
        }
    }
    PowerCalc::default()
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_feed_calc.html", escape = "none")]
pub struct LPFeedCalc {
    pub feed: String,
    pub year: String,
    pub calc: Vec<(String, Vec<(String, String)>)>,
}
pub async fn lp_feed_calc(
    Path((fd, yr)): Path<(String, String)>,
) -> impl axum::response::IntoResponse {
    let feed = fd.to_string();
    let year = yr.to_string();
    let lp = get_cal_lp(&yr, &fd[0..3]);
    let mut calc = Vec::<(String, Vec<(String, String)>)>::new();
    if let Some(lp) = lp {
        for flp in lp.feed {
            if flp.feed == *fd {
                for i in 0..365 {
                    let mut dd = Vec::<(String, String)>::new();
                    for j in 0..48 {
                        let ij = i * 48 + j;
                        let cl = match flp.time_r[ij] {
                            LoadProfVal::Value(_v) => "val".to_string(),
                            LoadProfVal::Null => "null".to_string(),
                            LoadProfVal::None => "none".to_string(),
                        };
                        let dt = format!("{:.1}", flp.time_v[ij]);
                        dd.push((cl, dt));
                    }
                    calc.push((format!("{:03}", i + 1), dd));
                }
                println!("fd:{fd} yr:{yr} lp:{} fd0:{}", flp.time_r.len(), flp.feed);
            }
        }
    }
    LPFeedCalc { feed, year, calc }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_pv2.html", escape = "none")]
pub struct LPPv2 {
    pub prvs: Vec<(String, Vec<String>)>,
}
pub async fn lp_pv2() -> impl axum::response::IntoResponse {
    let pv = grp1();
    //let pvsb = ld_p3_prv_sub();
    let mut prvs = Vec::<(String, Vec<String>)>::new();
    let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();
    for p in &pv {
        let pp = p.to_string();
        //println!("pv:{}", p);
        if let Some(sbv) = sbsl.get(&pp) {
            let mut psbv = Vec::<String>::new();
            for sb in sbv {
                if let Some(_sbif) = sbif.get(&sb.sbid) {
                    //println!("  sb:{}", sb.sbid);
                    psbv.push(sb.sbid.to_string());
                }
            }
            prvs.push((pp, psbv));
        }
    }
    LPPv2 { prvs }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_web22.html", escape = "none")]
pub struct LPWeb22 {
    pub prvs: Vec<(String, Vec<(String, String)>)>,
}
pub async fn lp_web22() -> impl axum::response::IntoResponse {
    let lp22 = get_all_lp("2022").unwrap();
    let mut sbm = HashMap::<String, SubLoadProf>::new();
    for slp in lp22 {
        sbm.insert(slp.sub.to_string(), slp);
    }
    let pv = grp1();
    //let pvsb = ld_p3_prv_sub();
    let mut prvs = Vec::<(String, Vec<(String, String)>)>::new();
    let sbsl = ld_pv_sbv_m();
    //let sbif = ld_p3_sub_inf();
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = sbsl.get(&pp) {
            let mut psbv = Vec::<(String, String)>::new();
            for sb in sbv {
                if let Some(slp) = sbm.get(&sb.sbid) {
                    let mut ldln = String::new();
                    if let Some(latlong) = slp.latlong {
                        let (x, y) = latlong;
                        let (xx, yy) = utm_latlong(x as f32, y as f32);
                        ldln = format!("https://maps.google.com/?q={:.4},{:.4}", xx, yy);
                    }
                    let tx = format!("{}:{:.1}/{:.1}", slp.sub, slp.calc_r.p_pk, slp.maxmva);
                    psbv.push((ldln, tx));
                }
            }
            prvs.push((pp, psbv));
        }
    }
    LPWeb22 { prvs }
}

use sglab02_lib::sg::mvline::utm_latlong;

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_web23.html", escape = "none")]
pub struct LPWeb23 {
    pub prvs: Vec<(String, Vec<(String, String)>)>,
}
pub async fn lp_web23() -> impl axum::response::IntoResponse {
    let lp22 = get_all_lp("2023").unwrap();
    let mut sbm = HashMap::<String, SubLoadProf>::new();
    for slp in lp22 {
        sbm.insert(slp.sub.to_string(), slp);
    }
    let pv = grp1();
    //let pvsb = ld_p3_prv_sub();
    let mut prvs = Vec::<(String, Vec<(String, String)>)>::new();
    let sbsl = ld_pv_sbv_m();
    //let sbif = ld_p3_sub_inf();
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = sbsl.get(&pp) {
            let mut psbv = Vec::<(String, String)>::new();
            for sb in sbv {
                if let Some(slp) = sbm.get(&sb.sbid) {
                    let mut ldln = String::new();
                    if let Some(latlong) = slp.latlong {
                        let (x, y) = latlong;
                        let (xx, yy) = utm_latlong(x as f32, y as f32);
                        ldln = format!("https://maps.google.com/?q={:.4},{:.4}", xx, yy);
                    }
                    let tx = format!("{}:{:.1}/{:.1}", slp.sub, slp.calc_r.p_pk, slp.maxmva);
                    psbv.push((ldln, tx));
                }
            }
            prvs.push((pp, psbv));
        }
    }
    LPWeb23 { prvs }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_web24.html", escape = "none")]
pub struct LPWeb24 {
    pub prvs: Vec<(String, Vec<(String, String)>)>,
}
pub async fn lp_web24() -> impl axum::response::IntoResponse {
    let lp22 = get_all_lp("2024").unwrap();
    let mut sbm = HashMap::<String, SubLoadProf>::new();
    for slp in lp22 {
        sbm.insert(slp.sub.to_string(), slp);
    }
    let pv = grp1();
    //let pvsb = ld_p3_prv_sub();
    let mut prvs = Vec::<(String, Vec<(String, String)>)>::new();
    let sbsl = ld_pv_sbv_m();
    //let sbif = ld_p3_sub_inf();
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = sbsl.get(&pp) {
            let mut psbv = Vec::<(String, String)>::new();
            for sb in sbv {
                if let Some(slp) = sbm.get(&sb.sbid) {
                    let mut ldln = String::new();
                    if let Some(latlong) = slp.latlong {
                        let (x, y) = latlong;
                        let (xx, yy) = utm_latlong(x as f32, y as f32);
                        ldln = format!("https://maps.google.com/?q={:.4},{:.4}", xx, yy);
                    }
                    let pc = slp.calc_r.p_pk / slp.maxmva * 100f32;
                    let pc = pc as i32;
                    let tx = format!("{}:{}", slp.sub, pc);
                    psbv.push((ldln, tx));
                }
            }
            prvs.push((pp, psbv));
        }
    }
    LPWeb24 { prvs }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LPWeb24aX {
    pub sub: String,
    pub loc: String,
    pub cls: String,
    pub pc: String,  // peak24 / capacity
    pub pc0: String, // peak23 / peak24
    pub pc1: String, // peak22 / peak24
    pub pc2: String, // peak22 / peak24
    pub nc: String,  // negative peak24 / capacity
    pub nc0: String, // negative peak23 / peak24
    pub nc1: String, // negative peak22 / peak24
    pub nc2: String, // negative peak22 / peak24
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_web24a.html", escape = "none")]
pub struct LPWeb24a {
    pub prvs: Vec<(String, Vec<LPWeb24aX>)>,
}
pub async fn lp_web24a() -> impl axum::response::IntoResponse {
    let (sbm, sbm0, sbm1, sbm2) = get_lps();
    /*
    let lp24 = get_all_lp("2024").unwrap();
    let lp23 = get_all_lp("2023").unwrap();
    let lp22 = get_all_lp("2022").unwrap();
    let mut sbm = HashMap::<String, SubLoadProf>::new();
    let mut sbm0 = HashMap::<String, SubLoadProf>::new();
    let mut sbm1 = HashMap::<String, SubLoadProf>::new();
    for slp in lp24 {
        sbm.insert(slp.sub.to_string(), slp);
    }
    for slp in lp23 {
        sbm0.insert(slp.sub.to_string(), slp);
    }
    for slp in lp22 {
        sbm1.insert(slp.sub.to_string(), slp);
    }
    */
    let pv = grp1();
    //let pvsb = ld_p3_prv_sub();
    let mut prvs = Vec::<(String, Vec<LPWeb24aX>)>::new();
    let sbsl = ld_pv_sbv_m();
    //let sbif = ld_p3_sub_inf();
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = sbsl.get(&pp) {
            let mut psbv = Vec::<LPWeb24aX>::new();
            for sb in sbv {
                if let (Some(slp), Some(slp0), Some(slp1), Some(slp2)) = (
                    sbm.get(&sb.sbid),
                    sbm0.get(&sb.sbid),
                    sbm1.get(&sb.sbid),
                    sbm2.get(&sb.sbid),
                ) {
                    let all =
                        calc_sub_char(&slp, &slp.calc_v, &slp0.calc_v, &slp1.calc_v, &slp2.calc_v);
                    psbv.push(all);
                }
            }
            prvs.push((pp, psbv));
        }
    }
    LPWeb24a { prvs }
}

pub type MapSubLP = HashMap<String, SubLoadProf>;

pub fn get_lps() -> (MapSubLP, MapSubLP, MapSubLP, MapSubLP) {
    let lp24 = get_all_lp("2024").unwrap();
    let lp23 = get_all_lp("2023").unwrap();
    let lp22 = get_all_lp("2022").unwrap();
    let lp21 = get_all_lp("2021").unwrap();
    let mut sbm = HashMap::<String, SubLoadProf>::new();
    let mut sbm0 = HashMap::<String, SubLoadProf>::new();
    let mut sbm1 = HashMap::<String, SubLoadProf>::new();
    let mut sbm2 = HashMap::<String, SubLoadProf>::new();
    for slp in lp24 {
        sbm.insert(slp.sub.to_string(), slp);
    }
    for slp in lp23 {
        sbm0.insert(slp.sub.to_string(), slp);
    }
    for slp in lp22 {
        sbm1.insert(slp.sub.to_string(), slp);
    }
    for slp in lp21 {
        sbm2.insert(slp.sub.to_string(), slp);
    }
    (sbm, sbm0, sbm1, sbm2)
}

fn calc_sub_char(
    slp: &SubLoadProf,
    val: &PowerCalc,
    val0: &PowerCalc,
    val1: &PowerCalc,
    val2: &PowerCalc,
) -> LPWeb24aX {
    let (xx, yy) = if let Some(latlong) = slp.latlong {
        let (x, y) = latlong;
        let (xx, yy) = utm_latlong(x as f32, y as f32);
        (xx, yy)
    } else {
        (0f32, 0f32)
    };
    let loc = format!("https://maps.google.com/?q={:.4},{:.4}", xx, yy);
    let pc = val.p_pk / slp.maxmva * 100f32;
    let pc0 = val0.p_pk / slp.maxmva * 100f32;
    let pc1 = val1.p_pk / slp.maxmva * 100f32;
    let pc2 = val2.p_pk / slp.maxmva * 100f32;
    let nc = val.n_pk / slp.maxmva * 100f32;
    let nc0 = val0.n_pk / slp.maxmva * 100f32;
    let nc1 = val1.n_pk / slp.maxmva * 100f32;
    let nc2 = val2.n_pk / slp.maxmva * 100f32;
    let pc0 = pc - pc0;
    let pc1 = pc - pc1;
    let pc2 = pc - pc2;
    let nc0 = nc - nc0;
    let nc1 = nc - nc1;
    let nc2 = nc - nc2;
    let cls = if pc > 30f32 && pc < 150f32 {
        if pc0 >= 5f32 && pc1 >= 5f32 {
            "OK"
        } else {
            if nc > 5f32 && nc0 >= 1f32 && nc1 >= 1f32 {
                "NG"
            } else {
                "EX"
            }
        }
    } else {
        "EX"
    };
    let pc = format!("{}", pc as i32);
    let pc0 = format!("{}", pc0 as i32);
    let pc1 = format!("{}", pc1 as i32);
    let pc2 = format!("{}", pc2 as i32);
    let nc = format!("{}", nc as i32);
    let nc0 = format!("{}", nc0 as i32);
    let nc1 = format!("{}", nc1 as i32);
    let nc2 = format!("{}", nc2 as i32);
    let sub = format!("{}", slp.sub);
    if cls == "OK" {
        //println!("{} {} {}", sub, loc, sf.mvax);
    }
    let cls = cls.to_string();
    LPWeb24aX {
        sub,
        loc,
        pc,
        pc0,
        pc1,
        pc2,
        nc,
        nc0,
        nc1,
        nc2,
        cls,
    }
}

fn calc_sub_d(
    slp: &SubLoadProf,
    val: &PowerCalc,
    val0: &PowerCalc,
    val1: &PowerCalc,
    val2: &PowerCalc,
) -> LPWeb24aX {
    let (xx, yy) = if let Some(latlong) = slp.latlong {
        let (x, y) = latlong;
        let (xx, yy) = utm_latlong(x as f32, y as f32);
        (xx, yy)
    } else {
        (0f32, 0f32)
    };
    let loc = format!("https://maps.google.com/?q={:.4},{:.4}", xx, yy);
    let pc = val.p_avg / slp.maxmva * 100f32;
    let pc0 = val0.p_avg / slp.maxmva * 100f32;
    let pc1 = val1.p_avg / slp.maxmva * 100f32;
    let pc2 = val2.p_avg / slp.maxmva * 100f32;
    let nc = val.n_avg / slp.maxmva * 100f32;
    let nc0 = val0.n_avg / slp.maxmva * 100f32;
    let nc1 = val1.n_avg / slp.maxmva * 100f32;
    let nc2 = val2.n_avg / slp.maxmva * 100f32;
    let pc0 = pc - pc0;
    let pc1 = pc - pc1;
    let pc2 = pc - pc2;
    let nc0 = nc - nc0;
    let nc1 = nc - nc1;
    let nc2 = nc - nc2;
    let cls = if pc > 30f32 && pc < 150f32 {
        if pc0 >= 5f32 && pc1 >= 5f32 {
            "OK"
        } else {
            if nc > 5f32 && nc0 >= 1f32 && nc1 >= 1f32 {
                "NG"
            } else {
                "EX"
            }
        }
    } else {
        "EX"
    };
    let pc = format!("{}", pc as i32);
    let pc0 = format!("{}", pc0 as i32);
    let pc1 = format!("{}", pc1 as i32);
    let pc2 = format!("{}", pc2 as i32);
    let nc = format!("{}", nc as i32);
    let nc0 = format!("{}", nc0 as i32);
    let nc1 = format!("{}", nc1 as i32);
    let nc2 = format!("{}", nc2 as i32);
    let sub = format!("{}", slp.sub);
    if cls == "OK" {
        //println!("{} {} {}", sub, loc, sf.mvax);
    }
    let cls = cls.to_string();
    LPWeb24aX {
        sub,
        loc,
        pc,
        pc0,
        pc1,
        pc2,
        nc,
        nc0,
        nc1,
        nc2,
        cls,
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_web24b.html", escape = "none")]
pub struct LPWeb24b {
    pub prvs: Vec<(String, Vec<(LPWeb24aX, LPWeb24aX, LPWeb24aX)>)>,
}
pub async fn lp_web24b() -> impl axum::response::IntoResponse {
    let (sbm, sbm0, sbm1, sbm2) = get_lps();
    let pv = grp1();
    //let pvsb = ld_p3_prv_sub();
    let mut prvs = Vec::<(String, Vec<(LPWeb24aX, LPWeb24aX, LPWeb24aX)>)>::new();
    let sbsl = ld_pv_sbv_m();
    //let sbif = ld_p3_sub_inf();
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = sbsl.get(&pp) {
            let mut psbv = Vec::<(LPWeb24aX, LPWeb24aX, LPWeb24aX)>::new();
            for sb in sbv {
                if let (Some(slp), Some(slp0), Some(slp1), Some(slp2)) = (
                    sbm.get(&sb.sbid),
                    sbm0.get(&sb.sbid),
                    sbm1.get(&sb.sbid),
                    sbm2.get(&sb.sbid),
                ) {
                    let all =
                        calc_sub_char(&slp, &slp.calc_v, &slp0.calc_v, &slp1.calc_v, &slp2.calc_v);
                    let day = calc_sub_char(
                        &slp,
                        &slp.calc_day,
                        &slp0.calc_day,
                        &slp1.calc_day,
                        &slp2.calc_day,
                    );
                    let ngt = calc_sub_char(
                        &slp,
                        &slp.calc_ngt,
                        &slp0.calc_ngt,
                        &slp1.calc_ngt,
                        &slp2.calc_ngt,
                    );
                    psbv.push((all, day, ngt));
                }
            }
            prvs.push((pp, psbv));
        }
    }
    LPWeb24b { prvs }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_web24c.html", escape = "none")]
pub struct LPWeb24c {
    pub prvs: Vec<(String, Vec<(LPWeb24aX, LPWeb24aX, LPWeb24aX)>)>,
}
pub async fn lp_web24c() -> impl axum::response::IntoResponse {
    let (sbm, sbm0, sbm1, sbm2) = get_lps();
    let pv = grp1();
    let mut prvs = Vec::<(String, Vec<(LPWeb24aX, LPWeb24aX, LPWeb24aX)>)>::new();
    let sbsl = ld_pv_sbv_m();
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = sbsl.get(&pp) {
            let mut psbv = Vec::<(LPWeb24aX, LPWeb24aX, LPWeb24aX)>::new();
            for sb in sbv {
                if let (Some(slp), Some(slp0), Some(slp1), Some(slp2)) = (
                    sbm.get(&sb.sbid),
                    sbm0.get(&sb.sbid),
                    sbm1.get(&sb.sbid),
                    sbm2.get(&sb.sbid),
                ) {
                    let all =
                        calc_sub_char(&slp, &slp.calc_v, &slp0.calc_v, &slp1.calc_v, &slp2.calc_v);
                    let day = calc_sub_char(
                        &slp,
                        &slp.calc_day,
                        &slp0.calc_day,
                        &slp1.calc_day,
                        &slp2.calc_day,
                    );
                    let ngt = calc_sub_char(
                        &slp,
                        &slp.calc_ngt,
                        &slp0.calc_ngt,
                        &slp1.calc_ngt,
                        &slp2.calc_ngt,
                    );
                    psbv.push((all, day, ngt));
                }
            }
            prvs.push((pp, psbv));
        }
    }
    LPWeb24c { prvs }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/sb_gr_gp1.html", escape = "none")]
pub struct SubGraphGrp1 {
    pub sub: String,
}
pub async fn gr_gp1(Path(sb): Path<String>) -> impl axum::response::IntoResponse {
    let sub = sb.to_string();
    SubGraphGrp1 { sub }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_sub_calc.html", escape = "none")]
pub struct LPSubCalc {
    pub sub: String,
    pub year: String,
    pub calc: Vec<(String, Vec<(String, String)>)>,
}
pub async fn lp_sub_calc(
    Path((sb, yr)): Path<(String, String)>,
) -> impl axum::response::IntoResponse {
    let sub = sb.to_string();
    let year = yr.to_string();
    let lp = get_cal_lp(&yr, &sub);
    let mut calc = Vec::<(String, Vec<(String, String)>)>::new();
    if let Some(lp) = lp {
        for i in 0..365 {
            let mut dd = Vec::<(String, String)>::new();
            for j in 0..48 {
                let ij = i * 48 + j;
                let cl = match lp.time_r[ij] {
                    LoadProfVal::Value(_v) => "val".to_string(),
                    LoadProfVal::Null => "null".to_string(),
                    LoadProfVal::None => "none".to_string(),
                };
                let dt = format!("{:.1}", lp.time_v[ij]);
                dd.push((cl, dt));
            }
            calc.push((format!("{:03}", i + 1), dd));
        }
        //println!("fd:{fd} yr:{yr} lp:{} fd0:{}", lp.time_r.len(), lp.feed);
    }
    LPSubCalc { sub, year, calc }
}

use askama_axum::IntoResponse;
pub enum HttpType {
    TextPlain,
    ImagePng,
}

pub fn ctype_header(tp: HttpType) -> axum::http::HeaderMap {
    let mut headers = axum::http::HeaderMap::new();
    match tp {
        HttpType::TextPlain => {
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static(&"text/plain"),
            );
        }
        HttpType::ImagePng => {
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static(&"image/png"),
            );
        }
    }
    headers
}

use crate::prc3::DayLoadDraw;

struct SubLoadDraw {
    pub fnm: String,
    pub lp: Vec<f32>,
}

impl DayLoadDraw for SubLoadDraw {
    fn fnm(&self) -> String {
        self.fnm.to_string()
    }
    fn lp(&self) -> Vec<f32> {
        self.lp.clone()
    }
}

pub async fn lp_gr_sb_yr(
    Path((sub, year)): Path<(String, String)>,
) -> impl axum::response::IntoResponse {
    let fdir = format!("{}/{}/{}", LP_PNG_DIR, year, sub);
    let _ = fs::create_dir_all(&fdir);
    let fnm = format!("{}/{}.png", fdir, sub);
    let lp = get_cal_lp(&year, &sub).unwrap().day_prof.clone();
    let sld = SubLoadDraw { fnm, lp };
    let rs = sld.draw_prof();
    match rs {
        Ok(bb) => (ctype_header(HttpType::ImagePng), bb).into_response(),
        Err(ee) => (ctype_header(HttpType::TextPlain), ee).into_response(),
    }
}

pub async fn lp_gr_sb_yr2(
    Path((sub, year)): Path<(String, String)>,
) -> impl axum::response::IntoResponse {
    let fdir = format!("{}/{}/{}", LP_PNG_DIR, year, sub);
    let _ = fs::create_dir_all(&fdir);
    let fnm = format!("{}/{}.png", fdir, sub);
    let lp = get_cal_lp(&year, &sub).unwrap().time_v.clone();
    let sld = SubLoadDraw { fnm, lp };
    let rs = sld.draw_prof();
    match rs {
        Ok(bb) => (ctype_header(HttpType::ImagePng), bb).into_response(),
        Err(ee) => (ctype_header(HttpType::TextPlain), ee).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct DrawPara {
    wd: usize,
    hg: usize,
}

impl Default for DrawPara {
    fn default() -> Self {
        Self { wd: 400, hg: 300 }
    }
}

//----------- draw1
use crate::drw::sb_dr1::SubDraw1;

#[derive(Debug, Default)]
pub struct SubGraphDraw1 {
    pub sub: String,
    pub fnm: String,
    pub lp: Vec<f32>,
    pub sz: (usize, usize),
    pub rf: Vec<(String, f32)>,
}

impl SubDraw1 for SubGraphDraw1 {
    fn sz(&self) -> (usize, usize) {
        if self.sz == (0, 0) {
            Self::SIZE
        } else {
            self.sz
        }
    }
    fn sub(&self) -> String {
        self.sub.to_string()
    }
    fn fnm(&self) -> String {
        self.fnm.to_string()
    }
    fn lp(&self) -> Vec<f32> {
        self.lp.clone()
    }
    fn rf(&self) -> Vec<(String, f32)> {
        self.rf.clone()
    }
}

pub async fn sb_gr_dr1(
    Path((sub, year)): Path<(String, String)>,
    drpr: Option<Query<DrawPara>>,
) -> impl axum::response::IntoResponse {
    let fdir = format!("{}/{}/{}", LP_PNG_DIR, year, sub);
    let _ = fs::create_dir_all(&fdir);
    let fnm = format!("{}/{}.png", fdir, sub);
    let slp = get_cal_lp(&year, &sub).unwrap();
    let lp = slp.day_prof.clone();
    let lcr = slp.calc_r;
    let mut rf = Vec::<(String, f32)>::new();
    rf.push(("avg".to_string(), lcr.p_avg));
    rf.push(("peak".to_string(), lcr.p_pk));
    //println!("DR1 ref: {:?}", rf);
    let mut sld = SubGraphDraw1 {
        sub: sub.to_string(),
        fnm,
        lp,
        rf,
        ..Default::default() //sz: (400, 300),
    };
    if let Some(dr) = drpr {
        sld.sz = (dr.wd, dr.hg);
    }
    let rs = sld.draw_prof();
    match rs {
        Ok(bb) => (ctype_header(HttpType::ImagePng), bb).into_response(),
        Err(ee) => (ctype_header(HttpType::TextPlain), ee).into_response(),
    }
}

//----------- draw2
use crate::drw::sb_dr2::SubDraw2;

#[derive(Debug, Default)]
pub struct SubGraphDraw2 {
    pub sub: String,
    pub fnm: String,
    pub lp: Vec<f32>,
    pub ct: f32,
    pub sz: (usize, usize),
}

impl SubDraw2 for SubGraphDraw2 {
    fn sz(&self) -> (usize, usize) {
        if self.sz == (0, 0) {
            Self::SIZE
        } else {
            self.sz
        }
    }
    fn sub(&self) -> String {
        self.sub.to_string()
    }
    fn fnm(&self) -> String {
        self.fnm.to_string()
    }
    fn lp(&self) -> Vec<f32> {
        self.lp.clone()
    }
    fn ct(&self) -> f32 {
        self.ct
    }
}

pub async fn sb_gr_dr2(
    Path((sub, year)): Path<(String, String)>,
    drpr: Option<Query<DrawPara>>,
) -> impl axum::response::IntoResponse {
    let fdir = format!("{}/{}/{}", LP_PNG_DIR, year, sub);
    let _ = fs::create_dir_all(&fdir);
    let fnm = format!("{}/{}.png", fdir, sub);
    //let lp = get_cal_lp(&year, &sub).unwrap().day_prof.clone();
    let slp = get_cal_lp(&year, &sub).unwrap();
    let lp = slp.day_prof.clone();
    let lcr = slp.calc_r;
    let ct = lcr.p_avg;

    let mut sld = SubGraphDraw2 {
        sub: sub.to_string(),
        fnm,
        lp,
        ct,
        ..Default::default() //sz: (0, 0),
    };
    if let Some(dr) = drpr {
        sld.sz = (dr.wd, dr.hg);
    }

    let rs = sld.draw_prof();
    match rs {
        Ok(bb) => (ctype_header(HttpType::ImagePng), bb).into_response(),
        Err(ee) => (ctype_header(HttpType::TextPlain), ee).into_response(),
    }
}

//----------- draw3
use crate::drw::sb_dr3::SubDraw3;

#[derive(Debug, Default)]
pub struct SubGraphDraw3 {
    pub sub: String,
    pub fnm: String,
    pub lp: (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>),
    pub ct: f32,
    pub sz: (usize, usize),
}

impl SubDraw3 for SubGraphDraw3 {
    fn sz(&self) -> (usize, usize) {
        if self.sz == (0, 0) {
            Self::SIZE
        } else {
            self.sz
        }
    }
    fn sub(&self) -> String {
        self.sub.to_string()
    }
    fn fnm(&self) -> String {
        self.fnm.to_string()
    }
    fn lp(&self) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>) {
        self.lp.clone()
    }
}

pub async fn sb_gr_dr3(
    Path(sub): Path<String>,
    drpr: Option<Query<DrawPara>>,
) -> impl axum::response::IntoResponse {
    let fdir = format!("{}/{}/dr3", LP_PNG_DIR, sub);
    let _ = fs::create_dir_all(&fdir);
    let fnm = format!("{}/{}.png", fdir, sub);
    let lp24 = get_cal_lp("2024", &sub).unwrap().day_prof.clone();
    let lp23 = get_cal_lp("2023", &sub).unwrap().day_prof.clone();
    let lp22 = get_cal_lp("2022", &sub).unwrap().day_prof.clone();
    let lp21 = get_cal_lp("2021", &sub).unwrap().day_prof.clone();
    let lp = (lp24, lp23, lp22, lp21);
    let mut sld = SubGraphDraw3 {
        sub: sub.to_string(),
        fnm,
        lp,
        ct: 10f32,
        ..Default::default() //sz: (0, 0),
    };
    if let Some(dr) = drpr {
        sld.sz = (dr.wd, dr.hg);
    }

    let rs = sld.draw_prof();
    match rs {
        Ok(bb) => (ctype_header(HttpType::ImagePng), bb).into_response(),
        Err(ee) => (ctype_header(HttpType::TextPlain), ee).into_response(),
    }
}

//----------- draw4
use crate::drw::sb_dr4::SubDraw4;

#[derive(Debug, Deserialize)]
pub struct DrawPara4 {
    wd: usize,
    hg: usize,
    mode: String,
}

impl Default for DrawPara4 {
    fn default() -> Self {
        Self {
            wd: 400,
            hg: 300,
            mode: "x".to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct SubGraphDraw4 {
    pub sub: String,
    pub fnm: String,
    pub lp: Vec<f32>,
    pub sz: (usize, usize),
}

impl SubDraw4 for SubGraphDraw4 {
    fn sz(&self) -> (usize, usize) {
        if self.sz == (0, 0) {
            Self::SIZE
        } else {
            self.sz
        }
    }
    fn sub(&self) -> String {
        self.sub.to_string()
    }
    fn fnm(&self) -> String {
        self.fnm.to_string()
    }
    fn lp(&self) -> Vec<f32> {
        self.lp.clone()
    }
}

pub async fn sb_gr_dr4(
    Path((sub, year)): Path<(String, String)>,
    drpr: Option<Query<DrawPara4>>,
) -> impl axum::response::IntoResponse {
    //println!("DR4");
    let fdir = format!("{}/{}/{}", LP_PNG_DIR, year, sub);
    let _ = fs::create_dir_all(&fdir);
    let fnm = format!("{}/{}.png", fdir, sub);
    //let lp = get_cal_lp(&year, &sub).unwrap().day_prof.clone();

    let mut lp = get_cal_lp(&year, &sub).unwrap().time_v;
    if let Some(dr) = &drpr {
        let no = &dr.mode[1..];
        if let Ok(no) = no.parse::<i32>() {
            //println!(" mode:{}-{}", dr.mode, no);
            if dr.mode.starts_with("D") {
                let i1 = ((no - 1) * 48) as usize;
                let i2 = i1 + 48;
                lp = lp[i1..i2].to_vec();
                //println!("D:{}-{}", i1, i2);
            } else if dr.mode.starts_with("M") {
                let i1 = ((no - 1) * 48 * 30) as usize;
                let i2 = i1 + 30 * 48;
                //println!("M:{}-{}", i1, i2);
                lp = lp[i1..i2].to_vec();
            } else if dr.mode.starts_with("Q") {
                let i1 = ((no - 1) * 48 * 30 * 3) as usize;
                let i2 = i1 + 30 * 3 * 48;
                //println!("Q:{}-{}", i1, i2);
                lp = lp[i1..i2].to_vec();
            } else {
                println!("ERR1: {}", dr.mode);
            }
        } else {
            println!("ERR2: '{}'", dr.mode);
        }
    } else {
        println!("ERR3 {}", drpr.is_some());
    }

    let mut sld = SubGraphDraw4 {
        sub: sub.to_string(),
        fnm,
        lp,
        ..Default::default() //sz: (400, 300),
    };
    if let Some(dr) = drpr {
        sld.sz = (dr.wd, dr.hg);
        //println!("mode: {}", dr.mode);
    }
    let rs = sld.draw_prof();
    match rs {
        Ok(bb) => (ctype_header(HttpType::ImagePng), bb).into_response(),
        Err(ee) => (ctype_header(HttpType::TextPlain), ee).into_response(),
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/lp_web24d.html", escape = "none")]
pub struct LPWeb24d {
    pub prvs: Vec<(String, Vec<(LPWeb24aX, LPWeb24aX, LPWeb24aX)>)>,
}
pub async fn lp_web24d() -> impl axum::response::IntoResponse {
    let (sbm, sbm0, sbm1, sbm2) = get_lps();
    let pv = grp1();
    let mut prvs = Vec::<(String, Vec<(LPWeb24aX, LPWeb24aX, LPWeb24aX)>)>::new();
    let sbsl = ld_pv_sbv_m();
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = sbsl.get(&pp) {
            let mut psbv = Vec::<(LPWeb24aX, LPWeb24aX, LPWeb24aX)>::new();
            for sb in sbv {
                if let (Some(slp), Some(slp0), Some(slp1), Some(slp2)) = (
                    sbm.get(&sb.sbid),
                    sbm0.get(&sb.sbid),
                    sbm1.get(&sb.sbid),
                    sbm2.get(&sb.sbid),
                ) {
                    let all =
                        calc_sub_d(&slp, &slp.calc_v, &slp0.calc_v, &slp1.calc_v, &slp2.calc_v);
                    let day = calc_sub_d(
                        &slp,
                        &slp.calc_day,
                        &slp0.calc_day,
                        &slp1.calc_day,
                        &slp2.calc_day,
                    );
                    let ngt = calc_sub_d(
                        &slp,
                        &slp.calc_ngt,
                        &slp0.calc_ngt,
                        &slp1.calc_ngt,
                        &slp2.calc_ngt,
                    );
                    psbv.push((all, day, ngt));
                }
            }
            prvs.push((pp, psbv));
        }
    }
    LPWeb24c { prvs }
}

pub const SUB_BESS: [(&'static str, &'static str, &'static str); 29] = [
    ("RAD", "GIS-115kV", "ระยอง 4"),
    ("PTP", "AIS-115kV", "พานทอง 2"),
    ("KBB", "AIS-115kV", "กระบี่ 2"),
    ("ARA", "AIS-115kV", "อรัญประเทศ 1"),
    ("ARB", "AIS-115kV", "อรัญประเทศ 2"),
    ("BLS", "GIS-115kV", "บ้านลาดทราย"),
    ("SNA", "AIS-115kV", "เสนา"),
    ("PHA", "AIS-115kV", "พนมสารคาม"),
    ("SNK", "AIS-115kV", "	สนามชัยเขต"),
    ("BNP", "AIS-115kV", "บ้านแพ้ว"),
    ("SMF", "GIS-115kV", "สมุทรสาคร 6"),
    ("KLO", "AIS-115kV", "คลองหลวง"),
    ("TYA", "AIS-115kV", "ธัญบุรี"),
    ("KBJ", "AIS-115kV", "กบินทร์บุรี 2"),
    ("MRM", "AIS-115kV", "แม่ริม"),
    ("SKP", "AIS-115kV", "สันกำแพง"),
    ("SRE", "AIS-115kV", "สระบุรี 5"),
    ("WGT", "GIS-115kV", "วังทอง"),
    ("DNA", "AIS-115kV", "ดำเนินสะดวก"),
    ("NOA", "AIS-115kV", "หนองเรือ"),
    ("OYK", "GIS-115kV", "อ้อมใหญ่ 2"),
    ("HYC", "GIS-115kV", "หาดใหญ่ 3"),
    ("SBY", "AIS-115kV", "สะบ้าย้อย"),
    ("WSA", "AIS-115kV", "เวียงสระ"),
    ("LAA", "AIS-115kV", "ลาดยาว"),
    ("NSB", "AIS-115kV", "นครสวรรค์ 2"),
    ("BUY", "AIS-115kV", "บัวใหญ่"),
    ("TLG", "AIS-115kV", "ถลาง 1"),
    ("RNB", "AIS-115kV", "ระนอง 2"),
];

use crate::prc4::SubBenInfo;
#[derive(Template, Debug, Default)]
#[template(path = "prc2/ben_bess1.html", escape = "none")]
pub struct BenBess1 {
    pub sb_ben: Vec<(&'static str, &'static str, &'static str, SubBenInfo)>,
}
use crate::prc4::ld_ben_bess1;
pub async fn ben_bess1() -> impl axum::response::IntoResponse {
    let mut sb_ben = Vec::<(&str, &str, &str, SubBenInfo)>::new();
    for (s, t, n) in &SUB_BESS {
        let ben = ld_ben_bess1(s);
        sb_ben.push((s, t, n, ben));
    }
    BenBess1 { sb_ben }
}

use crate::subtype::sub_type;
use sglab02_lib::sg::prc5::pv_sub;

#[derive(Template, Debug, Default)]
#[template(path = "prc2/ben_bess1.html", escape = "none")]
pub struct BenBess2 {
    pub sb_ben: Vec<(&'static str, &'static str, String, SubBenInfo)>,
}
pub async fn ben_bess2() -> impl axum::response::IntoResponse {
    let mut sb_ben = Vec::<(&str, &str, String, SubBenInfo)>::new();
    let pv = grp1();
    let pvsb = pv_sub();
    let sbif = ld_p3_sub_inf();
    let sbtp = sub_type();
    let mut tot = 0f32;
    let mut be1 = 0f32;
    let mut be2 = 0f32;
    let mut mwh = 0i32;
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = pvsb.get(&pp) {
            for sb in sbv {
                if let (Some(sf), Some(tp)) = (sbif.get(sb), sbtp.get(sb.as_str())) {
                    let _mx = sf.mvax.as_str();
                    let n = sf.name.to_string();
                    let ben = ld_ben_bess1(sb);
                    let mx_pw = ben.mx_pw;
                    //println!("pv:{pp} sb:{sb} mx:{mx} tp:{tp} mx:{mx_pw}");
                    if mx_pw > 0f32
                        && ben.grw < 7f32
                        && ben.be_start <= 3
                        && ben.trlm > 40f32
                        && (*tp == "AIS" || *tp == "GIS")
                    {
                        tot += ben.ls_ex_en;
                        mwh += (ben.ls_ex_en / 0.85f32 + 0.9f32) as i32;
                        be1 += ben.be_sub + ben.dec_ben / 1000000f32 + ben.q_ben;
                        be2 += ben.be_sub
                            + ben.dec_ben / 1000000f32
                            + ben.q_ben
                            + ben.ex_ben / 1000000f32;
                        sb_ben.push((sb, tp, n, ben));
                    }
                }
            }
        }
    }
    let bcst = mwh as f32 * 21f32;
    println!("BMWh: {tot:.1} mwh:{mwh} be1:{be1} be2:{be2} cost:{bcst:.1}");
    BenBess2 { sb_ben }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/sb_be_bess1.html", escape = "none")]
pub struct SubBeBess1 {
    pub sub: String,
}
pub async fn sb_be_bess1(Path(sb): Path<String>) -> impl axum::response::IntoResponse {
    let sub = sb.to_string();
    SubBeBess1 { sub }
}

//----------- draw5
use crate::drw::sb_dr5::SubDraw5;

#[derive(Debug, Default)]
pub struct SubGraphDraw5 {
    pub sub: String,
    pub fnm: String,
    pub lp: Vec<f32>,
    pub sz: (usize, usize),
    pub rf: Vec<(String, f32)>,
    pub yr: String,
}

impl SubDraw5 for SubGraphDraw5 {
    fn sz(&self) -> (usize, usize) {
        if self.sz == (0, 0) {
            Self::SIZE
        } else {
            self.sz
        }
    }
    fn sub(&self) -> String {
        self.sub.to_string()
    }
    fn fnm(&self) -> String {
        self.fnm.to_string()
    }
    fn lp(&self) -> Vec<f32> {
        self.lp.clone()
    }
    fn rf(&self) -> Vec<(String, f32)> {
        self.rf.clone()
    }
    fn yr(&self) -> String {
        self.yr.clone()
    }
}

pub async fn sb_gr_dr5(
    Path((sub, year)): Path<(String, String)>,
    drpr: Option<Query<DrawPara>>,
) -> impl axum::response::IntoResponse {
    let fdir = format!("{}/{}/{}_dr5", LP_PNG_DIR, year, sub);
    let _ = fs::create_dir_all(&fdir);
    let fnm = format!("{}/{}.png", fdir, sub);
    //println!("fnm:{fnm}");

    let sbbe: SubBenInfo = ld_ben_bess1(&sub);
    //println!("sbbe:{sbbe:?}");
    let yr = year.parse::<usize>().unwrap();
    let mut lp = vec![0f32; 48];
    for yb in &sbbe.yrben {
        if yb.year == yr {
            lp = yb.day_prof.clone();
            break;
        }
    }
    let mut rf = Vec::<(String, f32)>::new();
    rf.push(("trlm".to_string(), sbbe.trlm));
    rf.push(("trcr".to_string(), sbbe.trcr));

    /*
    let slp = get_cal_lp(&year, &sub).unwrap();
    let lp = slp.day_prof.clone();
    let lcr = slp.calc_r;
    println!("DR5 ref: {:?}", rf);
    */
    let mut sld = SubGraphDraw5 {
        sub: sub.to_string(),
        fnm,
        lp,
        rf,
        yr: format!("{}", yr),
        ..Default::default() //sz: (400, 300),
    };
    if let Some(dr) = drpr {
        sld.sz = (dr.wd, dr.hg);
    }
    let rs = sld.draw_prof();
    match rs {
        Ok(bb) => (ctype_header(HttpType::ImagePng), bb).into_response(),
        Err(ee) => (ctype_header(HttpType::TextPlain), ee).into_response(),
    }
}

//use crate::p_31::ld_pv_ev_rt_m;
use crate::p_31::ld_pv_eb_proj;
use crate::p_31::ld_pv_et_proj;
use crate::p_31::ld_pv_ev_proj;
use crate::p_31::ld_sb_eb_proj;
use crate::p_31::ld_sb_et_proj;
use crate::p_31::ld_sb_ev_proj;
//use thousands::Separable;
//use numfmt::Formatter;

#[derive(Template, Debug, Default)]
#[template(path = "prc2/pv_ev_proj.html", escape = "none")]
pub struct PvEvProj {
    ev_proj: Vec<Vec<AreaRatio>>,
}
pub async fn pv_ev_proj() -> impl axum::response::IntoResponse {
    if let Ok(ev_proj) = ld_pv_ev_proj() {
        PvEvProj { ev_proj }
    } else {
        PvEvProj::default()
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/sb_ev_proj.html", escape = "none")]
pub struct SbEvProj {
    ev_proj: Vec<Vec<AreaRatio>>,
}
pub async fn sb_ev_proj() -> impl axum::response::IntoResponse {
    if let Ok(ev_proj) = ld_sb_ev_proj() {
        SbEvProj { ev_proj }
    } else {
        SbEvProj::default()
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/pv_et_proj.html", escape = "none")]
pub struct PvEtProj {
    ev_proj: Vec<Vec<AreaRatio>>,
}
pub async fn pv_et_proj() -> impl axum::response::IntoResponse {
    if let Ok(ev_proj) = ld_pv_et_proj() {
        PvEtProj { ev_proj }
    } else {
        PvEtProj::default()
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/sb_et_proj.html", escape = "none")]
pub struct SbEtProj {
    ev_proj: Vec<Vec<AreaRatio>>,
}
pub async fn sb_et_proj() -> impl axum::response::IntoResponse {
    if let Ok(ev_proj) = ld_sb_et_proj() {
        SbEtProj { ev_proj }
    } else {
        SbEtProj::default()
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/pv_eb_proj.html", escape = "none")]
pub struct PvEbProj {
    ev_proj: Vec<Vec<AreaRatio>>,
}
pub async fn pv_eb_proj() -> impl axum::response::IntoResponse {
    if let Ok(ev_proj) = ld_pv_eb_proj() {
        PvEbProj { ev_proj }
    } else {
        PvEbProj::default()
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "prc2/sb_eb_proj.html", escape = "none")]
pub struct SbEbProj {
    ev_proj: Vec<Vec<AreaRatio>>,
}
pub async fn sb_eb_proj() -> impl axum::response::IntoResponse {
    if let Ok(ev_proj) = ld_sb_eb_proj() {
        SbEbProj { ev_proj }
    } else {
        SbEbProj::default()
    }
}

pub async fn lp_web1() -> Result<(), Box<dyn std::error::Error>> {
    println!("web1");
    let app = Router::new()
        .route(&format!("{}{}", WEBROOT, "/sb_eb_proj"), get(sb_eb_proj))
        .route(&format!("{}{}", WEBROOT, "/pv_eb_proj"), get(pv_eb_proj))
        .route(&format!("{}{}", WEBROOT, "/sb_et_proj"), get(sb_et_proj))
        .route(&format!("{}{}", WEBROOT, "/pv_et_proj"), get(pv_et_proj))
        .route(&format!("{}{}", WEBROOT, "/sb_ev_proj"), get(sb_ev_proj))
        .route(&format!("{}{}", WEBROOT, "/pv_ev_proj"), get(pv_ev_proj))
        .route(&format!("{}{}", WEBROOT, "/"), get(lp_web1_pv))
        .route(&format!("{}{}", WEBROOT, "/lp_sub/:sb"), get(lp_sub))
        .route(
            &format!("{}{}", WEBROOT, "/lp_feed_meas/:feed/:year"),
            get(lp_feed_meas),
        )
        .route(&format!("{}{}", WEBROOT, "/pv2"), get(lp_pv2))
        .route(&format!("{}{}", WEBROOT, "/web2"), get(lp_web2))
        .route(&format!("{}{}", WEBROOT, "/lp_sub2/:sb"), get(lp_sub2))
        .route(
            &format!("{}{}", WEBROOT, "/lp_feed_calc/:feed/:year"),
            get(lp_feed_calc),
        )
        .route(
            &format!("{}{}", WEBROOT, "/lp_sub_calc/:sub/:year"),
            get(lp_sub_calc),
        )
        .route(&format!("{}{}", WEBROOT, "/ben_bess1"), get(ben_bess1))
        .route(&format!("{}{}", WEBROOT, "/ben_bess2"), get(ben_bess2))
        .route(&format!("{}{}", WEBROOT, "/lp_web22"), get(lp_web22))
        .route(&format!("{}{}", WEBROOT, "/lp_web23"), get(lp_web23))
        .route(&format!("{}{}", WEBROOT, "/lp_web24"), get(lp_web24))
        .route(&format!("{}{}", WEBROOT, "/lp_web24a"), get(lp_web24a))
        .route(&format!("{}{}", WEBROOT, "/lp_web24b"), get(lp_web24b))
        .route(&format!("{}{}", WEBROOT, "/lp_web24c"), get(lp_web24c))
        .route(&format!("{}{}", WEBROOT, "/lp_web24d"), get(lp_web24d))
        .route(&format!("{}{}", WEBROOT, "/gr_gp1/:sb"), get(gr_gp1))
        .route(
            &format!("{}{}", WEBROOT, "/sb_be_bess1/:sb"),
            get(sb_be_bess1),
        )
        .route(
            &format!("{}{}", WEBROOT, "/sb_gr_dr1/:sb/:yr"),
            get(sb_gr_dr1),
        )
        .route(
            &format!("{}{}", WEBROOT, "/sb_gr_dr2/:sb/:yr"),
            get(sb_gr_dr2),
        )
        .route(&format!("{}{}", WEBROOT, "/sb_gr_dr3/:sb"), get(sb_gr_dr3))
        .route(
            &format!("{}{}", WEBROOT, "/sb_gr_dr4/:sb/:yr"),
            get(sb_gr_dr4),
        )
        .route(
            &format!("{}{}", WEBROOT, "/sb_gr_dr5/:sb/:yr"),
            get(sb_gr_dr5),
        )
        .route(
            &format!("{}{}", WEBROOT, "/lp_gr_sb_yr/:sub/:year"),
            get(lp_gr_sb_yr),
        )
        .route(
            &format!("{}{}", WEBROOT, "/lp_gr_sb_yr2/:sub/:year"),
            get(lp_gr_sb_yr2),
        );

    //let lisn = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    let lisn = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(lisn, app).await.unwrap();
    Ok(())
}

pub fn lp_ana5() {
    let lp24 = get_all_lp("2024");
    let lp23 = get_all_lp("2023");
    let lp22 = get_all_lp("2022");
    let lp21 = get_all_lp("2021");
    if let (Some(s24), Some(s23), Some(s22), Some(s21)) = (lp24, lp23, lp22, lp21) {
        println!("OK");
        let (mut pen24, mut nen24) = (0f32, 0f32);
        for sc in &s24 {
            pen24 += sc.calc_v.p_en / 1_000_000f32;
            nen24 += sc.calc_v.n_en / 1_000_000f32;
        }
        let (mut pen23, mut nen23) = (0f32, 0f32);
        for sc in &s23 {
            pen23 += sc.calc_v.p_en / 1_000_000f32;
            nen23 += sc.calc_v.n_en / 1_000_000f32;
        }
        let (mut pen22, mut nen22) = (0f32, 0f32);
        for sc in &s22 {
            pen22 += sc.calc_v.p_en / 1_000_000f32;
            nen22 += sc.calc_v.n_en / 1_000_000f32;
        }
        let (mut pen21, mut nen21) = (0f32, 0f32);
        for sc in &s21 {
            pen21 += sc.calc_v.p_en / 1_000_000f32;
            nen21 += sc.calc_v.n_en / 1_000_000f32;
        }
        println!("{pen24} {nen24} {pen23} {nen23} {pen22} {nen22} {pen21} {nen21}");
    }
}

pub fn ben_bess3() {
    let mut sb_ben = Vec::<(&str, &str, String, SubBenInfo)>::new();
    let pv = grp1();
    let pvsb = pv_sub();
    let sbif = ld_p3_sub_inf();
    let sbtp = sub_type();
    let mut tot = 0f32;
    let mut be1 = 0f32;
    let mut be2 = 0f32;
    let mut mwh = 0i32;
    for p in &pv {
        let pp = p.to_string();
        let mut pbe1 = 0f32;
        let mut pbe2 = 0f32;
        let mut pbe3 = 0f32;
        let mut pmwh = 0f32;
        if let Some(sbv) = pvsb.get(&pp) {
            for sb in sbv {
                if let (Some(sf), Some(tp)) = (sbif.get(sb), sbtp.get(sb.as_str())) {
                    let _mx = sf.mvax.as_str();
                    let n = sf.name.to_string();
                    let ben = ld_ben_bess1(sb);
                    let mx_pw = ben.mx_pw;
                    //println!("pv:{pp} sb:{sb} mx:{mx} tp:{tp} mx:{mx_pw}");
                    if mx_pw > 0f32
                        && ben.grw < 7f32
                        && ben.be_start <= 3
                        && ben.trlm > 40f32
                        && (*tp == "AIS" || *tp == "GIS")
                    {
                        tot += ben.ls_ex_en;
                        mwh += (ben.ls_ex_en / 0.85f32 + 0.9f32) as i32;
                        pmwh += ben.ls_ex_en / 0.85f32 + 0.9f32;
                        pbe1 += ben.be_sub * 1_000_000.0;
                        pbe2 += ben.dec_ben;
                        pbe3 += ben.q_ben * 1_000_000.0;
                        be1 += ben.be_sub + ben.dec_ben / 1000000f32 + ben.q_ben;
                        be2 += ben.be_sub
                            + ben.dec_ben / 1000000f32
                            + ben.q_ben
                            + ben.ex_ben / 1000000f32;
                        sb_ben.push((sb, tp, n, ben));
                    }
                }
            }
        }
        println!("{} {} {} {} {}", pp, pbe1, pbe2, pbe3, pmwh);
    }
    let bcst = mwh as f32 * 21f32;
    println!("BMWh: {tot:.1} mwh:{mwh} be1:{be1} be2:{be2} cost:{bcst:.1}");
}

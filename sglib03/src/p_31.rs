use sglab02_lib::sg::load::load_pvcamp;
use sglab02_lib::sg::prc3::ld_p3_prvs;
use sglab02_lib::sg::prc4::EVCalc;
use sglab02_lib::sg::wk5::EvDistCalc;
use std::collections::HashMap;

pub fn ev_reg_calc() -> HashMap<String, EvDistCalc> {
    let mut pv_ca_mp = load_pvcamp();
    let mut pv_ca_mp2 = HashMap::new();
    let mut cn = 0f64;
    for (_k, v) in &pv_ca_mp {
        cn += v;
        //println!("{k} - {v}");
    }
    println!("total: {cn}");
    pv_ca_mp.insert("กรุงเทพมหานคร".to_string(), 967297.0);
    for (k, v) in &pv_ca_mp {
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
        } else {
            println!("no adj {}", adx.0);
        }
    }

    println!("assign %{}", ass_sm);

    let mut pv_car_reg_mp = HashMap::new();
    let mut total = 0.0f32;
    for (k, v) in &pv_ca_mp2 {
        if ["กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ"].contains(&k.as_str())
        {
            continue;
        }
        let pv_ca_reg = EvDistCalc {
            id: k.to_string(),
            ev_no: *v as f32,
            ..Default::default()
        };
        total += pv_ca_reg.ev_no;
        pv_car_reg_mp.insert(k.to_string(), pv_ca_reg);
    }

    for v in pv_car_reg_mp.values_mut() {
        if total > 0.0 {
            v.ev_pc = v.ev_no / total;
        }
    }
    pv_car_reg_mp
}

use serde::{Deserialize, Serialize};
use sglab02_lib::sg::prc3::ld_fd_trs;
use sglab02_lib::sg::prc3::ld_p3_sub_inf;
use sglab02_lib::sg::prc5::prvs;
use sglab02_lib::sg::prc5::pv_sub;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AreaRatio {
    pub yr: u32,
    pub pv: String,
    pub sb: String,
    pub rt: f32,
    pub no: f32,
    pub mw: f32,
    pub mwh: f32,
}

pub fn ev_calc() {
    let pv_car_reg_mp = ev_reg_calc();
    let pvs = prvs();
    let pvsb = pv_sub();
    let sbif = ld_p3_sub_inf();
    let fdtr = ld_fd_trs();
    println!("len:{}", pv_car_reg_mp.len());
    let mut tt0 = 0f64;
    let mut rt_sm = 0f64;
    let mut sb_ev_rt_m = HashMap::<String, f32>::new();
    let mut pv_rt_m = HashMap::<String, AreaRatio>::new();
    let mut sb_rt_m = HashMap::<String, AreaRatio>::new();
    for pv in pvs {
        let pv = pv.to_string();
        if let (Some(evrg), Some(sbv)) = (pv_car_reg_mp.get(&pv), pvsb.get(&pv)) {
            //println!("{pv} MWh {} {}", evrg.ev_no, evrg.ev_pc);
            let pvrt = AreaRatio {
                pv: pv.clone(),
                rt: evrg.ev_pc,
                ..Default::default()
            };
            pv_rt_m.insert(pv.to_string(), pvrt);
            let mut sb_pw_mp = HashMap::<String, f64>::new();
            let mut pv_eg = 0f64;
            for sb in sbv {
                let mut eg = 0f64;
                if let Some(sf) = sbif.get(sb) {
                    for fd in &sf.feeders {
                        if let Some(txv) = fdtr.get(fd) {
                            for tx in txv {
                                eg += tx.eg5_sm;
                            }
                        }
                    }
                }
                eg /= 1000f64; // to MWh
                sb_pw_mp.insert(sb.clone(), eg);
                pv_eg += eg;
            }
            tt0 += evrg.ev_pc as f64;
            for (k, v) in &sb_pw_mp {
                let rt = v / pv_eg;
                let rt0 = rt * evrg.ev_pc as f64;
                if evrg.ev_pc > 1f32 {
                    println!("{pv} {k} {}", evrg.ev_pc);
                }
                sb_ev_rt_m.insert(k.to_string(), rt0 as f32);
                rt_sm += rt0;
                let sbrt = AreaRatio {
                    pv: pv.clone(),
                    sb: k.clone(),
                    rt: rt0 as f32,
                    ..Default::default()
                };
                sb_rt_m.insert(k.to_string(), sbrt);
            }
        }
    }
    let mut rt00 = 0f32;
    for (k, v) in &sb_ev_rt_m {
        rt00 += v;
        println!("{k} - {v} => {}", v * 100000f32);
    }
    println!("total: {tt0} {rt_sm} sb:{} {rt00}", sb_ev_rt_m.len());

    if let Ok(ser) = bincode::serialize(&pv_rt_m) {
        std::fs::write("../sgdata/pv_ev_rt_m.bin", ser).unwrap();
    }
    if let Ok(ser) = bincode::serialize(&sb_rt_m) {
        std::fs::write("../sgdata/sb_ev_rt_m.bin", ser).unwrap();
    }
    if let Ok(pvrt) = ld_pv_ev_rt_m() {
        println!("pv ev rt {}", pvrt.len());
    }
    if let Ok(sbrt) = ld_sb_ev_rt_m() {
        println!("sb ev rt {}", sbrt.len());
    }
}

use std::fs::File;
use std::io::BufReader;

pub fn ld_pv_ev_rt_m() -> Result<HashMap<String, AreaRatio>, Box<dyn std::error::Error>> {
    if let Ok(pvrt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, AreaRatio>>(
        BufReader::new(File::open("../sgdata/pv_ev_rt_m.bin")?),
    ) {
        Ok(pvrt)
    } else {
        Err("file ../sgdata/pv_ev_rt_m.bin".into())
    }
}

pub fn ld_sb_ev_rt_m() -> Result<HashMap<String, AreaRatio>, Box<dyn std::error::Error>> {
    if let Ok(pvrt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, AreaRatio>>(
        BufReader::new(File::open("../sgdata/sb_ev_rt_m.bin")?),
    ) {
        Ok(pvrt)
    } else {
        Err("file ../sgdata/sb_ev_rt_m.bin".into())
    }
}

/// EV CAR for province
pub fn pv_ev_proj() {
    let ev_ls_yr = 75690.0;
    let ev_ac_no = 89907.0 + ev_ls_yr;
    let (ev_rt0, ev_gw0) = (0.1, 0.007);
    let ev_mw = 0.011; // mw
    let ev_dy_hr = 4.0;

    let pv_ev_rt = ld_pv_ev_rt_m().unwrap();
    let pvs = prvs();
    //let pvsb = pv_sub();
    let mut pv_ev_proj = Vec::<Vec<AreaRatio>>::new();
    for pv in pvs {
        if let Some(a) = pv_ev_rt.get(pv) {
            let mut proj_v = Vec::<AreaRatio>::new();

            let mut pv_ev_ac_no = ev_ac_no * a.rt;
            let mut pv_ev_la_yr = ev_ls_yr * a.rt;
            let mut ev_rt = ev_rt0;
            let pv_ev_mw = ev_ac_no * ev_mw * ev_dy_hr;
            let pv_ev_mwh = pv_ev_mw * 360.0;
            let ar = AreaRatio {
                pv: pv.clone(),
                yr: 2023,
                no: ev_ac_no,
                mw: pv_ev_mw,
                mwh: pv_ev_mwh,
                ..Default::default()
            };
            proj_v.push(ar);
            for y in 2024..=2039 {
                ev_rt += ev_gw0;
                pv_ev_la_yr *= 1.0 + ev_rt;
                pv_ev_ac_no += pv_ev_la_yr;
                let pv_ev_mw = pv_ev_ac_no * ev_mw * ev_dy_hr;
                let pv_ev_mwh = pv_ev_mw * 360.0;
                let ar = AreaRatio {
                    pv: pv.clone(),
                    yr: y,
                    no: pv_ev_ac_no,
                    mw: pv_ev_mw,
                    mwh: pv_ev_mwh,
                    ..Default::default()
                };
                proj_v.push(ar);
            }
            pv_ev_proj.push(proj_v);
        } // end if get
    } // end for pv
    if let Ok(ser) = bincode::serialize(&pv_ev_proj) {
        std::fs::write("../sgdata/pv_ev_proj.bin", ser).unwrap();
    }
    if let Ok(ld) = ld_pv_ev_proj() {
        println!("pv ev proj {}", ld.len());
    }
}

pub fn ld_pv_ev_proj() -> Result<Vec<Vec<AreaRatio>>, Box<dyn std::error::Error>> {
    if let Ok(pvrt) = bincode::deserialize_from::<BufReader<File>, Vec<Vec<AreaRatio>>>(
        BufReader::new(File::open("../sgdata/pv_ev_proj.bin")?),
    ) {
        Ok(pvrt)
    } else {
        Err("file ../sgdata/pv_ev_proj.bin".into())
    }
}

/// EV CAR for substation
pub fn sb_ev_proj() {
    let ev_ls_yr = 75690.0;
    let ev_ac_no = 89907.0 + ev_ls_yr;
    let (ev_rt0, ev_gw0) = (0.1, 0.007);
    let ev_mw = 0.011; // mw
    let ev_dy_hr = 4.0;

    let sb_ev_rt = ld_sb_ev_rt_m().unwrap();
    let pvs = prvs();
    let pvsb = pv_sub();
    let mut sb_ev_proj = Vec::<Vec<AreaRatio>>::new();
    for pv in pvs {
        if let Some(sbv) = pvsb.get(pv) {
            for sb in sbv {
                if let Some(a) = sb_ev_rt.get(sb) {
                    let mut proj_v = Vec::<AreaRatio>::new();
                    let mut pv_ev_ac_no = ev_ac_no * a.rt;
                    let mut pv_ev_la_yr = ev_ls_yr * a.rt;
                    let mut ev_rt = ev_rt0;
                    let pv_ev_mw = ev_ac_no * ev_mw * ev_dy_hr;
                    let pv_ev_mwh = pv_ev_mw * 360.0;
                    let ar = AreaRatio {
                        pv: pv.clone(),
                        sb: sb.clone(),
                        yr: 2023,
                        no: ev_ac_no,
                        mw: pv_ev_mw,
                        mwh: pv_ev_mwh,
                        ..Default::default()
                    };
                    proj_v.push(ar);
                    for y in 2024..=2039 {
                        ev_rt += ev_gw0;
                        pv_ev_la_yr *= 1.0 + ev_rt;
                        pv_ev_ac_no += pv_ev_la_yr;
                        let pv_ev_mw = pv_ev_ac_no * ev_mw * ev_dy_hr;
                        let pv_ev_mwh = pv_ev_mw * 360.0;
                        let ar = AreaRatio {
                            pv: pv.clone(),
                            yr: y,
                            no: pv_ev_ac_no,
                            mw: pv_ev_mw,
                            mwh: pv_ev_mwh,
                            ..Default::default()
                        };
                        proj_v.push(ar);
                    }
                    sb_ev_proj.push(proj_v);
                } // end if get
            }
        }
    } // end for pv
    if let Ok(ser) = bincode::serialize(&sb_ev_proj) {
        std::fs::write("../sgdata/sb_ev_proj.bin", ser).unwrap();
    }
    if let Ok(ld) = ld_sb_ev_proj() {
        println!("sb ev proj {}", ld.len());
    }
}

pub fn ld_sb_ev_proj() -> Result<Vec<Vec<AreaRatio>>, Box<dyn std::error::Error>> {
    if let Ok(sbrt) = bincode::deserialize_from::<BufReader<File>, Vec<Vec<AreaRatio>>>(
        BufReader::new(File::open("../sgdata/sb_ev_proj.bin")?),
    ) {
        Ok(sbrt)
    } else {
        Err("file ../sgdata/sb_ev_proj.bin".into())
    }
}

/// EV Truck for province
pub fn pv_et_proj() {
    let ev_ls_yr = 238.0 * 4.0;
    let ev_ac_no = 2962.0 + ev_ls_yr;
    let (ev_rt0, ev_gw0) = (0.2, 0.005);
    let ev_mw = 0.250; // mw
    let ev_dy_hr = 8.0;

    let pv_ev_rt = ld_pv_ev_rt_m().unwrap();
    let pvs = prvs();
    //let pvsb = pv_sub();
    let mut pv_ev_proj = Vec::<Vec<AreaRatio>>::new();
    for pv in pvs {
        if let Some(a) = pv_ev_rt.get(pv) {
            let mut proj_v = Vec::<AreaRatio>::new();

            let mut pv_ev_ac_no = ev_ac_no * a.rt;
            let mut pv_ev_la_yr = ev_ls_yr * a.rt;
            let mut ev_rt = ev_rt0;
            let pv_ev_mw = ev_ac_no * ev_mw * ev_dy_hr;
            let pv_ev_mwh = pv_ev_mw * 360.0;
            let ar = AreaRatio {
                pv: pv.clone(),
                yr: 2023,
                no: ev_ac_no,
                mw: pv_ev_mw,
                mwh: pv_ev_mwh,
                ..Default::default()
            };
            proj_v.push(ar);
            for y in 2024..=2039 {
                ev_rt += ev_gw0;
                pv_ev_la_yr *= 1.0 + ev_rt;
                pv_ev_ac_no += pv_ev_la_yr;
                let pv_ev_mw = pv_ev_ac_no * ev_mw * ev_dy_hr;
                let pv_ev_mwh = pv_ev_mw * 360.0;
                let ar = AreaRatio {
                    pv: pv.clone(),
                    yr: y,
                    no: pv_ev_ac_no,
                    mw: pv_ev_mw,
                    mwh: pv_ev_mwh,
                    ..Default::default()
                };
                proj_v.push(ar);
            }
            pv_ev_proj.push(proj_v);
        } // end if get
    } // end for pv
    if let Ok(ser) = bincode::serialize(&pv_ev_proj) {
        std::fs::write("../sgdata/pv_et_proj.bin", ser).unwrap();
    }
    if let Ok(ld) = ld_pv_et_proj() {
        println!("pv et proj {}", ld.len());
    }
}

pub fn ld_pv_et_proj() -> Result<Vec<Vec<AreaRatio>>, Box<dyn std::error::Error>> {
    if let Ok(pvrt) = bincode::deserialize_from::<BufReader<File>, Vec<Vec<AreaRatio>>>(
        BufReader::new(File::open("../sgdata/pv_et_proj.bin")?),
    ) {
        Ok(pvrt)
    } else {
        Err("file ../sgdata/pv_et_proj.bin".into())
    }
}

/// EV Truck for substation
pub fn sb_et_proj() {
    let ev_ls_yr = 238.0 * 4.0;
    let ev_ac_no = 2962.0 + ev_ls_yr;
    let (ev_rt0, ev_gw0) = (0.2, 0.005);
    let ev_mw = 0.250; // mw
    let ev_dy_hr = 8.0;

    let sb_ev_rt = ld_sb_ev_rt_m().unwrap();
    let pvs = prvs();
    let pvsb = pv_sub();
    let mut sb_ev_proj = Vec::<Vec<AreaRatio>>::new();
    for pv in pvs {
        if let Some(sbv) = pvsb.get(pv) {
            for sb in sbv {
                if let Some(a) = sb_ev_rt.get(sb) {
                    let mut proj_v = Vec::<AreaRatio>::new();
                    let mut pv_ev_ac_no = ev_ac_no * a.rt;
                    let mut pv_ev_la_yr = ev_ls_yr * a.rt;
                    let mut ev_rt = ev_rt0;
                    let pv_ev_mw = ev_ac_no * ev_mw * ev_dy_hr;
                    let pv_ev_mwh = pv_ev_mw * 360.0;
                    let ar = AreaRatio {
                        pv: pv.clone(),
                        sb: sb.clone(),
                        yr: 2023,
                        no: ev_ac_no,
                        mw: pv_ev_mw,
                        mwh: pv_ev_mwh,
                        ..Default::default()
                    };
                    proj_v.push(ar);
                    for y in 2024..=2039 {
                        ev_rt += ev_gw0;
                        pv_ev_la_yr *= 1.0 + ev_rt;
                        pv_ev_ac_no += pv_ev_la_yr;
                        let pv_ev_mw = pv_ev_ac_no * ev_mw * ev_dy_hr;
                        let pv_ev_mwh = pv_ev_mw * 360.0;
                        let ar = AreaRatio {
                            pv: pv.clone(),
                            yr: y,
                            no: pv_ev_ac_no,
                            mw: pv_ev_mw,
                            mwh: pv_ev_mwh,
                            ..Default::default()
                        };
                        proj_v.push(ar);
                    }
                    sb_ev_proj.push(proj_v);
                } // end if get
            }
        }
    } // end for pv
    if let Ok(ser) = bincode::serialize(&sb_ev_proj) {
        std::fs::write("../sgdata/sb_et_proj.bin", ser).unwrap();
    }
    if let Ok(ld) = ld_sb_et_proj() {
        println!("sb et proj {}", ld.len());
    }
}

pub fn ld_sb_et_proj() -> Result<Vec<Vec<AreaRatio>>, Box<dyn std::error::Error>> {
    if let Ok(sbrt) = bincode::deserialize_from::<BufReader<File>, Vec<Vec<AreaRatio>>>(
        BufReader::new(File::open("../sgdata/sb_et_proj.bin")?),
    ) {
        Ok(sbrt)
    } else {
        Err("file ../sgdata/sb_et_proj.bin".into())
    }
}

/// EV Bike for province
pub fn pv_eb_proj() {
    /*
    let ev_ls_yr = 238.0 * 4.0;
    let ev_ac_no = 2962.0 + ev_ls_yr;
    let (ev_rt0, ev_gw0) = (0.2, 0.005);
    let ev_mw = 0.250; // mw
    let ev_dy_hr = 8.0;
    */

    let ev_ls_yr = 9059.0 * 4.0;
    let ev_ac_no = 47116.0 + ev_ls_yr;
    let (ev_rt0, ev_gw0) = (0.1, 0.005);
    let ev_mw = 0.0001; // 100w
    let ev_dy_hr = 6.0;

    let pv_ev_rt = ld_pv_ev_rt_m().unwrap();
    let pvs = prvs();
    //let pvsb = pv_sub();
    let mut pv_ev_proj = Vec::<Vec<AreaRatio>>::new();
    for pv in pvs {
        if let Some(a) = pv_ev_rt.get(pv) {
            let mut proj_v = Vec::<AreaRatio>::new();

            let mut pv_ev_ac_no = ev_ac_no * a.rt;
            let mut pv_ev_la_yr = ev_ls_yr * a.rt;
            let mut ev_rt = ev_rt0;
            let pv_ev_mw = ev_ac_no * ev_mw * ev_dy_hr;
            let pv_ev_mwh = pv_ev_mw * 360.0;
            let ar = AreaRatio {
                pv: pv.clone(),
                yr: 2023,
                no: ev_ac_no,
                mw: pv_ev_mw,
                mwh: pv_ev_mwh,
                ..Default::default()
            };
            proj_v.push(ar);
            for y in 2024..=2039 {
                ev_rt += ev_gw0;
                pv_ev_la_yr *= 1.0 + ev_rt;
                pv_ev_ac_no += pv_ev_la_yr;
                let pv_ev_mw = pv_ev_ac_no * ev_mw * ev_dy_hr;
                let pv_ev_mwh = pv_ev_mw * 360.0;
                let ar = AreaRatio {
                    pv: pv.clone(),
                    yr: y,
                    no: pv_ev_ac_no,
                    mw: pv_ev_mw,
                    mwh: pv_ev_mwh,
                    ..Default::default()
                };
                proj_v.push(ar);
            }
            pv_ev_proj.push(proj_v);
        } // end if get
    } // end for pv
    if let Ok(ser) = bincode::serialize(&pv_ev_proj) {
        std::fs::write("../sgdata/pv_eb_proj.bin", ser).unwrap();
    }
    if let Ok(ld) = ld_pv_eb_proj() {
        println!("pv eb proj {}", ld.len());
    }
}

pub fn ld_pv_eb_proj() -> Result<Vec<Vec<AreaRatio>>, Box<dyn std::error::Error>> {
    if let Ok(pvrt) = bincode::deserialize_from::<BufReader<File>, Vec<Vec<AreaRatio>>>(
        BufReader::new(File::open("../sgdata/pv_eb_proj.bin")?),
    ) {
        Ok(pvrt)
    } else {
        Err("file ../sgdata/pv_eb_proj.bin".into())
    }
}

/// EV Bike for substation
pub fn sb_eb_proj() {
    /*
    let ev_ls_yr = 238.0 * 4.0;
    let ev_ac_no = 2962.0 + ev_ls_yr;
    let (ev_rt0, ev_gw0) = (0.2, 0.005);
    let ev_mw = 0.250; // mw
    let ev_dy_hr = 8.0;
    */

    let ev_ls_yr = 9059.0 * 4.0;
    let ev_ac_no = 47116.0 + ev_ls_yr;
    let (ev_rt0, ev_gw0) = (0.1, 0.005);
    let ev_mw = 0.0001; // 100w
    let ev_dy_hr = 6.0;

    let sb_ev_rt = ld_sb_ev_rt_m().unwrap();
    let pvs = prvs();
    let pvsb = pv_sub();
    let mut sb_ev_proj = Vec::<Vec<AreaRatio>>::new();
    for pv in pvs {
        if let Some(sbv) = pvsb.get(pv) {
            for sb in sbv {
                if let Some(a) = sb_ev_rt.get(sb) {
                    let mut proj_v = Vec::<AreaRatio>::new();
                    let mut pv_ev_ac_no = ev_ac_no * a.rt;
                    let mut pv_ev_la_yr = ev_ls_yr * a.rt;
                    let mut ev_rt = ev_rt0;
                    let pv_ev_mw = ev_ac_no * ev_mw * ev_dy_hr;
                    let pv_ev_mwh = pv_ev_mw * 360.0;
                    let ar = AreaRatio {
                        pv: pv.clone(),
                        sb: sb.clone(),
                        yr: 2023,
                        no: ev_ac_no,
                        mw: pv_ev_mw,
                        mwh: pv_ev_mwh,
                        ..Default::default()
                    };
                    proj_v.push(ar);
                    for y in 2024..=2039 {
                        ev_rt += ev_gw0;
                        pv_ev_la_yr *= 1.0 + ev_rt;
                        pv_ev_ac_no += pv_ev_la_yr;
                        let pv_ev_mw = pv_ev_ac_no * ev_mw * ev_dy_hr;
                        let pv_ev_mwh = pv_ev_mw * 360.0;
                        let ar = AreaRatio {
                            pv: pv.clone(),
                            yr: y,
                            no: pv_ev_ac_no,
                            mw: pv_ev_mw,
                            mwh: pv_ev_mwh,
                            ..Default::default()
                        };
                        proj_v.push(ar);
                    }
                    sb_ev_proj.push(proj_v);
                } // end if get
            }
        }
    } // end for pv
    if let Ok(ser) = bincode::serialize(&sb_ev_proj) {
        std::fs::write("../sgdata/sb_eb_proj.bin", ser).unwrap();
    }
    if let Ok(ld) = ld_sb_eb_proj() {
        println!("sb eb proj {}", ld.len());
    }
}

pub fn ld_sb_eb_proj() -> Result<Vec<Vec<AreaRatio>>, Box<dyn std::error::Error>> {
    if let Ok(sbrt) = bincode::deserialize_from::<BufReader<File>, Vec<Vec<AreaRatio>>>(
        BufReader::new(File::open("../sgdata/sb_eb_proj.bin")?),
    ) {
        Ok(sbrt)
    } else {
        Err("file ../sgdata/sb_eb_proj.bin".into())
    }
}

pub fn ev_calc0() {
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
        if ["กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ"].contains(&k.as_str())
        {
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
    let (ev_rt0, ev_gw0) = (0.1, 0.007);
    let ev_mw = 0.011; // mw
    let ev_dy_hr = 4.0;

    let et_ls_yr = 238.0 * 4.0;
    let et_ac_no = 2962.0 + et_ls_yr;
    let (et_rt0, et_gw0) = (0.2, 0.005);
    let et_mw = 0.250; // mw
    let et_dy_hr = 8.0;

    let eb_ls_yr = 9059.0 * 4.0;
    let eb_ac_no = 47116.0;
    let (eb_rt0, eb_gw0) = (0.1, 0.005);
    let eb_mw = 0.0001; // 100w
    let eb_dy_hr = 6.0;

    let mut pv_ev_for = HashMap::<String, Vec<EVCalc>>::new();
    let mut pv_et_for = HashMap::<String, Vec<EVCalc>>::new();
    let mut pv_eb_for = HashMap::<String, Vec<EVCalc>>::new();
    for pv in &prvs {
        let mut pv_ev_cal = Vec::<EVCalc>::new();
        let mut pv_et_cal = Vec::<EVCalc>::new();
        let mut pv_eb_cal = Vec::<EVCalc>::new();

        if let Some(v) = pv_car_reg_mp.get(pv) {
            let mut pv_ev_ac_no = ev_ac_no * v.ev_pc;
            let mut pv_ev_la_yr = ev_ls_yr * v.ev_pc;
            let mut ev_rt = ev_rt0;

            let mut pv_et_ac_no = et_ac_no * v.ev_pc;
            let mut pv_et_la_yr = et_ls_yr * v.ev_pc;
            let mut et_rt = et_rt0;

            let mut pv_eb_ac_no = eb_ac_no * v.ev_pc;
            let mut pv_eb_la_yr = eb_ls_yr * v.ev_pc;
            let mut eb_rt = eb_rt0;

            for y in 2024..=2039 {
                ev_rt += ev_gw0;
                pv_ev_la_yr *= 1.0 + ev_rt;
                pv_ev_ac_no += pv_ev_la_yr;
                let pv_ev_mw = pv_ev_ac_no * ev_mw * ev_dy_hr;
                let pv_ev_mwh = pv_ev_mw * 360.0;

                et_rt += et_gw0;
                pv_et_la_yr *= 1.0 + et_rt;
                pv_et_ac_no += pv_et_la_yr;
                let pv_et_mw = pv_et_ac_no * et_mw * et_dy_hr;
                let pv_et_mwh = pv_et_mw * 360.0;

                eb_rt += eb_gw0;
                pv_eb_la_yr *= 1.0 + eb_rt;
                pv_eb_ac_no += pv_eb_la_yr;
                let pv_eb_mw = pv_eb_ac_no * eb_mw * eb_dy_hr;
                let pv_eb_mwh = pv_eb_mw * 360.0;

                if y >= 2025 {
                    let evca = EVCalc {
                        pv: pv.to_string(),
                        yr: y,
                        evno: pv_ev_la_yr,
                        evac: pv_ev_ac_no,
                        evmw: pv_ev_mw,
                        evmwh: pv_ev_mwh,
                    };
                    pv_ev_cal.push(evca);

                    let etca = EVCalc {
                        pv: pv.to_string(),
                        yr: y,
                        evno: pv_et_la_yr,
                        evac: pv_et_ac_no,
                        evmw: pv_et_mw,
                        evmwh: pv_et_mwh,
                    };
                    pv_et_cal.push(etca);

                    let ebca = EVCalc {
                        pv: pv.to_string(),
                        yr: y,
                        evno: pv_eb_la_yr,
                        evac: pv_eb_ac_no,
                        evmw: pv_eb_mw,
                        evmwh: pv_eb_mwh,
                    };
                    pv_eb_cal.push(ebca);
                }
            }
            pv_ev_for.insert(pv.to_string(), pv_ev_cal);
            pv_et_for.insert(pv.to_string(), pv_et_cal);
            pv_eb_for.insert(pv.to_string(), pv_eb_cal);
        }
    }
    let file = format!("{}/p48_pv_ev.bin", sglab02_lib::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&pv_ev_for) {
        std::fs::write(file, ser).unwrap();
    }

    let file = format!("{}/p48_pv_et.bin", sglab02_lib::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&pv_et_for) {
        std::fs::write(file, ser).unwrap();
    }

    let file = format!("{}/p48_pv_eb.bin", sglab02_lib::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&pv_eb_for) {
        std::fs::write(file, ser).unwrap();
    }
}

pub fn pv_adjust() -> Vec<(&'static str, f64, f64)> {
    vec![
        ("สมุทรสาคร", 5.0, 0.0),
        ("พระนครศรีอยุธยา", 6.0, 0.0),
        ("ปทุมธานี", 12.0, 0.0),
        ("ชลบุรี", 6.0, 0.0),
        ("ระยอง", 6.0, 0.0),
        ("ฉะเชิงเทรา", 6.0, 0.0),
        ("นครปฐม", 6.0, 0.0),  // 6.0
        ("ปราจีนบุรี", 6.0, 0.0), // 7.0
        ("สงขลา", 5.0, 0.0),
        ("ราชบุรี", 5.0, 0.0),
        ("บุรีรัมย์", 0.0, 0.0), // 1.5
        ("ภูเก็ต", 0.0, 3.0),
        ("นครสวรรค์", 3.0, 0.0),
        ("ระนอง", 2.0, 0.0),
        ("สมุทรสงคราม", 2.0, 0.0),
        ("กระบี่", 2.0, 0.0),
        ("เพชรบุรี", 2.4, 0.0),
        ("สุราษฎร์ธานี", 4.0, 0.0),
        ("สระบุรี", 2.5, 0.0),
        ("สระแก้ว", 2.0, 0.0),
        ("นครราชสีมา", 4.0, 0.0),
        ("เชียงใหม่", 4.0, 0.0),
        ("พิษณุโลก", 2.0, 0.0),
        ("ขอนแก่น", 5.6, 0.0),
        ("ลพบุรี", 1.5, 0.0),
        ("กรุงเทพมหานคร", 0.0, 30.0),
        ("นนทบุรี", 0.0, 25.0),
        ("สมุทรปราการ", 0.0, 15.0),
        ("ยะลา", 0.0, 80.0),
        ("นราธิวาส", 0.0, 80.0),
        ("ปัตตานี", 0.0, 80.0),
        ("สกลนคร", 0.0, 80.0),
        ("กาฬสินธุ์", 0.0, 80.0),
        ("ตรัง", 0.0, 50.0),
        ("มหาสารคาม", 0.0, 80.0),
        ("มุกดาหาร", 0.0, 80.0),
        ("อุดรธานี", 0.0, 80.0),
        ("พัทลุง", 0.0, 80.0),
        ("นครศรีธรรมราช", 0.0, 80.0),
        ("ศรีสะเกษ", 0.0, 80.0),
        ("ร้อยเอ็ด", 0.0, 80.0),
        ("สุรินทร์", 0.0, 80.0),
        ("กาฬสินธุ์", 0.0, 80.0),
        ("สุโขทัย", 0.0, 80.0),
        ("แพร่", 0.0, 80.0),
        ("ประจวบคีรีขันธ์", 0.0, 80.0),
        ("พะเยา", 0.0, 80.0),
        ("ชุมพร", 0.0, 80.0),
        ("นครพนม", 0.0, 80.0),
        ("พิจิตร", 0.0, 80.0),
        ("บึงกาฬ", 0.0, 80.0),
        ("หนองบัวลำภู", 0.0, 80.0),
        ("หนองคาย", 0.0, 80.0),
        ("ตราด", 0.0, 80.0),
        ("สตูล", 0.0, 80.0),
        ("ชัยนาท", 0.0, 80.0),
        ("สิงห์บุรี", 0.0, 80.0),
    ]
}

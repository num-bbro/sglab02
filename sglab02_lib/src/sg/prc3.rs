use crate::sg::dcl::LoadProfVal;
use crate::sg::gis1::ar_list;
use crate::sg::gis1::DbfVal;
use crate::sg::imp::ld_replan;
use crate::sg::imp::REPlan;
use crate::sg::ldp::base;
use crate::sg::mvline::latlong_utm;
use crate::sg::mvline::utm_latlong;
use crate::sg::prc1::grp1;
use crate::sg::prc1::p1_spp_conn;
use crate::sg::prc1::p1_vspp_conn;
use crate::sg::prc1::SPPConn;
use crate::sg::prc1::SubstInfo;
use crate::sg::prc1::VSPPConn;
use crate::sg::prc2::Transformer;
use crate::sg::wk4::DayLoad;
use crate::sg::wk4::Substation as Wk4Substation;
use crate::sg::wk4::Wk4Proc;
use crate::sg::wk4::YearLoad;
use crate::sg::wk5::ld_fd_es_m;
use crate::sg::wk5::EvDistCalc;
use crate::sg::wk5::EvalPara1;
use crate::sg::wk5::Substation as Wk5Substation;
use crate::sg::wk5::Tranx;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataCalc {
    pub prov: String,
    pub ssid: String,
    pub fdid: String,

    pub year_load: YearLoad,
    pub last_year_load: YearLoad,

    pub tx: Tranx,
    pub para1: EvalPara1,
    pub ev: EvDistCalc,

    pub target_year_solar_energy: f32,
    pub target_solar_power: f32,
    pub target_solar_energy_storage: f32,

    pub solar_energy_series: Vec<f32>,
    pub solar_power_series: Vec<f32>,
    pub solar_day_energy_series: Vec<f32>,
    pub solar_storage_series: Vec<f32>,
    pub solar_storage_cost_series: Vec<f32>,
    pub solar_revenue_series: Vec<f32>,

    pub ev_car_series: Vec<f32>,
    pub ev_power_series: Vec<f32>,
    pub ev_energy_series: Vec<f32>,
    pub ev_revenue_series: Vec<f32>,
    pub ev_batt_required_series: Vec<f32>,
    pub ev_batt_cost_series: Vec<f32>,
    pub tx_to_all_ratio: f32,

    pub infra_invest_year: f32,
    pub smart_trx_cost: f32,
    pub smart_m1p_cost: f32,
    pub smart_m3p_cost: f32,
    pub comm_cost_year: f32,
    pub platform_cost: f32,
    pub implement_cost: f32,
    pub meter_reading_cost: f32,
    pub outage_operation_cost: f32,
    pub loss_in_power_line_cost: f32,
    pub loss_in_phase_balance_cost: f32,

    pub operation_cost: f32,
    pub operation_cost_m1p: f32,
    pub operation_cost_m3p: f32,
    pub operation_cost_trx: f32,
    pub operation_cost_ess: f32,

    pub infra_invest_year_series: Vec<f32>,
    pub smart_trx_cost_series: Vec<f32>,
    pub smart_m1p_cost_series: Vec<f32>,
    pub smart_m3p_cost_series: Vec<f32>,
    pub comm_cost_year_series: Vec<f32>,
    pub platform_cost_series: Vec<f32>,
    pub implement_cost_series: Vec<f32>,
    pub meter_reading_cost_series: Vec<f32>,
    pub outage_operation_cost_series: Vec<f32>,
    pub loss_in_power_line_cost_series: Vec<f32>,
    pub loss_in_phase_balance_cost_series: Vec<f32>,

    pub operation_cost_series: Vec<f32>,
    pub operation_cost_m1p_series: Vec<f32>,
    pub operation_cost_m3p_series: Vec<f32>,
    pub operation_cost_trx_series: Vec<f32>,
    pub operation_cost_ess_series: Vec<f32>,

    pub financial_benefit: f32,
    pub economic_benefit: f32,
    pub total_cost: f32,

    pub financial_benefit_series: Vec<f32>,
    pub economic_benefit_series: Vec<f32>,
    pub total_cost_series: Vec<f32>,

    pub financial_benefit_npv: f32,
    pub economic_benefit_npv: f32,
    pub total_cost_npv: f32,

    pub financial_benefit_npv_series: Vec<f32>,
    pub economic_benefit_npv_series: Vec<f32>,
    pub total_cost_npv_series: Vec<f32>,
    pub firr_series: Vec<f32>,
    pub eirr_series: Vec<f32>,
    pub net_financial_return_series: Vec<f32>,
    pub net_economic_return_series: Vec<f32>,

    pub firr: f32,
    pub eirr: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeedTranMeter {
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
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PEAGrid {
    pub prvs: Vec<String>,
    pub prv_sub: HashMap<String, Vec<String>>,
    pub sub_inf: HashMap<String, SubstInfo>,
    pub prv_calc: HashMap<String, DataCalc>,
    pub sub_calc: HashMap<String, DataCalc>,
    pub feed_calc: HashMap<String, DataCalc>,
    //feed_ldp: HashMap::<String, FeederLoad>,
}

// PEA Grid
pub async fn prc31() -> Result<(), Box<dyn std::error::Error>> {
    let mut subinf = Vec::<SubstInfo>::new();
    if let Ok(file) = File::open(crate::sg::ldp::res("subinfo.bin")) {
        let rd = BufReader::new(file);
        if let Ok(sbif) = bincode::deserialize_from::<BufReader<File>, Vec<SubstInfo>>(rd) {
            subinf = sbif;
        }
    }
    println!("read sub info");

    let mut prvs = Vec::<String>::new();
    let mut prv_sub = HashMap::<String, Vec<String>>::new();
    let mut sub_inf = HashMap::<String, SubstInfo>::new();
    let /*mut*/ _sub_ldp = HashMap::<String, Wk5Substation>::new();

    let mut prv_calc = HashMap::<String, DataCalc>::new();
    let mut sub_calc = HashMap::<String, DataCalc>::new();
    let mut feed_calc = HashMap::<String, DataCalc>::new();

    let mut sbids = HashMap::<String, i32>::new();
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

            let mut prv = DataCalc::default();
            prv.prov = sb.prov.clone();
            prv_calc.insert(prv.prov.clone(), prv);
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

    let mut sbldp = HashMap::<String, i32>::new();
    for sb in &subldp {
        sbldp.insert(sb.sbst.to_string(), 1);
    }

    let mut cn = 0;
    println!("==== NO IN LOAD PROFILE ====");
    for (s, _) in &sub_inf {
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

    let /*mut*/ _txno = 0;
    for ss in &mut subldp {
        let mut ss2 = DataCalc::default();
        ss2.ssid = ss.sbst.to_string();
        ss2.prov = ss.prov.to_string();

        let re = Regex::new(r"..._[0-9][0-9][VW]B01.*").unwrap();

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
            if re.is_match(&fd.feed) == false {
                continue;
            }
            //println!("  {}.{}->{} : {}", cn, fd.feed, fdid, re.is_match(&fd.feed));
            if let Some(_fx) = feed_calc.get_mut(&fdid) {
                // if get feed
                println!(
                    "======================  {}.{}->{} : {}",
                    cn,
                    fd.feed,
                    fdid,
                    re.is_match(&fd.feed)
                );
            } else {
                let mut fd2 = DataCalc::default();
                fd2.prov = ss.prov.to_string();
                fd2.ssid = ss.sbst.to_string();
                fd2.fdid = fdid.to_string();
                fd2.year_load = fd.year_load.clone();
                fd2.last_year_load = fd.last_year_load.clone();
                feed_calc.insert(fdid, fd2);
            } //end if fd
        }
        sub_calc.insert(ss2.ssid.to_string(), ss2);
    }

    let mut grd = PEAGrid {
        prvs,
        prv_sub,
        sub_inf,
        prv_calc,
        sub_calc,
        feed_calc,
    };

    sub_ldp_calc(&mut grd).await;
    power_quality(&mut grd, stw).await;

    let file = format!("{}/p3_prvs.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&grd.prvs) {
        std::fs::write(file, ser).unwrap();
    }
    let file = format!("{}/p3_prv_sub.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&grd.prv_sub) {
        std::fs::write(file, ser).unwrap();
    }
    let file = format!("{}/p3_sub_inf.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&grd.sub_inf) {
        std::fs::write(file, ser).unwrap();
    }
    let file = format!("{}/p3_prv_calc.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&grd.prv_calc) {
        println!("pv {}", &grd.prv_calc.len());
        std::fs::write(file, ser).unwrap();
    }
    let file = format!("{}/p3_sub_calc.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&grd.sub_calc) {
        println!("sb {}", &grd.sub_calc.len());
        std::fs::write(file, ser).unwrap();
    }
    let file = format!("{}/p3_feed_calc.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&grd.feed_calc) {
        println!("fd {}", &grd.feed_calc.len());
        std::fs::write(file, ser).unwrap();
    }
    Ok(())
}

pub fn ld_p3_prvs() -> Vec<String> {
    if let Ok(f) = File::open(crate::sg::ldp::res("p3_prvs.bin")) {
        if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<String>>(BufReader::new(f))
        {
            return dt;
        }
    }
    Vec::<String>::new()
}

pub fn ld_p3_prv_sub() -> HashMap<String, Vec<String>> {
    if let Ok(f) = File::open(crate::sg::ldp::res("p3_prv_sub.bin")) {
        if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, Vec<String>>>(
            BufReader::new(f),
        ) {
            return dt;
        }
    }
    HashMap::<String, Vec<String>>::new()
}

pub fn ld_p3_sub_inf() -> HashMap<String, SubstInfo> {
    if let Ok(f) = File::open(crate::sg::ldp::res("p3_sub_inf.bin")) {
        if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, SubstInfo>>(
            BufReader::new(f),
        ) {
            return dt;
        }
    }
    HashMap::<String, SubstInfo>::new()
}

pub fn ld_fd_trs() -> HashMap<String, Vec<Transformer>> {
    if let Ok(f) = File::open(crate::sg::ldp::res("fd_trs.bin")) {
        if let Ok(dt) = bincode::deserialize_from::<
            BufReader<File>,
            HashMap<String, Vec<Transformer>>,
        >(BufReader::new(f))
        {
            return dt;
        }
    }
    HashMap::<String, Vec<Transformer>>::new()
}

pub fn ld_p3_calc(fnm: &str) -> HashMap<String, DataCalc> {
    if let Ok(f) = File::open(crate::sg::ldp::res(fnm)) {
        if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, DataCalc>>(
            BufReader::new(f),
        ) {
            return dt;
        }
    }
    HashMap::<String, DataCalc>::new()
}

pub async fn power_quality(grd: &mut PEAGrid, stw: f32) {
    for pv in &grd.prvs {
        // for pv
        if let (Some(sbs), Some(/*mut*/ pvca)) = (
            grd.prv_sub.get(pv.as_str()),
            grd.prv_calc.get_mut(pv.as_str()),
        ) {
            // if pvca
            for sb in sbs {
                // for sb
                if let (Some(sbif), Some(/*mut*/ sbca)) = (
                    grd.sub_inf.get(sb.as_str()),
                    grd.sub_calc.get_mut(sb.as_str()),
                ) {
                    // if sub
                    for fd in &sbif.feeders {
                        // for feeders
                        if let Some(/*mut*/ fdca) = grd.feed_calc.get_mut(fd.as_str()) {
                            // if fdca
                            fdca.year_load.power(stw).await;
                            fdca.last_year_load.power(stw).await;
                            println!("{} {} {}", pv, sb, fd);
                        } // if fdca
                    } // for feeders
                    sbca.year_load.power(stw).await;
                    sbca.last_year_load.power(stw).await;
                } // if sub
            } // for day
            pvca.year_load.power(stw).await;
            pvca.last_year_load.power(stw).await;
        } // if pvca
    } // for pv
}

pub async fn sub_ldp_calc(grd: &mut PEAGrid) {
    for pv in &grd.prvs {
        // for pv
        if let (Some(sbs), Some(/*mut*/ pvca)) = (
            grd.prv_sub.get(pv.as_str()),
            grd.prv_calc.get_mut(pv.as_str()),
        ) {
            // if pvca
            let mut pv_val = [0f32; 365 * 48];
            let mut last_pv_val = [0f32; 365 * 48];
            for sb in sbs {
                // for sb
                if let (Some(sbif), Some(/*mut*/ sbca)) = (
                    grd.sub_inf.get(sb.as_str()),
                    grd.sub_calc.get_mut(sb.as_str()),
                ) {
                    // if sub
                    let mut ss_val = [0f32; 365 * 48];
                    let mut last_ss_val = [0f32; 365 * 48];
                    for fd in &sbif.feeders {
                        // for feeders
                        if let Some(fdca) = grd.feed_calc.get(fd.as_str()) {
                            // if fdca
                            println!("{} {} {}", pv, sb, fd);
                            for (di, dl) in fdca.year_load.loads.iter().enumerate() {
                                // for day
                                for (hi, hl) in dl.load.iter().enumerate() {
                                    // for hour
                                    let ii = di * 48 + hi;
                                    if let LoadProfVal::Value(v) = hl {
                                        ss_val[ii] += v;
                                        pv_val[ii] += v;
                                    } else {
                                        print!("ERR {} {} {}\n", fd, di, hi);
                                    }
                                } // for hour
                            } // for day
                            for (di, dl) in fdca.last_year_load.loads.iter().enumerate() {
                                // for day
                                for (hi, hl) in dl.load.iter().enumerate() {
                                    // for hour
                                    let ii = di * 48 + hi;
                                    if let LoadProfVal::Value(v) = hl {
                                        last_ss_val[ii] += v;
                                        last_pv_val[ii] += v;
                                    } else {
                                        print!("ERR {} {} {}\n", fd, di, hi);
                                    }
                                } // for hour
                            } // for day
                        } // if fdca
                    } // for feeders
                    sbca.year_load = YearLoad::default();
                    for di in 0..365 {
                        // for day
                        let mut day_load = DayLoad::default();
                        day_load.day = di + 1;
                        for hi in 0..48 {
                            // for hour
                            let ii = di * 48 + hi;
                            day_load.load.push(LoadProfVal::Value(ss_val[ii]));
                        } // for hour
                        sbca.year_load.loads.push(day_load);
                    } // for day
                    sbca.last_year_load = YearLoad::default();
                    for di in 0..365 {
                        // for day
                        let mut day_load = DayLoad::default();
                        day_load.day = di + 1;
                        for hi in 0..48 {
                            // for hour
                            let ii = di * 48 + hi;
                            day_load.load.push(LoadProfVal::Value(last_ss_val[ii]));
                        } // for hour
                        sbca.last_year_load.loads.push(day_load);
                    } // for day
                } // if sub
            } // for sub
            pvca.year_load = YearLoad::default();
            for di in 0..365 {
                // for day
                let mut day_load = DayLoad::default();
                day_load.day = di + 1;
                for hi in 0..48 {
                    // for hour
                    let ii = di * 48 + hi;
                    day_load.load.push(LoadProfVal::Value(pv_val[ii]));
                } // for hour
                pvca.year_load.loads.push(day_load);
            } // for day
            pvca.last_year_load = YearLoad::default();
            for di in 0..365 {
                // for day
                let mut day_load = DayLoad::default();
                day_load.day = di + 1;
                for hi in 0..48 {
                    // for hour
                    let ii = di * 48 + hi;
                    day_load.load.push(LoadProfVal::Value(last_pv_val[ii]));
                } // for hour
                pvca.last_year_load.loads.push(day_load);
            } // for day
        } // if pvca
    } // for pv
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransEnergy {
    pub pv_id: String,
    pub sb_id: String,
    pub fd_id: String,
    pub tx_no: usize,
    pub tx_p: usize,
    pub tx_c: usize,
    pub tx_pw_no: HashMap<usize, (usize, usize, usize)>,
    pub txp_pw_no: HashMap<usize, (usize, usize, usize)>,
    pub txc_pw_no: HashMap<usize, (usize, usize, usize)>,
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
    pub eg2_sm: f64,
}

impl TransEnergy {
    fn add_tr(&mut self, tr: &Transformer) {
        self.tx_no += 1;
        if tr.tx_own == "P" {
            self.tx_p += 1;
        }
        if tr.tx_own == "C" {
            self.tx_c += 1;
        }
        let power = tr.tx_power as usize;
        if let Some((cn, mn, mx)) = self.tx_pw_no.get_mut(&power) {
            *cn += 1;
            *mn = if tr.mt_cnt < *mn { tr.mt_cnt } else { *mn };
            *mx = if tr.mt_cnt > *mx { tr.mt_cnt } else { *mx };
        } else {
            self.tx_pw_no.insert(power, (1, tr.mt_cnt, tr.mt_cnt));
        }

        if tr.tx_own == "P" {
            if let Some((cn, mn, mx)) = self.txp_pw_no.get_mut(&power) {
                *cn += 1;
                *mn = if tr.mt_cnt < *mn { tr.mt_cnt } else { *mn };
                *mx = if tr.mt_cnt > *mx { tr.mt_cnt } else { *mx };
            } else {
                self.txp_pw_no.insert(power, (1, tr.mt_cnt, tr.mt_cnt));
            }
        }

        if tr.tx_own == "C" {
            if let Some((cn, mn, mx)) = self.txc_pw_no.get_mut(&power) {
                *cn += 1;
                *mn = if tr.mt_cnt < *mn { tr.mt_cnt } else { *mn };
                *mx = if tr.mt_cnt > *mx { tr.mt_cnt } else { *mx };
            } else {
                self.txc_pw_no.insert(power, (1, tr.mt_cnt, tr.mt_cnt));
            }
        }

        let mn = tr.mt_cnt;
        self.tx_mt_min = if mn < self.tx_mt_min {
            mn
        } else {
            self.tx_mt_min
        };
        let mx = tr.mt_cnt;
        self.tx_mt_max = if mx > self.tx_mt_max {
            mx
        } else {
            self.tx_mt_max
        };
        self.mt_ph_a += tr.mt_ph_a;
        self.mt_ph_b += tr.mt_ph_b;
        self.mt_ph_c += tr.mt_ph_c;
        self.mt_1_ph += tr.mt_1_ph;
        self.mt_3_ph += tr.mt_3_ph;
        self.mt_else += tr.mt_else;
        self.mt_cnt += tr.mt_cnt;
        self.eg5_a += tr.eg5_a;
        self.eg5_b += tr.eg5_b;
        self.eg5_c += tr.eg5_c;
        self.eg5_1p += tr.eg5_1p;
        self.eg5_3p += tr.eg5_3p;
        self.eg5_sm += tr.eg5_sm;
        self.eg2_a += tr.eg2_a;
        self.eg2_b += tr.eg2_b;
        self.eg2_c += tr.eg2_c;
        self.eg2_1p += tr.eg2_1p;
        self.eg2_3p += tr.eg2_3p;
        self.eg2_sm += tr.eg2_sm;
    }
}

// find transformer energy on each feeder
pub async fn prc32() -> Result<(), Box<dyn std::error::Error>> {
    let prvs = ld_p3_prvs();
    let prv_sub = ld_p3_prv_sub();
    let sub_inf = ld_p3_sub_inf();
    let prv_calc = ld_p3_calc("p3_prv_calc.bin");
    let fd_trs = ld_fd_trs();
    println!(
        "prvs {} prv_sub:{} sub_inf:{} prv_calc:{} fd_trs:{}",
        prvs.len(),
        prv_sub.len(),
        sub_inf.len(),
        prv_calc.len(),
        fd_trs.len()
    );

    let mut pv_treg_m = HashMap::<String, TransEnergy>::new();
    let mut sb_treg_m = HashMap::<String, TransEnergy>::new();
    let mut fd_treg_m = HashMap::<String, TransEnergy>::new();
    for pv in prvs {
        // for pv
        println!("pv: {}", pv);
        let mut pv_treg = TransEnergy {
            pv_id: pv.clone(),
            ..Default::default()
        };
        if let Some(sbs) = prv_sub.get(pv.as_str()) {
            // if pvca
            for sb in sbs {
                // for sb
                let mut sb_treg = TransEnergy {
                    pv_id: pv.clone(),
                    sb_id: sb.clone(),
                    ..Default::default()
                };
                if let Some(sbif) = sub_inf.get(sb.as_str()) {
                    // if sub
                    for fd in &sbif.feeders {
                        // for feeders
                        let mut fd_treg = TransEnergy {
                            pv_id: pv.clone(),
                            sb_id: sb.clone(),
                            fd_id: fd.clone(),
                            ..Default::default()
                        };
                        if let Some(trs) = fd_trs.get(fd) {
                            for tr in trs {
                                fd_treg.add_tr(tr);
                                sb_treg.add_tr(tr);
                                pv_treg.add_tr(tr);
                            }
                            //println!("    feed: {} - {} - {:?}", fd, trs.len(), pw_trs);
                        }
                        fd_treg_m.insert(fd.clone(), fd_treg);
                    } // for feeders
                } // if sub
                println!("  sub: {} {:?}", sb, sb_treg.txp_pw_no);
                sb_treg_m.insert(sb.clone(), sb_treg);
            } // for sub
        } // if pv
        pv_treg_m.insert(pv.clone(), pv_treg);
    }
    let file = format!("{}/p3_pv_treg_m.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&pv_treg_m) {
        std::fs::write(file, ser).unwrap();
    }
    let file = format!("{}/p3_sb_treg_m.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&sb_treg_m) {
        std::fs::write(file, ser).unwrap();
    }
    let file = format!("{}/p3_fd_treg_m.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&fd_treg_m) {
        std::fs::write(file, ser).unwrap();
    }
    Ok(())
}

pub fn ld_p3_treg_m(fnm: &str) -> HashMap<String, TransEnergy> {
    if let Ok(f) = File::open(crate::sg::ldp::res(fnm)) {
        if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, TransEnergy>>(
            BufReader::new(f),
        ) {
            return dt;
        }
    }
    HashMap::<String, TransEnergy>::new()
}

// find transformer energy on each feeder
pub async fn prc33() -> Result<(), Box<dyn std::error::Error>> {
    let treg_m = ld_p3_treg_m("p3_pv_treg_m.bin");
    println!("{}", treg_m.len());
    let mut txno = 0;
    let mut mt1 = 0;
    let mut mt3 = 0;
    for (k, t) in treg_m {
        txno += t.tx_no;
        mt1 += t.mt_1_ph;
        mt3 += t.mt_3_ph;
        println!(
            "{} {} =({} {})  m1:{} m3:{}",
            k, t.tx_no, t.tx_p, t.tx_c, t.mt_1_ph, t.mt_3_ph
        );
    }
    println!("tx: {} {} {}", txno, mt1, mt3);
    Ok(())
}

// find transformer energy on each feeder
pub async fn prc34() -> Result<(), Box<dyn std::error::Error>> {
    /*
    let pv_calc = ld_p3_calc("p3_prv_calc.bin");
    for (pv, ca) in pv_calc {
        if ca.year_load.loads.len()<365 { continue; }
        if ca.year_load.loads[100].load.len()<48 { continue; }
        println!("{} - {:?}", pv, ca.year_load.loads[100].load[24]);
    }
    */

    /*
    let sppv = p1_spp_conn();
    for spp in &sppv {
        println!("spp: {} {:?}", spp.sbid, spp.ppif);
    }
    */

    let /*mut*/ _pv_calc = ld_p3_calc("p3_sub_calc.bin");

    /*
    for (pv, mut ca) in pv_calc {
        if ca.year_load.loads.len()<365 { continue; }
        if ca.year_load.loads[100].load.len()<48 { continue; }
        //ca.year_load.power(4f32);
        year_load_power(&mut ca.year_load);
        println!("{} - {} {} - {} {}", pv,
            ca.year_load.power_quality.pos_peak, ca.year_load.power_quality.neg_peak,
            ca.year_load.power_quality.pos_avg, ca.year_load.power_quality.neg_avg,
        );

    }
    */

    let pv_calc = ld_p3_calc("p3_feed_calc.bin");
    println!("FEEDER");
    for (pv, mut ca) in pv_calc {
        if ca.year_load.loads.len() < 365 {
            continue;
        }
        if ca.year_load.loads[100].load.len() < 48 {
            continue;
        }
        year_load_power(&mut ca.year_load);
        println!(
            "{} - {} {} - {} {}",
            pv,
            ca.year_load.power_quality.pos_peak,
            ca.year_load.power_quality.neg_peak,
            ca.year_load.power_quality.pos_avg,
            ca.year_load.power_quality.neg_avg,
        );
    }

    Ok(())
}

pub fn year_load_power(yl: &mut YearLoad) {
    let sdwd = 4usize;
    let (wl, wr) = (24 - sdwd, 24 + sdwd);

    yl.power_quality.pos_peak = 0f32;
    yl.power_quality.pos_cnt = 0usize;
    yl.power_quality.pos_sum = 0f32;
    yl.power_quality.pos_energy = 0f32;
    yl.power_quality.mid_day_energy = 0f32;
    yl.power_quality.neg_peak = 0f32;
    yl.power_quality.neg_cnt = 0usize;
    yl.power_quality.neg_sum = 0f32;
    yl.power_quality.neg_energy = 0f32;
    for dl in &mut yl.loads {
        dl.power_quality.pos_peak = 0f32;
        dl.power_quality.pos_cnt = 0usize;
        dl.power_quality.pos_sum = 0f32;
        dl.power_quality.pos_energy = 0f32;
        dl.power_quality.mid_day_energy = 0f32;
        dl.power_quality.neg_peak = 0f32;
        dl.power_quality.neg_cnt = 0usize;
        dl.power_quality.neg_sum = 0f32;
        dl.power_quality.neg_energy = 0f32;

        for (i, hl) in &mut dl.load.iter().enumerate() {
            if let LoadProfVal::Value(va) = hl {
                let v = *va;
                if v >= 0.0f32 {
                    if v > dl.power_quality.pos_peak {
                        dl.power_quality.pos_peak = v;
                    }
                    dl.power_quality.pos_cnt += 1;
                    dl.power_quality.pos_sum += v;
                    dl.power_quality.pos_energy += v;
                    if i >= wl && i < wr {
                        dl.power_quality.mid_day_energy += v;
                    }
                } else {
                    let v = -v;
                    if v > dl.power_quality.neg_peak {
                        dl.power_quality.neg_peak = v;
                    }
                    dl.power_quality.neg_cnt += 1;
                    dl.power_quality.neg_sum += v;
                    dl.power_quality.neg_energy += v;
                }
            }
        }
        if dl.power_quality.pos_cnt > 0 {
            dl.power_quality.pos_avg = dl.power_quality.pos_sum / dl.power_quality.pos_cnt as f32;
            dl.power_quality.pos_energy *= 0.5f32;
            dl.power_quality.mid_day_energy *= 0.5f32;
        }
        if dl.power_quality.neg_cnt > 0 {
            dl.power_quality.neg_avg = dl.power_quality.neg_sum / dl.power_quality.neg_cnt as f32;
            dl.power_quality.neg_energy *= 0.5f32;
        }

        if dl.power_quality.pos_peak > yl.power_quality.pos_peak {
            yl.power_quality.pos_peak = dl.power_quality.pos_peak;
        }
        yl.power_quality.pos_cnt += dl.power_quality.pos_cnt;
        yl.power_quality.pos_sum += dl.power_quality.pos_sum;
        yl.power_quality.pos_energy += dl.power_quality.pos_energy;
        yl.power_quality.mid_day_energy += dl.power_quality.mid_day_energy;
        if dl.power_quality.neg_peak > yl.power_quality.neg_peak {
            yl.power_quality.neg_peak = dl.power_quality.neg_peak;
        }
        yl.power_quality.neg_cnt += dl.power_quality.neg_cnt;
        yl.power_quality.neg_sum += dl.power_quality.neg_sum;
        yl.power_quality.neg_energy += dl.power_quality.neg_energy;
    }

    if yl.power_quality.pos_cnt > 0 {
        yl.power_quality.pos_avg = yl.power_quality.pos_sum / yl.power_quality.pos_cnt as f32;
        yl.power_quality.pos_energy *= 0.5f32;
        yl.power_quality.mid_day_energy *= 0.5f32;
    }
    if yl.power_quality.neg_cnt > 0 {
        yl.power_quality.neg_avg = yl.power_quality.neg_sum / yl.power_quality.neg_cnt as f32;
        yl.power_quality.neg_energy *= 0.5f32;
    }
}

// find transformer energy on each feeder
pub async fn prc35() -> Result<(), Box<dyn std::error::Error>> {
    //let prvs = ld_p3_prvs();

    let prvs = grp1();
    let prv_sub = ld_p3_prv_sub();
    let sub_inf = ld_p3_sub_inf();
    let mut pv_calc = ld_p3_calc("p3_feed_calc.bin");
    let treg_m = ld_p3_treg_m("p3_pv_treg_m.bin");
    let treg_sb_m = ld_p3_treg_m("p3_sb_treg_m.bin");

    // SPP
    let sppv = p1_spp_conn();
    let mut spp_sb_m1 = HashMap::<String, Vec<SPPConn>>::new();
    let mut spp_sb_m2 = HashMap::<String, Vec<SPPConn>>::new();
    for spp in &sppv {
        if let Some(sppv) = spp_sb_m1.get_mut(&spp.sbid) {
            sppv.push(spp.clone());
        } else {
            spp_sb_m1.insert(spp.sbid.to_string(), vec![spp.clone()]);
        }
        if let Some(sppv) = spp_sb_m2.get_mut(&spp.sbi2) {
            sppv.push(spp.clone());
        } else {
            spp_sb_m1.insert(spp.sbi2.to_string(), vec![spp.clone()]);
        }
    }

    // VSPP
    let vsppv = p1_vspp_conn();
    let mut vspp_sb_m = HashMap::<String, Vec<VSPPConn>>::new();
    for vspp in &vsppv {
        if let Some(vsppv) = vspp_sb_m.get_mut(&vspp.sbid) {
            vsppv.push(vspp.clone());
        } else {
            vspp_sb_m.insert(vspp.sbid.to_string(), vec![vspp.clone()]);
        }
    }

    // RE Plan
    let newre = ld_replan();
    let mut re_sb_m = HashMap::<String, Vec<REPlan>>::new();
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
    //let mut fd_req = String::new();
    //let mut pv35 = String::new();
    let /*mut*/ _fd_req = String::new();
    let /*mut*/ _pv35 = String::new();
    write!(
        sb_req,
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        "sb",
        "name",
        "pv",
        "pos_peak",
        "pos_avg",
        "pos_ngy",
        "neg_peak",
        "neg_avg",
        "neg_cnt",
        "neg_ngy",
        "mwh ",
        "mw1",
        "max pw",
        "trxno",
        "lat+long",
        "DT",
        "MT",
        "E5:GWh",
        "E2:GWh"
    )
    .unwrap();
    for pv in prvs {
        // for pv
        let (mut pv1, mut pv2) = (String::new(), String::new());
        let (mut pv_pos_peak, mut pv_pos_avg, mut pv_neg_peak, mut pv_neg_avg, mut pvmw, mut pvtr) =
            (0f32, 0f32, 0f32, 0f32, 0i32, 0usize);
        let mut spp_cn = 0;
        let mut vspp_cn = 0;
        let mut rep_cn = 0;
        let /*mut*/ _rep_pw = 0;
        let mut pv_es = 0f32;
        if let Some(sbs) = prv_sub.get(pv) {
            // if pvca
            for sb in sbs {
                // for sb
                //println!("  sb: {} >>>>>", sb);
                let (mut pos_peak, mut pos_avg, mut neg_peak, mut neg_avg) =
                    (0f32, 0f32, 0f32, 0f32);
                //let (mut ssmw, mut sstr) = (0i32,0usize);
                let (mut neg_cnt, mut pos_engy, mut neg_engy) = (0usize, 0f32, 0f32);
                let (mut ss1, mut ss2) = (String::new(), String::new());
                let mut sb_es = 0f32;
                if let Some(sbif) = sub_inf.get(sb.as_str()) {
                    // if sub
                    //sbif.a();
                    for fd in &sbif.feeders {
                        // for feeders
                        if let Some(/*mut*/ ca) = pv_calc.get_mut(fd) {
                            if ca.year_load.loads.len() < 365 {
                                continue;
                            }
                            if ca.year_load.loads[100].load.len() < 48 {
                                continue;
                            }
                            let mut fd_es = 0f32;
                            if let Some(es) = fd_es_m.get(fd) {
                                fd_es = *es;
                            }
                            year_load_power(&mut ca.year_load);
                            year_load_power(&mut ca.last_year_load);
                            write!(
                                ss2,
                                "===|===|{}|{}|{}|{}|{}\t{}\n",
                                fd,
                                ca.year_load.power_quality.pos_peak,
                                ca.year_load.power_quality.pos_avg,
                                ca.year_load.power_quality.neg_peak,
                                ca.year_load.power_quality.neg_avg,
                                fd_es,
                            )
                            .unwrap();
                            pos_peak += ca.year_load.power_quality.pos_peak;
                            pos_avg += ca.year_load.power_quality.pos_avg;
                            neg_peak += ca.year_load.power_quality.neg_peak;
                            neg_avg += ca.year_load.power_quality.neg_avg;
                            //if ca.year_load.power_quality.neg_avg>0.0 {
                            neg_cnt += ca.year_load.power_quality.neg_cnt;
                            neg_engy += ca.year_load.power_quality.neg_energy;
                            pos_engy += ca.year_load.power_quality.pos_energy;
                            //}
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
                    let (mut x, mut y) = (0f64, 0f64);
                    let mut ldln = String::new();
                    if let Some((a, b)) = sb_loc_m.get(sb) {
                        x = *a;
                        y = *b;
                        let (xx, yy) = utm_latlong(x as f32, y as f32);
                        ldln = format!("{:.4},{:.4}", xx, yy);
                    }
                    let sb_es0 = (sb_es + 0.5) as i32;
                    let /*mut*/ sb_es0 = sb_es0 as f32;
                    let /*mut*/ _sb_pw0 = sb_es0 * 0.5;
                    write!(
                        ss1,
                        "===|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                        sb,
                        sbif.name,
                        pv,
                        pos_peak,
                        pos_avg,
                        neg_peak,
                        neg_avg,
                        neg_cnt,
                        sbif.mvxn,
                        sbif.trxn,
                        x,
                        y,
                        sb_es
                    )
                    .unwrap();
                    let (mut sb_tx_no, mut sb_mt_cnt, mut sb_eg5_sm, mut sb_eg2_sm) =
                        (0usize, 0usize, 0f64, 0f64);
                    if let Some(treg) = treg_sb_m.get(sb) {
                        sb_tx_no = treg.tx_no;
                        sb_mt_cnt = treg.mt_cnt;
                        sb_eg5_sm = treg.eg5_sm / 1000000.0;
                        sb_eg2_sm = treg.eg2_sm / 1000000.0;
                    }
                    if pos_peak > 1.0 {
                        let mut mwh = pos_engy / 365.0 / 25.0;
                        if mwh > 8.0 {
                            mwh = 8.0;
                        }
                        //let mut mw1 = (neg_peak+neg_avg)*0.5;
                        let _mw2 = neg_avg * 1.5;
                        //if mw1>mw2 { mw1 = mw2; }
                        mwh = mwh.ceil();
                        if neg_cnt < 200 {
                            /*mw1 = 0.0; */
                            mwh = 0.0;
                        };
                        let mw1 = mwh * 0.5;
                        if mwh > 0.0 {
                            write!(sb_req, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                            sb, sbif.name, pv, pos_peak, pos_avg, pos_engy, neg_peak, neg_avg, neg_cnt, neg_engy,
                            mwh, mw1, sbif.mvxn, sbif.trxn, ldln,
                            sb_tx_no, sb_mt_cnt, sb_eg5_sm.ceil(), sb_eg2_sm.ceil()).unwrap();
                        }
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
                if ss2.len() > 2 {
                    write!(pv2, "{}", ss1).unwrap();
                    //write!(pv2,"{}", ss2);
                }
            }
            let (mut tx_no, mut mt_cnt, mut eg5_sm, mut eg2_sm) = (0usize, 0usize, 0f64, 0f64);
            if let Some(treg) = treg_m.get(pv) {
                tx_no = treg.tx_no;
                mt_cnt = treg.mt_cnt;
                eg5_sm = treg.eg5_sm / 1000000.0;
                eg2_sm = treg.eg2_sm / 1000000.0;
            }
            write!(
                pv_req,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                pv,
                pv_pos_peak,
                pv_pos_avg,
                pv_neg_peak,
                pv_neg_avg,
                pvmw,
                pvtr,
                tx_no,
                mt_cnt,
                eg5_sm,
                eg2_sm,
                spp_cn,
                vspp_cn,
                rep_cn,
                pv_es
            )
            .unwrap();
            //write!(pv_req, "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n"
            //    , pv, pv_pos_peak, pv_pos_avg, pv_neg_peak, pv_neg_avg, pvmw, pvtr, tx_no, mt_cnt, eg5_sm, eg2_sm, spp_cn, vspp_cn, rep_cn, pv_es);
            write!(
                pv1,
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                pv,
                pv_pos_peak,
                pv_pos_avg,
                pv_neg_peak,
                pv_neg_avg,
                pvmw,
                pvtr,
                tx_no,
                mt_cnt,
                eg5_sm,
                eg2_sm,
                spp_cn,
                vspp_cn,
                rep_cn,
                pv_es
            )
            .unwrap();
        }
        print!("{}", pv1);

        print!("{}", pv2);
    }
    if let Ok(_) = fs::write("prj1/p35_pv_req.txt", pv_req) {}
    if let Ok(_) = fs::write("prj1/p35_sb_req.txt", sb_req) {
        println!("WRITE FILE P35_SB");
    }
    Ok(())
}

// find substation location with MV breaker and HV power transformer
pub async fn prc37() -> Result<(), Box<dyn std::error::Error>> {
    let _ly = "DS_Switch";
    let ly = "DS_CircuitBreaker";
    let _ht = "DS_HVTransformer";
    let dr = "../sgdata/db1";

    let pat1 = Regex::new(r"...[0-1][0-9][VW]B-01").unwrap();
    let _pat2 = Regex::new(r"[A-Z][A-Z][A-Z].*").unwrap();

    let mut dbv = Vec::<HashMap<String, DbfVal>>::new();
    let mut pnv = Vec::<(f64, f64)>::new();
    let /*mut*/ _db_ht_v = Vec::<HashMap<String,DbfVal>>::new();
    let /*mut*/ _pn_ht_v = Vec::<(f64,f64)>::new();
    for ar in ar_list() {
        let dbw = format!("{}/{}_{}.db", dr, ar, ly);
        let pnw = format!("{}/{}_{}.pn", dr, ar, ly);
        if let (Ok(fdb), Ok(fpn)) = (File::open(dbw), File::open(pnw)) {
            if let (Ok(mut db), Ok(mut pn)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfVal>>>(
                    BufReader::new(fdb),
                ),
                bincode::deserialize_from::<BufReader<File>, Vec<(f64, f64)>>(BufReader::new(fpn)),
            ) {
                dbv.append(&mut db);
                pnv.append(&mut pn);
            }
        }
        /*
        let dbw = format!("{}/{}_{}.db", dr, ar, ht);
        let pnw = format!("{}/{}_{}.pn", dr, ar, ht);
        if let (Ok(fdb),Ok(fpn)) = (File::open(dbw), File::open(pnw)) {
            if let (Ok(mut db), Ok(mut pn)) = (
                bincode::deserialize_from::<BufReader<File>, Vec::<HashMap<String,DbfVal>>>(BufReader::new(fdb)),
                bincode::deserialize_from::<BufReader<File>, Vec::<(f64,f64)>>(BufReader::new(fpn)),
             ) {
                db_ht_v.append(&mut db);
                pn_ht_v.append(&mut pn);
            }
        }
        */
    }
    let mut sbpn_m = HashMap::<String, (f64, f64)>::new();
    println!("all {} = {}", pnv.len(), dbv.len());
    let mut cn = 0;
    for i in 0..dbv.len() {
        let db = &dbv[i];
        let pn = &pnv[i];
        if let Some(fid) = db.get("FACILITYID") {
            if let DbfVal::Character(Some(tx)) = fid {
                if pat1.is_match(&tx) {
                    let sbid = &tx[0..3];
                    if let Some(_pn) = sbpn_m.get(sbid) {
                    } else {
                        cn += 1;
                        sbpn_m.insert(sbid.to_string(), (pn.0, pn.1));
                        println!("{}.{} ({},{})", cn, sbid, pn.0, pn.1);
                    }
                }
            }
        }
    }

    let file = format!("{}/p3_sub_loc.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&sbpn_m) {
        std::fs::write(file, ser).unwrap();
    }

    let sblo = ld_sub_loc();
    println!("sub loc {}", sblo.len());

    /*
    let mut cn = 0;
    let mut trpn_m = HashMap::<String,(f64,f64)>::new();
    for i in 0..db_ht_v.len() {
        let db = &db_ht_v[i];
        let pn = &pn_ht_v[i];
        if let Some(fid) = db.get("FEEDERID") {
            if let DbfVal::Character(Some(tx)) = fid {
                if pat2.is_match(&tx) {
                    let sbid = &tx[0..3].to_string();
                    if let Some(pn) = trpn_m.get(sbid) {
                    } else {
                        cn += 1;
                        trpn_m.insert(sbid.to_string(), (pn.0, pn.1));
                        println!("{}.{} ({},{})", cn, sbid, pn.0, pn.1);
                    }
                }
            }
        }
    }
    */
    Ok(())
}

pub fn ld_sub_loc() -> HashMap<String, (f64, f64)> {
    if let Ok(f) = File::open(crate::sg::ldp::res("p3_sub_loc.bin")) {
        if let Ok(mut dt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, (f64, f64)>>(
            BufReader::new(f),
        ) {
            let adj_sub = [
                ("KWB", (13.7584, 100.9922)),
                ("EKB", (13.5564, 100.2863)),
                ("KBK", (14.0598, 101.8403)),
                ("DOP", (14.6566, 100.5947)),
                ("WAM", (14.8072, 101.1191)),
                ("BPB", (13.8533, 99.8709)),
                ("CBN", (13.6153, 99.6413)),
                ("NOM", (13.7948, 99.7668)),
                ("CHA", (16.5615, 102.0636)),
                ("ROA", (7.8162, 100.3566)),
                ("LAA", (15.7663, 99.8683)),
                ("HTA", (14.9223, 102.1719)),
                ("PSA", (13.4948, 101.1638)),
                ("BSH", (15.8127, 101.0075)),
                ("MRA", (15.3011, 100.1042)),
                ("BAM", (14.3371, 100.1979)),
                ("SSA", (13.4444, 100.0621)),
            ];
            for (sb, (x, y)) in adj_sub {
                //let (xx, yy) = utm_latlong(x as f32, y as f32);
                let (xx, yy) = latlong_utm(x as f32, y as f32);
                dt.insert(sb.to_string(), (xx.into(), yy.into()));
            }
            return dt;
        }
    }
    HashMap::<String, (f64, f64)>::new()
}

// find transformer energy on each feeder
pub async fn prc38() -> Result<(), Box<dyn std::error::Error>> {
    //let prvs = ld_p3_prvs();

    let prvs = grp1();
    let prv_sub = ld_p3_prv_sub();
    let sub_inf = ld_p3_sub_inf();
    let mut pv_calc = ld_p3_calc("p3_feed_calc.bin");
    let treg_m = ld_p3_treg_m("p3_pv_treg_m.bin");
    let treg_sb_m = ld_p3_treg_m("p3_sb_treg_m.bin");

    // SPP
    let sppv = p1_spp_conn();
    let mut spp_sb_m1 = HashMap::<String, Vec<SPPConn>>::new();
    let mut spp_sb_m2 = HashMap::<String, Vec<SPPConn>>::new();
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
    let mut vspp_sb_m = HashMap::<String, Vec<VSPPConn>>::new();
    for vspp in &vsppv {
        if let Some(vsppv) = vspp_sb_m.get_mut(&vspp.sbid) {
            vsppv.push(vspp.clone());
        } else {
            vspp_sb_m.insert(vspp.sbid.to_string(), vec![vspp.clone()]);
        }
    }

    // RE Plan
    let newre = ld_replan();
    let mut re_sb_m = HashMap::<String, Vec<REPlan>>::new();
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
    //let mut fd_req = String::new();
    //let mut pv35 = String::new();
    let /*mut*/ _fd_req = String::new();
    let /*mut*/ _pv35 = String::new();
    write!(
        sb_req,
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        "sb",
        "name",
        "pv",
        "pos_peak",
        "pos_avg",
        "pos_ngy",
        "neg_peak",
        "neg_avg",
        "neg_cnt",
        "neg_ngy",
        "mwh ",
        "mw1",
        "max pw",
        "trxno",
        "lat+long",
        "DT",
        "MT",
        "E5:GWh",
        "E2:GWh",
        "SPP",
        "VSPP",
        "REPL"
    )
    .unwrap();
    for pv in prvs {
        // for pv
        let (mut pv1, mut pv2) = (String::new(), String::new());
        let (mut pv_pos_peak, mut pv_pos_avg, mut pv_neg_peak, mut pv_neg_avg, mut pvmw, mut pvtr) =
            (0f32, 0f32, 0f32, 0f32, 0i32, 0usize);
        let mut spp_cn = 0;
        let mut vspp_cn = 0;
        let mut rep_cn = 0;
        let /*mut*/ _rep_pw = 0;
        let mut pv_es = 0f32;
        if let Some(sbs) = prv_sub.get(pv) {
            // if pvca
            for sb in sbs {
                // for sb
                //println!("  sb: {} >>>>>", sb);
                let (mut pos_peak, mut pos_avg, mut neg_peak, mut neg_avg) =
                    (0f32, 0f32, 0f32, 0f32);
                //let (mut ssmw, mut sstr) = (0i32,0usize);
                let (mut neg_cnt, mut pos_engy, mut neg_engy) = (0usize, 0f32, 0f32);
                let (mut ss1, mut ss2) = (String::new(), String::new());
                let mut sb_es = 0f32;
                if let Some(sbif) = sub_inf.get(sb.as_str()) {
                    // if sub
                    //sbif.a();
                    for fd in &sbif.feeders {
                        // for feeders
                        if let Some(/*mut*/ ca) = pv_calc.get_mut(fd) {
                            if ca.year_load.loads.len() < 365 {
                                continue;
                            }
                            if ca.year_load.loads[100].load.len() < 48 {
                                continue;
                            }
                            let mut fd_es = 0f32;
                            if let Some(es) = fd_es_m.get(fd) {
                                fd_es = *es;
                            }
                            year_load_power(&mut ca.year_load);
                            year_load_power(&mut ca.last_year_load);
                            write!(
                                ss2,
                                "===|===|{}|{}|{}|{}|{}\t{}\n",
                                fd,
                                ca.year_load.power_quality.pos_peak,
                                ca.year_load.power_quality.pos_avg,
                                ca.year_load.power_quality.neg_peak,
                                ca.year_load.power_quality.neg_avg,
                                fd_es,
                            )
                            .unwrap();
                            pos_peak += ca.year_load.power_quality.pos_peak;
                            pos_avg += ca.year_load.power_quality.pos_avg;
                            neg_peak += ca.year_load.power_quality.neg_peak;
                            neg_avg += ca.year_load.power_quality.neg_avg;
                            //if ca.year_load.power_quality.neg_avg>0.0 {
                            neg_cnt += ca.year_load.power_quality.neg_cnt;
                            neg_engy += ca.year_load.power_quality.neg_energy;
                            pos_engy += ca.year_load.power_quality.pos_energy;
                            //}
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
                    let (mut x, mut y) = (0f64, 0f64);
                    let mut ldln = String::new();
                    if let Some((a, b)) = sb_loc_m.get(sb) {
                        x = *a;
                        y = *b;
                        let (xx, yy) = utm_latlong(x as f32, y as f32);
                        ldln = format!("{:.4},{:.4}", xx, yy);
                    }
                    if ldln.len() == 0 {
                        continue;
                    }
                    ldln = format!("https://maps.google.com/?q={}", ldln);
                    let sb_es0 = (sb_es + 0.5) as i32;
                    let /*mut*/ sb_es0 = sb_es0 as f32;
                    let /*mut*/ _sb_pw0 = sb_es0 * 0.5;
                    write!(
                        ss1,
                        "===|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                        sb,
                        sbif.name,
                        pv,
                        pos_peak,
                        pos_avg,
                        neg_peak,
                        neg_avg,
                        neg_cnt,
                        sbif.mvxn,
                        sbif.trxn,
                        x,
                        y,
                        sb_es
                    )
                    .unwrap();
                    let (mut sb_tx_no, mut sb_mt_cnt, mut sb_eg5_sm, mut sb_eg2_sm) =
                        (0usize, 0usize, 0f64, 0f64);
                    if let Some(treg) = treg_sb_m.get(sb) {
                        sb_tx_no = treg.tx_no;
                        sb_mt_cnt = treg.mt_cnt;
                        sb_eg5_sm = treg.eg5_sm / 1000000.0;
                        sb_eg2_sm = treg.eg2_sm / 1000000.0;
                    }
                    if pos_peak > 1.0 && neg_cnt > 10000 && neg_engy > 5000.0 {
                        //let mut mwh = pos_engy / 365.0 / 25.0;
                        let mut mwh = pos_engy / 365.0 / 10.0;
                        //if mwh>8.0 { mwh = 8.0; }
                        //let mut mw1 = (neg_peak+neg_avg)*0.5;
                        let _mw2 = neg_avg * 1.5;
                        //if mw1>mw2 { mw1 = mw2; }
                        mwh = mwh.ceil();
                        if neg_cnt < 200 {
                            /*mw1 = 0.0;*/
                            mwh = 0.0;
                        };
                        let mw1 = mwh * 0.5;

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
                        if mwh > 0.0 && mt_tx > 10.0 {
                            write!(sb_req, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                            sb, sbif.name, pv, pos_peak, pos_avg, pos_engy, neg_peak, neg_avg, neg_cnt, neg_engy,
                            mwh, mw1, sbif.mvxn, sbif.trxn, ldln,
                            sb_tx_no, sb_mt_cnt, sb_eg5_sm.ceil(), sb_eg2_sm.ceil(),
                            spp, vspp, repl).unwrap();
                        }
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
                if ss2.len() > 2 {
                    write!(pv2, "{}", ss1).unwrap();
                    //write!(pv2,"{}", ss2);
                }
            }
            let (mut tx_no, mut mt_cnt, mut eg5_sm, mut eg2_sm) = (0usize, 0usize, 0f64, 0f64);
            if let Some(treg) = treg_m.get(pv) {
                tx_no = treg.tx_no;
                mt_cnt = treg.mt_cnt;
                eg5_sm = treg.eg5_sm / 1000000.0;
                eg2_sm = treg.eg2_sm / 1000000.0;
            }
            write!(
                pv_req,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                pv,
                pv_pos_peak,
                pv_pos_avg,
                pv_neg_peak,
                pv_neg_avg,
                pvmw,
                pvtr,
                tx_no,
                mt_cnt,
                eg5_sm,
                eg2_sm,
                spp_cn,
                vspp_cn,
                rep_cn,
                pv_es
            )
            .unwrap();
            //write!(pv_req, "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n"
            //    , pv, pv_pos_peak, pv_pos_avg, pv_neg_peak, pv_neg_avg, pvmw, pvtr, tx_no, mt_cnt, eg5_sm, eg2_sm, spp_cn, vspp_cn, rep_cn, pv_es);
            write!(
                pv1,
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                pv,
                pv_pos_peak,
                pv_pos_avg,
                pv_neg_peak,
                pv_neg_avg,
                pvmw,
                pvtr,
                tx_no,
                mt_cnt,
                eg5_sm,
                eg2_sm,
                spp_cn,
                vspp_cn,
                rep_cn,
                pv_es
            )
            .unwrap();
        }
        print!("{}", pv1);

        print!("{}", pv2);
    }
    if let Ok(_) = fs::write("prj1/p38_pv_req.txt", pv_req) {}
    if let Ok(_) = fs::write("prj1/p38_sb_req.txt", sb_req) {
        println!("WRITE FILE P35_SB");
    }
    Ok(())
}

// find transformer energy on each feeder
pub async fn prc39() -> Result<(), Box<dyn std::error::Error>> {
    //let prvs = ld_p3_prvs();

    let prvs = grp1();
    let prv_sub = ld_p3_prv_sub();
    let sub_inf = ld_p3_sub_inf();
    let mut pv_calc = ld_p3_calc("p3_feed_calc.bin");
    let treg_m = ld_p3_treg_m("p3_pv_treg_m.bin");
    let treg_sb_m = ld_p3_treg_m("p3_sb_treg_m.bin");

    // SPP
    let sppv = p1_spp_conn();
    let mut spp_sb_m1 = HashMap::<String, Vec<SPPConn>>::new();
    let mut spp_sb_m2 = HashMap::<String, Vec<SPPConn>>::new();
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
    let mut vspp_sb_m = HashMap::<String, Vec<VSPPConn>>::new();
    for vspp in &vsppv {
        if let Some(vsppv) = vspp_sb_m.get_mut(&vspp.sbid) {
            vsppv.push(vspp.clone());
        } else {
            vspp_sb_m.insert(vspp.sbid.to_string(), vec![vspp.clone()]);
        }
    }

    // RE Plan
    let newre = ld_replan();
    let mut re_sb_m = HashMap::<String, Vec<REPlan>>::new();
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
    write!(
        sb_req,
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        "sb",
        "name",
        "pv",
        "pos_peak",
        "pos_avg",
        "pos_ngy",
        "neg_peak",
        "neg_avg",
        "neg_cnt",
        "neg_ngy",
        "mwh ",
        "mw1",
        "max pw",
        "trxno",
        "lat+long",
        "DT",
        "MT",
        "E5:GWh",
        "E2:GWh",
        "SPP",
        "VSPP",
        "REPL"
    )
    .unwrap();
    for pv in prvs {
        // for pv
        let (mut pv1, mut pv2) = (String::new(), String::new());
        let (mut pv_pos_peak, mut pv_pos_avg, mut pv_neg_peak, mut pv_neg_avg, mut pvmw, mut pvtr) =
            (0f32, 0f32, 0f32, 0f32, 0i32, 0usize);
        let mut spp_cn = 0;
        let mut vspp_cn = 0;
        let mut rep_cn = 0;
        let /*mut*/ _rep_pw = 0;
        let mut pv_es = 0f32;
        if let Some(sbs) = prv_sub.get(pv) {
            // if pvca
            for sb in sbs {
                // for sb
                //println!("  sb: {} >>>>>", sb);
                let (mut pos_peak, mut pos_avg, mut neg_peak, mut neg_avg) =
                    (0f32, 0f32, 0f32, 0f32);
                //let (mut ssmw, mut sstr) = (0i32,0usize);
                let (mut neg_cnt, mut pos_engy, mut neg_engy) = (0usize, 0f32, 0f32);
                let (mut ss1, mut ss2) = (String::new(), String::new());
                let mut sb_es = 0f32;
                if let Some(sbif) = sub_inf.get(sb.as_str()) {
                    // if sub
                    //sbif.a();
                    for fd in &sbif.feeders {
                        // for feeders
                        if let Some(/*mut*/ ca) = pv_calc.get_mut(fd) {
                            if ca.year_load.loads.len() < 365 {
                                continue;
                            }
                            if ca.year_load.loads[100].load.len() < 48 {
                                continue;
                            }
                            let mut fd_es = 0f32;
                            if let Some(es) = fd_es_m.get(fd) {
                                fd_es = *es;
                            }
                            year_load_power(&mut ca.year_load);
                            year_load_power(&mut ca.last_year_load);
                            write!(
                                ss2,
                                "===|===|{}|{}|{}|{}|{}\t{}\n",
                                fd,
                                ca.year_load.power_quality.pos_peak,
                                ca.year_load.power_quality.pos_avg,
                                ca.year_load.power_quality.neg_peak,
                                ca.year_load.power_quality.neg_avg,
                                fd_es,
                            )
                            .unwrap();
                            pos_peak += ca.year_load.power_quality.pos_peak;
                            pos_avg += ca.year_load.power_quality.pos_avg;
                            neg_peak += ca.year_load.power_quality.neg_peak;
                            neg_avg += ca.year_load.power_quality.neg_avg;
                            //if ca.year_load.power_quality.neg_avg>0.0 {
                            neg_cnt += ca.year_load.power_quality.neg_cnt;
                            neg_engy += ca.year_load.power_quality.neg_energy;
                            pos_engy += ca.year_load.power_quality.pos_energy;
                            //}
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
                    let (mut x, mut y) = (0f64, 0f64);
                    let mut ldln = String::new();
                    if let Some((a, b)) = sb_loc_m.get(sb) {
                        x = *a;
                        y = *b;
                        let (xx, yy) = utm_latlong(x as f32, y as f32);
                        ldln = format!("{:.4},{:.4}", xx, yy);
                    }
                    if ldln.len() == 0 {
                        continue;
                    }
                    ldln = format!("https://maps.google.com/?q={}", ldln);
                    let sb_es0 = (sb_es + 0.5) as i32;
                    let /*mut*/ sb_es0 = sb_es0 as f32;
                    let /*mut*/ _sb_pw0 = sb_es0 * 0.5;
                    write!(
                        ss1,
                        "===|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                        sb,
                        sbif.name,
                        pv,
                        pos_peak,
                        pos_avg,
                        neg_peak,
                        neg_avg,
                        neg_cnt,
                        sbif.mvxn,
                        sbif.trxn,
                        x,
                        y,
                        sb_es
                    )
                    .unwrap();
                    let (mut sb_tx_no, mut sb_mt_cnt, mut sb_eg5_sm, mut sb_eg2_sm) =
                        (0usize, 0usize, 0f64, 0f64);
                    if let Some(treg) = treg_sb_m.get(sb) {
                        sb_tx_no = treg.tx_no;
                        sb_mt_cnt = treg.mt_cnt;
                        sb_eg5_sm = treg.eg5_sm / 1000000.0;
                        sb_eg2_sm = treg.eg2_sm / 1000000.0;
                    }
                    if pos_peak > 1.0 {
                        //let mut mwh = pos_engy / 365.0 / 25.0;
                        let mut mwh = pos_engy / 365.0 / 10.0;
                        //if mwh>8.0 { mwh = 8.0; }
                        //let mut mw1 = (neg_peak+neg_avg)*0.5;
                        let _mw2 = neg_avg * 1.5;
                        //if mw1>mw2 { mw1 = mw2; }
                        mwh = mwh.ceil();
                        if neg_cnt < 200 {
                            /*mw1 = 0.0;*/
                            mwh = 0.0;
                        };
                        let mw1 = mwh * 0.5;

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
                        if mt_tx > 10.0 {
                            write!(sb_req, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                            sb, sbif.name, pv, pos_peak, pos_avg, pos_engy, neg_peak, neg_avg, neg_cnt, neg_engy,
                            mwh, mw1, sbif.mvxn, sbif.trxn, ldln,
                            sb_tx_no, sb_mt_cnt, sb_eg5_sm.ceil(), sb_eg2_sm.ceil(),
                            spp, vspp, repl).unwrap();
                        }
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
                if ss2.len() > 2 {
                    write!(pv2, "{}", ss1).unwrap();
                    //write!(pv2,"{}", ss2);
                }
            }
            let (mut tx_no, mut mt_cnt, mut eg5_sm, mut eg2_sm) = (0usize, 0usize, 0f64, 0f64);
            if let Some(treg) = treg_m.get(pv) {
                tx_no = treg.tx_no;
                mt_cnt = treg.mt_cnt;
                eg5_sm = treg.eg5_sm / 1000000.0;
                eg2_sm = treg.eg2_sm / 1000000.0;
            }
            write!(
                pv_req,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                pv,
                pv_pos_peak,
                pv_pos_avg,
                pv_neg_peak,
                pv_neg_avg,
                pvmw,
                pvtr,
                tx_no,
                mt_cnt,
                eg5_sm,
                eg2_sm,
                spp_cn,
                vspp_cn,
                rep_cn,
                pv_es
            )
            .unwrap();
            //write!(pv_req, "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n"
            //    , pv, pv_pos_peak, pv_pos_avg, pv_neg_peak, pv_neg_avg, pvmw, pvtr, tx_no, mt_cnt, eg5_sm, eg2_sm, spp_cn, vspp_cn, rep_cn, pv_es);
            write!(
                pv1,
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                pv,
                pv_pos_peak,
                pv_pos_avg,
                pv_neg_peak,
                pv_neg_avg,
                pvmw,
                pvtr,
                tx_no,
                mt_cnt,
                eg5_sm,
                eg2_sm,
                spp_cn,
                vspp_cn,
                rep_cn,
                pv_es
            )
            .unwrap();
        }
        print!("{}", pv1);

        print!("{}", pv2);
    }
    if let Ok(_) = fs::write("prj1/p39_pv_req.txt", pv_req) {}
    if let Ok(_) = fs::write("prj1/p39_sb_req.txt", sb_req) {
        println!("WRITE FILE P35_SB");
    }
    Ok(())
}

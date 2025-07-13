//use crate::prc2::get_all_lp;
use crate::prc2::get_lps;
//use image::codecs::qoi;
//use crate::prc2::SubLoadProf;
use sglab02_lib::sg::prc1::grp1;
use sglab02_lib::sg::prc3::ld_p3_sub_inf;
use sglab02_lib::sg::prc4::ld_pv_sbv_m;
//use std::collections::HashMap;
use crate::prc2::SUB_BESS;
use num_traits::Pow;
use serde::{Deserialize, Serialize};
use sglab02_lib::sg::mvline::utm_latlong;
use sglab02_lib::sg::prc3::ld_p3_prv_sub;
use sglab02_lib::sg::prc5::prvs;
use std::collections::HashMap;
use std::fs::File;

pub const BC_SUBST_COST: f32 = 328.95;
pub const BC_DISCN_RATE: f32 = 4.86;
pub const BC_SUBST_YLEN: usize = 25;
pub const BC_BESS_YLEN: usize = 12;
pub const BC_POWER_FACT: f32 = 0.9;
pub const BC_TR_LOAD_LIM: f32 = 0.75;
pub const BC_TR_CRIT_LIM: f32 = 0.90;
pub const BC_POW_GRW_LIM: f32 = 4.00;
pub const BC_PEA_PROFIT: f32 = 0.42;
pub const BC_SELL_PRICE: f32 = 3.99;
pub const BC_ON_PEAK_COST: f32 = 3.6199;
pub const BC_OFFPEAK_COST: f32 = 2.3341;
pub const BC_NO_DAY_IN_YEAR: usize = 246;
pub const BC_BASE_YEAR: usize = 2024;
pub const BC_PROJ_YLEN: usize = 15;
pub const BC_ON_PEAK_BEGIN: usize = 18;
pub const BC_ON_PEAK_END: usize = 44;
pub const BC_ADMIN_COST: f32 = 6.0;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SubBenInfo {
    pub prv: String,
    pub sub: String,
    pub name: String,
    pub trlm: f32,                  // normal limit
    pub trcr: f32,                  // critical limit
    pub yr_start: usize,            // overload start year
    pub be_start: usize,            // benefit start year after implement
    pub yrben: Vec<SubYearBenInfo>, // each year benefit info 2024,2025,...,2039
    pub mx_pw: f32,                 // max power from 4 yr profile, 2*peak + 1*avg
    pub grw0: f32,                  // year grw rate in 100
    pub grw: f32,                   // year grw rate in 100
    pub pw_inc_yr: f32,             // power increase per year
    pub pw2mx_rt: f32,              // power per max power ratio mx_pw / trlm
    pub yr_len: usize,              // number of year
    pub be_sub: f32,                // totol benefit
    pub peek: f32,                  // peak power MW
    pub loc: String,
    pub p_en: f32,
    pub n_en: f32,
    pub ls_ex_en: f32, // last year exceed energy
    pub ls_ex_pw: f32, // last year exceed max power
    pub ac_ex_en: f32, // all year accumulated exceed energy
    pub ac_ex_be: f32, // all year accumulated exceed benefit
    pub yi_en: f32,
    pub yi_en_onp: f32,
    pub yi_en_ofp: f32,
    pub ex_ben_onp: f32,
    pub ex_ben_ofp: f32,
    pub ex_ben: f32,
    pub dec_ben: f32,
    pub q_bess: f32,
    pub q_cost: f32,
    pub q_ben: f32,
    pub bat_mwh: f32,
    pub bat_cost: f32,
    pub be_sub_save: Vec<(u32, f32)>,
    pub be_re_diff: Vec<(u32, f32)>,
    pub be_svg_save: Vec<(u32, f32)>,
    pub be_en_added: Vec<(u32, f32)>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SubYearBenInfo {
    pub year: usize,
    pub sub_cost: f32,
    pub sub_npv: f32,
    pub sub_save: f32,
    pub day_prof: Vec<f32>,
}

pub const LP_BESS_DIR: &str = "../sgdata/bess";
//use sglab02_lib::sg::prc1::SubstInfo;
use sglab02_lib::sg::prc3::ld_sub_loc;
use sglab02_lib::sg::prc4::Proc41Item;
use sglab02_lib::sg::prc5::pv_sub;

pub fn p4_ben1() {
    let _ = std::fs::create_dir_all(LP_BESS_DIR);
    let mut known = HashMap::<&str, &str>::new();
    for (s, t, _n) in SUB_BESS {
        known.insert(s, t);
    }

    let (sbm, sbm0, sbm1, sbm2) = get_lps();
    let pv = grp1();
    let pvsb = pv_sub();
    //let pv = prvs();
    //pub fn prvs() -> &'static Vec::<String> { PRVS.get_or_init(prvs_init) }
    let _sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();

    // BESS infra defer benefit calculation
    let r = BC_DISCN_RATE / 100f32;
    let n = BC_SUBST_YLEN as f32;
    //let n = BC_BESS_YLEN;
    let anrt = (1f32 - Pow::pow(1f32 + r, -n)) / r;
    let ancs = BC_SUBST_COST / anrt;
    //println!("anrt: {anrt} {ancs}");
    let cst: Vec<f64> = vec![ancs.into(); 25];
    //    let mut be0 = 0f64;
    //    let mut sb0 = 0f64;
    let mut subcst = Vec::<SubYearBenInfo>::new();
    for (i, v) in cst.iter().enumerate() {
        let fa = v / Pow::pow(1f64 + r as f64, i as f64);
        let be = if i < 12 {
            v * Pow::pow(1.03f64, i as f64)
        } else {
            0f64
        };
        //        sb0 += fa;
        //        be0 += be;
        subcst.push(SubYearBenInfo {
            year: i,
            sub_cost: *v as f32,
            sub_npv: fa as f32,
            sub_save: be as f32,
            ..Default::default()
        });
        //println!("  {i} {v:.1} {fa:.1} {be:.1}");
    }

    let qcsy = 10f32;
    //let r = BC_DISCN_RATE / 100f32;
    //let fa = qcsy / Pow::pow(1f32 + r, 10f32);
    for y in 0..=BC_PROJ_YLEN {
        let _be = qcsy * Pow::pow(1.03f32, y as f32);
        //println!("{} {:.1}", y, be);
    }

    //println!("sub:{sb0:.1} bene:{be0:.1}");
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = pvsb.get(&pp) {
            for sb in sbv {
                let sbid = sb.to_string();
                println!("pv:{pp} sb:{sbid}");
                if let (Some(sf), Some(slp), Some(slp0), Some(slp1), Some(slp2)) = (
                    sbif.get(&sbid),
                    sbm.get(&sbid),
                    sbm0.get(&sbid),
                    sbm1.get(&sbid),
                    sbm2.get(&sbid),
                ) {
                    let (xx, yy) = if let Some(latlong) = slp.latlong {
                        let (x, y) = latlong;
                        let (xx, yy) = utm_latlong(x as f32, y as f32);
                        (xx, yy)
                    } else {
                        (0f32, 0f32)
                    };
                    let loc = format!("https://maps.google.com/?q={:.4},{:.4}", xx, yy);

                    let mut grw = 0.0;

                    let rf24 = (slp.pk30_time_prof.calc_v.p_pk
                        + 2f32 * slp.pk30_time_prof.calc_v.p_avg)
                        / 3f32;
                    let rf23 = (slp0.pk30_time_prof.calc_v.p_pk
                        + 2f32 * slp0.pk30_time_prof.calc_v.p_avg)
                        / 3f32;
                    let rf22 = (slp1.pk30_time_prof.calc_v.p_pk
                        + 2f32 * slp1.pk30_time_prof.calc_v.p_avg)
                        / 3f32;
                    let rf21 = (slp2.pk30_time_prof.calc_v.p_pk
                        + 2f32 * slp2.pk30_time_prof.calc_v.p_avg)
                        / 3f32;
                    let mut daylp = slp.pk30_time_prof.time_v.clone();
                    let mut peek = slp.pk30_time_prof.calc_v.p_pk;
                    let mut p_en = slp.pk30_time_prof.calc_v.p_en;
                    let mut n_en = slp.pk30_time_prof.calc_v.n_en;
                    let mut en_onp = slp.pk30_time_prof.calc_onp.p_en;
                    let mut en_ofp = slp.pk30_time_prof.calc_ofp.p_en;

                    if rf23 > 0f32 {
                        let a24 = (rf24 - rf23) / rf23 * 100f32;
                        if a24 > grw {
                            grw = a24;

                            /*
                            daylp = slp0.day_prof.clone();
                            peek = slp0.calc_v.p_pk;
                            p_en = slp0.calc_v.p_en;
                            n_en = slp0.calc_v.n_en;
                            */

                            daylp = slp0.pk30_time_prof.time_v.clone();
                            peek = slp0.pk30_time_prof.calc_v.p_pk;
                            p_en = slp0.pk30_time_prof.calc_v.p_en;
                            n_en = slp0.pk30_time_prof.calc_v.n_en;
                            en_onp = slp0.pk30_time_prof.calc_onp.p_en;
                            en_ofp = slp0.pk30_time_prof.calc_ofp.p_en;
                        }
                    }
                    if rf22 > 0f32 {
                        let a23 = (rf23 - rf22) / rf22 * 100f32;
                        if a23 > grw {
                            grw = a23;

                            /*
                            daylp = slp1.day_prof.clone();
                            peek = slp1.calc_v.p_pk;
                            p_en = slp1.calc_v.p_en;
                            n_en = slp1.calc_v.n_en;
                            */

                            daylp = slp1.pk30_time_prof.time_v.clone();
                            peek = slp1.pk30_time_prof.calc_v.p_pk;
                            p_en = slp1.pk30_time_prof.calc_v.p_en;
                            n_en = slp1.pk30_time_prof.calc_v.n_en;
                            en_onp = slp1.pk30_time_prof.calc_onp.p_en;
                            en_ofp = slp1.pk30_time_prof.calc_ofp.p_en;
                        }
                    }
                    if rf21 > 0f32 {
                        let a22 = (rf22 - rf21) / rf21 * 100f32;
                        if a22 > grw {
                            grw = a22;

                            /*
                            daylp = slp2.day_prof.clone();
                            peek = slp2.calc_v.p_pk;
                            p_en = slp2.calc_v.p_en;
                            n_en = slp2.calc_v.n_en;
                            */

                            daylp = slp2.pk30_time_prof.time_v.clone();
                            peek = slp2.pk30_time_prof.calc_v.p_pk;
                            p_en = slp2.pk30_time_prof.calc_v.p_en;
                            n_en = slp2.pk30_time_prof.calc_v.n_en;
                            en_onp = slp2.pk30_time_prof.calc_onp.p_en;
                            en_ofp = slp2.pk30_time_prof.calc_ofp.p_en;
                        }
                    }
                    /*
                    if grw > BC_POW_GRW_LIM {
                        grw = BC_POW_GRW_LIM;
                    }
                    */

                    let trlm = slp.maxmva * BC_POWER_FACT * BC_TR_LOAD_LIM;
                    let trcr = slp.maxmva * BC_POWER_FACT * BC_TR_CRIT_LIM;
                    let pwmx = rf24.max(rf23.max(rf22.max(rf21)));

                    let mut pkt = trcr + 10f32;
                    //let mut sbbe = SubBenInfo::default();
                    let mut sbbe: SubBenInfo;
                    let grw0 = grw;
                    loop {
                        let pk0 = peek;
                        let grw1 = Pow::pow(pkt / pk0, 1f32 / BC_PROJ_YLEN as f32) - 1f32;
                        let grw1 = grw1 * 100f32;
                        grw = grw1;

                        let dppy = trlm * grw / 100f32; // MW/yr increase
                        let mxrt = pwmx / trlm * 100f32;

                        let yrno = (trlm - pwmx) / dppy;
                        let yr_start = yrno as usize;

                        let mut be0 = 0f32;
                        //if sb.sbid == "BNP" {
                        //println!("BNP");
                        let be_start = if yr_start < 4 { 1 } else { yr_start - 3 };
                        for yi in be_start - 1..BC_BESS_YLEN {
                            be0 += subcst[yi].sub_save;
                        }

                        let mut yrben = Vec::<SubYearBenInfo>::new();
                        for i in 0..=BC_PROJ_YLEN {
                            let mut day_prof = daylp.clone();
                            //println!("day_prof len:{}", day_prof.len());
                            for j in 0..48 {
                                day_prof[j] *= Pow::pow(1f32 + grw / 100f32, i as f32) as f32;
                            }
                            let yrb = SubYearBenInfo {
                                year: i + 2024,
                                day_prof,
                                ..Default::default()
                            };
                            yrben.push(yrb);
                        }

                        let mut ls_ex_sm = 0f32;
                        let mut ls_ex_pw = 0f32;
                        for i in 0..48 {
                            let dv = yrben[BC_PROJ_YLEN].day_prof[i] - trcr;
                            if dv >= 0f32 {
                                ls_ex_pw = if dv > ls_ex_pw { dv } else { ls_ex_pw };
                                ls_ex_sm += dv;
                            }
                        }
                        let ls_ex_en = ls_ex_sm * 0.5f32;
                        if ls_ex_en > 20f32 {
                            pkt *= 0.99f32;
                            continue;
                        }

                        //let sub = &sb.sbid;
                        //let _ybe = &yrben[yr_start];
                        let uc_onp = BC_SELL_PRICE - BC_ON_PEAK_COST;
                        let uc_ofp = BC_SELL_PRICE - BC_OFFPEAK_COST;

                        let en0 = p_en * Pow::pow(1f32 + grw / 100f32, yr_start as f32) as f32;
                        let enn = en_onp * Pow::pow(1f32 + grw / 100f32, yr_start as f32) as f32;
                        let enf = en_ofp * Pow::pow(1f32 + grw / 100f32, yr_start as f32) as f32;

                        let mut be_en_added = Vec::<(u32, f32)>::new();
                        for n in 3..=yr_start {
                            be_en_added.push(((n + 2025).try_into().unwrap(), 0f32));
                        }

                        let (mut _aen0, mut aenn, mut aenf) = (0f32, 0f32, 0f32);
                        //let (mut aenn0, mut aenf0) = (0f32, 0f32);
                        //let mut aen0 = 0f32;

                        for n in yr_start + 1..=BC_PROJ_YLEN {
                            _aen0 += p_en * Pow::pow(1f32 + grw / 100f32, n as f32) - en0;
                            aenn += en_onp * Pow::pow(1f32 + grw / 100f32, n as f32) - enn;
                            aenf += en_ofp * Pow::pow(1f32 + grw / 100f32, n as f32) - enf;

                            let aennx = en_onp * Pow::pow(1f32 + grw / 100f32, n as f32) - enn;
                            let aenfx = en_ofp * Pow::pow(1f32 + grw / 100f32, n as f32) - enf;

                            let aenny = aennx * uc_onp * 1000f32 * BC_NO_DAY_IN_YEAR as f32;
                            let aenfy = aenfx * uc_ofp * 1000f32 * BC_NO_DAY_IN_YEAR as f32;

                            let aen = (aenny + aenfy) * 0.94f32;
                            be_en_added.push(((n + 2025).try_into().unwrap(), aen));

                            //aen0 += aen;
                        }
                        let en0 = en0 * BC_NO_DAY_IN_YEAR as f32;
                        let enn = enn * BC_NO_DAY_IN_YEAR as f32;
                        let enf = enf * BC_NO_DAY_IN_YEAR as f32;
                        let ex_ben_onp = aenn * uc_onp * 1000f32 * BC_NO_DAY_IN_YEAR as f32;
                        let ex_ben_ofp = aenf * uc_ofp * 1000f32 * BC_NO_DAY_IN_YEAR as f32;
                        let ex_ben = (ex_ben_onp + ex_ben_ofp) * 0.94f32;
                        //println!("{} {}=={}", sf.sbid, aen0, ex_ben);

                        let mut be_re_diff = Vec::<(u32, f32)>::new();
                        let mut yr_diff = ls_ex_en * (BC_ON_PEAK_COST - BC_OFFPEAK_COST) * 1000f32;
                        for yi in 0..BC_BESS_YLEN {
                            yr_diff *= 1.04;
                            be_re_diff.push((yi as u32 + 2028, yr_diff));
                        }

                        let dec_ben = ls_ex_en
                            * (BC_ON_PEAK_COST - BC_OFFPEAK_COST)
                            * 1000f32
                            * BC_NO_DAY_IN_YEAR as f32;
                        /*
                        println!(
                            " sb:{sub} yr:{yr_start} en:{p_en:.1} en_onp:{en_onp:.1} en_ofp:{en_ofp:.1}"
                        );
                        println!("  en0:{en0:.1} enn:{enn:.1} enf:{enf:.1} aen0:{aen0:.1} aann:{aenn:.1} anf:{aenf:.1}");
                        println!(
                            "  be_on:{ex_ben_onp} be_of:{ex_ben_ofp} exbe:{ex_ben} de:{dec_ben}"
                        );
                        */
                        //let ex_ben = (ex_ben_onp + ex_ben_ofp) * (1f32 - CB_ADMIN_COST / 100f32);
                        //println!(" sb:{} mw:{ls_ex_pw} mwh:{ls_ex_en}", sb.sbid);

                        //let qbes = trcr * 0.4663; // tan 25
                        let qbes = (pkt - trlm) * 0.4663; // tan 25
                        let qcst = qbes * 4f32; // 4 million bht
                                                //println!("{qbes}- {qcst}");
                        let _r = BC_DISCN_RATE / 100f32;
                        //let fa = qcsy / Pow::pow(1f32 + r, 10f32);
                        //
                        //print!("B3 {}", sf.sbid);
                        let mut be_svg_save = Vec::<(u32, f32)>::new();
                        for y in 3..yr_start {
                            //print!(" {}", y + 2025);
                            be_svg_save.push((y as u32 + 2025, 0f32));
                        }
                        let mut ben3 = 0f32;
                        for y in yr_start..=BC_PROJ_YLEN {
                            let be = qcst / 10f32 * Pow::pow(1.03f32, y as f32);
                            //print!(" {}-{}", y + 2025, be);
                            be_svg_save.push((y as u32 + 2025, be * 1_000_000f32));
                            //println!("  {} {:.1}", y, be);
                            ben3 += be;
                        }
                        //println!();
                        //println!("  BEN3: {ben2}");

                        let mut ac_ex_sm = 0f32;
                        let mut ac_ex_be = 0f32;
                        for y in 0..=BC_PROJ_YLEN {
                            for i in 0..48 {
                                //let dv = yrben[y].day_prof[i] - trcr;
                                let dv = yrben[y].day_prof[i] - trlm;
                                if dv >= 0f32 {
                                    let up = if i >= BC_ON_PEAK_BEGIN && i < BC_ON_PEAK_END {
                                        //let up = if i >= 18 && i < 44 {
                                        let df = BC_ON_PEAK_COST - BC_OFFPEAK_COST;
                                        df + BC_PEA_PROFIT
                                        //BC_PEA_PROFIT
                                    } else {
                                        BC_PEA_PROFIT
                                    };
                                    ac_ex_sm += dv;
                                    ac_ex_be += dv * up * 0.5f32;
                                }
                            }
                        }
                        ac_ex_sm *= BC_NO_DAY_IN_YEAR as f32;
                        ac_ex_be *= BC_NO_DAY_IN_YEAR as f32;
                        sbbe = SubBenInfo::default();
                        sbbe.prv = pp.to_string();
                        sbbe.sub = sbid.to_string();
                        sbbe.name = sf.name.to_string();
                        sbbe.trlm = trlm;
                        sbbe.trcr = trcr;
                        sbbe.mx_pw = pwmx;
                        sbbe.grw0 = grw0;
                        sbbe.grw = grw;
                        sbbe.pw_inc_yr = dppy;
                        sbbe.pw2mx_rt = mxrt;
                        sbbe.yr_start = yr_start;
                        sbbe.be_start = if yr_start < 4 { 1 } else { yr_start - 3 };
                        sbbe.yr_len = BC_BESS_YLEN - sbbe.be_start;
                        sbbe.peek = peek;
                        sbbe.loc = loc.clone();
                        sbbe.p_en = p_en;
                        sbbe.n_en = n_en;
                        sbbe.be_sub = be0;

                        sbbe.yrben = yrben;
                        sbbe.ls_ex_en = ls_ex_en;
                        sbbe.ls_ex_pw = ls_ex_pw;

                        sbbe.ac_ex_en = ac_ex_sm * 0.5f32;
                        sbbe.ac_ex_be = ac_ex_be;

                        sbbe.yi_en = en0;
                        sbbe.yi_en_onp = enn;
                        sbbe.yi_en_ofp = enf;
                        sbbe.ex_ben_onp = ex_ben_onp;
                        sbbe.ex_ben_ofp = ex_ben_ofp;
                        sbbe.ex_ben = ex_ben;
                        sbbe.dec_ben = dec_ben;
                        sbbe.q_bess = qbes;
                        sbbe.q_cost = qcst;
                        sbbe.q_ben = ben3;
                        let bat_mwh = (ls_ex_en / 0.85 + 0.9f32) as i32;
                        sbbe.bat_mwh = bat_mwh as f32;
                        sbbe.bat_cost = bat_mwh as f32 * 21f32;

                        //print!("{}", sf.sbid);
                        let mut be_sub_save = Vec::<(u32, f32)>::new();
                        for yi in 0..BC_BESS_YLEN {
                            let mut be = 0f32;
                            if yi >= sbbe.be_start {
                                be = subcst[yi].sub_save;
                                //print!(" {}-{be}", yi + 2028);
                            }
                            be_sub_save.push((yi as u32 + 2028, be * 1_000_000f32));
                        }
                        //println!();
                        sbbe.be_sub_save = be_sub_save;
                        sbbe.be_re_diff = be_re_diff;
                        sbbe.be_svg_save = be_svg_save;
                        sbbe.be_en_added = be_en_added;

                        println!(
                            " ================ sb:{} be:{} st:{}",
                            sbbe.sub, sbbe.bat_mwh, sbbe.yr_start
                        );
                        break;
                    }

                    let fnm = &format!("{}/{}.bin", LP_BESS_DIR, sbbe.sub);
                    if let Ok(ser) = bincode::serialize(&sbbe) {
                        std::fs::write(fnm, ser).unwrap();
                        println!("save {fnm}");
                    }
                } // end load prof
            } // end loop substation
        } // end get list of substation
    }
}

use std::io::BufReader;
pub fn ld_ben_bess1(sb: &str) -> SubBenInfo {
    let fnm = format!("{}/{}.bin", LP_BESS_DIR, sb);
    if let Ok(f) = std::fs::File::open(&fnm) {
        if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, SubBenInfo>(BufReader::new(f))
        {
            return dt;
        }
    }
    SubBenInfo::default()
}

pub fn p4_ben3() {
    let _ = std::fs::create_dir_all(LP_BESS_DIR);
    let mut known = HashMap::<&str, &str>::new();
    for (s, t, _n) in SUB_BESS {
        known.insert(s, t);
    }

    let (sbm, sbm0, sbm1, sbm2) = get_lps();
    let pv = grp1();
    let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();

    // BESS infra defer benefit calculation
    let r = BC_DISCN_RATE / 100f32;
    let n = BC_SUBST_YLEN as f32;
    let anrt = (1f32 - Pow::pow(1f32 + r, -n)) / r;
    let ancs = BC_SUBST_COST / anrt;
    let cst: Vec<f64> = vec![ancs.into(); 25];
    let mut subcst = Vec::<SubYearBenInfo>::new();
    for (i, v) in cst.iter().enumerate() {
        let fa = v / Pow::pow(1f64 + r as f64, i as f64);
        let be = if i < 12 {
            v * Pow::pow(1.03f64, i as f64)
        } else {
            0f64
        };
        subcst.push(SubYearBenInfo {
            year: i,
            sub_cost: *v as f32,
            sub_npv: fa as f32,
            sub_save: be as f32,
            ..Default::default()
        });
    }

    //println!("sub:{sb0:.1} bene:{be0:.1}");
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = sbsl.get(&pp) {
            for sb in sbv {
                if let (Some(sf), Some(slp), Some(slp0), Some(slp1), Some(slp2)) = (
                    sbif.get(&sb.sbid),
                    sbm.get(&sb.sbid),
                    sbm0.get(&sb.sbid),
                    sbm1.get(&sb.sbid),
                    sbm2.get(&sb.sbid),
                ) {
                    let (xx, yy) = if let Some(latlong) = slp.latlong {
                        let (x, y) = latlong;
                        let (xx, yy) = utm_latlong(x as f32, y as f32);
                        (xx, yy)
                    } else {
                        (0f32, 0f32)
                    };
                    let loc = format!("https://maps.google.com/?q={:.4},{:.4}", xx, yy);

                    let mut sbbe = SubBenInfo::default();
                    sbbe.sub = sb.sbid.to_string();
                    sbbe.name = sf.name.to_string();
                    for y in 2024..=2039 {
                        let mut sybe = SubYearBenInfo::default();
                        sybe.year = y;
                        sbbe.yrben.push(sybe);
                    }
                    let mut grw = 1.0;
                    let rf24 = slp.pk30_time_prof.calc_v.p_pk;
                    let rf23 = slp0.pk30_time_prof.calc_v.p_pk;
                    let rf22 = slp1.pk30_time_prof.calc_v.p_pk;
                    let rf21 = slp2.pk30_time_prof.calc_v.p_pk;
                    let mut daylp = slp.pk30_time_prof.time_v.clone();
                    let mut peek = slp.pk30_time_prof.calc_v.p_pk;
                    let mut p_en = slp.pk30_time_prof.calc_v.p_en;
                    let mut n_en = slp.pk30_time_prof.calc_v.n_en;

                    if rf23 > 0f32 {
                        let a24 = (rf24 - rf23) / rf23 * 100f32;
                        if a24 > grw {
                            grw = a24;
                            daylp = slp0.pk30_time_prof.time_v.clone();
                            peek = slp0.pk30_time_prof.calc_v.p_pk;
                            p_en = slp0.pk30_time_prof.calc_v.p_en;
                            n_en = slp0.pk30_time_prof.calc_v.n_en;
                        }
                    }
                    if rf22 > 0f32 {
                        let a23 = (rf23 - rf22) / rf22 * 100f32;
                        if a23 > grw {
                            grw = a23;
                            daylp = slp1.pk30_time_prof.time_v.clone();
                            peek = slp1.pk30_time_prof.calc_v.p_pk;
                            p_en = slp1.pk30_time_prof.calc_v.p_en;
                            n_en = slp1.pk30_time_prof.calc_v.n_en;
                        }
                    }
                    if rf21 > 0f32 {
                        let a22 = (rf22 - rf21) / rf21 * 100f32;
                        if a22 > grw {
                            grw = a22;
                            daylp = slp2.pk30_time_prof.time_v.clone();
                            peek = slp2.pk30_time_prof.calc_v.p_pk;
                            p_en = slp2.pk30_time_prof.calc_v.p_en;
                            n_en = slp2.pk30_time_prof.calc_v.n_en;
                        }
                    }
                    if grw > BC_POW_GRW_LIM {
                        grw = BC_POW_GRW_LIM;
                    }
                    let trlm = slp.maxmva * BC_POWER_FACT * BC_TR_LOAD_LIM;
                    let trcr = slp.maxmva * BC_POWER_FACT * BC_TR_CRIT_LIM;
                    let pwmx = rf24.max(rf23.max(rf22.max(rf21)));
                    let dppy = trlm * grw / 100f32; // MW/yr increase
                    let yrno = (trlm - pwmx) / dppy;
                    let mxrt = pwmx / trlm * 100f32;
                    let yr_start = yrno as usize;
                    /*
                    let mut ls_daylp = daylp.clone();
                    for i in 0..48 {
                        ls_daylp[i] *= Pow::pow(1f32 + grw / 100f32, 15);
                    }
                    */
                    sbbe.trlm = trlm;
                    sbbe.trcr = trcr;
                    sbbe.mx_pw = pwmx;
                    sbbe.grw = grw;
                    sbbe.pw_inc_yr = dppy;
                    sbbe.pw2mx_rt = mxrt;
                    sbbe.yr_start = yr_start;
                    sbbe.be_start = if yr_start < 4 { 1 } else { yr_start - 3 };
                    sbbe.yr_len = BC_BESS_YLEN - sbbe.be_start;
                    sbbe.peek = peek;
                    sbbe.loc = loc;
                    sbbe.p_en = p_en;
                    sbbe.n_en = n_en;
                    let mut be0 = 0f32;
                    //if sb.sbid == "BNP" {
                    //println!("BNP");
                    for yi in sbbe.be_start - 1..BC_BESS_YLEN {
                        be0 += subcst[yi].sub_save;
                    }

                    print!("{}", sf.sbid);
                    let mut be_sub_save = Vec::<(u32, f32)>::new();
                    for yi in 0..BC_BESS_YLEN {
                        let mut be = 0f32;
                        if yi >= sbbe.be_start {
                            be = subcst[yi].sub_save;
                            print!(" {}-{be}", yi + 2028);
                        }
                        be_sub_save.push((yi as u32 + 2028, be));
                    }
                    println!();
                    sbbe.be_sub_save = be_sub_save;

                    //}
                    sbbe.be_sub = be0;

                    //let mut be_re_diff = Vec::<(u32, f32)>::new();
                    for i in 0..=BC_PROJ_YLEN {
                        let mut day_prof = daylp.clone();
                        //println!("day_prof len:{}", day_prof.len());
                        for j in 0..48 {
                            day_prof[j] *= Pow::pow(1f32 + grw / 100f32, i as f32) as f32;
                        }
                        let yrben = SubYearBenInfo {
                            year: i + 2024,
                            day_prof,
                            ..Default::default()
                        };

                        sbbe.yrben[i] = yrben;
                    }
                    let mut ls_ex_sm = 0f32;
                    let mut ls_ex_pw = 0f32;
                    for i in 0..48 {
                        let dv = sbbe.yrben[BC_PROJ_YLEN].day_prof[i] - sbbe.trcr;
                        if dv >= 0f32 {
                            ls_ex_pw = if dv > ls_ex_pw { dv } else { ls_ex_pw };
                            ls_ex_sm += dv;
                        }
                    }
                    let ls_ex_en = ls_ex_sm * 0.5f32;
                    sbbe.ls_ex_en = ls_ex_en;
                    sbbe.ls_ex_pw = ls_ex_pw;

                    let mut ac_ex_sm = 0f32;
                    let mut ac_ex_be = 0f32;
                    for y in 0..=BC_PROJ_YLEN {
                        for i in 0..48 {
                            let dv = sbbe.yrben[y].day_prof[i] - sbbe.trcr;
                            if dv >= 0f32 {
                                let up = if i >= 18 && i < 44 {
                                    3.6199f32
                                } else {
                                    2.3341f32
                                };
                                ac_ex_sm += dv;
                                ac_ex_be += dv * up * 0.5f32;
                            }
                        }
                    }
                    sbbe.ac_ex_en = ac_ex_sm * 0.5f32;
                    sbbe.ac_ex_be = ac_ex_be;
                    let fnm = &format!("{}/{}.bin", LP_BESS_DIR, sbbe.sub);
                    if let Ok(ser) = bincode::serialize(&sbbe) {
                        std::fs::write(fnm, ser).unwrap();
                        //println!("save {fnm}");
                    }
                } // end load prof
            } // end loop substation
        } // end get list of substation
    }
}

pub fn p4_ben2() {
    let pv = prvs();
    let pvsb = ld_p3_prv_sub();
    let sbsl = ld_pv_sbv_m();
    println!("selc:{}", sbsl.len());
    let sbif = ld_p3_sub_inf();
    let sblo = ld_sub_loc();
    let mut _cn1 = 0;
    let mut cn2 = 0;
    let mut _cn3 = 0;
    for p in pv {
        let pp = p.to_string();
        if let Some(sbs) = pvsb.get(&pp) {
            let mut sbsv = &Vec::<Proc41Item>::new();
            if let Some(sbv) = sbsl.get(&pp) {
                sbsv = sbv;
            }
            for sb in sbs {
                if let (Some(sbf), Some((x, y))) = (sbif.get(sb), sblo.get(sb)) {
                    _cn1 += 1;
                    let mut cc = 0;
                    for it in sbsv {
                        if it.sbid == *sb {
                            cc += 1;
                            break;
                        }
                    }
                    if cc > 0 {
                        _cn3 += 1;
                        continue;
                    } else {
                        cn2 += 1;
                    }
                    let (xx, yy) = utm_latlong(*x as f32, *y as f32);
                    let loc = format!("https://maps.google.com/?q={:.4},{:.4}", xx, yy);
                    let nm = sbf.name.to_string();
                    println!("{cn2}   {sb}   {loc}   {nm}");
                }
            }
        }
    }
}

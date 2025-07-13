use serde::{Deserialize, Serialize};
//use sglab02_lib::web::wk5t10;
//use sglab02_lib::sg::prc5::sub_inf;
use sglib03::prc1::LoadProfVal;
use sglib03::prc2::get_all_lp;
//use sglib03::prc2::SubLoadProf;
use phf::phf_map;
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DayLoadProf {
    pub day: usize,
    pub time_r: Vec<f32>,
    pub sum: f32,
}

pub fn calc_lp_sb() -> Result<(), Box<dyn Error>> {
    let lpyr = ["2021", "2022", "2023", "2024"];
    let mut lps = vec![];
    for yr in lpyr {
        let lp = get_all_lp(yr).unwrap();
        println!("{yr} = {}", lp.len());
        lps.push(lp);
    }
    //let mut sbms = vec![];
    let mut y_sb_lp = HashMap::<String, HashMap<String, Vec<f32>>>::new();
    for lp in lps {
        let yr = lp[0].year.to_string();
        println!("====={}", lp[0].year);
        let mut sb_lp = HashMap::<String, Vec<f32>>::new();
        for slp in lp {
            let mut dlps = vec![];
            for d in 0..365 {
                let mut dlp = vec![];
                let mut sum = 0f32;
                for h in 0..48 {
                    let dh = d * 48 + h;
                    if let LoadProfVal::Value(v) = slp.time_r[dh] {
                        sum += if v > 0.0 { v } else { 0.0 };
                        dlp.push(v);
                    }
                }
                if dlp.len() == 48 {
                    let dslp = DayLoadProf {
                        day: d,
                        time_r: dlp,
                        sum,
                    };
                    dlps.push(dslp);
                }
            }
            dlps.sort_by(|a, b| b.sum.partial_cmp(&a.sum).unwrap());
            if dlps.len() > 2 {
                //print!(" {}={}", slp.sub, dlps.len());
                let cn = if dlps.len() > 30 { 30 } else { dlps.len() };
                let mut sm = [0f32; 48];
                for dlp in dlps.iter().take(cn) {
                    for (h, s) in sm.iter_mut().enumerate() {
                        *s += dlp.time_r[h];
                    }
                }
                for s in &mut sm {
                    *s /= cn as f32;
                }
                sb_lp.insert(slp.sub.to_string(), sm.to_vec());
            };
        } // loop end for sub
        y_sb_lp.insert(yr, sb_lp);
    }
    let fou = "/mnt/e/CHMBACK/pea-data/data1/y_sb_lp0.bin".to_string();
    if let Ok(ser) = bincode::serialize(&y_sb_lp) {
        println!("write {fou}");
        std::fs::write(fou, ser).unwrap();
    }
    Ok(())
}

use regex::Regex;
use sglab02_lib::sg::prc5::sub_inf;
use sglib03::prc1::FeederLoadRaw;
use sglib03::prc2::LP_RAW_DIR;
use std::fs::File;
use std::io::BufReader;

pub fn calc_lp_fd() -> Result<(), Box<dyn Error>> {
    let lpyr = ["2021", "2022", "2023", "2024"];
    let sbif = sub_inf();
    //let mut lps = vec![];
    let mut y_fd_lp = HashMap::<String, HashMap<String, Vec<f32>>>::new();
    let mut y_sb_lp = HashMap::<String, HashMap<String, Vec<f32>>>::new();
    //let re = Regex::new(r"(...)_([0-9][0-9]).+").unwrap();
    let re = Regex::new(r"(...)_([0-9][0-9])[VW].+").unwrap();
    for yr in lpyr {
        let mut sb_lp = HashMap::<String, Vec<f32>>::new();
        let mut fd_lp = HashMap::<String, Vec<f32>>::new();
        for (s, _sf) in sbif {
            let fsb = format!("{LP_RAW_DIR}/lp{yr}/{s}.bin");
            if let Ok(f) = File::open(&fsb) {
                if let Ok(flps) = bincode::deserialize_from::<
                    BufReader<File>,
                    Vec<Box<FeederLoadRaw>>,
                >(BufReader::new(f))
                {
                    /*
                    println!(
                        "sb:{}-{}={} ar:{}",
                        s,
                        flps.len(),
                        sf.feeders.len(),
                        sf.arid
                    );
                    */
                    //let mut fd_lp = HashMap::<String, [f32; 48]>::new();
                    //let mut sb_ylp = vec![LoadProfVal::Value(0f32); 365 * 48];
                    let mut sb_ylp = vec![LoadProfVal::None; 365 * 48];
                    for flp in flps {
                        if let Some(cap) = re.captures_iter(flp.feed.as_str()).next() {
                            let sid = &cap[1];
                            let fid = &cap[2];
                            let fid = format!("{sid}{fid}");

                            //println!("  {} => {fid}", flp.feed);
                            let mut dlps = vec![];
                            for d in 0..365 {
                                let mut dlp = vec![];
                                let mut sum = 0f32;
                                for h in 0..48 {
                                    let dh = d * 48 + h;
                                    if let LoadProfVal::Value(v) = flp.time_r[dh] {
                                        sum += if v > 0.0 { v } else { 0.0 };
                                        dlp.push(v);
                                        match sb_ylp[dh] {
                                            LoadProfVal::None => {
                                                sb_ylp[dh] = LoadProfVal::Value(v);
                                            }
                                            LoadProfVal::Value(sv) => {
                                                sb_ylp[dh] = LoadProfVal::Value(v + sv);
                                            }
                                            _ => {}
                                        }
                                        if let LoadProfVal::Value(sv) = &mut sb_ylp[dh] {
                                            *sv += v;
                                        } else {
                                            sb_ylp[dh] = LoadProfVal::None;
                                        }
                                    }
                                }
                                if dlp.len() == 48 {
                                    let dslp = DayLoadProf {
                                        day: d,
                                        time_r: dlp,
                                        sum,
                                    };
                                    dlps.push(dslp);
                                }
                            }
                            dlps.sort_by(|a, b| b.sum.partial_cmp(&a.sum).unwrap());

                            if dlps.len() > 2 {
                                //print!(" {}={}", slp.sub, dlps.len());
                                let cn = if dlps.len() >= 30 { 30 } else { dlps.len() };
                                let mut sm = [0f32; 48];
                                for dlp in dlps.iter().take(cn) {
                                    for (h, s) in sm.iter_mut().enumerate() {
                                        *s += dlp.time_r[h];
                                    }
                                }
                                for s in &mut sm {
                                    *s /= cn as f32;
                                }
                                //fd_lp.insert(flp.feed.to_string(), sm.to_vec());
                                fd_lp.insert(fid, sm.to_vec());
                            }; // end if valid data
                        } // end if valid feeder 1-10
                    } // end loop for feeder

                    let mut dlps = vec![];
                    for d in 0..365 {
                        let mut dlp = vec![];
                        let mut sum = 0f32;
                        for h in 0..48 {
                            let dh = d * 48 + h;
                            if let LoadProfVal::Value(v) = sb_ylp[dh] {
                                sum += if v > 0.0 { v } else { 0.0 };
                                dlp.push(v);
                            }
                        }
                        if dlp.len() == 48 {
                            let dslp = DayLoadProf {
                                day: d,
                                time_r: dlp,
                                sum,
                            };
                            dlps.push(dslp);
                        }
                    }
                    dlps.sort_by(|a, b| b.sum.partial_cmp(&a.sum).unwrap());
                    if dlps.len() > 2 {
                        //print!(" {}={}", slp.sub, dlps.len());
                        let mut cn = 0;
                        let mut sm = [0f32; 48];
                        for dlp in dlps.iter().take(30) {
                            cn += 1;
                            for (h, s) in sm.iter_mut().enumerate() {
                                *s += dlp.time_r[h];
                            }
                        }
                        for s in &mut sm {
                            *s /= cn as f32;
                        }
                        sb_lp.insert(s.to_string(), sm.to_vec());
                    } else {
                        //println!("sub bad {s}");
                    } // end if valid data
                } // end deserialized
            } // end file open
              //println!("{fsb}");
        } // end loop for substation
        println!("yr:{yr} sb:{}", sb_lp.len());
        y_sb_lp.insert(yr.to_string(), sb_lp);
        println!("yr:{yr} fd:{}", fd_lp.len());
        y_fd_lp.insert(yr.to_string(), fd_lp);
    } // end loop for year
    let fou = "/mnt/e/CHMBACK/pea-data/data1/y_sb_lp.bin".to_string();
    if let Ok(ser) = bincode::serialize(&y_sb_lp) {
        println!("write {fou}");
        std::fs::write(fou, ser).unwrap();
    }
    let fou = "/mnt/e/CHMBACK/pea-data/data1/y_fd_lp.bin".to_string();
    if let Ok(ser) = bincode::serialize(&y_fd_lp) {
        println!("write {fou}");
        std::fs::write(fou, ser).unwrap();
    }
    Ok(())
}

//use sglib03::drw::sb_dr5::SubDraw5;
//use sglib03::prc2::SubGraphDraw5;
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

use crate::aoj::sub_latlong_adjust;
use sglab02_lib::sg::mvline::latlong_utm;
use sglib03::subtype::SUB_TYPES;
use std::collections::HashSet;

pub fn sub_all_coll() -> Result<(), Box<dyn Error>> {
    let sbif = sub_inf();
    //pub fn sub_inf() -> &'static HashMap<String, SubstInfo> {    SUB_INF.get_or_init(sub_inf_init) }

    let mut sbs_2 = HashMap::<String, (String, f32, f32)>::new();
    let re = Regex::new(r"q=([0-9]+\.[0-9]+),([0-9]+\.[0-9]+)").unwrap();
    let adjxy = sub_latlong_adjust();
    for (sb, cf, gm) in &SUB_TYPES {
        if let Some(cap) = re.captures_iter(gm).next() {
            let x = &cap[1];
            let y = &cap[2];
            let mut xx = x.parse::<f32>().unwrap();
            let mut yy = y.parse::<f32>().unwrap();
            let sbid = sb.to_string();
            if let Some((x1, y1)) = adjxy.get(&sbid) {
                xx += x1;
                yy += y1;
            }
            let (sb_x, sb_y) = latlong_utm(xx, yy);
            //println!("=== {sb} x:{xx} y:{yy} utm:{sb_x},{sb_y}");
            //if let Some(_) = sbs_2.get(&sb.to_string()) {
            let sbid = sb.to_string();
            //if let Some(_) = sbs_2.get(&sbid) {
            //if sbs_2.get(&sbid).is_some() {
            if sbs_2.contains_key(&sbid) {
                println!(" ERROR 2 {sb}");
            } else {
                sbs_2.insert(sb.to_string(), (cf.to_string(), sb_x, sb_y));
            }
        } else {
            println!(" ERROR {sb}");
            continue;
        }
    }
    let mut sbs_1 = HashSet::<String>::new();

    let fsb = "/mnt/e/CHMBACK/pea-data/data1/y_sb_lp.bin".to_string();
    if let Ok(fsb) = File::open(&fsb) {
        let fsb = BufReader::new(fsb);
        if let Ok(ysblp) = bincode::deserialize_from::<
            BufReader<File>,
            HashMap<String, HashMap<String, Vec<f32>>>,
        >(fsb)
        {
            let yrs = ["2021", "2022", "2023", "2024"];
            for yr in yrs {
                if let Some(sblp) = ysblp.get(yr) {
                    //for (sb, _lp) in sblp {
                    for sb in sblp.keys() {
                        sbs_1.insert(sb.to_string());
                    }
                }
            }
        }
    }
    println!("sbf {}", sbif.len());
    println!("sbs_1: {}", sbs_1.len());
    println!("sbs_2: {}", sbs_2.len());
    let mut d0_b_1 = 0;
    let mut d0_b_2 = 0;
    for (s, _) in sbif {
        if !sbs_1.contains(s) {
            d0_b_1 += 1;
        }
        if !sbs_2.contains_key(s) {
            d0_b_2 += 1;
        }
    }
    let mut d1_b_0 = 0;
    let mut d1_b_2 = 0;
    for s in &sbs_1 {
        if !sbif.contains_key(s) {
            d1_b_0 += 1;
        }
        if !sbs_2.contains_key(s) {
            d1_b_2 += 1;
        }
    }
    println!(" d0_b_1: {d0_b_1}, d0_b_2: {d0_b_2}");
    println!(" d1_b_0: {d1_b_0}, d1_b_2: {d1_b_2}");
    Ok(())
}

pub fn draw_lp() -> Result<(), Box<dyn Error>> {
    //let yr = "2023";
    //let sb = "BUY";
    let dir = "/mnt/e/CHMBACK/pea-data/draw/sblp".to_string();
    let fsb = "/mnt/e/CHMBACK/pea-data/data1/y_sb_lp.bin".to_string();
    let ffd = "/mnt/e/CHMBACK/pea-data/data1/y_fd_lp.bin".to_string();
    if let (Ok(fsb), Ok(ffd)) = (File::open(&fsb), File::open(&ffd)) {
        let fsb = BufReader::new(fsb);
        let ffd = BufReader::new(ffd);
        if let (Ok(ysblp), Ok(yfdlp)) = (
            bincode::deserialize_from::<BufReader<File>, HashMap<String, HashMap<String, Vec<f32>>>>(
                fsb,
            ),
            bincode::deserialize_from::<BufReader<File>, HashMap<String, HashMap<String, Vec<f32>>>>(
                ffd,
            ),
        ) {
            //let yr = "2022";
            let yrs = ["2021", "2022", "2023", "2024"];
            for yr in yrs {
                if let Some(sblp) = ysblp.get(yr) {
                    for (sb, lp) in sblp {
                        if sb != "BUY" {
                            //continue;
                        }
                        println!("{sb}");
                        let ydir = format!("{dir}/{yr}");
                        std::fs::create_dir_all(&ydir)?;
                        let fnm = format!("{dir}/{yr}/{sb}.png");
                        let mut sld = SubGraphDraw5 {
                            sub: sb.to_string(),
                            fnm,
                            lp: lp.clone(),
                            yr: yr.to_string(),
                            ..Default::default() //sz: (400, 300),
                        };
                        sld.sz = (400, 300);
                        sld.draw_prof()?;
                        for i in 1..=20 {
                            let fid = format!("{sb}{i:02}");
                            if let Some(fdlp) = yfdlp.get(yr) {
                                if let Some(lp) = fdlp.get(&fid) {
                                    println!("=={yr} {fid}");
                                    let ydir = format!("{dir}/{yr}");
                                    std::fs::create_dir_all(&ydir)?;
                                    let fnm = format!("{dir}/{yr}/{fid}.png");
                                    println!("{fnm}");
                                    let mut sld = SubGraphDraw5 {
                                        sub: fid.to_string(),
                                        fnm,
                                        lp: lp.clone(),
                                        yr: yr.to_string(),
                                        ..Default::default() //sz: (400, 300),
                                    };
                                    sld.sz = (400, 300);
                                    sld.draw_prof()?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

use ab_glyph::FontVec;
use ab_glyph::PxScale;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;
use std::fs;
use std::io::Read;

pub trait SubDraw5 {
    fn sub(&self) -> String;
    fn fnm(&self) -> String;
    fn lp(&self) -> Vec<f32>;
    fn yr(&self) -> String;
    const SIZE: (usize, usize) = (400, 300);
    fn sz(&self) -> (usize, usize) {
        Self::SIZE
    }
    fn mg(&self) -> (usize, usize, usize, usize) {
        (48, 40, 45, 25) // top,bottom,left,right
    }
    fn tik(&self) -> Vec<f32> {
        let lp = self.lp();
        let lp = &lp;
        let ymx = lp.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let ymx = *ymx;
        let ymn = lp.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let ymn = ymn.abs();
        let dy = (ymx + 4f32) / 5f32;
        let dy = if ymn > ymx { (ymn + 4f32) / 5f32 } else { dy };

        let mut tk = 0f32;
        let mut tiks = vec![];
        for _i in 0..10 {
            tk += dy;
            tiks.push(-tk as f32);
            if tk as f32 > ymn {
                break;
            }
        }
        tiks.reverse();
        //println!("{}.tik1 - {tiks:?}", self.sub());
        let mut tk = 0f32;
        tiks.push(tk as f32);
        for _i in 0..10 {
            tk += dy;
            tiks.push(tk as f32);
            if tk as f32 > ymx {
                break;
            }
        }
        //println!("{}.tik2 - {tiks:?}", self.sub());
        tiks
    }
    fn si(&self) -> (usize, usize) {
        let (wd, hg) = self.sz();
        let (mt, mb, ml, mr) = self.mg();
        (wd - ml - mr, hg - mt - mb)
    }
    fn image(&self) -> RgbImage {
        let (wd, hg) = self.sz();
        RgbImage::new(wd as u32, hg as u32)
    }
    fn wht(&self) -> Rgb<u8> {
        Rgb([255u8, 255u8, 255u8])
    }
    fn blk(&self) -> Rgb<u8> {
        Rgb([0u8, 0u8, 0u8])
    }
    fn grn(&self) -> Rgb<u8> {
        Rgb([0u8, 130u8, 0u8])
    }
    fn blu(&self) -> Rgb<u8> {
        Rgb([0u8, 0u8, 100u8])
    }
    fn yel(&self) -> Rgb<u8> {
        Rgb([200u8, 200u8, 0u8])
    }
    fn red(&self) -> Rgb<u8> {
        Rgb([130u8, 0u8, 0u8])
    }
    fn gr1(&self) -> Rgb<u8> {
        Rgb([230u8, 230u8, 230u8])
    }
    fn gr2(&self) -> Rgb<u8> {
        Rgb([242u8, 242u8, 242u8])
    }
    fn gr3(&self) -> Rgb<u8> {
        Rgb([150u8, 150u8, 150u8])
    }
    fn rf(&self) -> Vec<(String, f32)> {
        Vec::<(String, f32)>::new()
    }
    //--- drawing
    fn draw_prof(&self) -> Result<Vec<u8>, String> {
        let fnm = self.fnm();
        let lpv = self.lp();
        let mut image = self.image();
        let (wd, hg) = self.sz();
        let (mt, mb, ml, mr) = self.mg();
        let (wdi, hgi) = (wd - ml - mr, hg - mt - mb);
        let tik = self.tik();
        let vwd = tik[tik.len() - 1] - tik[0];
        let hgrt = hgi as f32 / vwd;
        let zrlv = (0f32 - tik[0]) / vwd;
        let ory = (mt + hgi) as i32 - (hgi as f32 * zrlv) as i32;
        let scl = PxScale::from(24.0);
        let sc2 = PxScale::from(28.0);
        let font_vec = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
        let font = FontVec::try_from_vec(font_vec).expect("Font Vec");
        let (wht, blk, _grn) = (self.wht(), self.blk(), self.grn());
        let (gr1, gr2, gr3) = (self.gr1(), self.gr2(), self.gr3());
        let wdrt = wdi as f32 / 48f32;

        //println!("..1");
        // all day
        draw_filled_rect_mut(
            &mut image,
            Rect::at(0, 0).of_size(wd as u32, hg as u32),
            wht,
        );
        // morning
        draw_filled_rect_mut(
            &mut image,
            Rect::at(ml as i32, mt as i32).of_size(wdi as u32 / 4 as u32, hgi as u32),
            gr2,
        );
        // night
        draw_filled_rect_mut(
            &mut image,
            Rect::at((ml + wdi * 3 / 4) as i32, mt as i32)
                .of_size(wdi as u32 / 4 as u32, hgi as u32),
            gr2,
        );
        //println!("..2");

        // x tick & label
        for i in 0..=48 {
            if i % 4 != 0 {
                continue;
            }
            let xi = i as f32 * wdrt + ml as f32;
            //let y1 = (hg - mb) as f32;
            let y1 = ory as f32;
            let y2 = y1 + 5f32;
            draw_line_segment_mut(&mut image, (xi, y1), (xi, y2), blk);
            let (y1, y2) = (mt as f32, (mt + hgi) as f32);
            draw_line_segment_mut(&mut image, (xi, y1), (xi, y2), gr1);

            let hi = i / 2;
            let hi = format!("{:02}น", hi);
            let xi = xi as i32 - 8;
            let yi = y2 as i32;
            draw_text_mut(&mut image, blk, xi, yi, scl, &font, &hi);
        }

        for (_nm, v) in self.rf() {
            let yi = ory as f32 - v * hgrt;
            let (x1, x2) = (ml as f32, (wd - mr) as f32);
            //println!("nm:{} v:{} x1:{x1} x2:{x2} yi:{yi}", nm, v);
            draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), self.yel());
            //draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), grn);
        }

        // y ticks & label
        for v in &tik {
            let yi = ory as f32 - v * hgrt;
            let x1 = ml as f32;
            let x2 = x1 - 5f32;
            draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), blk);

            let (x1, x2) = (ml as f32, (wd - mr) as f32);
            draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), gr1);

            let x1 = ml as f32;
            let x2 = x1 - 5f32;
            let va = format!("{:.1}", v);
            let xi = x2 as i32 - 15 - va.len() as i32 * 4;
            let yi = yi as i32 - 10;
            draw_text_mut(&mut image, blk, xi, yi, scl, &font, &va);
        }

        let col = self.grn();
        for (di, v) in lpv.iter().enumerate() {
            let di = di % 48;
            let xi = di as f32 * wdrt + ml as f32;
            let yi = ory as f32 - v * hgrt;
            let x2 = xi + wdrt; //10f32;
            let _dy = v * hgrt;
            draw_line_segment_mut(&mut image, (xi, yi), (x2, yi), col);
            draw_line_segment_mut(&mut image, (xi + wdrt, yi), (xi + wdrt, ory as f32), col);
            draw_line_segment_mut(&mut image, (xi, yi), (xi, ory as f32), col);
        }

        let sb = &self.sub();
        let lb = format!("{} [MW]", sb);
        draw_text_mut(&mut image, blk, 20, 12, sc2, &font, &lb);
        draw_text_mut(&mut image, blk, 180, 12, sc2, &font, &self.yr());
        // border lines

        // up bar
        let (x1, y1, x2, y2) = (ml as f32, mt as f32, (wd - ml) as f32, mt as f32);
        draw_line_segment_mut(&mut image, (x1, y1), (x2, y2), gr1);
        // low bar
        let (x1, y1, x2, y2) = (
            ml as f32,
            (mt + hgi) as f32,
            (wd - ml) as f32,
            (mt + hgi) as f32,
        );
        draw_line_segment_mut(&mut image, (x1, y1), (x2, y2), gr1);

        // left pipe
        let (x1, y1, x2, y2) = (ml as f32, mt as f32, ml as f32, (mt + hgi) as f32);
        draw_line_segment_mut(&mut image, (x1 - 1f32, y1), (x2 - 1f32, y2), gr3);

        // right pipe
        let (x1, y1, x2, y2) = (
            (ml + wdi) as f32,
            mt as f32,
            (ml + wdi) as f32,
            (mt + hgi) as f32,
        );
        draw_line_segment_mut(&mut image, (x1 + 1f32, y1), (x2 + 1f32, y2), gr1);

        let x1 = ml as f32;
        let x2 = x1 + wdi as f32; //10f32;
        let yi = ory as f32;
        draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), blk);

        //println!("..3 {fnm}");
        // show data
        if image.save(&fnm).is_ok() {
            //println!("..4 {fnm}");
            if let Ok(mut f) = File::open(&fnm) {
                if let Ok(mt) = fs::metadata(&fnm) {
                    //println!("..5 {}", mt.len());
                    let mut buffer = vec![0; mt.len() as usize];
                    if let Ok(_) = f.read(&mut buffer) {
                        return Ok(buffer);
                    }
                }
            }
        }

        Ok(vec![])
    }
}

use crate::geo1::is_area;
use crate::geo1::NodeInfo;
use crate::geo1::NodeType;
use sglab02_lib::sg::gis1::ar_list;
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CnlTrans {
    pub trid: String,
    pub pea: String,
    pub n1d: u64,
    pub n1d_f: u64,
    pub ix: usize,
    pub lix: usize,
    pub mts: Vec<usize>,
}

pub fn p9_set_cnl_trans() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let nds = format!("/mnt/e/CHMBACK/pea-data/data1/p9_{ar}_nodes.bin");
        let fcnl = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_mt.bin");
        if let (Ok(nds), Ok(fcnl)) = (File::open(&nds), File::open(&fcnl)) {
            let nds = BufReader::new(nds);
            let fcnl = BufReader::new(fcnl);
            if let (Ok(nds), Ok(mut cnls)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlData>>(fcnl),
            ) {
                //let mut cnltrs = Vec::<(u64, usize)>::new();
                let mut ctrs = Vec::<CnlTrans>::new();
                let mut ctrm = HashMap::<String, usize>::new();
                let (mut t11, mut t01, mut t10, mut t22, mut t20, mut t02, mut tt1, mut tt2) =
                    (0, 0, 0, 0, 0, 0, 0, 0);
                for (n1d, nd) in &nds {
                    let mut tr1 = 0;
                    let mut tr2 = 0;
                    let mut tr_ps = Vec::<String>::new();
                    for n in &nd.nodes {
                        if let NodeType::Bridge = n.ntp {
                            match n.ly.as_str() {
                                "DS_Transformer" => {
                                    tr1 += 1;
                                    tt1 += 1;
                                }
                                "GIS_LVCNL" => {
                                    tr2 += 1;
                                    tt2 += 1;
                                    let cnl = &mut cnls[n.ix];
                                    if let (Some(n1d0), Some(pea)) = (&cnl.tr_n1d, &cnl.tr_pea) {
                                        //cnl.tr_n1d = Some(*nid0);
                                        let trid = format!("{}-{pea}", *n1d0);
                                        if let Some(_) = ctrm.get(&trid) {
                                            println!("  ERROR duplicate TRF {pea}");
                                        } else {
                                            let ctr = CnlTrans {
                                                trid: trid.to_string(),
                                                pea: pea.to_string(),
                                                //n1d: *n1d,
                                                n1d: *n1d0,
                                                n1d_f: *n1d,
                                                ix: n.ix,
                                                lix: ctrs.len(),
                                                mts: vec![],
                                            };
                                            ctrm.insert(trid.to_string(), ctr.lix);
                                            ctrs.push(ctr);
                                        }
                                        tr_ps.push(pea.to_string());
                                    }
                                }
                                _ => {}
                            }
                        }
                    } // end loop eq in node
                    if tr1 + tr2 > 0 {
                        if tr1 == 1 && tr2 == 1 {
                            t11 += 1;
                        } else if tr1 == 0 && tr2 == 1 {
                            t01 += 1;
                        } else if tr1 == 1 && tr2 == 0 {
                            t10 += 1;
                        } else if tr1 >= 2 && tr2 == 0 {
                            t20 += 1;
                        } else if tr1 == 0 && tr2 >= 2 {
                            t02 += 1;
                        } else {
                            t22 += 1;
                            /*
                            for p in tr_ps {
                                print!(" {p}");
                            }
                            println!();
                            */
                        }
                    }
                } // end loop node
                for cnl in &cnls {
                    if let (Some(n1d), Some(pea)) = (&cnl.tr_n1d, &cnl.tr_pea) {
                        let trid = format!("{n1d}-{pea}");
                        if let Some(lix) = ctrm.get_mut(&trid) {
                            ctrs[*lix].mts.push(cnl.ix);
                            //ctr.mts.push(cnl.ix);
                        } else {
                            println!("ERROR 2 no pea no for tr {trid}");
                        }
                    }
                }
                println!("{ar} = t11:{t11} t10:{t10} t01:{t01} t20:{t20} t02:{t02} t22:{t22}, tt1:{tt1} tt2:{tt2}");
                let fou = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
                if let Ok(ser) = bincode::serialize(&ctrs) {
                    println!("write {fou}");
                    std::fs::write(fou, ser).unwrap();
                }
            } // end file open
        } // end file open
    } // end ar
    Ok(())
}

use crate::geo1::CnlData;
use crate::geo1::MeterBill;

pub fn p10_check_trans() -> Result<(), Box<dyn Error>> {
    let yms = vec!["202402", "202405"];
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let fcmt = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_mt.bin");
        let fctr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
        //let fcnl = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_tr.bin");
        if let (Ok(fcmt), Ok(fctr)) = (File::open(&fcmt), File::open(&fctr)) {
            let fcmt = BufReader::new(fcmt);
            let fctr = BufReader::new(fctr);
            if let (Ok(cmts), Ok(ctrs)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<CnlData>>(fcmt),
                bincode::deserialize_from::<BufReader<File>, HashMap<String, CnlTrans>>(fctr),
            ) {
                println!("{ar}. fcmt: {}  fctr: {}", cmts.len(), ctrs.len());
                let mut cn = 0;
                let mut cmt_i = HashMap::<String, usize>::new();
                let mut cmt_p = HashMap::<String, usize>::new();
                for (ix, cmt) in cmts.iter().enumerate() {
                    if let (Some(pea), Some(ins)) = (&cmt.mt_pea, &cmt.mt_ins) {
                        let ins = ins.to_string();
                        if let Some(_cmt) = cmt_i.get(&ins) {
                            cn += 1;
                        } else {
                            cmt_i.insert(ins, ix);
                        }
                        //let np = format!("{}-{}", n1d, pea);
                        let np = pea.to_string();
                        if let Some(_cmt) = cmt_p.get(&np) {
                            cn += 1;
                        } else {
                            cmt_p.insert(np, ix);
                        }
                    }
                }
                println!(" meter duplicate {cn}");
                for ym in &yms {
                    let fmb = format!("/mnt/e/CHMBACK/pea-data/data1/{ym}_{ar}_bil.bin");
                    if let Ok(fbil) = File::open(fmb) {
                        let fbil = BufReader::new(fbil);
                        if let Ok(fbil) =
                            bincode::deserialize_from::<BufReader<File>, Vec<MeterBill>>(fbil)
                        {
                            let mut mtbil = vec![Vec::<usize>::new(); cmts.len()];
                            let mut cn1 = 0;
                            let mut cn2 = 0;
                            for (ix, bil) in fbil.iter().enumerate() {
                                if let Some(mix) = cmt_i.get(&bil.inst) {
                                    mtbil[*mix].push(ix);
                                } else if let Some(mix) = cmt_p.get(&bil.pea) {
                                    mtbil[*mix].push(ix);
                                    cn1 += 1;
                                } else {
                                    cn2 += 1;
                                }
                            }
                            let fm2b =
                                format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ym}_{ar}_m2b.bin");
                            println!("  {ym} - {} cn1:{cn1} cn2:{cn2}", fbil.len());
                            println!("  mt cn:{} mt2bil:{}", cmts.len(), mtbil.len());
                            if let Ok(ser) = bincode::serialize(&mtbil) {
                                println!("   write to {fm2b}");
                                std::fs::write(fm2b, ser).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubFeedTrans {
    pub sbid: String,
    pub conf: String,
    pub n1d_s: u64,
    pub n1d_f: u64,
    //pub feed: HashMap<String, Vec<String>>,
    pub feed: HashMap<String, Vec<usize>>,
}

use crate::aoj::DbfData;
use crate::geo1::find_node;
use crate::geo1::n1d_2_utm;
use crate::geo1::utm_2_n1d;
//use crate::geo1::DB2_DIR;
use sglab02_lib::sg::mvline::utm_latlong;

pub fn p11_form_sub() -> Result<(), Box<dyn Error>> {
    //let yms = vec!["202402", "202405"];
    let ym = "202405";
    let mut sb_nid_cf = HashMap::<String, (u64, String)>::new();
    let re = Regex::new(r"q=([0-9]+\.[0-9]+),([0-9]+\.[0-9]+)").unwrap();
    let adjxy = sub_latlong_adjust();
    for (sb, cf, gm) in &SUB_TYPES {
        if let Some(cap) = re.captures_iter(gm).next() {
            let x = &cap[1];
            let y = &cap[2];
            let mut xx = x.parse::<f32>().unwrap();
            let mut yy = y.parse::<f32>().unwrap();
            let sbid = sb.to_string();
            if let Some((x1, y1)) = adjxy.get(&sbid) {
                xx += x1;
                yy += y1;
            }
            let (sb_x, sb_y) = latlong_utm(xx, yy);
            let n1d = utm_2_n1d(sb_x, sb_y);
            sb_nid_cf.insert(sb.to_string(), (n1d, cf.to_string()));
        }
    }

    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        println!("proc {ar} - reading");
        //let mveq = "DS_CircuitBreaker";
        let mveq = "DS_MVConductor";
        let fnds = format!("/mnt/e/CHMBACK/pea-data/data1/p9_{ar}_nodes.bin");
        let fctr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
        let fcmt = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_mt.bin");
        let fbil = format!("/mnt/e/CHMBACK/pea-data/data1/{ym}_{ar}_bil.bin");
        let fm2b = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ym}_{ar}_m2b.bin");
        let mvat = format!("/mnt/e/CHMBACK/pea-data/db2/{ar}_{mveq}.at");
        //let mvat = format!("{DB2_DIR}/{ar}_{mveq}.at");
        //let hvat = format!("{DB2_DIR}/{ar}_DS_HVConductor.at");
        //let hvat = format!("{DB2_DIR}/{ar}_{hveq}.at");
        //println!("mvat: {mvat}");

        if let (Ok(fnds), Ok(fctr), Ok(fcmt), Ok(fbil), Ok(fm2b), Ok(mvat)) = (
            File::open(&fnds),
            File::open(&fctr),
            File::open(&fcmt),
            File::open(&fbil),
            File::open(&fm2b),
            File::open(&mvat),
        ) {
            println!("proc 2");
            let fnds = BufReader::new(fnds);
            let fctr = BufReader::new(fctr);
            let fcmt = BufReader::new(fcmt);
            let fbil = BufReader::new(fbil);
            let fm2b = BufReader::new(fm2b);
            //let hvat = BufReader::new(hvat);
            let mvat = BufReader::new(mvat);
            if let (Ok(fnds), Ok(ctrs), Ok(fcmt), Ok(fbil), Ok(fm2b), Ok(mvat)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(fnds),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(fctr),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlData>>(fcmt),
                bincode::deserialize_from::<BufReader<File>, Vec<MeterBill>>(fbil),
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<usize>>>(fm2b),
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(mvat),
            ) {
                println!("proc 3");
                //
                println!(
                    "{ar} nds:{} ctr:{} cmt:{} bil:{} m2b:{} mv:{}",
                    fnds.len(),
                    ctrs.len(),
                    fcmt.len(),
                    fbil.len(),
                    fm2b.len(),
                    //hvat.len(),
                    mvat.len(),
                );
                let mut n1ds = vec![];
                let mut eqnds = vec![];
                //let mut mvnds = HashMap::<String, Vec<u64>>::new();
                for (n1d, ndif) in &fnds {
                    n1ds.push(*n1d);
                    for gn in &ndif.nodes {
                        //if gn.ly == "DS_MVConductor" {
                        if gn.ly == mveq {
                            eqnds.push(*n1d);
                        }
                    }
                }
                n1ds.sort();
                eqnds.sort();
                println!("\n  prepare nodes data Breaker: {}", eqnds.len());

                let mut nosbh = HashMap::<String, usize>::new();
                let mut bcn = 0;
                let mut sb_fd_tr_hm = HashMap::<String, SubFeedTrans>::new();
                for (ti, ctr) in ctrs.iter().enumerate() {
                    //let trid = ctr.trid.to_string();
                    let cmt = &fcmt[ctr.ix];
                    if let Some(fid) = &cmt.tr_fid {
                        let sbid = &fid[0..3];
                        let sbid = sbid.to_string();
                        if let Some((nd, cf)) = sb_nid_cf.get(&sbid) {
                            let fid = fid.to_string();
                            if let Some(sb_fd_tr) = sb_fd_tr_hm.get_mut(&sbid) {
                                if let Some(fd_tr) = sb_fd_tr.feed.get_mut(&fid) {
                                    fd_tr.push(ti);
                                    //fd_tr.push(trid.to_string());
                                } else {
                                    sb_fd_tr.feed.insert(fid.to_string(), vec![ti]);
                                    //.insert(fid.to_string(), vec![trid.to_string()]);
                                }
                            } else {
                                let n1d_s = *nd;
                                //println!("SB {sbid} {n1d_s} - hvnds:{}", hvnds.len());
                                let n1d_f = find_node(n1d_s, &eqnds);
                                let (sx, sy) = n1d_2_utm(n1d_s);
                                let (fx, fy) = n1d_2_utm(n1d_f);
                                let (dx, dy) = ((sx - fx).abs(), (sy - fy).abs());
                                let conf = cf.to_string();
                                if dx + dy > 200.0 {
                                    let (st, sl) = utm_latlong(sx, sy);
                                    let (ft, fl) = utm_latlong(fx, fy);
                                    println!("{ar}-{sbid} == ({dx:.2},{dy:.2}) => surv:{st},{sl} find:{ft},{fl}");
                                }
                                let feed = HashMap::from([(fid.to_string(), vec![ti])]);
                                //HashMap::from([(fid.to_string(), vec![trid.to_string()])]);
                                let sb_fd_tr = SubFeedTrans {
                                    sbid: sbid.to_string(),
                                    n1d_s,
                                    n1d_f,
                                    conf,
                                    feed,
                                };
                                sb_fd_tr_hm.insert(sbid.to_string(), sb_fd_tr);
                            }
                        } else if let Some(cn) = nosbh.get_mut(&sbid) {
                            *cn += 1;
                        } else {
                            nosbh.insert(sbid.to_string(), 1);
                        }
                    }
                    //println!("{ar} n1d:{} mts:{}", ctr.n1d, ctr.mts.len());
                    for mi in &ctr.mts {
                        let _mt = &fcmt[*mi];
                        let m2b = &fm2b[*mi];
                        bcn += m2b.len();
                        //println!("  mt: {:?} bil:{}", mt.mt_pea, m2b.len());
                    }
                }
                println!("NO found {}", nosbh.len());

                let mut sb_fd_tr = Vec::<SubFeedTrans>::new();
                //let mut keys = sb_fd_tr_hm.keys().to_vec();
                let mut keys: Vec<String> =
                    sb_fd_tr_hm.iter().map(|(k, _)| k.to_string()).collect();
                keys.sort();
                for k in keys {
                    let sub = sb_fd_tr_hm.get(&k).unwrap().clone();
                    sb_fd_tr.push(sub);
                }
                let fsb = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ar}_sb_fd_tr.bin");
                if let Ok(ser) = bincode::serialize(&sb_fd_tr) {
                    println!(" write to {fsb} - {}", sb_fd_tr.len());
                    std::fs::write(fsb, ser).unwrap();
                }
                println!("  bcn:{bcn}");
            }
        }
    }
    Ok(())
}

pub fn p12_check_sbfd() -> Result<(), Box<dyn Error>> {
    /*
    let ym = "202405";
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let fsbf = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ar}_sb_fd_tr_hm.bin");
        let fctr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
        let fcmt = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_mt.bin");
        let fbil = format!("/mnt/e/CHMBACK/pea-data/data1/{ym}_{ar}_bil.bin");
        let fm2b = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ym}_{ar}_m2b.bin");
        if let (Ok(fsbf), Ok(fctr), Ok(fcmt), Ok(fbil), Ok(fm2b)) = (
            File::open(&fsbf),
            File::open(&fctr),
            File::open(&fcmt),
            File::open(&fbil),
            File::open(fm2b),
        ) {
            let fsbf = BufReader::new(fsbf);
            let fctr = BufReader::new(fctr);
            let fcmt = BufReader::new(fcmt);
            let fbil = BufReader::new(fbil);
            let fm2b = BufReader::new(fm2b);
            /*
            if let Ok(fsbf) = bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(fctr) {
                println!("fsbf:{}", fsbf.len());
            }
            */
            if let (Ok(fsbf), Ok(ctrs), Ok(fcmt), Ok(fbil), Ok(fm2b)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<String, SubFeedTrans>>(fsbf),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(fctr),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlData>>(fcmt),
                bincode::deserialize_from::<BufReader<File>, Vec<MeterBill>>(fbil),
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<usize>>>(fm2b),
            ) {
                println!(
                    "{ar} fsbf:{} ctr:{} cmt:{} bil:{} m2b:{}",
                    fsbf.len(),
                    ctrs.len(),
                    fcmt.len(),
                    fbil.len(),
                    fm2b.len()
                );
                let mut ctrm = HashMap::<String, CnlTrans>::new();
                for ctr in &ctrs {
                    ctrm.insert(ctr.trid.to_string(), ctr.clone());
                }
                for (sb, fdtr) in &fsbf {
                    println!("{ar} - {sb} - fd:{}", fdtr.feed.len());
                    let mut fds: Vec<String> =
                        fdtr.feed.clone().into_iter().map(|(k, _)| k).collect();
                    fds.sort();
                    for fd in &fds {
                        let trids = fdtr.feed.get(fd).unwrap();
                        //for (fd, trids) in &fdtr.feed {
                        let mut cn = 0;
                        let mut mttp = HashMap::<String, (u32, f32)>::new();
                        for id in trids {
                            //if let Some(ctr) = fctr.get(id) {
                            if let Some(ctr) = ctrm.get(&id) {
                                cn += 1;
                                for mi in &ctr.mts {
                                    let _mt = &fcmt[*mi];
                                    let m2b = &fm2b[*mi];
                                    for mi in m2b {
                                        let bil: &MeterBill = &fbil[*mi];
                                        let volt = bil.volt.to_string();
                                        if let Some((cn, en)) = mttp.get_mut(&volt) {
                                            *cn += 1;
                                            *en += bil.kwh15;
                                        } else {
                                            mttp.insert(volt, (1, 0f32));
                                        }
                                    }
                                    //bcn += m2b.len();
                                    //println!("  mt: {:?} bil:{}", mt.mt_pea, m2b.len());
                                }
                            }
                        }
                        if mttp.len() > 0 {
                            println!("  {fd} trx:{cn} mt:{mttp:?}");
                        }
                    }
                }
            }
        }
    }
            */
    Ok(())
}

use sglab02_lib::sg::imp::xlsx_info;
use std::fs::read_dir;

pub static AR_TH_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "ก.1" => "C1",
    "ก.2" => "C2",
    "ก.3" => "C3",
    "น.1" => "N1",
    "น.2" => "N2",
    "น.3" => "N3",
    "ฉ.1" => "NE1",
    "ฉ.2" => "NE2",
    "ฉ.3" => "NE3",
    "ต.1" => "S1",
    "ต.2" => "S2",
    "ต.3" => "S3",
};

pub static TH_MON_NO: phf::Map<&'static str, u32> = phf_map! {
    "ม.ค." => 1,
    "ก.พ." => 2,
    "มี.ค." => 3,
    "เม.ย." =>4,
    "พ.ค." => 5,
    "มิ.ย." => 6,
    "ก.ค." => 7,
    "ส.ค." => 8,
    "ก.ย." => 9,
    "ต.ค." => 10,
    "พ.ย." => 11,
    "ธ.ค." => 12,
};

pub static TH_VOLTA_NAME: phf::Map<&'static str, &'static str> = phf_map! {
    "PEA VOLTA บางจาก บางปะอิน" => "PEA VOLTA บางจาก บางปะอิน (สายเอเชีย กม. 62 ขาออก)",
    "PEA VOLTA บางจาก เมืองพิษณุโลก (เซ็นทรัล)" => "PEA VOLTA บางจาก เมืองพิษณุโลก (ข้างเซ็นทรัล)",
    "PEA VOLTA บางจาก นครราชสีมา (กม. 141)" => "PEA VOLTA บางจาก เมืองนครราชสีมา (มิตรภาพ กม. 141)",
    "PEA VOLTA บางจาก วังมะนาว (ขาออก)" => "PEA VOLTA บางจาก วังมะนาว (เพชรเกษม ขาออก)",
    "PEA VOLTA บางจาก นครราชสีมา (กม. 134)" => "PEA VOLTA บางจาก เมืองนครราชสีมา (มิตรภาพ กม. 134)",
    "PEA VOLTA บางจาก เมืองบุรีรัมย์ (คูเมือง)" => "PEA VOLTA บางจาก เมืองบุรีรัมย์ (คูเมือง-พุทไธสง)",
    "PEA VOLTA บางจาก ท่าม่วง" => "PEA VOLTA บางจาก ท่าม่วง (ติดโฮมโปร กาญจนบุรี)",
    "PEA VOLTA บางจาก เมืองนครปฐม (กม. 61 ขาเข้า)" => "PEA VOLTA บางจาก เมืองนครปฐม (เพชรเกษม กม. 61 ขาเข้า)",
    "PEA VOLTA ชลบุรี" => "PEA VOLTA บางจาก เมืองชลบุรี",
    "PEA VOLTA ชลบุรี 2" => "PEA VOLTA บางจาก เมืองชลบุรี (หนองมน)",
    "สถานี เข้าท่า บ้านบึง (ร่วมเครือข่าย PEA VOLTA)" => "PEA VOLTA บ้านบึง",
    "บริษัท พรีไซซ ซิสเท็ม (ร่วมเครือข่าย PEA VOLTA)" => "PEA VOLTA สศช.",
    "คาลเท็กซ์ สาขาน้ำมันดาวพนม (เครือข่าย PEA VOLTA)" => "PEA VOLTA สศช.",
    "สถานีชาร์จ พาข้าวโคราช (เครือข่าย PEA VOLTA)" => "PEA VOLTA สศช.",
    "PEA VOLTA นครราชสีมา 2" => "PEA VOLTA นครราชสีมา 2 (หัวทะเล)",
    "PEA VOLTA อู่เชิดชัย เมืองนครราชสีมา" => "PEA VOLTA อู่เชิดชัย",
    "PEA VOLTA คาลเท็กซ์ ศรีราชา 1 (เขาคันทรง)" => "PEA VOLTA คาลเท็กซ์ ศรีราชา (เขาคันทรง)",
    "แอนดาซ พัทยา จอมเทียน บีซ (เครือข่าย PEA VOLTA)" => "PEA VOLTA จอมเทียน",
    "สถานีโดมินิค คาร์วอซ บางแสน (เครือข่าย PEA VOLTA)" => "PEA VOLTA บางแสน",
    "PEA VOLTA โรงพยาบาลพริ้นซ์ ศรีสะเกษ" => "PEA VOLTA โรงพยาบาลพริ้นซ์ ศรีสะเกษ ",
    "PEA VOLTA ลพบุรี" => "PEA VOLTA เขตลพบุรี",
    "สถานี ม.ทักษิณ(สงขลา)(ร่วมเครือข่าย PEA VOLTA)" => "PEA VOLTA เมืองสงขลา",
    "สถานี ม.ทักษิณ(พัทลุง)(ร่วมเครือข่าย PEA VOLTA)" => "PEA VOLTA พัทลุง",
    "สถานีชาร์จ โรงแรม แฮปปี้ อินน์ รีสอร์ท" => "PEA VOLTA สศช.",
    "สถานีชาร์จ หจก.กิมปิโตรเลียม" => "PEA VOLTA สศช.",
    "สถานีเอสซีจี เซรามิกส์ นิคมอุตสาหกรรมหนองแค" => "PEA VOLTA สศช.",
    "สนามกอล์ฟพัทยาคันทรีคลับ" => "PEA VOLTA สศช.",
    "สถานีสหกรณ์การเกษตรลำลูกกา" => "PEA VOLTA สศช.",
    "บริษัท ดูลูฟวร์ จำกัด'" => "PEA VOLTA สศช.",
    "สถานีชาร์จ เอส ดับเบิลยู เอ็น เค. ค้าส่ง" => "PEA VOLTA สศช.",
    "สถานีธนารักษ์พัฒนาสินทรัพย์ 2" => "PEA VOLTA สศช.",
    "สถานีธนารักษ์พัฒนาสินทรัพย์ 1" => "PEA VOLTA สศช.",
    "สถานีชาร์จครัวกล้วยหอม แก่งกระจาน" => "PEA VOLTA สศช.",
    "บ้านชาร์จรถไฟฟ้า หัวหิน 52" => "PEA VOLTA สศช.",
    "ลอฟท์ มาเนีย บูติค โฮเทล" => "PEA VOLTA สศช.",
    "สถานีธนารักษ์พัฒนาสินทรัพย์ 4" => "PEA VOLTA สศช.",
    "สถานีบมจ.พีโอออยล์ กม. 12 สาขา 10" => "PEA VOLTA สศช.",
    "สถานีบมจ.พีโอออยล์ บายพาส สาขา 22" => "PEA VOLTA สศช.",
    "ฮับสถานีชาร์จยานยนต์ไฟฟ้า เมืองประจวบฯ" => "PEA VOLTA สศช.",
    "บริษัท ดูลูฟวร์ จำกัด" => "PEA VOLTA สศช.",
    "ชีจรรย์ กอล์ฟ รีสอร์ท (เครือข่าย PEA VOLTA)" => "PEA VOLTA ชลบุรี 3",
    "สถานีชาร์จ ฮูลิแกน คาเฟ่ (เครือข่าย VOLTA)" => "PEA VOLTA ชลบุรี 3",
    "สถานี เข้าท่า บ้านบึง" => "PEA VOLTA สศช.",
    "บริษัท พรีไซซ ซิสเท็ม" => "PEA VOLTA สศช.",
    "สถานีชาร์จ พาข้าวโคราช (เครือข่าย PEA VOLTA)" => "PEA VOLTA สศช.",
    "สถานีเอสซีจี เซรามิคส์ นิคมอุตสาหกรรมหนองแค" => "PEA VOLTA สศช.",
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SppData {
    pub ar: String,
    pub area: Option<String>,
    pub loc: Option<String>,
    pub lat: Option<f32>,
    pub lon: Option<f32>,
    pub n1d: Option<u64>,
    pub mw: Option<f32>,
    pub fuel: Option<String>,
    pub kv: Option<u32>,
    pub sub: Option<String>,
    pub cod: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VsppData {
    pub ar: String,
    pub pid: Option<String>,
    pub cid: Option<String>,
    pub sta: Option<String>,
    pub area: Option<String>,
    pub tamb: Option<String>,
    pub amp: Option<String>,
    pub prov: Option<String>,
    pub fuel: Option<String>,
    pub kv: Option<f32>,
    pub sbnm: Option<String>,
    pub fdno: Option<u32>,
    pub fdid: Option<String>,
    pub lat: Option<f32>,
    pub lon: Option<f32>,
    pub kw: Option<f32>,
    pub n1d: Option<u64>,
    pub vsco: Option<VsppContr>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VsppContr {
    pub pid: String,
    pub name: Option<String>,
    pub loc: Option<String>,
    pub fuel: Option<String>,
    pub mwi: Option<f32>,
    pub mwr: Option<f32>,
    pub scod: Option<String>,
    pub yr: Option<u32>,
    pub sta: Option<String>,
    pub rem: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VoltaStation {
    pub ar: String,
    pub name: Option<String>,
    pub loc: Option<String>,
    pub tamb: Option<String>,
    pub amp: Option<String>,
    pub prov: Option<String>,
    pub area: Option<String>,
    pub brni: Option<String>,
    pub brna: Option<String>,
    pub lat: Option<f32>,
    pub lon: Option<f32>,
    pub stno: Option<String>,
    pub ba: Option<String>,
    pub pca: Option<String>,
    pub cca: Option<String>,
    pub func: Option<String>,
    pub rem: Option<String>,
    pub n1d: Option<u64>,
    pub sell: Vec<(u32, f32)>,
    pub chgr: Vec<(u32, u32)>, // pw, no
}

use sglab02_lib::sg::imp::XlsSheet;

pub async fn p12_read_volta() -> Result<(), Box<dyn std::error::Error>> {
    let fd = "/mnt/e/CHMBACK/pea-data/inp1/peavolta".to_string();
    let repe = Regex::new(r"[^0-9][^0-9]([^0-9].[0-9]+)").unwrap();
    let re = Regex::new(r"(.\..\.) ([0-9][0-9])").unwrap();
    let mut vdir = vec![fd];
    let mut flst = vec![];
    while let Some(dr) = vdir.pop() {
        if let Ok(paths) = read_dir(dr) {
            for pt in paths.flatten() {
                let pt = pt.path();
                let pn = pt.display().to_string();
                if pt.is_dir() {
                    vdir.push(pn);
                } else if pn.ends_with(".xlsx") {
                    flst.push(pn);
                }
            }
        }
    }
    println!("FILE - {}", flst.len());
    for f in &flst {
        println!(" {f}");
    }
    let mut evst = None;
    let mut chgr = None;
    let mut mons = Vec::<(u32, XlsSheet)>::new();
    if let Ok(xlsv) = xlsx_info(&flst).await {
        println!("xlsv: {}", xlsv.len());
        for x in xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
            if x.shnm == "Station" {
                evst = Some(x);
            } else if x.shnm == "Charger" {
                chgr = Some(x);
            } else if let Some(cap) = re.captures_iter(x.shnm.as_str()).next() {
                let thmo = &cap[1].to_string();
                let thyr = &cap[2].to_string();
                let yr = thyr.parse::<u32>().unwrap();
                let mo = TH_MON_NO.get(thmo.as_str());
                if let Some(mo) = mo {
                    let tm = yr * 100 + mo;
                    //println!("m:{thmo} y:{thyr}, tm:{tm}");
                    mons.push((tm, x));
                }
            }
        }
    }
    let mut volta = HashMap::<String, VoltaStation>::new();
    let mut st2nm = HashMap::<String, String>::new();
    if let Some(x) = evst {
        println!("OK found = '{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
        let hds = &x.data[0];
        let rws = &x.data[1..];
        println!("  {hds:?}");
        for rw in rws {
            let name = if !rw[1].is_empty() {
                Some(rw[1].to_string())
            } else {
                None
            };
            let loc = if !rw[2].is_empty() {
                Some(rw[2].to_string())
            } else {
                None
            };
            let tamb = if !rw[3].is_empty() {
                Some(rw[3].to_string())
            } else {
                None
            };
            let amp = if !rw[4].is_empty() {
                Some(rw[4].to_string())
            } else {
                None
            };
            let prov = if !rw[5].is_empty() {
                Some(rw[5].to_string())
            } else {
                None
            };
            let area = if !rw[6].is_empty() {
                Some(rw[6].to_string())
            } else {
                None
            };
            let brni = if !rw[7].is_empty() {
                Some(rw[7].to_string())
            } else {
                None
            };
            let brna = if !rw[8].is_empty() {
                Some(rw[8].to_string())
            } else {
                None
            };
            let lat = if let Ok(v) = rw[9].parse::<f32>() {
                Some(v)
            } else {
                None
            };
            let lon = if let Ok(v) = rw[10].parse::<f32>() {
                Some(v)
            } else {
                None
            };
            let stno = if !rw[11].is_empty() {
                Some(rw[11].to_string())
            } else {
                None
            };
            let ba = if !rw[12].is_empty() {
                Some(rw[12].to_string())
            } else {
                None
            };
            let pca = if !rw[13].is_empty() {
                Some(rw[13].to_string())
            } else {
                None
            };
            let cca = if !rw[14].is_empty() {
                Some(rw[14].to_string())
            } else {
                None
            };
            let func = if !rw[15].is_empty() {
                Some(rw[15].to_string())
            } else {
                None
            };
            let rem = if !rw[16].is_empty() {
                Some(rw[16].to_string())
            } else {
                None
            };
            let n1d = if let (Some(lat), Some(lon)) = (lat, lon) {
                let (x, y) = latlong_utm(lat, lon);
                Some(utm_2_n1d(x, y))
            } else {
                None
            };
            //println!("{area:?} => ");
            let ar = if let Some(th) = &area {
                if let Some(cap) = repe.captures_iter(th).next() {
                    let pid = cap[1].to_string();
                    if let Some(ar) = AR_TH_CODE.get(&pid) {
                        ar.to_string()
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            };
            //println!("{area:?} => {ar}");
            let volt = VoltaStation {
                ar,
                name,
                loc,
                tamb,
                amp,
                prov,
                area,
                brni,
                brna,
                lat,
                lon,
                stno,
                ba,
                pca,
                cca,
                func,
                rem,
                n1d,
                ..Default::default()
            };
            if let (Some(nm), Some(st)) = (&volt.name, &volt.stno) {
                //println!("STATION nm:{nm:?} st:{st:?}");
                st2nm.insert(st.to_string(), nm.to_string());
                volta.insert(nm.to_string(), volt);
            }
        }
    }
    if let Some(x) = chgr {
        println!("CHARGER SHEET =========================");
        //let hds = &x.data[0];
        let rws = &x.data[1..];
        //println!("  {hds:?}");
        for rw in rws {
            let pw = rw[8].to_string();
            let no = rw[9].to_string();
            let st = rw[23].to_string();
            //println!("CHARGER {pw:?} {no:?} {st:?} {vnm:?}");
            if let Some(vnm) = st2nm.get(&st) {
                if let Some(vol) = volta.get_mut(vnm) {
                    if let (Ok(p), Ok(n)) = (pw.parse::<u32>(), no.parse::<u32>()) {
                        vol.chgr.push((p, n));
                    } else {
                        println!("   parse ERROR {pw} {no}");
                    }
                    //pub chgr: Vec<(u32, u32)>, // pw, no
                    //println!("  vol found");
                } else {
                    println!("  vol NOT found");
                }
            }
        }
    }
    //let mut mons = Vec::<(u32, XlsSrheet)>::new();
    if !mons.is_empty() {
        let rest = Regex::new(r"(ชื่อสถานี).*").unwrap();
        let reuc = Regex::new(r"(จำนวนหน่วย).*").unwrap();
        let reid = Regex::new(r"(รหัสสถานี).*").unwrap();
        for (m, x) in mons {
            //println!(" {m}");
            let mut nmcol = None;
            let mut uncol = None;
            let mut idcol = None;
            for (j, rw) in x.data.iter().enumerate() {
                for (i, cv) in rw.iter().enumerate() {
                    if let Some(_cap) = rest.captures_iter(cv.as_str()).next() {
                        nmcol = Some((j, i));
                    } else if let Some(_cap) = reuc.captures_iter(cv.as_str()).next() {
                        uncol = Some((j, i));
                    } else if let Some(_cap) = reid.captures_iter(cv.as_str()).next() {
                        idcol = Some((j, i));
                    }
                }
            }
            //println!(" station {nmcol:?} {uncol:?}");
            if let (Some((sj, si)), Some((uj, ui))) = (nmcol, uncol) {
                if sj == uj {
                    for (j, rw) in x.data.iter().enumerate() {
                        if j <= sj {
                            continue;
                        }
                        let (st, uc) = (rw[si].to_string(), rw[ui].to_string());
                        let st = if let Some(s) = TH_VOLTA_NAME.get(&st) {
                            s.to_string()
                        } else {
                            st
                        };
                        let uc = uc.parse::<f32>().unwrap_or(0f32);
                        if let Some(x) = volta.get_mut(&st) {
                            x.sell.push((m, uc));
                        } else {
                            print!(" not found '{st}'");
                            if let Some((_j, i)) = idcol {
                                print!(" {}", rw[i]);
                            }
                            println!();
                        }
                    } // end loop data
                } // end if recheck
            } // end if data exist
        } // end loop mon
    } // end if mon
    let mut ar_volt = HashMap::<String, Vec<VoltaStation>>::new();
    for (_k, vol) in volta {
        //println!("{:?} {:?} {}", vol.ar, vol.name, vol.sell.len());
        if let Some(vols) = ar_volt.get_mut(&vol.ar) {
            vols.push(vol);
        } else {
            ar_volt.insert(vol.ar.to_string(), vec![vol]);
        }
    }
    for (ar, vols) in ar_volt {
        if ar.is_empty() {
            continue;
        }
        //println!(" {ar} - vol:{}", vols.len());
        let arv = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_volta.bin");
        if let Ok(ser) = bincode::serialize(&vols) {
            println!("write {ar} to {arv}");
            std::fs::write(arv, ser).expect("?");
        }
    }
    //volta.insert(nm.to_string(), volt);
    Ok(())
}

pub async fn p12_read_der() -> Result<(), Box<dyn std::error::Error>> {
    let fd = "/mnt/e/CHMBACK/pea-data/ข้อมูล DERS".to_string();
    let mut vdir = vec![fd];
    let mut flst = vec![];
    while let Some(dr) = vdir.pop() {
        if let Ok(paths) = read_dir(dr) {
            for pt in paths.flatten() {
                let pt = pt.path();
                let pn = pt.display().to_string();
                if pt.is_dir() {
                    vdir.push(pn);
                } else if pn.ends_with(".xlsx") {
                    flst.push(pn);
                }
            }
        }
    }
    println!("FILE - {}", flst.len());
    for f in &flst {
        println!(" {f}");
    }
    if let Ok(xlsv) = xlsx_info(&flst).await {
        println!("xlsv: {}", xlsv.len());
        let mut spp = None;
        let mut vspp = None;
        let mut vsco = None;
        for x in xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
            if x.shnm == "SPP หลัก" {
                spp = Some(x);
            } else if x.shnm == "สถานะ VSPP - หลัก" {
                vspp = Some(x);
            } else if x.shnm == "สถานะทำสัญญา" {
                vsco = Some(x);
            }
        }
        let mut pr_vsco = HashMap::<String, VsppContr>::new();
        if let Some(vsco) = vsco {
            for rw in vsco.data {
                println!("{}", rw[0]);
                let pid = rw[1].to_string();
                if pid.len() > 0 {
                    let name = Some(rw[2].to_string());
                    let loc = Some(rw[3].to_string());
                    let fuel = Some(rw[4].to_string());
                    let mwi = if let Ok(v) = rw[5].parse::<f32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let mwr = if let Ok(v) = rw[6].parse::<f32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let scod = Some(rw[7].to_string());
                    let yr = if let Ok(v) = rw[8].parse::<u32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let sta = Some(rw[9].to_string());
                    let rem = Some(rw[10].to_string());
                    let vsco = VsppContr {
                        pid,
                        name,
                        loc,
                        fuel,
                        mwi,
                        mwr,
                        scod,
                        yr,
                        sta,
                        rem,
                    };
                    pr_vsco.insert(vsco.pid.to_string(), vsco);
                }
            }
        }
        if let Some(vspp) = vspp {
            let hds = &vspp.data[0];
            let rws = &vspp.data[1..];
            let mut ar_vspp = HashMap::<String, Vec<VsppData>>::new();
            for rw in rws {
                if let Some(ar) = AR_TH_CODE.get(&rw[4]) {
                    let ar = ar.to_string();
                    let prj = rw[1].to_string();
                    let pid = Some(rw[1].to_string());
                    let cid = Some(rw[2].to_string());
                    let sta = Some(rw[3].to_string());
                    let area = Some(rw[4].to_string());
                    let tamb = Some(rw[5].to_string());
                    let amp = Some(rw[6].to_string());
                    let prov = Some(rw[7].to_string());
                    let fuel = Some(rw[8].to_string());
                    let kv = if let Ok(v) = rw[9].parse::<f32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let sbnm = Some(rw[10].to_string());
                    let fdno = if let Ok(v) = rw[11].parse::<u32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let fdid = Some(rw[12].to_string());
                    let lat = if let Ok(v) = rw[12].parse::<f32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let lon = if let Ok(v) = rw[13].parse::<f32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let kw = if let Ok(v) = rw[14].parse::<f32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let n1d = if let (Some(lat), Some(lon)) = (lat, lon) {
                        let (x, y) = latlong_utm(lat, lon);
                        Some(utm_2_n1d(x, y))
                    } else {
                        None
                    };
                    //let vsco = pr_vsco.get(&prj).map(|vsco| vsco.clone());
                    let vsco = pr_vsco.get(&prj).cloned();
                    let vspp = VsppData {
                        ar,
                        pid,
                        cid,
                        sta,
                        area,
                        tamb,
                        amp,
                        prov,
                        fuel,
                        kv,
                        sbnm,
                        fdno,
                        fdid,
                        lat,
                        lon,
                        kw,
                        n1d,
                        vsco,
                    };
                    if let Some(vspps) = ar_vspp.get_mut(&vspp.ar) {
                        vspps.push(vspp);
                    } else {
                        ar_vspp.insert(vspp.ar.to_string(), vec![vspp]);
                    }
                }
            }
            println!("{} hds:{hds:?} rws:{}", vspp.data.len(), rws.len());
            for (ar, vspps) in ar_vspp {
                let far = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_vspp.bin");
                if let Ok(ser) = bincode::serialize(&vspps) {
                    println!("write {} to {far}", vspps.len());
                    std::fs::write(far, ser).unwrap();
                }
            }
        }
        if let Some(spp) = spp {
            let hds = &spp.data[0];
            let rws = &spp.data[1..];
            let mut ar_spp = HashMap::<String, Vec<SppData>>::new();
            for rw in rws {
                if let Some(ar) = AR_TH_CODE.get(&rw[6]) {
                    let ar = ar.to_string();
                    let area = Some(rw[6].to_string());
                    let loc = Some(rw[1].to_string());
                    let lat = if let Ok(v) = rw[2].parse::<f32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let lon = if let Ok(v) = rw[3].parse::<f32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let mw = if let Ok(v) = rw[4].parse::<f32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let fuel = Some(rw[5].to_string());
                    let kv = if let Ok(v) = rw[7].parse::<u32>() {
                        Some(v)
                    } else {
                        None
                    };
                    let sub = Some(rw[8].to_string());
                    let cod = Some(rw[9].to_string());
                    let n1d = if let (Some(lat), Some(lon)) = (lat, lon) {
                        let (x, y) = latlong_utm(lat, lon);
                        Some(utm_2_n1d(x, y))
                    } else {
                        None
                    };
                    let spp = SppData {
                        ar,
                        loc,
                        lat,
                        lon,
                        n1d,
                        mw,
                        fuel,
                        area,
                        kv,
                        sub,
                        cod,
                    };
                    if let Some(spps) = ar_spp.get_mut(&spp.ar) {
                        spps.push(spp);
                    } else {
                        ar_spp.insert(spp.ar.to_string(), vec![spp]);
                    }
                }
            }
            println!("{} hds:{hds:?} rws:{}", spp.data.len(), rws.len());
            for (ar, spps) in ar_spp {
                let far = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_spp.bin");
                if let Ok(ser) = bincode::serialize(&spps) {
                    println!("write {} to {far}", spps.len());
                    std::fs::write(far, ser).unwrap();
                }
            }
        }
    }
    Ok(())
}

pub async fn p12_read_gpp() -> Result<(), Box<dyn std::error::Error>> {
    let fxls = "/mnt/e/CHMBACK/pea-data/inp1/msme_gpp.csv".to_string();
    println!("read {fxls}");
    if let Ok(mut rdr) = csv::Reader::from_path(&fxls) {
        // if read file
        for rc in rdr.records().flatten() {
            if let (Some(tp), Some(prv), Some(sz), Some(sec), Some(y24), Some(y23)) = (
                rc.get(0),
                rc.get(1),
                rc.get(2),
                rc.get(3),
                rc.get(5),
                rc.get(6),
            ) {
                if tp == "PGPP" && prv != "ALL" && sz == "ALL" && sec == "ALL" {
                    println!("{prv} {y24} {y23}");
                }
            }
        }
    }
    Ok(())
}

//let yms = vec!["202402", "202405"];

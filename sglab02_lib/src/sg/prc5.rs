use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use axum::routing::get;
use axum::Router;
//use tokio::sync::oneshot;
use askama::Template;
//use askama_axum::IntoResponse;
use std::sync::OnceLock;
use std::collections::HashMap;

pub async fn prc51() -> Result<(), Box<dyn std::error::Error>> {
    println!("draw image");
    let (ww, hh) = (640,400);
    let mut image = RgbImage::new(ww, hh);
    let wht = Rgb([255u8, 255u8, 255u8]);
    let _grn = Rgb([0u8, 130u8, 0u8]);
    let blk = Rgb([0u8, 0u8, 0u8]);

    let font = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let _font = Font::try_from_vec(font).unwrap();
    let _scale = Scale { x: 15.0, y: 15.0 };

    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(ww, hh), wht);

    let (x1,y1,x2,y2) = (20f32, 20f32, 100f32, 100f32);
    draw_line_segment_mut(&mut image, (x1, y1), (x2, y2), blk);

    //draw_text_mut(&mut image, grn, x1 as i32, y1 as i32, scale, &font, "TEXT");

    if let Ok(_) = image.save("image.png") {}

    Ok(())
}

#[derive(Template, Debug)]
#[template(path = "prc5/base.html", escape = "none")]
pub struct Prc52Temp {
    #[allow(dead_code)]
    pub title: String,
}

impl Prc52Temp {
    #[allow(dead_code)]
    async fn new() -> Self {
        let title = "HOME";
        let title = title.to_string();
        Prc52Temp { title }
    }
}

#[allow(dead_code)]
pub async fn handler() -> Prc52Temp {
    Prc52Temp::new().await
}

async fn handler3() -> impl axum::response::IntoResponse { 
    let _filename = String::from("image.png");
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"text/css")
    );
    (
        headers,
        concat!(
            "b: { font-color: red; }\n",
            "i: { font-color: blue; }\n",
        )
    )
}

async fn handler4() -> impl axum::response::IntoResponse { 

    let _filename = String::from("image.png");
    let mut headers = axum::http::HeaderMap::new();

    let data = Bytes::from(vec![0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,]);
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"text/css")
    );
    use axum::body::Bytes;
    (
        headers,
        data,
    )
}

async fn handler5() -> impl axum::response::IntoResponse { 

    let _filename = String::from("image.png");
    let mut headers = axum::http::HeaderMap::new();

    let (ww, hh) = (640,400);
    let mut image = RgbImage::new(ww, hh);
    let wht = Rgb([255u8, 255u8, 255u8]);
    let _grn = Rgb([0u8, 130u8, 0u8]);
    let blk = Rgb([0u8, 0u8, 0u8]);
    let font = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let _font = Font::try_from_vec(font).unwrap();
    let _scale = Scale { x: 15.0, y: 15.0 };
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(ww, hh), wht);
    let (x1,y1,x2,y2) = (20f32, 20f32, 100f32, 100f32);
    draw_line_segment_mut(&mut image, (x1, y1), (x2, y2), blk);

    use ab_glyph::PxScale;
    use ab_glyph::FontVec;
    
    let uniform_scale_24px = PxScale::from(24.0);

    let font_vec = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let font = FontVec::try_from_vec(font_vec).expect("Font Vec");

    draw_text_mut(&mut image, blk, 100, 100, uniform_scale_24px, &font, "OKOKOK");

    use std::fs;
    use std::fs::File;
    use std::io::Read;

    let filename = "image3.png";
    if let Ok(_) = image.save(filename) {}
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    
    println!("img:{}", buffer.len());

    let _data = Bytes::from(vec![0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,]);
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    use axum::body::Bytes;
    (
        headers,
        buffer,
    )
}

use axum::extract::Path;
use ab_glyph::PxScale;
use ab_glyph::FontVec;
use std::fs;
use std::fs::File;
use std::io::Read;

pub async fn pv_dw(Path(pvnm): Path<String>) -> impl axum::response::IntoResponse { 
    let (ww, hh) = (640,400);
    let mut image = RgbImage::new(ww, hh);

    let wht = Rgb([255u8, 255u8, 255u8]);
    let _grn = Rgb([0u8, 130u8, 0u8]);
    let blk = Rgb([0u8, 0u8, 0u8]);
    let font = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let _font = Font::try_from_vec(font).unwrap();
    let _scale = Scale { x: 15.0, y: 15.0 };
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(ww, hh), wht);
    let (x1,y1,x2,y2) = (20f32, 20f32, 100f32, 100f32);
    draw_line_segment_mut(&mut image, (x1, y1), (x2, y2), blk);

    let pv_dir = format!("{}/pvdw", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(pv_dir);
    let pv_file = format!("{}/pvdw/pvdw_{}.png", crate::sg::imp::data_dir(), pvnm);
    //println!("pvdir {}", pv_file);


    let uniform_scale_24px = PxScale::from(24.0);
    let font_vec = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let font = FontVec::try_from_vec(font_vec).expect("Font Vec");

    draw_text_mut(&mut image, blk, 100, 100, uniform_scale_24px, &font, &pvnm);


    let filename = pv_file.as_str();
    if let Ok(_) = image.save(filename) {}
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    //use axum::body::Bytes;
    (
        headers,
        buffer,
    )
}

use crate::sg::prc1::SubstInfo;
use crate::sg::prc3::ld_p3_calc;
use crate::sg::prc3::ld_p3_prv_sub;
use crate::sg::prc3::ld_p3_sub_inf;
use crate::sg::prc3::DataCalc;
use crate::sg::prc3::ld_p3_prvs;
use crate::sg::prc3::ld_fd_trs;
use crate::sg::prc2::Transformer;
use crate::sg::dcl::LoadProfVal;
use plotters::prelude::*;

pub static PRVS: OnceLock<Vec::<String>> = OnceLock::new();
pub fn prvs() -> &'static Vec::<String> { PRVS.get_or_init(prvs_init) }
fn prvs_init() -> Vec::<String> { ld_p3_prvs() }

pub static PV_SUB: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();
pub fn pv_sub() -> &'static HashMap<String, Vec<String>> { PV_SUB.get_or_init(pv_sub_init) }
fn pv_sub_init() -> HashMap<String, Vec<String>> {    ld_p3_prv_sub() }

pub static SUB_INF: OnceLock<HashMap<String, SubstInfo>> = OnceLock::new();
pub fn sub_inf() -> &'static HashMap<String, SubstInfo> {    SUB_INF.get_or_init(sub_inf_init) }
fn sub_inf_init() -> HashMap<String, SubstInfo> {    ld_p3_sub_inf() }

pub static PRV_CALC: OnceLock<HashMap<String, DataCalc>> = OnceLock::new();
pub fn prv_calc() -> &'static HashMap<String, DataCalc> {    PRV_CALC.get_or_init(prv_calc_init) }
fn prv_calc_init() -> HashMap<String, DataCalc> {    ld_p3_calc("p3_prv_calc.bin") }

pub static SUB_CALC: OnceLock<HashMap<String, DataCalc>> = OnceLock::new();
pub fn sub_calc() -> &'static HashMap<String, DataCalc> {    SUB_CALC.get_or_init(sub_calc_init) }
fn sub_calc_init() -> HashMap<String, DataCalc> {    ld_p3_calc("p3_sub_calc.bin") }

pub static FEED_CALC: OnceLock<HashMap<String, DataCalc>> = OnceLock::new();
pub fn feed_calc() -> &'static HashMap<String, DataCalc> {    FEED_CALC.get_or_init(feed_calc_init) }
fn feed_calc_init() -> HashMap<String, DataCalc> {    ld_p3_calc("p3_feed_calc.bin") }

pub static FD_TRS: OnceLock<HashMap::<String,Vec<Transformer>>> = OnceLock::new();
pub fn fd_trs() -> &'static HashMap::<String,Vec<Transformer>> { FD_TRS.get_or_init(fd_trs_init) }
fn fd_trs_init() -> HashMap::<String,Vec<Transformer>> { ld_fd_trs() }

pub static SUB_LOC: OnceLock<HashMap::<String,(f64,f64)>> = OnceLock::new();
pub fn sub_loc() -> &'static HashMap::<String,(f64,f64)> { SUB_LOC.get_or_init(sub_loc_init) }
fn sub_loc_init() -> HashMap::<String,(f64,f64)> { ld_sub_loc() }

pub static PV_RG_MAP: OnceLock<HashMap::<String,Vec::<Vec::<(f64, f64)>>>> = OnceLock::new();
pub fn pv_rg_map() -> &'static HashMap::<String,Vec::<Vec::<(f64, f64)>>> { PV_RG_MAP.get_or_init(pv_rg_map_init) }
fn pv_rg_map_init() -> HashMap::<String,Vec::<Vec::<(f64, f64)>>> { ld_pv_rg_mp() }

fn data_minmax(data: &Vec::<Vec<(f64,f64)>>) -> (f64,f64) {
    let mut minv = 0_f64;
    let mut maxv = 0_f64;
    for l in data {
        for (_x, y) in l {
            if *y > maxv {
                maxv = *y as f64;
            }
            if *y < minv {
                minv = *y as f64;
            }
        }
    }
    (minv,maxv)
}

pub async fn pv_dw2(Path(pvnm): Path<String>) -> impl axum::response::IntoResponse { 
    let pv_dir = format!("{}/pvdw", crate::sg::imp::data_dir());
    let _wht = Rgb([255u8, 255u8, 255u8]);
    let _blk = Rgb([0u8, 0u8, 0u8]);

    let _ = std::fs::create_dir_all(pv_dir);
    let pv_file = format!("{}/pvdw/pvdw_{}.png", crate::sg::imp::data_dir(), pvnm);

    let mut data = vec![
        vec![(0f64,1f64), (1f64,2f64), (2f64,1f64), (3f64,2f64), (4f64,1f64), (5f64,2f64), ],
        vec![(0f64,1.5f64), (1f64,2.5f64), (2f64,1.5f64), (3f64,2.5f64), (4f64,1.5f64), (5f64,2.5f64), ],
    ];
    if let (Some(sbs), Some(calc)) = (pv_sub().get(&pvnm), prv_calc().get(&pvnm)) {
        println!("pv:{} sub:{} calc:{}", pvnm, sbs.len(), calc.year_load.loads.len());
        data = Vec::<Vec<(f64,f64)>>::new();
        for dyld in &calc.year_load.loads {
            let mut seri = Vec::<(f64,f64)>::new();
            //println!("{}-{}", dyld.day, dyld.load.len());
            let mut xx = 0.0f64;
            for lpv in &dyld.load {
                if let LoadProfVal::Value(v) = lpv {
                    seri.push((xx, *v as f64));
                    xx += 0.5;
                }
            }
            data.push(seri);
        }
    }
    let filename = pv_file.as_str();

    let (minv,maxv) = data_minmax(&data);

    let drawing_area =
        BitMapBackend::new(pv_file.as_str(), (640, 400)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area);

    chart_builder
        .margin(10)
        .set_left_and_bottom_label_area_size(30);

    let mut ctx = chart_builder
        .build_cartesian_2d(0.0..24.0, minv..maxv)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();
    for (_i, ls) in data.iter().enumerate() {
        ctx.draw_series(LineSeries::new(
            ls.iter().map(|(x, y)| (*x, *y)),
            RGBColor(0, 0, 0),
        ))
        .unwrap();
    }
    drawing_area.present().expect("draw");

    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    
    println!("pv: {}", pvnm);

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    //use axum::body::Bytes;
    (
        headers,
        buffer,
    )
}


///////////////////////////////////////////////////////
fn month_st_ed(ym: &String) -> (usize, usize, usize, usize) {
    let mon = vec![31,28,31,30,31,30, 31,31,30,31,30,31];
    let (mut st, mut ed) = (1usize,365usize);
    if ym.starts_with("M") {
        let mut ii = 0;
        st = 1; ed = 1;
        if let Ok(i) = ym[1..].parse::<i32>() {
            for j in 0..i {
                ii += mon[j as usize];
                if j<i-1 {
                    st = ii+1;
                } else {
                    ed = ii;
                }
            }
        }
    } else if ym.starts_with("D") {
        st = 1; ed = 1;
        if let Ok(i) = ym[1..].parse::<i32>() {
            st = i as usize;
            ed = st;
            let mut ii = 0;
            let mut d1 = 1;
            //let mut d2 = 1;
            for j in 0..12 {
                ii += mon[j as usize];
                if st<=ii { let d2 = ii; return (st, ed, d1, d2); }
                d1 = ii+1;
            }
        }
    }
    (st, ed, st, ed)
}

fn pv_png_ym_gen(pvnm: String, ym: String) -> Vec<u8> {
    let (st,ed,_d1,_d2) = month_st_ed(&ym);
    //println!("II: {} = {}-{} {}-{}", ym, st, ed, d1, d2);
    let pv_dir = format!("{}/pv_ym", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&pv_dir);
    let pv_file = format!("{}/pv_ym_{}_{}.png", pv_dir, pvnm, ym);

    let _wht = Rgb([255u8, 255u8, 255u8]);
    let _blk = Rgb([0u8, 0u8, 0u8]);
    let mut data = vec![
        vec![(0f64,1f64), (1f64,2f64), (2f64,1f64), (3f64,2f64), (4f64,1f64), (5f64,2f64), ],
    ];
    if let (Some(_sbs), Some(calc)) = (pv_sub().get(&pvnm), prv_calc().get(&pvnm)) {
        data = Vec::<Vec<(f64,f64)>>::new();
        for ii in st-1..ed {
            let dyld = &calc.year_load.loads[ii];
            //for dyld in &calc.year_load.loads {
            let mut seri = Vec::<(f64,f64)>::new();
            let mut xx = 0.0f64;
            for lpv in &dyld.load {
                if let LoadProfVal::Value(v) = lpv {
                    seri.push((xx, *v as f64));
                    xx += 0.5;
                }
            }
            data.push(seri);
        }
    }
    let filename = pv_file.as_str();
    let (minv,maxv) = data_minmax(&data);
    let drawing_area =
        BitMapBackend::new(pv_file.as_str(), (640, 400)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area);

    chart_builder
        .margin(10)
        .set_left_and_bottom_label_area_size(30);

    let mut ctx = chart_builder
        .build_cartesian_2d(0.0..24.0, minv..maxv)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();
    for (_i, ls) in data.iter().enumerate() {
        ctx.draw_series(LineSeries::new(
            ls.iter().map(|(x, y)| (*x, *y)),
            RGBColor(0, 0, 0),
        ))
        .unwrap();
    }
    drawing_area.present().expect("draw");

    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    

    buffer
}

fn sb_png_ym_gen(sbid: String, ym: String) -> Vec<u8> {
    let (st,ed,_d1,_d2) = month_st_ed(&ym);
    //println!("II: {} = {}-{} {}-{}", ym, st, ed, d1, d2);
    let sb_dir = format!("{}/sb_ym", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&sb_dir);
    let sb_file = format!("{}/sb_ym_{}_{}.png", sb_dir, sbid, ym);

    let _wht = Rgb([255u8, 255u8, 255u8]);
    let _blk = Rgb([0u8, 0u8, 0u8]);
    let mut data = vec![
        vec![(0f64,1f64), (1f64,2f64), (2f64,1f64), (3f64,2f64), (4f64,1f64), (5f64,2f64), ],
    ];
    if let Some(calc) = sub_calc().get(&sbid) {
        data = Vec::<Vec<(f64,f64)>>::new();
        for ii in st-1..ed {
            let dyld = &calc.year_load.loads[ii];
            let mut seri = Vec::<(f64,f64)>::new();
            let mut xx = 0.0f64;
            for lpv in &dyld.load {
                if let LoadProfVal::Value(v) = lpv {
                    seri.push((xx, *v as f64));
                    xx += 0.5;
                }
            }
            data.push(seri);
        }
    }
    let filename = sb_file.as_str();
    let (minv,maxv) = data_minmax(&data);
    let drawing_area =
        BitMapBackend::new(sb_file.as_str(), (640, 400)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area);

    chart_builder
        .margin(10)
        .set_left_and_bottom_label_area_size(30);

    let mut ctx = chart_builder
        .build_cartesian_2d(0.0..24.0, minv..maxv)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();
    for (_i, ls) in data.iter().enumerate() {
        ctx.draw_series(LineSeries::new(
            ls.iter().map(|(x, y)| (*x, *y)),
            RGBColor(0, 0, 0),
        ))
        .unwrap();
    }
    drawing_area.present().expect("draw");

    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    

    buffer
}

///////////////////////////////////////////////////////
#[derive(Template, Debug, Default)]
#[template(path = "prc5/pv_pg_ym.html", escape = "none")]
pub struct ProvPageYmTemp {
    pub title: String,
    pub pvnm: String,
    pub ym: String,
    pub dts: Vec<String>, 
    pub sbs: Vec<String>,
    pub subs: Vec<(String,String,String,usize,usize,i32,String)>,
}

pub async fn pv_pg_ym(Path((pvnm,ym)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let title = pvnm.to_string();
    let mut tmp = ProvPageYmTemp::default();
    tmp.title = title.to_string();
    tmp.pvnm = pvnm.to_string();
    tmp.ym = ym.to_string();
    let (_st,_ed,d1,d2) = month_st_ed(&ym);
    if d2-d1<100 {
        for i in d1..=d2 {
            tmp.dts.push(format!("D{}",i));
        }
    }
    if let Some(sbv) = pv_sub().get(&pvnm) {
        for sb in sbv {
            tmp.sbs.push(sb.to_string());
        }
    }

    if let Some(sbs) = pv_sub().get(&pvnm) {
        //println!("{} - sub:{}", pvnm, sbs.len());
        for sb in sbs {
            //println!("  sb:{}", sb);
            if let (Some((x,y)),Some(sbif)) = (sub_loc().get(sb), sub_inf().get(sb)) {
                let (xx, yy) = utm_latlong(*x as f32, *y as f32);
                let ldln = format!("{:.4},{:.4}", xx, yy);
                let ldln = format!("https://maps.google.com/?q={}", ldln);
                //println!("     loc:{},{}: {} {} {} {} {} - {}", x, y, sbif.name, sbif.enam, sbif.trxn, sbif.feno, sbif.mvxn, ldln);
                tmp.subs.push((sb.to_string(), sbif.name.to_string(), sbif.enam.to_string(), sbif.trxn, sbif.feno, sbif.mvxn, ldln));
            }
        }
    }
    
    tmp
}

pub async fn pv_png_ym(Path((pvnm,ym)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, pv_png_ym_gen(pvnm, ym), )
}
///////////////////////////////////////////////////////

///////////////////////////////////////////////////////
#[derive(Template, Debug, Default)]
#[template(path = "prc5/sb_pg_ym.html", escape = "none")]
pub struct SubPageYmTemp {
    pub title: String,
    pub sbid: String,
    pub ym: String,
    pub dts: Vec<String>, 
    pub feeds: Vec<String>,
    //pub fdcas: Vec<&'a DataCalc>,
}

pub async fn sb_pg_ym(Path((sbid,ym)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let title = sbid.to_string();
    let mut tmp = SubPageYmTemp::default();
    tmp.title = title.to_string();
    tmp.sbid = sbid.to_string();
    tmp.ym = ym.to_string();
    let (_st,_ed,d1,d2) = month_st_ed(&ym);
    if d2-d1<100 {
        for i in d1..=d2 {
            tmp.dts.push(format!("D{}",i));
        }
    }
    if let Some(sbif) = sub_inf().get(&sbid) {
        for fd in &sbif.feeders {
            tmp.feeds.push(fd.to_string());
            /*
            if let Some(fdca) = feed_calc().get(fd) {
                tmp.fdcas.push(fdca);
            }
            */
        }
    }
    tmp
}

pub async fn sb_png_ym(Path((sbid,ym)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, sb_png_ym_gen(sbid, ym), )
}
///////////////////////////////////////////////////////

pub async fn prc52() -> Result<(), Box<dyn std::error::Error>> {
    let _prv = pv_sub_init();
    let _base = crate::sg::ldp::base();
    let app = Router::new()
    .route("/", get(prvs_pg))
    .route("/h3", get(handler3))
    .route("/h4", get(handler4))
    .route("/h5", get(handler5))
    .route("/pv_dw/:pvnm", get(pv_dw))
    .route("/pv_dw2/:pvnm", get(pv_dw2))
    .route("/pv/:pvnm", get(pv_dw2))
    .route("/pv_pg_ym/:pvnm/:ym", get(pv_pg_ym))
    .route("/pv_png_ym/:pvnm/:ym", get(pv_png_ym))
    .route("/sb_pg_ym/:pvnm/:ym", get(sb_pg_ym))
    .route("/sb_png_ym/:pvnm/:ym", get(sb_png_ym))
    .route("/pv_pg_sub_map/:pvnm", get(pv_pg_sub_map))
    .route("/pv_png_sub_map/:pvnm", get(pv_png_sub_map))
    ;
    
    let lisn = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(lisn, app).await.unwrap();
    Ok(())
}

/////////////////////////////////////////////////////////
///////////////////////////////////////////////////////
#[derive(Template, Debug, Default)]
#[template(path = "prc5/prvs_pg.html", escape = "none")]
pub struct PrvsPageTemp {
    pub title: String,
    pub prvs: Vec<String>, 
    pub no: i32,
}

impl PrvsPageTemp {
    #[allow(dead_code)]
    fn next(&mut self) -> i32 {
        self.no += 1;
        self.no
    }
}


pub async fn prvs_pg() -> impl axum::response::IntoResponse { 
    let mut tmp = PrvsPageTemp::default();
    tmp.title = "จังหวัด".to_string();
    tmp.prvs.append(&mut prvs().clone());
    tmp
}

/////////////////////////////////////////////////////////
///////////////////////////////////////////////////////

#[derive(Template, Debug, Default)]
#[template(path = "prc5/pv_pg_sub_map.html", escape = "none")]
pub struct ProvPageSubMapTemp {
    pub title: String,
    pub pvnm: String,
    pub subs: Vec<(String,String,String,usize,usize,i32,String)>,
}

use crate::sg::prc3::ld_sub_loc;
use crate::sg::mvline::utm_latlong;
pub async fn pv_pg_sub_map(Path(pvnm): Path<String>) -> impl axum::response::IntoResponse { 
    let mut tmp = ProvPageSubMapTemp::default();
    tmp.title = pvnm.to_string();
    tmp.pvnm = pvnm.to_string();
    let _pvsb = pv_sub();
    let _sblo = sub_loc();
    //println!("{} {}", pvsb.len(), sblo.len());
    if let Some(sbs) = pv_sub().get(&pvnm) {
        //println!("{} - sub:{}", pvnm, sbs.len());
        for sb in sbs {
            //println!("  sb:{}", sb);
            if let (Some((x,y)),Some(sbif)) = (sub_loc().get(sb), sub_inf().get(sb)) {
                let (xx, yy) = utm_latlong(*x as f32, *y as f32);
                let ldln = format!("{:.4},{:.4}", xx, yy);
                let ldln = format!("https://maps.google.com/?q={}", ldln);
                //println!("     loc:{},{}: {} {} {} {} {} - {}", x, y, sbif.name, sbif.enam, sbif.trxn, sbif.feno, sbif.mvxn, ldln);
                tmp.subs.push((sb.to_string(), sbif.name.to_string(), sbif.enam.to_string(), sbif.trxn, sbif.feno, sbif.mvxn, ldln));
            }
        }
    }
    tmp
}

fn pv_png_sub_map_gen(pv: String) -> Vec<u8> {
    let pv_dir = format!("{}/pv_sub_map", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&pv_dir);
    let pv_file = format!("{}/pv_sub_map_{}.png", pv_dir, pv);

    let (ww, hh, mx, my) = (600,400, 25, 25);
    let mut image = RgbImage::new(ww, hh);
    let wht = Rgb([255u8, 255u8, 255u8]);
    let _grn = Rgb([0u8, 130u8, 0u8]);
    let blk = Rgb([0u8, 0u8, 0u8]);
    let font_vec = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let font = FontVec::try_from_vec(font_vec).expect("Font Vec");
    let uniform_scale_24px = PxScale::from(24.0);

    let mut pnts = Vec::<(f32,f32,String)>::new();
    if let Some(sbs) = pv_sub().get(&pv) {
        //println!("{} - sub:{}", pv, sbs.len());
        for sb in sbs {
            //println!("  sb:{}", sb);
            if let (Some((x,y)),Some(_sbif)) = (sub_loc().get(sb), sub_inf().get(sb)) {
                pnts.push((*x as f32, *y as f32, sb.to_string()));
            }
        }
    }

    //let pv_rg = pv_rg_map();
    let mut pv_rg = Vec::<Vec::<(f64, f64)>>::new();
    if let Some(dt) = pv_rg_map().get(&pv) {
        pv_rg = dt.clone();
    }

    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(ww, hh), wht);

    if pnts.len()>0 && pv_rg.len()>0 {
        let (mut x0,mut x1,mut y0, mut y1) = (pnts[0].0,pnts[0].0, pnts[0].1,pnts[0].1);
        for pn in &pnts {
            if pn.0<x0 { x0 = pn.0; }
            if pn.0>x1 { x1 = pn.0; }
            if pn.1<y0 { y0 = pn.1; }
            if pn.1>y1 { y1 = pn.1; }
        }
        //println!("C1: {},{} - {},{}", x0,y0, x1,y1);
        for pg in &pv_rg {
            for pp in pg {
                let pn = (pp.0 as f32, pp.1 as f32);
                if pn.0<x0 { x0 = pn.0; }
                if pn.0>x1 { x1 = pn.0; }
                if pn.1<y0 { y0 = pn.1; }
                if pn.1>y1 { y1 = pn.1; }
            }
        }
        //println!("C2: {},{} - {},{}", x0,y0, x1,y1);
        let wx = (ww - 2*mx) as f32;
        let hy = (hh - 2*my) as f32;
        let xo = mx as f32;
        let yo = my as f32;
        let fl = 2.0 * mx as f32 - 1.0;

        //println!("{} {} {} {}", xo, yo, wx, hy);
        // borders
        draw_line_segment_mut(&mut image, (0.0, 0.0), (0.0+wx +fl, 0.0), blk);
        draw_line_segment_mut(&mut image, (0.0, 0.0+hy+fl), (0.0+wx+fl, 0.0+hy+fl), blk);
        draw_line_segment_mut(&mut image, (0.0, 0.0), (0.0, 0.0+hy+fl), blk);
        draw_line_segment_mut(&mut image, (0.0+wx+fl, 0.0), (0.0+wx+fl, 0.0+hy+fl), blk);

        let _pnw = if x1-x0>y1-y0 { x1 - x0 } else { y1 - y0 };
        let _fmw = if x1-x0>y1-y0 { wx } else { hy };

        let (mut pnw, mut fmw, mut ofx, mut ofy) = (0f32,0f32,0f32,0f32);
        if y1>y0 {
            let prt = (x1-x0)/(y1-y0);
            let wrt = wx / hy;
            if prt>wrt {
            //if x1-x0>y1-y0 {
                // wide
                pnw = x1 - x0;
                fmw = wx;
                let ymx = (y1 - y0) / pnw * fmw;
                let of = (hy - ymx) * 0.5;
                //println!("high {} {} {}", ymx, wx, of);
                ofy = -of;
            } else {
                // high
                pnw = y1 - y0;
                fmw = hy;
                let xmx = (x1 - x0) / pnw * fmw;
                let of = (wx - xmx) * 0.5;
                //println!("wid {} {} {}", xmx, hy, of);
                ofx = of;
            }
        }
        //println!("pnw:{} fmw:{}", pnw, fmw);

        // province
        for pg in &pv_rg {
            if pg.len()<2 { continue; }
            let mut lst = (pg[0].0 as f32, pg[0].1 as f32);
            for i in 1..pg.len() {
                let pn = (pg[i].0 as f32, pg[i].1 as f32);
                let xs = xo + (lst.0 - x0) / pnw * fmw;
                let ys = yo + hy - (lst.1 - y0) / pnw * fmw;
                let xe = xo + (pn.0 - x0) / pnw * fmw;
                let ye = yo + hy - (pn.1 - y0) / pnw * fmw;
                draw_line_segment_mut(&mut image, (xs+ofx,ys+ofy), (xe+ofx,ye+ofy), blk);
                lst = pn;
            }
        }

        for pn in &pnts {
            let xx = xo + (pn.0 - x0) / pnw * fmw+ofx;
            let yy = yo + hy - (pn.1 - y0) / pnw * fmw+ofy;

            draw_text_mut(&mut image, blk, xx as i32, yy as i32, uniform_scale_24px, &font, &pn.2);
            draw_line_segment_mut(&mut image, (xx, yy-5.0), (xx, yy+5.0), blk);
            draw_line_segment_mut(&mut image, (xx-5.0, yy), (xx+5.0, yy), blk);
        }

    }

    if let Ok(_) = image.save(&pv_file) {}

    let mut f = File::open(&pv_file).expect("no file found");
    let metadata = fs::metadata(&pv_file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    
    buffer
}

pub async fn pv_png_sub_map(Path(pvnm): Path<String>) -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, pv_png_sub_map_gen(pvnm), )
}

use crate::sg::gis1::ar_list;
//use crate::sg::gis1::gis_dir;
//use crate::sg::gis1::db1_dir;
use crate::sg::gis1::db2_dir;
use crate::sg::gis1::DbfVal;
use std::io::BufReader;
use crate::sg::prc5::DbfVal::Character;

fn pv_en_th_nm() -> HashMap::<&'static str, &'static str> {
    [
        ("AMNAT CHAROEN", "อำนาจเจริญ"), 
        ("ANG THONG", "อ่างทอง"), 
        ("BANGKOK", "กรุงเทพมหานคร"), 
        ("BUENG KAN", "บึงกาฬ"), 
        ("BURI RAM", "บุรีรัมย์"), 
        ("CHACHOENGSAO", "ฉะเชิงเทรา"), 
        ("CHAI NAT", "ชัยนาท"), 
        ("CHAIYAPHUM", "ชัยภูมิ"), 
        ("CHANTHABURI", "จันทบุรี"), 
        ("CHIANG MAI", "เชียงใหม่"), 
        ("CHIANG RAI", "เชียงราย"), 
        ("CHON BURI", "ชลบุรี"), 
        ("CHUMPHON", "ชุมพร"), 
        ("KALASIN", "กาฬสินธุ์"), 
        ("KAMPHAENG PHET", "กำแพงเพชร"), 
        ("KANCHANABURI", "กาญจนบุรี"), 
        ("KHON KAEN", "ขอนแก่น"), 
        ("KRABI", "กระบี่"), 
        ("LAMPANG", "ลำปาง"), 
        ("LAMPHUN", "ลำพูน"), 
        ("LOEI", "เลย"), 
        ("LOP BURI", "ลพบุรี"), 
        ("MAE HONG SON", "แม่ฮ่องสอน"), 
        ("MAHA SARAKHAM", "มหาสารคาม"), 
        ("MUKDAHAN", "มุกดาหาร"), 
        ("NAKHON NAYOK", "นครนายก"), 
        ("NAKHON PHANOM", "นครพนม"), 
        ("NAKHON PATHOM", "นครปฐม"), 
        ("NAKHON RATCHASIMA", "นครราชสีมา"), 
        ("NAKHON SAWAN", "นครสวรรค์"), 
        ("NAKHON SI THAMMARAT", "นครศรีธรรมราช"), 
        ("NAN", "น่าน"), 
        ("NARATHIWAT", "นราธิวาส"), 
        ("NONG BUA LAM PHU", "หนองบัวลำภู"), 
        ("NONG KHAI", "หนองคาย"), 
        ("NONTHABURI", "นนทบุรี"), 
        ("PATHUM THANI", "ปทุมธานี"), 
        ("PATTANI", "ปัตตานี"), 
        ("PETCHABURI", "เพชรบุรี"), 
        ("PHANGNGA", "พังงา"), 
        ("PHATTHALUNG", "พัทลุง"), 
        ("PHAYAO", "พะเยา"), 
        ("PHETCHABUN", "เพชรบูรณ์"), 
        ("PHICHIT", "พิจิตร"), 
        ("PHITSANULOK", "พิษณุโลก"), 
        ("PHRA NAKHON SI AYUTTHAYA", "พระนครศรีอยุธยา"), 
        ("PHRAE", "แพร่"), 
        ("PHUKET", "ภูเก็ต"), 
        ("PRACHIN BURI", "ปราจีนบุรี"), 
        ("PRACHUAP KHIRI KHAN", "ประจวบคีรีขันธ์"), 
        ("RANONG", "ระนอง"), 
        ("RATCHABURI", "ราชบุรี"), 
        ("RAYONG", "ระยอง"), 
        ("ROI ET", "ร้อยเอ็ด"), 
        ("SA KAEO", "สระแก้ว"), 
        ("SAKON NAKHON", "สกลนคร"), 
        ("SAMUT PRAKAN", "สมุทรปราการ"), 
        ("SAMUT SAKHON", "สมุทรสาคร"), 
        ("SAMUT SONGKHRAM", "สมุทรสงคราม"), 
        ("SARABURI", "สระบุรี"), 
        ("SATUN", "สตูล"), 
        ("SI SA KET", "ศรีสะเกษ"), 
        ("SING BURI", "สิงห์บุรี"), 
        ("SISAKET", "ศรีสะเกษ"), 
        ("SONGKHLA", "สงขลา"), 
        ("SUKHOTHAI", "สุโขทัย"), 
        ("SUPHAN BURI", "สุพรรณบุรี"), 
        ("SURAT THANI", "สุราษฎร์ธานี"), 
        ("SURIN", "สุรินทร์"), 
        ("TAK", "ตาก"), 
        ("TRANG", "ตรัง"), 
        ("TRAT", "ตราด"), 
        ("UBON RATCHATHANI", "อุบลราชธานี"), 
        ("UDON THANI", "อุดรธานี"), 
        ("UTHAI THANI", "อุทัยธานี"), 
        ("UTTARADIT", "อุตรดิตถ์"), 
        ("YALA", "ยะลา"), 
        ("YASOTHON", "ยโสธร"), 
    ].iter().cloned().collect()
}

pub static PV_EN_TH: OnceLock<HashMap::<&'static str, &'static str>> = OnceLock::new();
pub fn pv_en_th() -> &'static HashMap::<&'static str, &'static str> { PV_EN_TH.get_or_init(pv_en_th_init) }
fn pv_en_th_init() -> HashMap::<&'static str, &'static str> { pv_en_th_nm() }

pub async fn prc53() -> Result<(), Box<dyn std::error::Error>> {
    let wdir = db2_dir();
    let ly = "LB_Changwat";
    let mut pv_nms = Vec::<String>::new();
    let mut pv_rgs = Vec::<Vec::<Vec::<(f64, f64)>>>::new();
    for r in ar_list() {
        let dbf = format!("{}/{}_{}.db", wdir, r, ly);
        let rgf = format!("{}/{}_{}.rg", wdir, r, ly);
        //println!("{}, {}", dbf, rgf);
        if let Ok(f) = File::open(dbf) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfVal>>>(BufReader::new(f)) {
                let mut pvnms = Vec::<String>::new();
                for pv in &dt {
                    let enm = pv.get("CHANGWAT_1").unwrap();
                    if let Character(Some(enm)) = enm {
                        if let Some(th) = pv_en_th().get(enm.as_str()) {
                            //println!("{} = {}", enm, th);
                            pvnms.push(th.to_string());
                        } else {
                            println!("ERROR {}", enm);
                        }
                    } else {
                        println!("{:?}", pv);
                    }
                }
                pv_nms.append(&mut pvnms);
                //println!("nams {} {}", dt.len(), pvnms.len());
            }
        }
        if let Ok(f) = File::open(rgf) {
            if let Ok(dt) = bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(BufReader::new(f)) {
                for rg in &dt {
                    pv_rgs.push(rg.clone());
                }
                //println!("vec {}", dt.len());
            }
        }
        //println!("{} {}", pv_nms.len(), pv_rgs.len());
    }
    let mut pv_rg_mp = HashMap::<String,Vec::<Vec::<(f64, f64)>>>::new();
    for i in 0..pv_nms.len() {
        pv_rg_mp.insert(pv_nms[i].clone(), pv_rgs[i].clone());
    }
    let file = format!("{}/pv_rg_mp.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&pv_rg_mp) {
        std::fs::write(file, ser).unwrap();
    }
    let _pv_rg_mp2 = ld_pv_rg_mp();
    //println!("{} = {}", pv_rg_mp.len(), pv_rg_mp2.len());

    Ok(())
}

pub fn ld_pv_rg_mp() -> HashMap::<String,Vec::<Vec::<(f64, f64)>>> {
    if let Ok(f) = File::open(crate::sg::ldp::res("pv_rg_mp.bin")) {
        if let Ok(dt)=bincode::deserialize_from::<BufReader<File>, HashMap::<String,Vec::<Vec::<(f64, f64)>>>>(BufReader::new(f)) {
            return dt;
        }
    }
    HashMap::<String,Vec::<Vec::<(f64, f64)>>>::new()
}

///////////////////////////////////////////////////////
#[derive(Template, Debug, Default)]
#[template(path = "prc5/fd_pg_ym.html", escape = "none")]
pub struct FeedPageYmTemp {
    pub title: String,
    pub fdid: String,
    pub ym: String,
    pub dts: Vec<String>, 
    pub trans: Vec<Transformer1>,
}

#[derive(Debug, Default)]
pub struct Transformer1 {
    pub no: i32,
    pub tx_id: String,
    pub tx_power: f64,
    pub tx_own: String,
    pub mt_1_ph: usize,
    pub mt_3_ph: usize,
    pub gm: String,
}

use crate::sg::prc6::ld_p62_fd_trans;
pub async fn fd_pg_ym(Path((fdid,ym)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let title = fdid.to_string();
    let mut tmp = FeedPageYmTemp::default();
    tmp.title = title.to_string();
    tmp.fdid = fdid.to_string();
    tmp.ym = ym.to_string();
    let (_st,_ed,d1,d2) = month_st_ed(&ym);
    if d2-d1<100 {
        for i in d1..=d2 {
            tmp.dts.push(format!("D{}",i));
        }
    }

    let trs = ld_p62_fd_trans(&fdid);
    let trlo = ld_p63_fd_tr_lo(&fdid);
    let mut no = 0;
    for tr in trs {
        no += 1;
        let mut tr1 = Transformer1::default();
        tr1.no = no;
        tr1.tx_id = tr.tx_id.to_string();
        tr1.tx_power = tr.tx_power;
        tr1.tx_own = tr.tx_own.to_string();
        tr1.mt_1_ph = tr.mt_1_ph;
        tr1.mt_3_ph = tr.mt_3_ph;
        //let tr1 = Transformer1 { no, tx_id, tx_power, tx_own, mt_1_ph, mt_3_ph };
        if let Some(fo) = trlo.get(&tr.tx_id) {
            let (xx, yy) = utm_latlong(fo.x as f32, fo.y as f32);
            let ldln = format!("{:.4},{:.4}", xx, yy);
            tr1.gm = format!("https://maps.google.com/?q={}", ldln);
        }
        tmp.trans.push(tr1);
    }
    tmp
}

pub async fn fd_png_ym(Path((fdid,ym)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, fd_png_ym_gen(fdid, ym), )
}

use crate::sg::prc6::ld_p61_fd_calc;
fn fd_png_ym_gen(fdid: String, ym: String) -> Vec<u8> {
    let (st,ed,_d1,_d2) = month_st_ed(&ym);
    //println!("II: {} = {}-{} {}-{}", ym, st, ed, d1, d2);
    let fd_dir = format!("{}/fd_ym", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&fd_dir);
    let fd_file = format!("{}/fd_ym_{}_{}.png", fd_dir, fdid, ym);

    let _wht = Rgb([255u8, 255u8, 255u8]);
    let _blk = Rgb([0u8, 0u8, 0u8]);
    let /*mut*/ _data = vec![
        vec![(0f64,1f64), (1f64,2f64), (2f64,1f64), (3f64,2f64), (4f64,1f64), (5f64,2f64), ],
    ];

    let calc = ld_p61_fd_calc(&fdid);

    let mut data = Vec::<Vec<(f64,f64)>>::new();
    for ii in st-1..ed {
        let dyld = &calc.year_load.loads[ii];
        let mut seri = Vec::<(f64,f64)>::new();
        let mut xx = 0.0f64;
        for lpv in &dyld.load {
            if let LoadProfVal::Value(v) = lpv {
                seri.push((xx, *v as f64));
                xx += 0.5;
            }
        }
        data.push(seri);
    }

    let filename = fd_file.as_str();
    let (minv,maxv) = data_minmax(&data);
    let drawing_area =
        BitMapBackend::new(fd_file.as_str(), (640, 400)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area);

    chart_builder
        .margin(10)
        .set_left_and_bottom_label_area_size(30);

    let mut ctx = chart_builder
        .build_cartesian_2d(0.0..24.0, minv..maxv)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();
    for (_i, ls) in data.iter().enumerate() {
        ctx.draw_series(LineSeries::new(
            ls.iter().map(|(x, y)| (*x, *y)),
            RGBColor(0, 0, 0),
        ))
        .unwrap();
    }
    drawing_area.present().expect("draw");

    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    

    buffer
}


/////////////////////////////////////////////////////////
///////////////////////////////////////////////////////

#[derive(Template, Debug, Default)]
#[template(path = "prc5/fd_pg_map.html", escape = "none")]
pub struct FeedPageMapTemp {
    pub title: String,
    pub fdid: String,
    pub trans: Vec<Transformer1>,
    pub pop: String,
}

use crate::sg::prc6::ld_p63_fd_tr_lo;
pub async fn fd_pg_map(Path(fdid): Path<String>) -> impl axum::response::IntoResponse { 
    let mut tmp = FeedPageMapTemp::default();
    tmp.title = fdid.to_string();
    tmp.fdid = fdid.to_string();
    tmp.pop = fd_pop_map_gen(&fdid);
    let trs = ld_p62_fd_trans(&fdid);
    let trlo = ld_p63_fd_tr_lo(&fdid);
    let mut no = 0;
    for tr in trs {
        no += 1;
        let mut tr1 = Transformer1::default();
        tr1.no = no;
        tr1.tx_id = tr.tx_id.to_string();
        tr1.tx_power = tr.tx_power;
        tr1.tx_own = tr.tx_own.to_string();
        tr1.mt_1_ph = tr.mt_1_ph;
        tr1.mt_3_ph = tr.mt_3_ph;
        if let Some(fo) = trlo.get(&tr.tx_id) {
            let (xx, yy) = utm_latlong(fo.x as f32, fo.y as f32);
            let ldln = format!("{:.4},{:.4}", xx, yy);
            tr1.gm = format!("https://maps.google.com/?q={}", ldln);
        }
        //let tr1 = Transformer1 { no, tx_id, tx_power, tx_own, mt_1_ph, mt_3_ph };
        //println!("{:?}", tr);
        tmp.trans.push(tr1);
    }

    tmp
}

pub async fn fd_png_map(Path(fdid): Path<String>) -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, fd_png_map_gen(fdid), )
}

use imageproc::drawing::draw_hollow_circle_mut;

fn fd_png_map_gen(fdid: String) -> Vec<u8> {
    let fd_dir = format!("{}/fd_map", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&fd_dir);
    let fd_file = format!("{}/fd_map_{}.png", fd_dir, fdid);
    //println!("fd: {}", fd_file);

    let mut sb_pnts = Vec::<(f32,f32,String)>::new();
    let sbid = fdid[0..3].to_string();
    if let Some(lo) = sub_loc().get(&sbid) {
        //println!("{} = {},{}", sbid, lo.0, lo.1);
        sb_pnts.push((lo.0 as f32, lo.1 as f32, "X".to_string()));
    }

    let (ww, hh, mx, my) = (600,400, 25, 25);
    let mut image = RgbImage::new(ww, hh);
    let wht = Rgb([255u8, 255u8, 255u8]);
    let grn = Rgb([0u8, 130u8, 0u8]);
    let blk = Rgb([0u8, 0u8, 0u8]);
    let font_vec = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let _font = FontVec::try_from_vec(font_vec).expect("Font Vec");
    let _uniform_scale_24px = PxScale::from(24.0);

    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(ww, hh), wht);

    let wx = (ww - 2*mx) as f32;
    let hy = (hh - 2*my) as f32;
    let xo = mx as f32;
    let yo = my as f32;
    let fl = 2.0 * mx as f32 - 1.0;

    //println!("{} {} {} {}", xo, yo, wx, hy);
    // borders
    draw_line_segment_mut(&mut image, (0.0, 0.0), (0.0+wx +fl, 0.0), blk);
    draw_line_segment_mut(&mut image, (0.0, 0.0+hy+fl), (0.0+wx+fl, 0.0+hy+fl), blk);
    draw_line_segment_mut(&mut image, (0.0, 0.0), (0.0, 0.0+hy+fl), blk);
    draw_line_segment_mut(&mut image, (0.0+wx+fl, 0.0), (0.0+wx+fl, 0.0+hy+fl), blk);

    let mut pnts = Vec::<(f32,f32,String)>::new();
    let trlo = ld_p63_fd_tr_lo(&fdid);
    //println!("trlo: {}", trlo.len());
    for (_tr,fo) in trlo {
        let pn = (fo.x as f32, fo.y as f32, "X".to_string());
        pnts.push(pn);
        //println!("{} {} {} {}", fo.feeder_id, fo.facility_id, fo.x, fo.y);
    }

    if pnts.len()>0 {
        let (mut x0,mut x1,mut y0, mut y1) = (pnts[0].0,pnts[0].0, pnts[0].1,pnts[0].1);
        for pn in &pnts {
            if pn.0<x0 { x0 = pn.0; }
            if pn.0>x1 { x1 = pn.0; }
            if pn.1<y0 { y0 = pn.1; }
            if pn.1>y1 { y1 = pn.1; }
        }
        //println!("C2: {},{} - {},{}", x0,y0, x1,y1);
        let _pnw = if x1-x0>y1-y0 { x1 - x0 } else { y1 - y0 };
        let _fmw = if x1-x0>y1-y0 { wx } else { hy };

        let (mut pnw, mut fmw, mut ofx, mut ofy) = (0f32,0f32,0f32,0f32);
        if y1>y0 {
            let prt = (x1-x0)/(y1-y0);
            let wrt = wx / hy;
            if prt>wrt {
            //if x1-x0>y1-y0 {
                // wide
                pnw = x1 - x0;
                fmw = wx;
                let ymx = (y1 - y0) / pnw * fmw;
                let of = (hy - ymx) * 0.5;
                //println!("high {} {} {}", ymx, wx, of);
                ofy = -of;
            } else {
                // high
                pnw = y1 - y0;
                fmw = hy;
                let xmx = (x1 - x0) / pnw * fmw;
                let of = (wx - xmx) * 0.5;
                //println!("wid {} {} {}", xmx, hy, of);
                ofx = of;
            }
        }
        let cols = vec![
            Rgb([130u8, 0u8, 0u8]),
            Rgb([0u8, 130u8, 0u8]),
            Rgb([0u8, 0u8, 130u8]),
            Rgb([130u8, 130u8, 0u8]),
            Rgb([0u8, 13u8, 130u8]),
        ];

        let mut cn = 0;
        for pn in &sb_pnts {
            let xx = xo + (pn.0 - x0) / pnw * fmw + ofx;
            let yy = yo + hy - (pn.1 - y0) / pnw * fmw + ofy;
            draw_hollow_circle_mut(&mut image, (xx as i32, yy as i32), 4, grn);
            draw_line_segment_mut(&mut image, (xx, yy-5.0), (xx, yy+5.0), blk);
            draw_line_segment_mut(&mut image, (xx-5.0, yy), (xx+5.0, yy), blk);
        }

        for pn in &pnts {
            let co = cn % cols.len();
            let co = cols[co];

            let xx = xo + (pn.0 - x0) / pnw * fmw + ofx;
            let yy = yo + hy - (pn.1 - y0) / pnw * fmw + ofy;

            draw_line_segment_mut(&mut image, (xx-3.0, yy-3.0), (xx+3.0, yy-3.0), co);
            draw_line_segment_mut(&mut image, (xx-3.0, yy-3.0), (xx+0.0, yy+3.0), co);
            draw_line_segment_mut(&mut image, (xx+3.0, yy-3.0), (xx+0.0, yy+3.0), co);

            cn += 1;
        }

    }

    if let Ok(_) = image.save(&fd_file) {}

    let mut f = File::open(&fd_file).expect("no file found");
    let metadata = fs::metadata(&fd_file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    
    buffer
}

fn fd_pop_map_gen(fdid: &String) -> String {
    let mut pop = String::new();

    let mut sb_pnts = Vec::<(f32,f32,String)>::new();
    let sbid = fdid[0..3].to_string();
    if let Some(lo) = sub_loc().get(&sbid) {
        //println!("{} = {},{}", sbid, lo.0, lo.1);
        sb_pnts.push((lo.0 as f32, lo.1 as f32, "X".to_string()));
    }
    let (ww, hh, mx, my) = (600,400, 25, 25);
    let wx = (ww - 2*mx) as f32;
    let hy = (hh - 2*my) as f32;
    let xo = mx as f32;
    let yo = my as f32;
    let _fl = 2.0 * mx as f32 - 1.0;

    let mut pnts = Vec::<(f32,f32,String)>::new();
    let trlo = ld_p63_fd_tr_lo(&fdid);
    for (_tr,fo) in trlo {
        let pn = (fo.x as f32, fo.y as f32, fo.facility_id.to_string());
        pnts.push(pn);
    }

    if pnts.len()>0 {
        let (mut x0,mut x1,mut y0, mut y1) = (pnts[0].0,pnts[0].0, pnts[0].1,pnts[0].1);
        for pn in &pnts {
            if pn.0<x0 { x0 = pn.0; }
            if pn.0>x1 { x1 = pn.0; }
            if pn.1<y0 { y0 = pn.1; }
            if pn.1>y1 { y1 = pn.1; }
        }

        //println!("C2: {},{} - {},{}", x0,y0, x1,y1);
        let _pnw = if x1-x0>y1-y0 { x1 - x0 } else { y1 - y0 };
        let _fmw = if x1-x0>y1-y0 { wx } else { hy };

        let (mut pnw, mut fmw, mut ofx, mut ofy) = (0f32,0f32,0f32,0f32);
        if y1>y0 {
            let prt = (x1-x0)/(y1-y0);
            let wrt = wx / hy;
            if prt>wrt {
                pnw = x1 - x0;
                fmw = wx;
                let ymx = (y1 - y0) / pnw * fmw;
                let of = (hy - ymx) * 0.5;
                ofy = -of;
            } else {
                // high
                pnw = y1 - y0;
                fmw = hy;
                let xmx = (x1 - x0) / pnw * fmw;
                let of = (wx - xmx) * 0.5;
                ofx = of;
            }
        }
        /*
        for pn in &sb_pnts {
            let xx = xo + (pn.0 - x0) / pnw * fmw + ofx;
            let yy = yo + hy - (pn.1 - y0) / pnw * fmw + ofy;
        }
        */
        use std::fmt::Write;
        write!(pop, "<map name='image-map'>\n").unwrap();
        for pn in &pnts {
            let xx = xo + (pn.0 - x0) / pnw * fmw + ofx;
            let yy = yo + hy - (pn.1 - y0) / pnw * fmw + ofy;
            
            let xi = xx as i32;
            let yi = yy as i32;
            let co = format!("{},{},{},{}",xi-5, yi-5, xi+5, yi+5);
            write!(pop, "<area target='GMAP' alt='A-{}' title='เลขหม้อแปลง\n {}' href='#' coords='{}' shape='rect'>\n", pn.2, pn.2, co).unwrap();
        }
        write!(pop, "</map>\n").unwrap();
    }
    pop
}


/////////////////////////////////////////////////////////
///////////////////////////////////////////////////////

#[derive(Template, Debug, Default)]
#[template(path = "prc5/tr_pg_map.html", escape = "none")]
pub struct TranPageMapTemp {
    pub title: String,
    pub fdid: String,
    pub trid: String,
    pub trx: Transformer,
    pub meters: Vec<TranPageMeterInfo>,
    pub pop: String,
}

#[derive(Debug, Clone, Default)]
pub struct TranPageMeterInfo {
    pub meter_id: String,
    pub meter_phase: String,
    pub meter_office: String,
    pub e5: f32,
    pub e2: f32,
    pub x: f32,
    pub y: f32,
    pub gm: String,
}

use crate::sg::prc6::ld_p65_fd_tr_mt;
use crate::sg::prc6::ld_p68_mt_mp;
use crate::sg::prc6::Prc6MeterInfo2;

pub async fn tr_pg_map(Path((fdid,trid)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let mut tmp = TranPageMapTemp::default();
    tmp.title = fdid.to_string();
    tmp.fdid = fdid.to_string();
    tmp.trid = trid.to_string();

    tmp.pop = tr_pop_map_gen(fdid.to_string(), trid.to_string());

    //pub fn ld_p65_fd_tr_mt(fd: &String) -> HashMap<String, Vec<MeterInfo>> {
    //pub fn ld_p68_mt_mp(fd: &String) -> HashMap::<String,Prc6MeterInfo2> {

    let trs = ld_p62_fd_trans(&fdid);
    let /*mut*/ _no = 0;
    let mut trx = Transformer::default();
    for tr in trs {
        if tr.tx_id==trid {
            trx = tr.clone();
            break;
        }
    }
    //println!("trid: {}", trx.tx_id);

    tmp.trx = trx;
    let tr_mt = ld_p65_fd_tr_mt(&fdid);
    let mt_mp = ld_p68_mt_mp(&fdid);

    if let Some(mts) = tr_mt.get(&trid) {
        //println!("meter {}", mts.len());
        for mt in mts {
            if let Some(mtif) = mt_mp.get(&mt.meter_id) {
                //println!("  {}", mt.meter_id);
                if mtif.x<1000f32 || mtif.y<1000f32 { continue; }
                let mut mt = TranPageMeterInfo::default();
                mt.meter_id = mtif.meter_id.to_string();
                mt.meter_phase = mtif.meter_phase.to_string();
                mt.meter_office = mtif.meter_office.to_string();
                mt.e5 = mtif.e5;
                mt.e2 = mtif.e2;
                mt.x = mtif.x;
                mt.y = mtif.y;
                let (xx, yy) = utm_latlong(mt.x, mt.y);
                let ldln = format!("{:.4},{:.4}", xx, yy);
                let ldln = format!("https://maps.google.com/?q={}", ldln);
                mt.gm = ldln;
                tmp.meters.push(mt);
            }
        }
    }
    tmp
}

pub async fn tr_png_map(Path((fdid, trid)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, tr_png_map_gen(fdid, trid), )
}

fn tr_png_map_gen(fdid: String, trid: String) -> Vec<u8> {
    let fd_dir = format!("{}/tr_map", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&fd_dir);
    let fd_file = format!("{}/tr_map_{}.png", fd_dir, fdid);
    //println!("fd: {}", fd_file);

    let trs = ld_p62_fd_trans(&fdid);
    //let mut trx = Transformer::default();
    for tr in trs {
        if tr.tx_id==trid {
            //trx = tr.clone();
            break;
        }
    }
    //println!("trid: {}", trx.tx_id);
    let tr_mt = ld_p65_fd_tr_mt(&fdid);
    let mt_mp = ld_p68_mt_mp(&fdid);

    let mut mtifs = Vec::<Prc6MeterInfo2>::new();
    if let Some(mts) = tr_mt.get(&trid) {
        //println!("meter {}", mts.len());
        for mt in mts {
            if let Some(mtif) = mt_mp.get(&mt.meter_id) {
                //println!("  {}", mt.meter_id);
                mtifs.push(mtif.clone());
            }
        }
    }

    let (ww, hh, mx, my) = (600,400, 25, 25);
    let mut image = RgbImage::new(ww, hh);
    let wht = Rgb([255u8, 255u8, 255u8]);
    let _grn = Rgb([0u8, 130u8, 0u8]);
    let blk = Rgb([0u8, 0u8, 0u8]);
    let font_vec = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let _font = FontVec::try_from_vec(font_vec).expect("Font Vec");
    let _uniform_scale_24px = PxScale::from(24.0);

    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(ww, hh), wht);

    let wx = (ww - 2*mx) as f32;
    let hy = (hh - 2*my) as f32;
    let xo = mx as f32;
    let yo = my as f32;
    let fl = 2.0 * mx as f32 - 1.0;

    // borders
    draw_line_segment_mut(&mut image, (0.0, 0.0), (0.0+wx +fl, 0.0), blk);
    draw_line_segment_mut(&mut image, (0.0, 0.0+hy+fl), (0.0+wx+fl, 0.0+hy+fl), blk);
    draw_line_segment_mut(&mut image, (0.0, 0.0), (0.0, 0.0+hy+fl), blk);
    draw_line_segment_mut(&mut image, (0.0+wx+fl, 0.0), (0.0+wx+fl, 0.0+hy+fl), blk);

    let mut sb_pnts = Vec::<(f32,f32,String)>::new();
    let mut pnts = Vec::<(f32,f32,String)>::new();
    let trlo = ld_p63_fd_tr_lo(&fdid);
    for (tr,fo) in trlo {
        if tr==trid {
            let pn = (fo.x as f32, fo.y as f32, "X".to_string());
            sb_pnts.push(pn);
            break;
        }
    }
    for mtif in &mtifs {
        let pn = (mtif.x as f32, mtif.y as f32, "X".to_string());
        if pn.0<1000f32 || pn.1<1000f32 { continue; }
        pnts.push(pn);
    }

    if pnts.len()>0 {
        let (mut x0,mut x1,mut y0, mut y1) = (pnts[0].0,pnts[0].0, pnts[0].1,pnts[0].1);
        for pn in &pnts {
            if pn.0<x0 { x0 = pn.0; }
            if pn.0>x1 { x1 = pn.0; }
            if pn.1<y0 { y0 = pn.1; }
            if pn.1>y1 { y1 = pn.1; }
        }
        //println!("C2: {},{} - {},{}", x0,y0, x1,y1);
        let _pnw = if x1-x0>y1-y0 { x1 - x0 } else { y1 - y0 };
        let _fmw = if x1-x0>y1-y0 { wx } else { hy };

        let (mut pnw, mut fmw, mut ofx, mut ofy) = (0f32,0f32,0f32,0f32);
        if y1>y0 {
            let prt = (x1-x0)/(y1-y0);
            let wrt = wx / hy;
            if prt>wrt {
            //if x1-x0>y1-y0 {
                // wide
                pnw = x1 - x0;
                fmw = wx;
                let ymx = (y1 - y0) / pnw * fmw;
                let of = (hy - ymx) * 0.5;
                //println!("high {} {} {}", ymx, wx, of);
                ofy = -of;
            } else {
                // high
                pnw = y1 - y0;
                fmw = hy;
                let xmx = (x1 - x0) / pnw * fmw;
                let of = (wx - xmx) * 0.5;
                //println!("wid {} {} {}", xmx, hy, of);
                ofx = of;
            }
        }

        let cols = vec![
            Rgb([130u8, 0u8, 0u8]),
            Rgb([0u8, 130u8, 0u8]),
            Rgb([0u8, 0u8, 130u8]),
            Rgb([130u8, 130u8, 0u8]),
            Rgb([0u8, 13u8, 130u8]),
        ];

        // province
        let mut cn = 0;
        for pn in &sb_pnts {
            let xx = xo + (pn.0 - x0) / pnw * fmw;
            let yy = yo + hy - (pn.1 - y0) / pnw * fmw;
            let w = 3.0;
            draw_line_segment_mut(&mut image, (xx-w+ofx, yy-w+ofy), (xx+w+ofx, yy-w+ofy), blk);
            draw_line_segment_mut(&mut image, (xx-w+ofx, yy-w+ofy), (xx+0.0+ofx, yy+w+ofy), blk);
            draw_line_segment_mut(&mut image, (xx+w+ofx, yy-w+ofy), (xx+0.0+ofx, yy+w+ofy), blk);

        }

        for pn in &pnts {
            let co = cn % cols.len();
            let co = cols[co];

            let xx = xo + (pn.0 - x0) / pnw * fmw;
            let yy = yo + hy - (pn.1 - y0) / pnw * fmw;

            let w = 3.0;
            draw_line_segment_mut(&mut image, (xx-w+ofx, yy-w+ofy), (xx+w+ofx, yy-w+ofy), co);
            draw_line_segment_mut(&mut image, (xx-w+ofx, yy+w+ofy), (xx+w+ofx, yy+w+ofy), co);
            draw_line_segment_mut(&mut image, (xx-w+ofx, yy-w+ofy), (xx-w+ofx, yy+w+ofy), co);
            draw_line_segment_mut(&mut image, (xx+w+ofx, yy-w+ofy), (xx+w+ofx, yy+w+ofy), co);
            cn += 1;
        }

    }

    if let Ok(_) = image.save(&fd_file) {}

    let mut f = File::open(&fd_file).expect("no file found");
    let metadata = fs::metadata(&fd_file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    
    buffer
}

fn tr_pop_map_gen(fdid: String, trid: String) -> String {
    let fd_dir = format!("{}/tr_map", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&fd_dir);
    let _fd_file = format!("{}/tr_map_{}.png", fd_dir, fdid);

    let trs = ld_p62_fd_trans(&fdid);
    //let mut trx = Transformer::default();
    for tr in trs {
        if tr.tx_id==trid {
            //trx = tr.clone();
            break;
        }
    }
    //println!("trid: {}", trx.tx_id);
    let tr_mt = ld_p65_fd_tr_mt(&fdid);
    let mt_mp = ld_p68_mt_mp(&fdid);

    let mut mtifs = Vec::<Prc6MeterInfo2>::new();
    if let Some(mts) = tr_mt.get(&trid) {
        //println!("meter {}", mts.len());
        for mt in mts {
            if let Some(mtif) = mt_mp.get(&mt.meter_id) {
                //println!("  {}", mt.meter_id);
                mtifs.push(mtif.clone());
            }
        }
    }

    let (ww, hh, mx, my) = (600,400, 25, 25);

    let wx = (ww - 2*mx) as f32;
    let hy = (hh - 2*my) as f32;
    let xo = mx as f32;
    let yo = my as f32;
    let _fl = 2.0 * mx as f32 - 1.0;

    let mut sb_pnts = Vec::<(f32,f32,String)>::new();
    let mut pnts = Vec::<(f32,f32,String)>::new();
    let trlo = ld_p63_fd_tr_lo(&fdid);
    for (tr,fo) in trlo {
        if tr==trid {
            let pn = (fo.x as f32, fo.y as f32, fo.facility_id.to_string());
            sb_pnts.push(pn);
            break;
        }
    }
    for mtif in &mtifs {
        let pn = (mtif.x as f32, mtif.y as f32, mtif.meter_id.to_string());
        if pn.0<1000f32 || pn.1<1000f32 { continue; }
        pnts.push(pn);
    }

    use std::fmt::Write;

    let mut pop = String::new();
    write!(pop, "<map name='image-map'>\n").unwrap();
    if pnts.len()>0 {
        let (mut x0,mut x1,mut y0, mut y1) = (pnts[0].0,pnts[0].0, pnts[0].1,pnts[0].1);
        for pn in &pnts {
            if pn.0<x0 { x0 = pn.0; }
            if pn.0>x1 { x1 = pn.0; }
            if pn.1<y0 { y0 = pn.1; }
            if pn.1>y1 { y1 = pn.1; }
        }
        let _pnw = if x1-x0>y1-y0 { x1 - x0 } else { y1 - y0 };
        let _fmw = if x1-x0>y1-y0 { wx } else { hy };

        let (mut pnw, mut fmw, mut ofx, mut ofy) = (0f32,0f32,0f32,0f32);
        if y1>y0 {
            let prt = (x1-x0)/(y1-y0);
            let wrt = wx / hy;
            if prt>wrt {
                // wide
                pnw = x1 - x0;
                fmw = wx;
                let ymx = (y1 - y0) / pnw * fmw;
                let of = (hy - ymx) * 0.5;
                //println!("high {} {} {}", ymx, wx, of);
                ofy = -of;
            } else {
                // high
                pnw = y1 - y0;
                fmw = hy;
                let xmx = (x1 - x0) / pnw * fmw;
                let of = (wx - xmx) * 0.5;
                //println!("wid {} {} {}", xmx, hy, of);
                ofx = of;
            }
        }
        //meter_id
        for pn in &pnts {
            let xx = xo + (pn.0 - x0) / pnw * fmw + ofx;
            let yy = yo + hy - (pn.1 - y0) / pnw * fmw + ofy;

            let _w = 3.0;
            let xi = xx as i32;
            let yi = yy as i32;
            let co = format!("{},{},{},{}",xi-5, yi-5, xi+5, yi+5);
            write!(pop, "<area target='GMAP' alt='A-{}' title='เลขมิเตอร์\n {}' href='#' coords='{}' shape='rect'>\n", pn.2, pn.2, co).unwrap();
        }

    }
    write!(pop, "</map>\n").unwrap();
    pop
}

/////////////////////////////////////////////////////////////
fn th_png_map_gen() -> Vec<u8> {
    let th_dir = format!("{}/th_map", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&th_dir);
    let th_file = format!("{}/th_map.png", th_dir);

    let (ww, hh, mx, my) = (400,600, 25, 25);
    let mut image = RgbImage::new(ww, hh);
    let wht = Rgb([255u8, 255u8, 255u8]);
    let grn = Rgb([0u8, 130u8, 0u8]);
    let blk = Rgb([0u8, 0u8, 0u8]);
    let font_vec = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let _font = FontVec::try_from_vec(font_vec).expect("Font Vec");
    let _uniform_scale_24px = PxScale::from(24.0);

    //use crate::sg::prc5::DbfVal::Float;
    let mut pv_rgs = Vec::<(String, Vec::<Vec::<(f64, f64)>>, f32)>::new();
    let (mut avmx, mut avmn) = (0f32, 0f32);
    let mut b_fst = true;
    let mut cnt = 0usize;
    for (pv,rg) in pv_rg_map() {
        let /*mut*/ pwavg = 0f32;
        if let Some(calc) = prv_calc().get(pv) {
            let va = calc.year_load.power_quality.pos_avg;
            println!("{} va: {}", pv, va);
            if b_fst {
                avmx = va;
                avmn = va;
                b_fst = false;
            } else {
                if va>avmx { avmx = va; }
                if va<avmn { avmn = va; }
            }
            cnt += 1;
        }
        pv_rgs.push((pv.to_string(), rg.clone(), pwavg));
    }
    println!("{} {} {}", avmx, avmn, cnt);

    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(ww, hh), wht);
    if pv_rgs.len()>0 {
        let (x,y) = &pv_rgs[0].1[0][0];
        let (x,y) = (*x, *y);
        let (mut x0, mut x1, mut y0, mut y1) = (x as f32,x as f32,y as f32,y as f32);
        for (_pv, rg, _av) in &pv_rgs {
            //println!("{} - {}", pv, rg.len());
            for pg in rg {
                for pp in pg {
                    let pn = (pp.0 as f32, pp.1 as f32);
                    if pn.0<x0 { x0 = pn.0; }
                    if pn.0>x1 { x1 = pn.0; }
                    if pn.1<y0 { y0 = pn.1; }
                    if pn.1>y1 { y1 = pn.1; }
                }
            }
        }
        let wx = (ww - 2*mx) as f32;
        let hy = (hh - 2*my) as f32;
        let xo = mx as f32;
        let yo = my as f32;
        let fl = 2.0 * mx as f32 - 1.0;
        draw_line_segment_mut(&mut image, (0.0, 0.0), (0.0+wx +fl, 0.0), blk);
        draw_line_segment_mut(&mut image, (0.0, 0.0+hy+fl), (0.0+wx+fl, 0.0+hy+fl), blk);
        draw_line_segment_mut(&mut image, (0.0, 0.0), (0.0, 0.0+hy+fl), blk);
        draw_line_segment_mut(&mut image, (0.0+wx+fl, 0.0), (0.0+wx+fl, 0.0+hy+fl), blk);

        let (mut pnw, mut fmw, mut ofx, mut ofy) = (0f32,0f32,0f32,0f32);
        if y1>y0 {
            let prt = (x1-x0)/(y1-y0);
            let wrt = wx / hy;
            if prt>wrt {
                pnw = x1 - x0;
                fmw = wx;
                let ymx = (y1 - y0) / pnw * fmw;
                let of = (hy - ymx) * 0.5;
                //println!("high {} {} {}", ymx, wx, of);
                ofy = -of;
            } else {
                // high
                pnw = y1 - y0;
                fmw = hy;
                let xmx = (x1 - x0) / pnw * fmw;
                let of = (wx - xmx) * 0.5;
                //println!("wid {} {} {}", xmx, hy, of);
                ofx = of;
            }
        }

        use imageproc::point::Point;
        use imageproc::drawing::draw_hollow_polygon_mut;
        use imageproc::drawing::draw_polygon_mut;

        for (_pv, rg, _av) in &pv_rgs {
            let _lv = -1;
            //println!("{} av:{} mn:{} mx:{}", pv, av, avmn, avmx);
            for pg in rg {
                if pg.len()<2 { continue; }
                let mut pli = Vec::<Point<i32>>::new();
                let mut ply = Vec::<Point<f32>>::new();
                for i in 0..pg.len()-1 {
                    let pp = &pg[i];
                    let pn = (pp.0 as f32, pp.1 as f32);
                    let x = xo + (pn.0 - x0) / pnw * fmw + ofx;
                    let y = yo + hy - (pn.1 - y0) / pnw * fmw + ofy;
                    ply.push(Point{x,y});
                    let x = x as i32;
                    let y = y as i32;
                    if pli.len()>0 && pli[0].x==x && pli[0].y==y {
                    } else {
                        pli.push(Point{x,y});
                    }
                }
                if pli.len()>3 {
                    draw_polygon_mut(&mut image, &pli, grn);
                }
                draw_hollow_polygon_mut(&mut image, &ply, blk);
            }
        }
    }
    if let Ok(_) = image.save(&th_file) {}
    let mut f = File::open(&th_file).expect("no file found");
    let metadata = fs::metadata(&th_file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    
    buffer
}

pub async fn th_png_map() -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, th_png_map_gen(), )
}


/////////////////////////////////////////////////////////////
fn th_pop_map_gen() -> String {
    let mut pop = String::new();
    let (ww, hh, mx, my) = (400,600, 25, 25);
    //use crate::sg::prc5::DbfVal::Float;
    let mut pv_rgs = Vec::<(String, Vec::<Vec::<(f64, f64)>>, f32)>::new();
    let (mut avmx, mut avmn) = (0f32, 0f32);
    let mut b_fst = true;
    let mut cnt = 0usize;
    for (pv,rg) in pv_rg_map() {
        let /*mut*/ pwavg = 0f32;
        if let Some(calc) = prv_calc().get(pv) {
            let va = calc.year_load.power_quality.pos_avg;
            println!("{} va: {}", pv, va);
            if b_fst {
                avmx = va;
                avmn = va;
                b_fst = false;
            } else {
                if va>avmx { avmx = va; }
                if va<avmn { avmn = va; }
            }
            cnt += 1;
        }
        pv_rgs.push((pv.to_string(), rg.clone(), pwavg));
    }
    println!("{} {} {}", avmx, avmn, cnt);

    if pv_rgs.len()>0 {
        let (x,y) = &pv_rgs[0].1[0][0];
        let (x,y) = (*x, *y);
        let (mut x0, mut x1, mut y0, mut y1) = (x as f32,x as f32,y as f32,y as f32);
        for (_pv, rg, _av) in &pv_rgs {
            //println!("{} - {}", pv, rg.len());
            for pg in rg {
                for pp in pg {
                    let pn = (pp.0 as f32, pp.1 as f32);
                    if pn.0<x0 { x0 = pn.0; }
                    if pn.0>x1 { x1 = pn.0; }
                    if pn.1<y0 { y0 = pn.1; }
                    if pn.1>y1 { y1 = pn.1; }
                }
            }
        }
        let wx = (ww - 2*mx) as f32;
        let hy = (hh - 2*my) as f32;
        let xo = mx as f32;
        let yo = my as f32;
        let _fl = 2.0 * mx as f32 - 1.0;
        let (mut pnw, mut fmw, mut ofx, mut ofy) = (0f32,0f32,0f32,0f32);
        if y1>y0 {
            let prt = (x1-x0)/(y1-y0);
            let wrt = wx / hy;
            if prt>wrt {
                pnw = x1 - x0;
                fmw = wx;
                let ymx = (y1 - y0) / pnw * fmw;
                let of = (hy - ymx) * 0.5;
                //println!("high {} {} {}", ymx, wx, of);
                ofy = -of;
            } else {
                // high
                pnw = y1 - y0;
                fmw = hy;
                let xmx = (x1 - x0) / pnw * fmw;
                let of = (wx - xmx) * 0.5;
                //println!("wid {} {} {}", xmx, hy, of);
                ofx = of;
            }
        }

        use imageproc::point::Point;
        use std::fmt::Write;
        write!(pop, "<map name='image-map'>\n").unwrap();
        for (pv, rg, _av) in &pv_rgs {
            let _lv = -1;
            //println!("{} av:{} mn:{} mx:{}", pv, av, avmn, avmx);
            for pg in rg {
                if pg.len()<2 { continue; }
                let mut pli = Vec::<Point<i32>>::new();
                //let mut ply = Vec::<Point<f32>>::new();
                for i in 0..pg.len()-1 {
                    let pp = &pg[i];
                    let pn = (pp.0 as f32, pp.1 as f32);
                    let x = xo + (pn.0 - x0) / pnw * fmw + ofx;
                    let y = yo + hy - (pn.1 - y0) / pnw * fmw + ofy;
                    //ply.push(Point{x,y});
                    let x = x as i32;
                    let y = y as i32;
                    if pli.len()>0 && pli[0].x==x && pli[0].y==y {
                    } else {
                        pli.push(Point{x,y});
                    }
                }
                if pli.len()>5 {
                    let mut cos = String::new();
                    for pn in &pli {
                        if cos.len()>0 { write!(cos, ",").unwrap(); }
                        write!(cos, "{},{}", pn.x, pn.y).unwrap();
                    }
                    write!(pop, "<area target='GMAP' alt='A-{}' title='{}' href='/p54/pv_pg_sub_map/{}' coords='{}' shape='poly'>\n", pv, pv, pv, cos).unwrap();
                }
            }
        }
        write!(pop, "</map>\n").unwrap();
    }
    pop
}

/////////////////////////////////////////////////////////
///////////////////////////////////////////////////////

#[derive(Template, Debug, Default)]
#[template(path = "prc5/th_pg_map.html", escape = "none")]
pub struct ThaiPageMapTemp {
    pub title: String,
    pub pop: String,
}

pub async fn th_pg_map() -> impl axum::response::IntoResponse { 
    let mut tmp = ThaiPageMapTemp::default();
    tmp.title = "พื้นที่ 74 จังหวัด".to_string();
    tmp.pop = th_pop_map_gen();
    tmp
}


///////////////////////////////////////////////////////
pub async fn prc54() -> Result<(), Box<dyn std::error::Error>> {
    let _prv = pv_sub_init();
    let _base = crate::sg::ldp::base();
    let app = Router::new()
        .route("/p54/", get(prvs_pg))
        .route("/p54/pv_dw/:pvnm", get(pv_dw))
        .route("/p54/pv_dw2/:pvnm", get(pv_dw2))
        .route("/p54/pv/:pvnm", get(pv_dw2))
        .route("/p54/pv_pg_ym/:pvnm/:ym", get(pv_pg_ym))
        .route("/p54/pv_png_ym/:pvnm/:ym", get(pv_png_ym))
        .route("/p54/sb_pg_ym/:pvnm/:ym", get(sb_pg_ym))
        .route("/p54/sb_png_ym/:pvnm/:ym", get(sb_png_ym))
        .route("/p54/pv_pg_sub_map/:pvnm", get(pv_pg_sub_map))
        .route("/p54/pv_png_sub_map/:pvnm", get(pv_png_sub_map))
        .route("/p54/fd_pg_ym/:fdid/:ym", get(fd_pg_ym))
        .route("/p54/fd_png_ym/:fdid/:ym", get(fd_png_ym))
        .route("/p54/fd_pg_map/:fdid", get(fd_pg_map))
        .route("/p54/fd_png_map/:fdid", get(fd_png_map))
        .route("/p54/tr_pg_map/:fdid/:trid", get(tr_pg_map))
        .route("/p54/tr_png_map/:fdid/:trid", get(tr_png_map))
        .route("/p54/th_png_map", get(th_png_map))
        .route("/p54/th_pg_map", get(th_pg_map))
        .route("/p54/ev_png_map/:yr", get(crate::sg::prc7::ev_png_map))
        .route("/p54/ev_pg_map/:yr", get(crate::sg::prc7::ev_pg_map))
        .route("/p54/re_png_map/:yr", get(crate::sg::re_pg_map::re_png_map))
        .route("/p54/re_pg_map/:yr", get(crate::sg::re_pg_map::re_pg_map))
        .route("/p54/fe_tr_pg_map/:fdid/:trid", get(crate::sg::fe_tr_map::fe_tr_pg_map))
        .route("/p54/fe_tr_png_map/:fdid/:trid", get(crate::sg::fe_tr_map::fe_tr_png_map))
        ;
    
    let lisn = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(lisn, app).await.unwrap();
    Ok(())
}


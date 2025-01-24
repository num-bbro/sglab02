use crate::sg::prc5::sub_loc;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, /*draw_text_mut*/};
use imageproc::rect::Rect;
//use rusttype::{Font, Scale};
use askama::Template;
//use askama_axum::IntoResponse;
//use std::collections::HashMap;
use ab_glyph::FontVec;
use ab_glyph::PxScale;
//use crate::sg::prc5::pv_rg_map;
//use crate::sg::prc5::prv_calc;
//use crate::sg::gis1::DbfVal;
use std::fs;
use std::fs::File;
use std::io::Read;
//use crate::sg::wk5::EvDistCalc;
//use crate::sg::load::load_pvcamp;
use axum::extract::Path;
use crate::sg::prc5::Transformer1;
use crate::sg::prc6::ld_p62_fd_trans;
use crate::sg::mvline::utm_latlong;

/////////////////////////////////////////////////////////
///////////////////////////////////////////////////////

#[derive(Template, Debug, Default)]
#[template(path = "prc5/fe_tr_map.html", escape = "none")]
pub struct FeedPageMapTemp {
    pub title: String,
    pub fdid: String,
    pub trid: String,
    pub trans: Vec<Transformer1>,
    pub pop: String,
}

use crate::sg::prc6::ld_p63_fd_tr_lo;
pub async fn fe_tr_pg_map(Path((fdid,trid)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let mut tmp = FeedPageMapTemp::default();
    tmp.title = fdid.to_string();
    tmp.fdid = fdid.to_string();
    tmp.trid = trid.to_string();
    tmp.pop = fe_tr_map_pop_gen(&fdid);
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

pub async fn fe_tr_png_map(Path((fdid,trid)): Path<(String,String)>) -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, fe_tr_map_png_gen(fdid, trid), )
}

use imageproc::drawing::draw_hollow_circle_mut;

fn fe_tr_map_png_gen(fdid: String, trid: String) -> Vec<u8> {
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
        let pn = (fo.x as f32, fo.y as f32, fo.facility_id.to_string());
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
        let red = Rgb([130u8, 0u8, 0u8]);
        let cols = vec![
            Rgb([0u8, 130u8, 0u8]),
            Rgb([0u8, 0u8, 130u8]),
            Rgb([130u8, 130u8, 0u8]),
            Rgb([0u8, 130u8, 130u8]),
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

            if pn.2==trid {
                draw_line_segment_mut(&mut image, (xx-6.0, yy-6.0), (xx+6.0, yy+6.0), red);
                draw_line_segment_mut(&mut image, (xx-6.0, yy+6.0), (xx+6.0, yy-6.0), red);
            }

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

fn fe_tr_map_pop_gen(fdid: &String) -> String {
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
            let lk = format!("../../fe_tr_pg_map/{fdid}/{}", pn.2);
            //println!("lk {} - {}", pn.2, lk);
            let co = format!("{},{},{},{}",xi-5, yi-5, xi+5, yi+5);
            write!(pop, "<area target='GMAP' alt='A-{}' title='เลขหม้อแปลง\n {}' href='{}' coords='{}' shape='rect'>\n", pn.2, pn.2, lk, co).unwrap();
        }
        write!(pop, "</map>\n").unwrap();
    }
    pop
}



use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, /*draw_text_mut*/};
use imageproc::rect::Rect;
//use rusttype::{Font, Scale};
use askama::Template;
//use askama_axum::IntoResponse;
use std::collections::HashMap;
use ab_glyph::FontVec;
use ab_glyph::PxScale;
use crate::sg::prc5::pv_rg_map;
use crate::sg::prc5::prv_calc;
//use crate::sg::gis1::DbfVal;
use std::fs;
use std::fs::File;
use std::io::Read;
//use crate::sg::wk5::EvDistCalc;
//use crate::sg::load::load_pvcamp;
use axum::extract::Path;

/////////////////////////////////////////////////////////////
fn re_png_map_gen(yr: String) -> Vec<u8> {
    let dir = format!("{}/re_map", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&dir);
    let file = format!("{}/re_map.png", dir);

    //////////////////////////////////////////// GET DATA - START
    let mut yrid = 2040;
    if let Ok(y) = yr.parse::<i32>() {
        yrid = y + 543;
    }
    //println!("pop year {}", yrid);

    let mut pv_re_mp =  HashMap::<String, Vec::<(String, i32, f32)>>::new();
    let newre = ld_replan();
    let sbinf = sub_inf(); //HashMap<String, SubstInfo>
    for rep in &newre {
        if let Some(sbif) = sbinf.get(&rep.sbid) {
            //println!("{} {} {} {}", sbif.prov, rep.year, rep.cate, rep.pwmw);
            if let (Ok(pw),Ok(yr)) = (rep.pwmw.parse::<f32>(), rep.year.parse::<i32>()) {
                //println!("C1: {}  pw: {} {}", sbif.prov, pw, yr);
                if let Some(rev) = pv_re_mp.get_mut(&sbif.prov) {
                    rev.push((rep.sbid.to_string(), yr, pw));
                } else {
                    pv_re_mp.insert(sbif.prov.to_string(), vec![(rep.sbid.to_string(), yr, pw)]);
                }
            }
        }
    }

    let mut pv_pw_mp =  HashMap::<String, f32>::new();
    let mut bfst = true;
    let (mut pwmx, mut pwmn) = (0.0, 0.0, );
    for (pv, rev) in &pv_re_mp {
        let mut pw = 0.0;
        for (_,y,p) in rev {
            if *y<=yrid {
                pw += p;
            }
        }
        if pw==0.0 { continue; }
        pv_pw_mp.insert(pv.to_string(), pw);
        //println!("{} - {}", pv, pw);
        if bfst {
            pwmx = pw;
            pwmn = pw;
            bfst = false;
        } else {
            if pw>pwmx { pwmx = pw; }
            if pw<pwmn { pwmn = pw; }
        }
    }
    //////////////////////////////////////////// GET DATA - END

    let (ww, hh, mx, my) = (400,600, 25, 25);
    let mut image = RgbImage::new(ww, hh);
    let wht = Rgb([255u8, 255u8, 255u8]);
    let _grn = Rgb([0u8, 130u8, 0u8]);
    let blk = Rgb([0u8, 0u8, 0u8]);
    let font_vec = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
    let _font = FontVec::try_from_vec(font_vec).expect("Font Vec");
    let _uniform_scale_24px = PxScale::from(24.0);

    let mut pv_rgs = Vec::<(String, Vec::<Vec::<(f64, f64)>>, f32)>::new();
    let (mut avmx, mut avmn) = (0f32, 0f32);
    let mut b_fst = true;
    //let mut cnt = 0usize;
    for (pv,rg) in pv_rg_map() {
        let /*mut*/ pwavg = 0f32;
        if let Some(calc) = prv_calc().get(pv) {
            let va = calc.year_load.power_quality.pos_avg;
            //println!("{} va: {}", pv, va);
            if b_fst {
                avmx = va;
                avmn = va;
                b_fst = false;
            } else {
                if va>avmx { avmx = va; }
                if va<avmn { avmn = va; }
            }
            //cnt += 1;
        }
        pv_rgs.push((pv.to_string(), rg.clone(), pwavg));
    }

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

        // find color shades
        const CNT : usize = 16;
        let mut cols = [Rgb([0u8, 0u8, 0u8]); CNT];
        for i in 0..CNT/4 {
            let i0 = i;
            let co = i0 as f32 * 260.0 / CNT as f32 * 4.0;
            cols[i] = Rgb([0u8, co as u8, 255u8]);
            //println!("col1 {} : {} : {:?} : {}", i, i0, cols[i], co);
        }
        for i in CNT/4..CNT/2 {
            let i0 = i-CNT/4;
            let co = (CNT/4-1 - i0) as f32 * 260.0 / CNT as f32 * 4.0;
            cols[i] = Rgb([0u8, 255u8, co as u8]);
            //println!("col2 {} : {} : {:?}", i, i0, cols[i]);
        }
        for i in CNT/2..CNT*3/4 {
            let i0 = i-CNT/2;
            let co = (i0+1) as f32 * 260.0 / CNT as f32 * 4.0;
            cols[i] = Rgb([co as u8, 255u8, 0u8]);
            //println!("col3 {} : {} : {:?}", i, i0, cols[i]);
        }
        for i in CNT*3/4..CNT {
            let i0 = i-CNT*3/4;
            let co = (CNT/4-1 - i0) as f32 * 260.0 / CNT as f32 * 4.0;
            cols[i] = Rgb([255u8, co as u8, 0u8]);
            //println!("col1 {} : {} : {:?}", i, i0, cols[i]);
        }

        for (pv, rg, _av) in &pv_rgs {
            let mut co = wht;
            if let Some(pw) = pv_pw_mp.get(pv) {
                if pwmx>pwmn {
                    let pc = (pw - pwmn) / (pwmx - pwmn);
                    if pc>=1.0 {
                        co = cols[CNT-1];
                    } else {
                        co = cols[(pc * CNT as f32) as usize]
                    }
                    //println!("{} - {} - pc: {}", pv, pw, pc);
                }
            }

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
                    draw_polygon_mut(&mut image, &pli, co);
                }
                draw_hollow_polygon_mut(&mut image, &ply, blk);
            }
        }
    }
    if let Ok(_) = image.save(&file) {}
    let mut f = File::open(&file).expect("no file found");
    let metadata = fs::metadata(&file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    
    buffer
}

pub async fn re_png_map(Path(yr): Path<String>) -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, re_png_map_gen(yr), )
}


use crate::sg::imp::ld_replan;
use crate::sg::prc5::sub_inf;

/////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct MapPopInfo {
    pop: String,
    pwto: f32,
    pwmn: f32,
    pwmx: f32,
    ppno: usize,
}

fn re_pop_map_gen(yr: String) -> MapPopInfo {

    let mut yrid = 2040;
    if let Ok(y) = yr.parse::<i32>() {
        yrid = y + 543;
    }
    println!("pop year {}", yrid);

    let mut pv_re_mp =  HashMap::<String, Vec::<(String, i32, f32)>>::new();
    let newre = ld_replan();
    let sbinf = sub_inf(); //HashMap<String, SubstInfo>
    for rep in &newre {
        if let Some(sbif) = sbinf.get(&rep.sbid) {
            //println!("{} {} {} {}", sbif.prov, rep.year, rep.cate, rep.pwmw);
            if let (Ok(pw),Ok(yr)) = (rep.pwmw.parse::<f32>(), rep.year.parse::<i32>()) {
                //println!("C1: {}  pw: {} {}", sbif.prov, pw, yr);
                if let Some(rev) = pv_re_mp.get_mut(&sbif.prov) {
                    rev.push((rep.sbid.to_string(), yr, pw));
                } else {
                    pv_re_mp.insert(sbif.prov.to_string(), vec![(rep.sbid.to_string(), yr, pw)]);
                }
            }
        }
    }

    let mut pv_pw_mp =  HashMap::<String, f32>::new();
    let mut bfst = true;
    let (mut pwmx, mut pwmn) = (0.0, 0.0, );
    let mut pwto = 0.0;
    let mut ppno = 0;
    for (pv, rev) in &pv_re_mp {
        let mut pw = 0.0;
        for (_,y,p) in rev {
            if *y<=yrid {
                pw += p;
                ppno += 1;
            }
        }
        if pw==0.0 { continue; }
        pwto += pw;
        pv_pw_mp.insert(pv.to_string(), pw);
        //println!("prv sum : {} - {}", pv, pw);
        if bfst {
            pwmx = pw;
            pwmn = pw;
            bfst = false;
        } else {
            if pw>pwmx { pwmx = pw; }
            if pw<pwmn { pwmn = pw; }
        }
    }

    let mut pop = String::new();
    let (ww, hh, mx, my) = (400,600, 25, 25);
    let mut pv_rgs = Vec::<(String, Vec::<Vec::<(f64, f64)>>, f32)>::new();
    let (mut avmx, mut avmn) = (0f32, 0f32);
    let mut b_fst = true;
    //let mut cnt = 0usize;
    for (pv,rg) in pv_rg_map() {
        let /*mut*/ pwavg = 0f32;
        if let Some(calc) = prv_calc().get(pv) {
            let va = calc.year_load.power_quality.pos_avg;
            //println!("{} va: {}", pv, va);
            if b_fst {
                avmx = va;
                avmn = va;
                b_fst = false;
            } else {
                if va>avmx { avmx = va; }
                if va<avmn { avmn = va; }
            }
            //cnt += 1;
        }
        pv_rgs.push((pv.to_string(), rg.clone(), pwavg));
    }
    //println!("{} {} {}", avmx, avmn, cnt);

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

        //let evgrw = car_reg_2023_c();

        write!(pop, "<map name='image-map'>\n").unwrap();
        for (pv, rg, _av) in &pv_rgs {
            let _lv = -1;
            //println!("{} av:{} mn:{} mx:{}", pv, av, avmn, avmx);
            let mut poptxt = String::from(pv);

            if let Some(xp) = pv_pw_mp.get(pv) {
                poptxt = format!("{}, - {:.2}mw", pv, xp.separate_with_commas());
                if let Some(rev) = pv_re_mp.get(pv) {
                    for (_s, y, p) in rev {
                        write!(poptxt, "\nกำลัง {} mw ในปี พ.ศ. {}", p.separate_with_commas(), y).unwrap();
                    }
                }
            }

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
                    write!(pop, "<area target='GMAP' alt='A-{}' title='{}' href='/pv_pg_sub_map/{}' coords='{}' shape='poly'>\n", poptxt, poptxt, pv, cos).unwrap();
                }
            }
        }
        write!(pop, "</map>\n").unwrap();
    }
    pwto = pwto.ceil();
    pwmn = pwmn.ceil();
    pwmx = pwmx.ceil();
    MapPopInfo { pop, pwto, pwmn, pwmx, ppno, }
}

/////////////////////////////////////////////////////////
///////////////////////////////////////////////////////

use thousands::Separable;

#[derive(Template, Debug, Default)]
#[template(path = "prc5/re_pg_map.html", escape = "none")]
pub struct REPageMapTemp {
    pub title: String,
    pub info: MapPopInfo,
    pub yr: String,
}

/////////////////////////////////////////////////////////////
pub async fn re_pg_map(Path(yr): Path<String>) -> impl axum::response::IntoResponse { 
    let mut tmp = REPageMapTemp::default();
    tmp.title = "พื้นที่ 74 จังหวัด".to_string();
    let info = re_pop_map_gen(yr.to_string());
    tmp.info = info;
    tmp.yr = yr;
    tmp
}


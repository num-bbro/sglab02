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
use crate::sg::wk5::EvDistCalc;
use crate::sg::load::load_pvcamp;
use axum::extract::Path;

/////////////////////////////////////////////////////////////
fn ev_png_map_gen(_yr: String) -> Vec<u8> {
    let ev_dir = format!("{}/ev_map", crate::sg::imp::data_dir());
    let _ = std::fs::create_dir_all(&ev_dir);
    let ev_file = format!("{}/ev_map.png", ev_dir);

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

        // find min max of data
        let evgrw = car_reg_2023_c();
        let (mut pcmx, mut pcmn) = (0f32,0f32);
        for (_pv,evds) in &evgrw {
            pcmx = evds.ev_pc;
            pcmn = evds.ev_pc;
        }
        for (_pv,evds) in &evgrw {
            if evds.ev_pc>pcmx { pcmx = evds.ev_pc; }
            if evds.ev_pc<pcmn { pcmn = evds.ev_pc; }
        }
        //println!("pc {}-{}", pcmn, pcmx);

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
            if let Some(evds) = evgrw.get(pv) {
                let sc = (evds.ev_pc - pcmn) / (pcmx - pcmn);
                let ii = CNT as f32 * sc;
                let ii = if ii==CNT as f32 { CNT - 1 } else { ii as usize };
                co = cols[ii];
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
    if let Ok(_) = image.save(&ev_file) {}
    let mut f = File::open(&ev_file).expect("no file found");
    let metadata = fs::metadata(&ev_file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");    
    buffer
}

pub async fn ev_png_map(Path(yr): Path<String>) -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"image/png")
    );
    ( headers, ev_png_map_gen(yr), )
}


pub fn ev_2023_new() -> f32 { 75690.0f32 }
//pub fn ev_2023_acc() -> f32 { 89907.0f32 }

/////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct MapPopInfo {
    pop: String,
    pv_ev_la_yr0: f32,
    pv_ev_ac_no0: f32,
    pv_ev_mw0: f32,
    pv_ev_mwh0: f32,
    pv_et_la_yr0: f32,
    pv_et_ac_no0: f32,
    pv_et_mw0: f32,
    pv_et_mwh0: f32,
}

fn ev_pop_map_gen(yr: String) -> MapPopInfo {
    let mut yrid = 2040;
    if let Ok(y) = yr.parse::<i32>() {
        yrid = y;
    }
    //println!("pop year {}", yrid);

    let mut pop = String::new();
    let (ww, hh, mx, my) = (400,600, 25, 25);
    let mut pv_rgs = Vec::<(String, Vec::<Vec::<(f64, f64)>>, f32)>::new();
    let (mut avmx, mut avmn) = (0f32, 0f32);
    let mut b_fst = true;
    //let mut _cnt = 0usize;
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
           // cnt += 1;
        }
        pv_rgs.push((pv.to_string(), rg.clone(), pwavg));
    }
    //println!("{} {} {}", avmx, avmn, cnt);

    let (mut pv_ev_la_yr0, mut pv_ev_ac_no0, mut pv_ev_mw0, mut pv_ev_mwh0) = (0f32,0f32,0f32,0f32,);
    let (mut pv_et_la_yr0, mut pv_et_ac_no0, mut pv_et_mw0, mut pv_et_mwh0) = (0f32,0f32,0f32,0f32,);

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

        let evgrw = car_reg_2023_c();
        
        let (ev_rt0,ev_gw0) = (0.1,0.007);
        let (et_rt0,et_gw0) = (0.2,0.005);
    
        let ev_mw = 0.007; // mw
        let ev_dy_hr = 3.0;
        let et_mw = 0.150; // mw
        let et_dy_hr = 6.0;

        let ev_ls_yr = 75690.0;
        let ev_ac_no = 89907.0 + ev_ls_yr;
    
        let et_ls_yr = 238.0 * 3.0;
        let et_ac_no = 2962.0 + et_ls_yr;

        /*
        write!(poptxt, "\nรถยนต์ไฟฟ้าใหม่: {} คัน", pv_ev_la_yr.ceil());
        write!(poptxt, "\nรถยนต์สะสม: {} คัน", pv_ev_ac_no.ceil());
        write!(poptxt, "\nกำลังอัดประจุ: {} mw", .ceil());
        write!(poptxt, "\nพลังงานต่อปี: {} mwh", pv_ev_mwh.ceil());
        write!(poptxt, "\nรถบรรทุก+บัสใหม่: {} คัน", pv_et_la_yr.ceil());
        write!(poptxt, "\nบรรทุก+บัสสะสม: {} คัน", pv_et_ac_no.ceil());
        write!(poptxt, "\nกำลังอัดประจุ: {} mw", pv_et_mw.ceil());
        write!(poptxt, "\nพลังงานต่อปี: {} mwh", pv_et_mwh.ceil());
        */

        write!(pop, "<map name='image-map'>\n").unwrap();
        for (pv, rg, _av) in &pv_rgs {
            let _lv = -1;
            //println!("{} av:{} mn:{} mx:{}", pv, av, avmn, avmx);

            let mut poptxt = String::from(pv);
            if let Some(v) = evgrw.get(pv) {
                let pc = v.ev_pc * 100.0;
                let evds = ev_2023_new() * v.ev_pc;
                let evds = evds as usize;
                poptxt = format!("{} - {:.2}%\n{} คัน", pv, pc, evds);

                let mut pv_ev_ac_no = ev_ac_no * v.ev_pc;
                let mut pv_ev_la_yr = ev_ls_yr * v.ev_pc;
                let mut ev_rt = ev_rt0;

                let mut pv_et_ac_no = et_ac_no * v.ev_pc;
                let mut pv_et_la_yr = et_ls_yr * v.ev_pc;
                let mut et_rt = et_rt0;

                for y in 2024..=yrid {
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

                    if y==yrid {
                        poptxt = format!("{} - {:.2}%", pv, pc);
                        write!(poptxt, "\nรถยนต์ไฟฟ้าใหม่: {} คัน", pv_ev_la_yr.ceil().separate_with_commas()).unwrap();
                        write!(poptxt, "\nรถยนต์สะสม: {} คัน", pv_ev_ac_no.ceil().separate_with_commas()).unwrap();
                        write!(poptxt, "\nกำลังอัดประจุ: {} mw", pv_ev_mw.ceil().separate_with_commas()).unwrap();
                        write!(poptxt, "\nพลังงานต่อปี: {} mwh", pv_ev_mwh.ceil().separate_with_commas()).unwrap();
                        write!(poptxt, "\nรถบรรทุก+บัสใหม่: {} คัน", pv_et_la_yr.ceil().separate_with_commas()).unwrap();
                        write!(poptxt, "\nบรรทุก+บัสสะสม: {} คัน", pv_et_ac_no.ceil().separate_with_commas()).unwrap();
                        write!(poptxt, "\nกำลังอัดประจุ: {} mw", pv_et_mw.ceil().separate_with_commas()).unwrap();
                        write!(poptxt, "\nพลังงานต่อปี: {} mwh", pv_et_mwh.ceil().separate_with_commas()).unwrap();
                        
                        pv_ev_la_yr0 += pv_ev_la_yr;
                        pv_ev_ac_no0 += pv_ev_ac_no;
                        pv_ev_mw0 += pv_ev_mw;
                        pv_ev_mwh0 += pv_ev_mwh;
                        pv_et_la_yr0 += pv_et_la_yr;
                        pv_et_ac_no0 += pv_et_ac_no;
                        pv_et_mw0 += pv_et_mw;
                        pv_et_mwh0 += pv_et_mwh;

                    }
                    
                }
                
                //println!("{}", poptxt);
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

    pv_ev_la_yr0 = pv_ev_la_yr0.ceil();
    pv_ev_ac_no0 = pv_ev_ac_no0.ceil();
    pv_ev_mw0 = pv_ev_mw0.ceil();
    pv_ev_mwh0 = pv_ev_mwh0.ceil();
    pv_et_la_yr0 = pv_et_la_yr0.ceil();
    pv_et_ac_no0 = pv_et_ac_no0.ceil();
    pv_et_mw0 = pv_et_mw0.ceil();
    pv_et_mwh0 = pv_et_mwh0.ceil();

    MapPopInfo { 
        pop,
        pv_ev_la_yr0,
        pv_ev_ac_no0,
        pv_ev_mw0,
        pv_ev_mwh0,
        pv_et_la_yr0,
        pv_et_ac_no0,
        pv_et_mw0,
        pv_et_mwh0,
    }
}

/////////////////////////////////////////////////////////
///////////////////////////////////////////////////////
use thousands::Separable;

#[derive(Template, Debug, Default)]
#[template(path = "prc5/ev_pg_map.html", escape = "none")]
pub struct EVPageMapTemp {
    pub title: String,
    pub info: MapPopInfo,
    pub yr: String,
}

pub async fn ev_pg_map(Path(yr): Path<String>) -> impl axum::response::IntoResponse { 
    let mut tmp = EVPageMapTemp::default();
    tmp.title = "พื้นที่ 74 จังหวัด".to_string();
    tmp.info = ev_pop_map_gen(yr.to_string());
    tmp.yr = yr;
    tmp
}


fn pv_adjust_c() -> Vec::<(&'static str, f64, f64)> {
    vec![
        ("ชลบุรี", 1.4, 0.0,),
        ("ระยอง", 4.5, 0.0,),
        ("ฉะเชิงเทรา", 5.0, 0.0,),
        ("ปราจีนบุรี", 5.0, 0.0,), // 7.0
        ("นครปฐม", 5.0, 0.0,), // 6.0
        ("ภูเก็ต", 0.0, 3.0,),
        ("สมุทรสาคร", 4.0, 0.0,),
        ("พระนครศรีอยุธยา", 5.0, 0.0,),
        ("ปทุมธานี", 13.0, 0.0,),
        ("กรุงเทพมหานคร", 0.0, 30.0,),
        ("นนทบุรี", 0.0, 25.0,),
        ("สมุทรปราการ", 0.0, 15.0,),
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
        ("บุรีรัมย์", 0.0, 0.0,), // 1.5

        ("ตรัง", 0.0, 50.0,),
        ("ยะลา", 0.0, 80.0,),
        ("นราธิวาส", 0.0, 80.0,),
        ("ปัตตานี", 0.0, 80.0,),
        ("สกลนคร", 0.0, 80.0,),
        ("กาฬสินธุ์", 0.0, 80.0,),
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
    ]
}

fn car_reg_2023_c() -> HashMap<String,EvDistCalc> {
    let mut pv_ca_mp = load_pvcamp();
    let mut pv_ca_mp2 = HashMap::new();
    //let mut _cnt0 = 0.0;
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
    //let mut _ass_sm = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let _ts = adx.0.to_string();
        if let Some(cn) = pv_ca_mp2.get_mut(&adx.0.to_string()) {
            let ad = tk0 * ev_adx[i].1 / 100.0;
            //ass_sm += ev_adx[i].1;
            *cn += ad;
        }
    }
    
    //println!("assign %{}", ass_sm);

    let mut pv_car_reg_mp = HashMap::<String,EvDistCalc>::new();
    let mut total = 0.0f32;
    for (k, v) in &pv_ca_mp2 {
        if ["กรุงเทพมหานคร","นนทบุรี","สมุทรปราการ"].contains(&k.as_str()) {
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

    pv_car_reg_mp
}

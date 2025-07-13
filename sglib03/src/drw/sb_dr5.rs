use ab_glyph::FontVec;
use ab_glyph::PxScale;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;
use std::fs;
use std::fs::File;
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
        let ymx = lp.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
        let ymx = ymx.unwrap();
        let dy = (*ymx as i32 + 4) / 5;
        let dy = dy / 5 * 5;
        //println!("tik:{dy} ymx:{:.1}", *ymx);
        let mut tiks = vec![-10f32];
        let mut tk = 0;
        tiks.push(tk as f32);
        for _i in 0..10 {
            tk += dy;
            tiks.push(tk as f32);
            if tk as f32 > *ymx {
                break;
            }
        }
        //println!("{:?}", tiks);
        tiks
        //return vec![-25f32, 0f32, 25f32, 50f32, 75f32, 100f32];
        //vec![-20f32, 0f32, 20f32, 40f32, 60f32, 80f32, 100f32]
        //vec![-10f32, 0f32, 20f32, 40f32, 60f32, 80f32]
        //vec![-5f32, 0f32, 5f32, 10f32, 15f32, 20f32]
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
        let font_vec = Vec::from(include_bytes!("../THSarabunNew.ttf") as &[u8]);
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

            //draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), blk);
        }

        let col = self.grn();
        for (di, v) in lpv.iter().enumerate() {
            let di = di % 48;
            let xi = di as f32 * wdrt + ml as f32;
            let yi = ory as f32 - v * hgrt;
            let x2 = xi + wdrt; //10f32;
            let _dy = v * hgrt;
            /*
            let (ox, oy, sx, sy) = if *v > 0f32 {
                (xi as i32, yi as i32, (wdrt + 1.0f32) as u32, dy as u32)
            } else {
                (xi as i32, ory as i32, (wdrt + 1.0f32) as u32, -dy as u32)
            };
            //draw_filled_rect_mut(&mut image, Rect::at(ox, oy).of_size(sx, sy), col);
            */

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

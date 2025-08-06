use askama::Template;
use axum;
use axum::routing::get;
use axum::Router;
use num_traits::Pow;
use sglib03::p_31::AreaRatio;
use sglib03::prc4::SubBenInfo;

use std::error::Error;
use std::io::Read;
use std::io::Write;
use zip::write::SimpleFileOptions;

pub const WEBROOT: &str = "";

pub const INFLATION_RATE: f32 = 0.03;
pub const TRANS_FAIL_REDUCE_RATE: f32 = 0.02;
pub const OP_YEAR_START: u32 = 2028;
pub const OP_YEAR_END: u32 = 2039;

use sglab02_lib::sg::prc3::ld_p3_prv_sub;
use sglab02_lib::sg::prc3::ld_p3_sub_inf;
use sglab02_lib::sg::prc5::prvs;
use std::io;

use thousands::Separable;

#[derive(Template, Debug, Default)]
#[template(path = "web1.html", escape = "none")]
pub struct Web1 {
    pub prvs: Vec<(String, Vec<(String, String)>)>,
}
pub async fn web1() -> impl axum::response::IntoResponse {
    let pv = prvs();
    let pvsb = ld_p3_prv_sub();
    let mut prvs = Vec::<(String, Vec<(String, String)>)>::new();
    //let sbsl = ld_pv_sbv_m();
    let sbif = ld_p3_sub_inf();
    for p in pv {
        let pp = p.to_string();
        if let Some(sbv) = pvsb.get(&pp) {
            let mut psbv = Vec::<(String, String)>::new();
            for sb in sbv {
                if let Some(sbif) = sbif.get(sb) {
                    psbv.push((sb.to_string(), sbif.name.to_string()));
                }
            }
            prvs.push((pp, psbv));
        }
    }

    Web1 { prvs }
}

use axum::extract::Path;
#[derive(Template, Debug, Default)]
#[template(path = "sub1.html", escape = "none")]
pub struct Sub1 {
    pub msg: Vec<String>,
}
pub async fn sub1(Path(sb): Path<String>) -> impl axum::response::IntoResponse {
    let mut msg = Vec::new();
    let _ = sb_docx_gen(sb, &mut msg).is_ok();
    Sub1 { msg }
}
pub async fn run() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route(&format!("{}{}", WEBROOT, "/sub1/:sb"), get(sub1))
        .route(&format!("{}{}", WEBROOT, "/web1"), get(web1))
        .route(&format!("{}/", WEBROOT), get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("http://localhost:3000/web1");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

use crate::prc41::ld_sb_tr0;
use docx_rs::*;
use std::collections::HashMap;

const TABSHADE: &str = "#ecffe6";
pub const CELL_MARGIN: i32 = 100;

fn tr_gen(hd: Vec<String>, dt: Vec<Vec<String>>, wd: Vec<usize>, al: Vec<AlignmentType>) -> Table {
    let mut rows = Vec::<TableRow>::new();
    let mut hdrw = Vec::<TableCell>::new();
    for i in 0..10 {
        if i < hd.len() {
            let mut rr = Run::new();
            rr = rr.add_text(hd[i].to_string());
            rr = rr.fonts(RunFonts::new().cs(THFONT));
            rr = rr.size(TX_SZ);
            let mut pa = Paragraph::new();
            pa = pa.add_run(rr);
            pa = pa.align(AlignmentType::Center);
            let mut ce = TableCell::new();
            ce = ce.add_paragraph(pa);
            ce = ce.shading(Shading::new().fill(TABSHADE));
            if i < wd.len() {
                ce = ce.width(wd[i], WidthType::Dxa);
            }
            hdrw.push(ce);
        }
    }
    rows.push(TableRow::new(hdrw));
    for rw in dt {
        let mut dtcs = Vec::<TableCell>::new();
        for i in 0..10 {
            if i < hd.len() && i < rw.len() {
                let mut rr = Run::new();
                rr = rr.add_text(rw[i].to_string());
                rr = rr.fonts(RunFonts::new().cs(THFONT));
                rr = rr.size(TX_SZ);
                let mut pa = Paragraph::new();
                pa = pa.add_run(rr);
                if i < al.len() {
                    pa = pa.align(al[i]);
                }
                pa = pa.indent(Some(CELL_MARGIN), None, Some(CELL_MARGIN), None);
                let mut ce = TableCell::new();
                ce = ce.add_paragraph(pa);
                dtcs.push(ce);
            } else {
                break;
            }
        }
        rows.push(TableRow::new(dtcs));
    }
    Table::new(rows)
}

use crate::prc42::trs_price;

pub trait AmtProj {
    fn get_amt(&self, yr: u32) -> Result<f32, Box<dyn Error>>;
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TransFailReduce {
    pub proj: Vec<(u32, f32)>,
}
impl AmtProj for TransFailReduce {
    fn get_amt(&self, yr: u32) -> Result<f32, Box<dyn Error>> {
        for (y, f) in &self.proj {
            if *y == yr {
                return Ok(*f);
            }
        }
        Err("no year data".into())
    }
}
/// tr_val : power:u32, count: u32, value (THB): u32
pub fn ben_trx(va_p: &Vec<(u32, u32, u32)>) -> TransFailReduce {
    let mut pr_p = 0u32;
    for (_, _, p) in va_p {
        pr_p += p;
    }
    let mut y_prj = Vec::<(u32, f32)>::new();
    let mut y_pr = pr_p as f32;
    for yr in OP_YEAR_START..=OP_YEAR_END {
        y_pr *= 1f32 + INFLATION_RATE as f32;
        let b1 = y_pr as f32 * TRANS_FAIL_REDUCE_RATE;
        y_prj.push((yr, b1));
    }
    TransFailReduce { proj: y_prj }
}

/// tr_val : return power:u32, count: u32, value (THB): u32
pub fn tr_val(tr_cn: &HashMap<u32, u32>) -> Vec<(u32, u32, u32)> {
    let mut vals = Vec::<(u32, u32, u32)>::new();
    let mut ks: Vec<u32> = tr_cn.clone().into_iter().map(|(k, _)| k).collect();
    ks.sort();
    for k in ks {
        let v = tr_cn[&k].clone();
        let vv = v * trs_price(k as usize) as u32;
        vals.push((k, v, vv));
    }
    vals
}

//fn tr_tab(tr_cn: HashMap<u32, u32>) -> Table {
/// create ms-word table for transformer
fn tr_tab(vals: &Vec<(u32, u32, u32)>) -> Table {
    let wd = vec![600usize, 3000usize, 1500usize, 2000usize];
    let hd = vec![
        "ลำดับ".to_string(),
        "หม้อแปลง".to_string(),
        "จำนวน (ตัว)".to_string(),
        "มูลค่า (บาท)".to_string(),
    ];
    let al = vec![
        AlignmentType::Center,
        AlignmentType::Left,
        AlignmentType::Right,
        AlignmentType::Right,
    ];
    let mut cn = 0;

    //let vals = tr_val(&tr_cn);
    let mut dt = Vec::<Vec<String>>::new();
    for (s, c, v) in vals {
        cn += 1;
        let cc = c.separate_with_commas();
        let vv = v.separate_with_commas();
        let rw = vec![
            format!("{cn}"),
            format!(" หม้อแปลงขนาด {s} kVA"),
            format!("{cc} "),
            format!("{vv} "),
        ];
        dt.push(rw);
    }
    tr_gen(hd, dt, wd, al)
}

/*
fn tb3_tab(dt: Vec<Vec<String>>) -> Table {
    let wd = vec![1200usize, 2000usize, 2000usize];
    let hd = vec![
        "ปี พ.ศ.".to_string(),
        "จำนวนคัน".to_string(),
        "MWH".to_string(),
    ];
    let al = vec![
        AlignmentType::Center,
        AlignmentType::Right,
        AlignmentType::Right,
    ];
    tr_gen(hd, dt, wd, al)
}
*/

pub fn tb2_tab(vals: &Vec<Vec<String>>) -> Table {
    let wd = vec![3000usize, 1500usize];
    let hd = vec!["รายการ".to_string(), "จำนวน".to_string()];
    let al = vec![AlignmentType::Left, AlignmentType::Right];
    let dt = vals.clone();
    tr_gen(hd, dt, wd, al)
}

pub fn docx_adj(pin: &str, fout: &str) {
    let tdir = "temp";
    let fout = std::path::Path::new(fout);
    let fout = std::fs::File::create(fout).unwrap();
    let mut zout = zip::ZipWriter::new(fout);
    let file = std::fs::File::open(pin).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };
        if file.is_dir() {
            let dir = format!("{}", outpath.display());
            let dir0 = format!("{}/{}", tdir, outpath.display());
            //println!("dir0:{dir0}");
            std::fs::create_dir_all(&dir0).unwrap();
            zout.add_directory(&dir, SimpleFileOptions::default())
                .expect("?");
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    //fs::create_dir_all(p).unwrap();
                }
            }
            let fnm = format!("{}", outpath.display());
            let fnm0 = format!("{}/{}", tdir, outpath.display());

            let mut outfile = std::fs::File::create(&fnm0).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();

            let mut f = std::fs::File::open(&fnm0).expect("no file found");
            let metadata = std::fs::metadata(&fnm0).expect("unable to read metadata");
            let mut buff = vec![0; metadata.len() as usize];
            f.read_exact(&mut buff).expect("buffer overflow");
            if fnm == "word/document.xml" {
                let buf2 = String::from_utf8(buff.clone()).unwrap();
                let buf2 = buf2.replace("<w:rFonts", "<w:cs/><w:rFonts");
                //println!("fnm:{fnm} 1:{} 2:{}", buff.len(), buf2.len());
                buff = buf2.as_bytes().to_vec();
            }
            let options = SimpleFileOptions::default();
            zout.start_file(fnm, options).expect("?");
            zout.write_all(&buff).expect("?");
        }
    }
    zout.finish().expect("?");
}

const HD1SZ: usize = 48;
const HD2SZ: usize = 40;
const HD3SZ: usize = 36;
const TX_SZ: usize = 36;
const THFONT: &str = "TH Sarabun New";

#[allow(dead_code)]
pub fn para_h1(tx: &str) -> Paragraph {
    para(tx, "Header 1", HD1SZ, false)
}
#[allow(dead_code)]
pub fn para_h2(tx: &str) -> Paragraph {
    para(tx, "Header 2", HD2SZ, false)
}
#[allow(dead_code)]
pub fn para_h3(tx: &str) -> Paragraph {
    para(tx, "Header 3", HD3SZ, false)
}
#[allow(dead_code)]
pub fn para_nm(tx: &str) -> Paragraph {
    para(tx, "Normal", TX_SZ, false)
}
#[allow(dead_code)]
pub fn para_n1(tx: &str) -> Paragraph {
    para1(tx, "Normal", TX_SZ, false, 750)
}

#[allow(dead_code)]
pub fn page_h1(tx: &str) -> Paragraph {
    para(tx, "Header 1", HD1SZ, true)
}
#[allow(dead_code)]
pub fn page_h2(tx: &str) -> Paragraph {
    para(tx, "Header 2", HD2SZ, true)
}
#[allow(dead_code)]
pub fn page_h3(tx: &str) -> Paragraph {
    para(tx, "Header 3", HD3SZ, true)
}

pub fn para1(tx: &str, stl: &str, sz: usize, pg: bool, ind: i32) -> Paragraph {
    Paragraph::new()
        .add_run(
            Run::new()
                .add_text(tx)
                .size(sz)
                .fonts(RunFonts::new().cs(THFONT)),
        )
        .style(stl)
        .page_break_before(pg)
        .indent(
            None,
            Some(docx_rs::SpecialIndentType::FirstLine(ind)),
            None,
            None,
        )
    //.indent(Some(ind), None, None, None)
}

pub fn para(tx: &str, stl: &str, sz: usize, pg: bool) -> Paragraph {
    Paragraph::new()
        .add_run(
            Run::new()
                .add_text(tx)
                .size(sz)
                .fonts(RunFonts::new().cs(THFONT)),
        )
        .style(stl)
        .page_break_before(pg)
}

use crate::prc41::SubCalc;

pub fn tb2_gen(sbca: &SubCalc) -> Vec<Vec<String>> {
    vec![
        vec![
            "จำนวนมิเตอร์เฟส เอ".to_string(),
            format!("{} ตัว", sbca.mt_ph_a.separate_with_commas()),
        ],
        vec![
            "จำนวนมิเตอร์เฟส บี".to_string(),
            format!("{} ตัว", sbca.mt_ph_b.separate_with_commas()),
        ],
        vec![
            "จำนวนมิเตอร์เฟส ซี".to_string(),
            format!("{} ตัว", sbca.mt_ph_c.separate_with_commas()),
        ],
        vec![
            "จำนวนมิเตอร์หนึ่งเฟส".to_string(),
            format!("{} ตัว", sbca.mt_1_ph.separate_with_commas()),
        ],
        vec![
            "จำนวนมิเตอร์สามเฟส".to_string(),
            format!("{} ตัว", sbca.mt_3_ph.separate_with_commas()),
        ],
        vec![
            "หน่วยการใช้ไฟฟ้าทั้งปี".to_string(),
            format!("{} MWh", (sbca.eg_sm as u32).separate_with_commas()),
        ],
        vec![
            "หน่วยการใช้ไฟฟ้าเฟสเอทั้งปี".to_string(),
            format!("{} MWh", (sbca.eg_a as u32).separate_with_commas()),
        ],
        vec![
            "หน่วยการใช้ไฟฟ้าเฟสบีทั้งปี".to_string(),
            format!("{} MWh", (sbca.eg_b as u32).separate_with_commas()),
        ],
        vec![
            "หน่วยการใช้ไฟฟ้าเฟสซีทั้งปี".to_string(),
            format!("{} MWh", (sbca.eg_c as u32).separate_with_commas()),
        ],
    ]
}

use sglab02_lib::sg::prc1::SubstInfo;
use sglib03::p_31::ld_sb_eb_proj;
use sglib03::p_31::ld_sb_et_proj;
use sglib03::p_31::ld_sb_ev_proj;

pub const DOCX0_PATH: &str = "./out/docx0";
pub const DOCX_PATH: &str = "./out/docx";
pub const PDF_PATH: &str = "./out/pdf";

pub fn sub_info1(sf: &SubstInfo, sbtr: &SubCalc, mut docx: Docx) -> Docx {
    let tb2 = tb2_gen(sbtr);
    let tb2 = tb2_tab(&tb2);
    let tb2 = tb2.indent(DOC_TB_INDENT);
    let tt = format!("สถานีไฟฟ้าย่อย {} {}", sf.name, sf.sbid);
    docx = docx.add_paragraph(page_h1(&tt));
    docx = docx.add_table(tb2);
    docx = docx.add_paragraph(para_nm(""));
    docx
}

pub fn sub_info2(sf: &SubstInfo, tp: &str, ben: &SubBenInfo, mut docx: Docx) -> Docx {
    let tt = format!("ประเภทของสถานีไฟฟ้า {tp}");
    docx = docx.add_paragraph(para_nm(&tt));
    let mvxn = format!(" {}", sf.mvxn);
    docx = docx.add_paragraph(para_nm(&mvxn));
    let ep = format!("ปริมาณปริมาณพลังงานบวก {} MWh", ben.p_en);
    docx = docx.add_paragraph(para_nm(&ep));
    docx = docx.add_paragraph(para_nm(""));
    docx
}

pub const DOC_TB_INDENT: i32 = 500;

pub fn sub_info_p(ms: &str, va_p: &Vec<(u32, u32, u32)>, mut docx: Docx) -> Docx {
    // ------ transformer and value of PEA
    let tr_p = tr_tab(va_p);
    let (mut cn_p, mut pr_p) = (0u32, 0u32);
    for (_, c, p) in va_p {
        cn_p += c;
        pr_p += p;
    }
    let cn_p = cn_p.separate_with_commas();
    let pr_p = pr_p.separate_with_commas();
    let tr_p = tr_p.indent(DOC_TB_INDENT);
    docx = docx.add_paragraph(page_h2(&format!("ข้อมูลหม้อแปลงจำหน่าย {ms}")));
    docx = docx.add_paragraph(para_nm("หม้อแปลงของ กฟภ มีรายละเอียดดังต่อไปนี้"));
    docx = docx.add_table(tr_p);
    docx = docx.add_paragraph(para_nm(""));
    let ms = format!("รวมจำนวนหม้อแปลง {cn_p} ตัว มูลค่ารวม {pr_p} บาท");
    docx = docx.add_paragraph(para_nm(ms.as_str()));

    docx
}

pub fn sub_ev_docx(sb_ev: &SubRatioProj, ms: &str, mut docx: Docx) -> Docx {
    let mut evre = Vec::<Vec<String>>::new();
    for (y, c, e, _p) in &sb_ev.proj {
        let yr = format!("{y}");
        let cn = c.separate_with_commas();
        let wh = (*e as u32).separate_with_commas();
        let vv = vec![yr, cn, wh];
        evre.push(vv);
    }
    docx = docx.add_paragraph(page_h2(&format!("ประมาณการความต้องการอัดประจุ{ms}")));
    docx = docx.add_paragraph(para_nm("รายละเอียดดังต่อไปนี้"));
    let wd = vec![1200usize, 2000usize, 2000usize];
    let hd = vec![
        "ปี พ.ศ.".to_string(),
        "จำนวนคัน".to_string(),
        "MWH".to_string(),
    ];
    let al = vec![
        AlignmentType::Center,
        AlignmentType::Right,
        AlignmentType::Right,
    ];
    let tb3 = tr_gen(hd, evre, wd, al);
    let tb3 = tb3.indent(DOC_TB_INDENT);
    docx = docx.add_table(tb3);
    docx = docx.add_paragraph(para_nm(""));

    docx
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SubRatioProj {
    pub proj: Vec<(u32, u32, f32, f32)>,
}

pub const EV_UNIT_PRICE: f32 = 0.4;
pub const ET_UNIT_PRICE: f32 = 0.0;
pub const EB_UNIT_PRICE: f32 = 0.4;

pub fn sub_ratio_proj(sf: &SubstInfo, sb_ev: &Vec<Vec<AreaRatio>>, up: f32) -> SubRatioProj {
    let mut proj = Vec::<(u32, u32, f32, f32)>::new();
    for v in sb_ev {
        let v1 = &v[0];
        if v1.sb == sf.sbid {
            let v2 = &v[1..];
            for ar in v2 {
                proj.push((ar.yr, ar.no as u32, ar.mwh, ar.mwh * up * 1000f32));
            }
        }
    }
    SubRatioProj { proj }
}

pub const EV_ADJ_2_0: [(&str, f32); 19] = [
    ("ระยอง", 1.0 + 1.8),
    ("ชลบุรี", 1.0 + 0.48),
    ("กระบี่", 1.0 + 0.10),
    ("สระแก้ว", 1.0 + 1.10),
    ("พระนครศรีอยุธยา", 1.0 + 0.50),
    ("ฉะเชิงเทรา", 1.0 + 0.50),
    ("สมุทรสาคร", 1.0 + 1.70),
    ("ปทุมธานี", 1.0 + 2.10),
    ("บุรีรัมย์", 1.0 + 2.30),
    ("เชียงใหม่", 1.0 + 1.86),
    ("พิษณุโลก", 1.0 + 2.00),
    ("ราชบุรี", 1.0 + 1.20),
    ("ขอนแก่น", 1.0 + 1.11),
    ("นครปฐม", 1.0 + 1.30),
    ("สงขลา", 1.0 + 1.69),
    ("นครสวรรค์", 1.0 + 1.12),
    ("นครราชสีมา", 1.0 + 3.49),
    ("ลพบุรี", 1.0 + 1.74),
    ("ภูเก็ต", 1.0 + 2.66),
];

use std::sync::OnceLock;

pub static EV_ADJ_3_0L: OnceLock<HashMap<&'static str, f32>> = OnceLock::new();
pub fn ev_adj_3() -> &'static HashMap<&'static str, f32> {
    EV_ADJ_3_0L.get_or_init(ev_adj_3_init)
}
fn ev_adj_3_init() -> HashMap<&'static str, f32> {
    let mut evadj = HashMap::<&'static str, f32>::new();
    for (pv, va) in &EV_ADJ_2_0 {
        evadj.insert(pv, *va);
    }
    evadj
}

pub fn sub_ratio_proj_2(sf: &SubstInfo, sb_ev: &Vec<Vec<AreaRatio>>, up: f32) -> SubRatioProj {
    let mut proj = Vec::<(u32, u32, f32, f32)>::new();
    let ev_adj_2h = ev_adj_3();
    for v in sb_ev {
        let v1 = &v[0];
        if v1.sb == sf.sbid {
            let v2 = &v[1..];
            for ar in v2 {
                let yr = ar.yr;
                let mut no = ar.no as u32;
                let mut mwh = ar.mwh;
                let mut prc = ar.mwh * up * 1000f32;
                if let Some(va) = ev_adj_2h.get(&sf.prov.as_str()) {
                    no = (ar.no * va) as u32;
                    mwh = ar.mwh * va;
                    prc = ar.mwh * up * va * 1000f32;
                }
                proj.push((yr, no, mwh, prc));
            }
        }
    }
    SubRatioProj { proj }
}

impl AmtProj for SubRatioProj {
    fn get_amt(&self, yr: u32) -> Result<f32, Box<dyn Error>> {
        for (y, _, _, p) in &self.proj {
            if yr == *y {
                return Ok(*p);
            }
        }
        Err("no year data".into())
    }
}

use crate::prc43::BENET;
use sglib03::prc4::ld_ben_bess1;
use sglib03::subtype::sub_type;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BenProj {
    pub proj: Vec<(u32, f32)>,
}
impl AmtProj for BenProj {
    fn get_amt(&self, yr: u32) -> Result<f32, Box<dyn Error>> {
        for (y, p) in &self.proj {
            if yr == *y {
                return Ok(*p);
            }
        }
        Err("no year data".into())
    }
}

pub fn ben_amt_proj(ben: &SubBenInfo) -> (BenProj, BenProj, BenProj, BenProj) {
    //print!("====  BESS  ");
    //println!("be_sub_save: {:?}", ben.be_sub_save);
    let be_sub_save = BenProj {
        proj: ben.be_sub_save.clone(),
    };
    //println!("be_re_diff: {:?}", ben.be_re_diff);
    let be_re_diff = BenProj {
        proj: ben.be_re_diff.clone(),
    };
    //println!("be_svg_save: {:?}", ben.be_svg_save);
    let be_svg_save = BenProj {
        proj: ben.be_svg_save.clone(),
    };
    //println!("be_en_added: {:?}", ben.be_en_added);
    let be_en_added = BenProj {
        proj: ben.be_en_added.clone(),
    };
    (be_sub_save, be_re_diff, be_svg_save, be_en_added)
}

pub const ENERGY_GRW_RATE: f32 = 0.04f32;
pub const ECO_GRW_RATE: f32 = 0.04f32;
pub const UNIT_PRICE: f32 = 4.45f32;
pub const BALANCE_RATE: f32 = 0.33;

pub fn ben_unbalan(sbtr: &SubCalc) -> BenProj {
    //print!("====  UNBALANCE  ");
    //println!("PHASE {} {} {}", sbtr.eg_a, sbtr.eg_b, sbtr.eg_c);
    let ab = (sbtr.eg_a - sbtr.eg_b).abs();
    let bc = (sbtr.eg_b - sbtr.eg_c).abs();
    let ca = (sbtr.eg_c - sbtr.eg_a).abs();
    let mn = sbtr.eg_a.min(sbtr.eg_b.min(sbtr.eg_c));
    let mx = ab.max(bc.max(ca));
    let pbst = if mn == 0f64 { 1f64 } else { (mx - mn) / mn };
    //println!("  min en:{mn} max dif:{mx}");
    let al0 = sbtr.eg_sm * pbst;
    //println!("  {} al:{al0:.2}", pbst * 100f64);
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = al0 * BALANCE_RATE as f64 * UNIT_PRICE as f64;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        //println!(" {} {be}", y + 2028);
        proj.push((y + 2028, be as f32));
        //let fa = v / Pow::pow(1f64 + r as f64, i as f64);
    }
    BenProj { proj }
}

pub const NON_TECLOSS_CAP_RATE: f32 = 0.31f32;
pub const NON_TECLOSS_IMP_RATE: f32 = 0.04f32;

pub fn ben_non_tech(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  NONTECHLOSS  ");
    //println!("ben {}", sbtr.eg_sm);
    //println!("ben {}", ben.p_en);
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = sbtr.eg_sm * NON_TECLOSS_CAP_RATE as f64 * NON_TECLOSS_IMP_RATE as f64;
        let be = be * 10f64;
        let be = be * UNIT_PRICE as f64;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        proj.push((y + 2028, be as f32));
    }
    BenProj { proj }
}

pub const SMETER_ACCU_IMPRV: f32 = 0.01f32;
pub const SMETER_BILL_IMPRV: f32 = 0.4f32;

pub fn ben_bill_accu(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  BILLACCU  ");
    //print!("bill acc {}", sbtr.eg_sm);
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = sbtr.eg_sm as f64 * SMETER_ACCU_IMPRV as f64 * SMETER_BILL_IMPRV as f64;
        let be = be * 30f64;
        let be = be * UNIT_PRICE as f64;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const CASH_FLOW_COST: f32 = 0.0569;

pub fn ben_cash_flow(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  CASHFLOW ");
    //println!("cash flow {}", sbtr.eg_sm);
    let al0 = sbtr.eg_sm;
    //let al_80 = al0 * 0.8f64;
    //let al_20 = al0 * 0.2f64;
    let dl_80 = 2.5;
    let dl_20 = 12.5;
    let dl_0 = dl_80 * 0.8f64 + dl_20 * 0.2f64;
    let dl_d = dl_0 - 2f64;
    //
    let dl_m1 = al0 * UNIT_PRICE as f64 / 365f64 * dl_d * CASH_FLOW_COST as f64;
    //println!(" delay: {dl_0} -> {dl_d} = {dl_m1}");

    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = dl_m1;
        // adjust
        let be = be * 40f64;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const DR_DEV_PLAN_RATE: f32 = 0.02f32;

pub fn ben_dr_save(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //let cap1 = 80_000_000f64 / 22_000_000f64;
    //let cap2 = 20_000_000f64 / 22_000_000f64;
    //print!("====  Demand Response ");
    let mt_1_ph = sbtr.mt_1_ph as f64 * DR_DEV_PLAN_RATE as f64;
    let mt_3_ph = sbtr.mt_3_ph as f64 * DR_DEV_PLAN_RATE as f64;
    let cap3 = mt_1_ph * 2_500f64;
    let cap4 = mt_3_ph * 4_650f64;
    let opx1 = cap3 * 0.005;
    let opx2 = cap4 * 0.005;
    let opx3 = (mt_1_ph + mt_3_ph) * 55f64 * 12f64;
    let opx4 = cap3 * 0.05f64;
    let opx5 = cap4 * 0.05f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = if y == 0 { cap3 + cap4 } else { 0f64 };
        let be = be + opx1 + opx2 + opx3 + opx4 + opx5;
        // adjust
        let be = be * 1.1f64;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const BOX_LINE_NEED_RATE: f32 = 0.05f32;
pub const BOX_LINE_UNIT_COST: f32 = 172.41f32;

pub fn ben_boxline_save(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  BOX : ");
    let boxcnt = (sbtr.mt_1_ph + sbtr.mt_3_ph) as f64 * BOX_LINE_NEED_RATE as f64;
    let boxex = boxcnt * BOX_LINE_UNIT_COST as f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = boxex;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const METER_PER_WORKER: f32 = 5825f32;
pub const WORKER_MONTH_SALARY: f32 = 35_000f32;
pub const WORKER_BONUS_MONTH: f32 = 1f32;
pub const WORKER_SAVING_RATE: f32 = 0.03f32;
pub const WORKER_SOC_SEC_RATE: f32 = 0.05f32;
pub const WORKER_REDUCE_RATE: f32 = 0.25f32;
pub const SALARY_INCR_RATE: f32 = 0.04f32;

pub fn ben_work_save(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  WORKER : ");
    let wk_cnt = (sbtr.mt_1_ph + sbtr.mt_3_ph) as f64 / METER_PER_WORKER as f64;
    //print!(" wkcn:{wk_cnt}");
    let mn_exp =
        WORKER_MONTH_SALARY as f64 * (1.0 + WORKER_SAVING_RATE + WORKER_SOC_SEC_RATE) as f64;
    let yr_exp = mn_exp * 12f64 + WORKER_MONTH_SALARY as f64 * WORKER_BONUS_MONTH as f64;
    let yr_exp = yr_exp * wk_cnt;
    //print!(" mn:{mn_exp} yr:{yr_exp}");
    let wk_redu = yr_exp * WORKER_REDUCE_RATE as f64;
    //print!(" rd:{wk_redu}");
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = wk_redu;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + SALARY_INCR_RATE as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const METER_SELLABLE_RATE: f32 = 0.33f32;
pub const M3P_SELL_PRICE: f32 = 100f32;
pub const M1P_SELL_PRICE: f32 = 50f32;

pub fn ben_sell_meter(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  SELL METER");
    let m1p = sbtr.mt_1_ph as f64 * METER_SELLABLE_RATE as f64;
    let m3p = sbtr.mt_3_ph as f64 * METER_SELLABLE_RATE as f64;
    let m1p_s = m1p * M1P_SELL_PRICE as f64;
    let m3p_s = m3p * M3P_SELL_PRICE as f64;
    let m1p_y = m1p_s / 12f64;
    let m3p_y = m3p_s / 12f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = m1p_y + m3p_y;
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const EMTR_CNT_RATIO: f32 = 0.05f32;
pub const EMTR_SWAP_RATE: f32 = 0.1f32;
pub const EMTR_REPL_RATE: f32 = 0.02f32;
pub const EMTR_1P_COST: f32 = 525f32;
pub const EMTR_3P_COST: f32 = 1_285f32;
pub const EMTR_1P_SWAP: f32 = 100f32;
pub const EMTR_3P_SWAP: f32 = 200f32;
pub const EMTR_1P_REPL: f32 = 250f32;
pub const EMTR_3P_REPL: f32 = 400f32;
pub const EMTR_COST_UP: f32 = 0.02f32;

pub fn ben_emeter(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  EMETER");
    let m1_cnt = sbtr.mt_1_ph as f64 * EMTR_CNT_RATIO as f64;
    let m3_cnt = sbtr.mt_3_ph as f64 * EMTR_CNT_RATIO as f64;
    let m1_sw_c = m1_cnt * EMTR_SWAP_RATE as f64;
    let m3_sw_c = m3_cnt * EMTR_SWAP_RATE as f64;
    let m1_sw_e = m1_sw_c * (EMTR_1P_COST + EMTR_1P_SWAP) as f64;
    let m3_sw_e = m3_sw_c * (EMTR_3P_COST + EMTR_3P_SWAP) as f64;
    let m1_rp_c = m1_cnt * EMTR_REPL_RATE as f64;
    let m3_rp_c = m3_cnt * EMTR_REPL_RATE as f64;
    let m1_rp_e = m1_rp_c * (EMTR_1P_COST + EMTR_1P_REPL) as f64;
    let m3_rp_e = m3_rp_c * (EMTR_3P_COST + EMTR_3P_REPL) as f64;
    let ex = m1_sw_e + m3_sw_e + m1_rp_e + m3_rp_e;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + EMTR_COST_UP as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const MT_READ_COST: f32 = 6.2f32;
pub const READ_COST_UP: f32 = 0.04f32;

pub fn ben_mt_read(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  READING");
    let m1_rd = sbtr.mt_1_ph as f64 * MT_READ_COST as f64 * 12f64;
    let m3_rd = sbtr.mt_3_ph as f64 * MT_READ_COST as f64 * 12f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = m1_rd + m3_rd;
        let be = be * Pow::pow(1f64 + READ_COST_UP as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const M1_DISCON_COST: f32 = 130f32;
pub const M3_DISCON_COST: f32 = 190f32;
pub const M1_DISCON_RATE: f32 = 0.004f32;
pub const M3_DISCON_RATE: f32 = 0.001f32;
pub const DISCON_COST_UP: f32 = 0.04f32;

pub fn ben_mt_disconn(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  DISCON");
    let m1_cn = sbtr.mt_1_ph as f64 * M1_DISCON_RATE as f64;
    let m3_cn = sbtr.mt_3_ph as f64 * M3_DISCON_RATE as f64;
    let m1_ex = m1_cn * M1_DISCON_COST as f64;
    let m3_ex = m3_cn * M3_DISCON_COST as f64;

    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = m1_ex + m3_ex;
        let be = be * 200f64;
        let be = be * Pow::pow(1f64 + DISCON_COST_UP as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const TOU_METER_RATIO: f32 = 0.2;
pub const TOU_SELLABLE_RATE: f32 = 0.3f32;
//const TOU_1P_RATIO: f32 = 0.74f32;
//const TOU_3P_RATIO: f32 = 0.26f32;
pub const TOU_1P_SELL_PRICE: f32 = 350f32;
pub const TOU_3P_SELL_PRICE: f32 = 857f32;

pub fn ben_tou_sell(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  SELL METER");
    let m1p = sbtr.mt_1_ph as f64 * TOU_METER_RATIO as f64 * TOU_SELLABLE_RATE as f64;
    let m3p = sbtr.mt_3_ph as f64 * TOU_METER_RATIO as f64 * TOU_SELLABLE_RATE as f64;
    let m1p_s = m1p * TOU_1P_SELL_PRICE as f64;
    let m3p_s = m3p * TOU_3P_SELL_PRICE as f64;
    let m1p_y = m1p_s / 12f64;
    let m3p_y = m3p_s / 12f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = m1p_y + m3p_y;
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const TOU_READ_COST: f32 = 18f32;
pub const TOU_COST_UP: f32 = 0.04f32;

pub fn ben_tou_read(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  TOU READ");
    let m1p = sbtr.mt_1_ph as f64 * TOU_METER_RATIO as f64 * 12f64;
    let m3p = sbtr.mt_3_ph as f64 * TOU_METER_RATIO as f64 * 12f64;
    let m1_rd = m1p as f64 * TOU_READ_COST as f64;
    let m3_rd = m3p as f64 * TOU_READ_COST as f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = m1_rd + m3_rd;
        let be = be * Pow::pow(1f64 + TOU_COST_UP as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const TOU_UPDATE_COST: f32 = 200f32;

pub fn ben_tou_update(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  TOU UPDATE");
    let m1p = sbtr.mt_1_ph as f64 * TOU_METER_RATIO as f64;
    let m3p = sbtr.mt_3_ph as f64 * TOU_METER_RATIO as f64;
    let m1_rd = m1p as f64 * TOU_UPDATE_COST as f64;
    let m3_rd = m3p as f64 * TOU_UPDATE_COST as f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = m1_rd + m3_rd;
        let be = be * Pow::pow(1f64 + TOU_COST_UP as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const OUT_MT_HOUR_YEAR: f32 = 0.0011f32; // 125/116000
pub const LABOR_COST_HOUR: f32 = 2_000f32;

pub fn ben_outage_labor(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  OUTAGE LABOR");
    let hr = (sbtr.mt_1_ph + sbtr.mt_3_ph) as f64 * OUT_MT_HOUR_YEAR as f64;
    let ex = hr * LABOR_COST_HOUR as f64 * 5f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const CALL_CENTER_COST_MT: f32 = 3.33f32;
pub const CALL_CENTER_COST_UP: f32 = 0.04f32;

pub fn ben_reduce_complain(sbtr: &SubCalc, _ben: &SubBenInfo) -> BenProj {
    //print!("====  COMPLAIN");
    let ex = (sbtr.mt_1_ph + sbtr.mt_3_ph) as f64 * CALL_CENTER_COST_MT as f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + CALL_CENTER_COST_UP as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub const M1P_COST: f32 = 2500f32;
pub const M3P_COST: f32 = 4500f32;
pub const TRX_COST: f32 = 30_000f32;
pub const ESS_COST: f32 = 20_970_000f32;
pub const PLATFORM_COST: f32 = 1500f32;

pub const M1P_IMP_COST: f32 = 300f32;
pub const M3P_IMP_COST: f32 = 500f32;
pub const TRX_IMP_COST: f32 = 2000f32;

pub const M1P_OP_COST: f32 = 250f32;
pub const M3P_OP_COST: f32 = 450f32;
pub const TRX_OP_COST: f32 = 3000f32;
pub const ESS_OP_COST: f32 = 900_000f32;
pub const PLATFORM_OP_COST: f32 = 225f32;
pub const COMM_COST: f32 = 30f32;

//pub const PLATFORM_COST: f32 = 2500f32;
//pub const PLATFORM_OP_COST: f32 = 375f32;
//pub const COMM_COST: f32 = 50f32;

pub const ASSET_WORTH_RATIO: f32 = 0.6f32;

pub fn ben_asset_value(sbtr: &SubCalc, ben: &SubBenInfo) -> BenProj {
    //print!("====  ASSET");
    let m1i = sbtr.mt_1_ph as f64 * M1P_COST as f64;
    let m3i = sbtr.mt_3_ph as f64 * M3P_COST as f64;
    let txp = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txc = sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txi = (txp + txc) as f64 * TRX_COST as f64;
    let mut esi = 0f64;
    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32 {
        esi = ben.bat_cost as f64 * 1_000_000_f64;
    }
    let ass = (m1i + m3i + txi + esi) * ASSET_WORTH_RATIO as f64;
    //print!("  m1:{m1i} m3:{m3i} t:{txi} b:{esi} = as:{ass}\n");
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..11 {
        proj.push((y + 2028, 0f32));
    }
    proj.push((11 + 2028, ass as f32));
    //println!();
    BenProj { proj }
}

pub const MODEL_ENTRY_RATIO: f32 = 0.05f32;
pub const MODEL_ENTRY_COST: f32 = 2000f32;

pub fn ben_model_entry(sbtr: &SubCalc, ben: &SubBenInfo) -> BenProj {
    //print!("====  MODEL ENTRY");
    let txp = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txc = sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let mut cnt = (txp + txc + sbtr.mt_1_ph as u32 + sbtr.mt_3_ph as u32) as f64;
    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32 {
        cnt += 1.0;
    }
    let ent_cn = cnt * MODEL_ENTRY_RATIO as f64;
    let ent_ex = ent_cn * MODEL_ENTRY_COST as f64;

    //print!("  cn:{ent_cn} ex:{ent_ex} \n");
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ent_ex;
        let be = be * Pow::pow(1f64 + CALL_CENTER_COST_UP as f64, y as f64);
        //print!(" {} - {be}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub fn sb_docx_gen(sb: String, msg: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir_all(DOCX0_PATH);
    let _ = std::fs::create_dir_all(DOCX_PATH);
    let _ = std::fs::create_dir_all(PDF_PATH);
    let fnm = format!("{DOCX0_PATH}/sub-repo-{sb}0.docx");
    let fnm2 = format!("{DOCX_PATH}/sub-repo-{sb}1.docx");
    let path = std::path::Path::new(&fnm);
    let file = std::fs::File::create(path).unwrap();

    let sfb = ld_p3_sub_inf();
    //let sf = sfb.get(&sb).unwrap();
    let sbtr = ld_sb_tr0();
    let ev = ld_sb_ev_proj()?;
    let eb = ld_sb_eb_proj()?;
    let et = ld_sb_et_proj()?;
    let sbtp = sub_type();

    let mut docx = Docx::new();
    if let (Some(sbtr), Some(sf)) = (sbtr.get(&sb), sfb.get(&sb)) {
        println!("got info");
        // Vec<Vec<AreaRatio>>
        let sb_ev = sub_ratio_proj(sf, &ev, EV_UNIT_PRICE);
        let sb_eb = sub_ratio_proj(sf, &eb, EB_UNIT_PRICE);
        let sb_et = sub_ratio_proj(sf, &et, ET_UNIT_PRICE);

        // transformer value
        let va_p = tr_val(&sbtr.p_tx_cn_m);
        let va_c = tr_val(&sbtr.c_tx_cn_m);

        let ben = ld_ben_bess1(&sb);
        docx = sub_info1(sf, sbtr, docx);
        let tp = sbtp.get(sb.as_str()).unwrap();
        docx = sub_info2(sf, tp, &ben, docx);
        let ms = "การไฟฟ้าส่วนภูมิภาค";
        docx = sub_info_p(ms, &va_p, docx);
        let ms = "ผู้ใช้ไฟฟ้า";
        docx = sub_info_p(ms, &va_c, docx);

        let ms = "รถยนต์ไฟฟ้า";
        docx = sub_ev_docx(&sb_ev, ms, docx);
        let ms = "รถบรรทุก/โดยสารไฟฟ้า";
        docx = sub_ev_docx(&sb_et, ms, docx);
        let ms = "รถจักรยานยนต์ไฟฟ้า";
        docx = sub_ev_docx(&sb_eb, ms, docx);

        let mut bens = HashMap::<u32, Box<dyn AmtProj>>::new();
        let mut emp = Vec::<(u32, f32)>::new();
        for y in OP_YEAR_START..=OP_YEAR_END {
            emp.push((y, 0f32));
        }

        //println!("p_en: {}", ben.p_en);
        bens.insert(1, Box::new(sb_ev));
        bens.insert(2, Box::new(sb_et));
        bens.insert(3, Box::new(sb_eb));
        let be1 = ben_trx(&va_p);
        bens.insert(4, Box::new(BenProj { proj: emp.clone() }));
        bens.insert(5, Box::new(be1));
        let ben6 = ben_unbalan(sbtr);
        bens.insert(6, Box::new(ben6));
        let ben7 = ben_non_tech(sbtr, &ben);
        bens.insert(7, Box::new(ben7));
        let ben8 = ben_bill_accu(sbtr, &ben);
        bens.insert(8, Box::new(ben8));
        let ben9 = ben_cash_flow(sbtr, &ben);
        bens.insert(9, Box::new(ben9));
        let ben10 = ben_dr_save(sbtr, &ben);
        bens.insert(10, Box::new(ben10));

        if ben.mx_pw > 0f32
            && ben.grw < 7f32
            && ben.be_start <= 3
            && ben.trlm > 40f32
            && (*tp == "AIS" || *tp == "GIS")
        {
            let (be_sub_save, be_re_diff, be_svg_save, be_en_added) = ben_amt_proj(&ben);
            bens.insert(11, Box::new(be_sub_save));
            bens.insert(12, Box::new(be_svg_save));
            bens.insert(13, Box::new(be_en_added));
            bens.insert(14, Box::new(be_re_diff));
        } else {
            bens.insert(11, Box::new(BenProj { proj: emp.clone() }));
            bens.insert(12, Box::new(BenProj { proj: emp.clone() }));
            bens.insert(13, Box::new(BenProj { proj: emp.clone() }));
            bens.insert(14, Box::new(BenProj { proj: emp.clone() }));
        }
        let ben15 = ben_boxline_save(sbtr, &ben);
        bens.insert(15, Box::new(ben15));
        let ben16 = ben_work_save(sbtr, &ben);
        bens.insert(16, Box::new(ben16));
        let ben17 = ben_sell_meter(sbtr, &ben);
        bens.insert(17, Box::new(ben17));
        let ben18 = ben_emeter(sbtr, &ben);
        bens.insert(18, Box::new(ben18));
        let ben19 = ben_mt_read(sbtr, &ben);
        bens.insert(19, Box::new(ben19));
        let ben20 = ben_mt_disconn(sbtr, &ben);
        bens.insert(20, Box::new(ben20));
        let ben21 = ben_tou_sell(sbtr, &ben);
        bens.insert(21, Box::new(ben21));
        let ben22 = ben_tou_read(sbtr, &ben);
        bens.insert(22, Box::new(ben22));
        let ben23 = ben_tou_update(sbtr, &ben);
        bens.insert(23, Box::new(ben23));
        let ben24 = ben_outage_labor(sbtr, &ben);
        bens.insert(24, Box::new(ben24));
        let ben25 = ben_reduce_complain(sbtr, &ben);
        bens.insert(25, Box::new(ben25));
        let ben26 = ben_asset_value(sbtr, &ben);
        bens.insert(26, Box::new(ben26));
        let ben27 = ben_model_entry(sbtr, &ben);
        bens.insert(27, Box::new(ben27));

        let mut ii = 0u32;

        for (h, c) in &BENET {
            ii += 1;
            let hh = format!("{ii}.{h}");
            docx = docx.add_paragraph(page_h2(&hh));
            docx = docx.add_paragraph(para_nm(" "));
            let mut lnv = Vec::<String>::new();
            let mut lns = String::new();
            for ln in c.lines() {
                if ln.is_empty() {
                    lnv.push(lns);
                    lns = String::new();
                } else {
                    use std::fmt::Write;
                    write!(lns, "{}", ln).expect("?");
                }
            }
            if !lns.is_empty() {
                lnv.push(lns);
            }
            for ln in lnv {
                docx = docx.add_paragraph(para_n1(&ln));
            }
            if let Some(ampj) = bens.get(&ii) {
                //println!("{ii}: found");
                let tb = tr_tab_ben2(ampj);
                docx = docx.add_table(tb);
                docx = docx.add_paragraph(para_nm(""));
            }
        }
    }
    docx.build().pack(file)?;
    docx_adj(&fnm, &fnm2);
    println!("f1:{fnm} f2:{fnm2}");
    msg.push(format!("SUB STATION:{sb}"));
    Ok(())
}

use std::rc::Rc;

pub fn tr_tab_ben3(ampj: &Rc<dyn AmtProj>) -> Table {
    let wd = vec![1000usize, 1500usize, 2000usize];
    let hd = vec![
        "ลำดับ".to_string(),
        "ปี พ.ศ.".to_string(),
        "มูลค่า (บาท)".to_string(),
    ];
    let al = vec![
        AlignmentType::Center,
        AlignmentType::Center,
        AlignmentType::Right,
    ];
    let mut cn = 0;
    let mut dt = Vec::<Vec<String>>::new();
    for y in OP_YEAR_START..=OP_YEAR_END {
        cn += 1;
        let yt = y + 543;
        let v = ampj.get_amt(y).unwrap();
        let v = v as u32;
        let v = v.separate_with_commas();
        let rw = vec![format!("{cn}"), format!("{yt}"), format!("{v}")];
        dt.push(rw);
    }
    tr_gen(hd, dt, wd, al).indent(500)
}

pub fn tr_tab_ben2(ampj: &Box<dyn AmtProj>) -> Table {
    let wd = vec![1000usize, 1500usize, 2000usize];
    let hd = vec![
        "ลำดับ".to_string(),
        "ปี พ.ศ.".to_string(),
        "มูลค่า (บาท)".to_string(),
    ];
    let al = vec![
        AlignmentType::Center,
        AlignmentType::Center,
        AlignmentType::Right,
    ];
    let mut cn = 0;
    let mut dt = Vec::<Vec<String>>::new();
    for y in OP_YEAR_START..=OP_YEAR_END {
        cn += 1;
        let yt = y + 543;
        let v = ampj.get_amt(y).unwrap();
        let v = v as u32;
        let v = v.separate_with_commas();
        let rw = vec![format!("{cn}"), format!("{yt}"), format!("{v}")];
        dt.push(rw);
    }
    tr_gen(hd, dt, wd, al).indent(DOC_TB_INDENT)
}

use sglab02_lib::sg::prc4::grp1;
use sglab02_lib::sg::prc4::ld_pv_sbv_m;

use regex::Regex;

pub const BEN_ID_START: u32 = 1;
pub const BEN_ID_END: u32 = 27;

pub fn ben_calc_0() -> Result<(), Box<dyn Error>> {
    let pv = grp1();
    let sbsl = ld_pv_sbv_m();
    let _sbif = ld_p3_sub_inf();
    let sfb = ld_p3_sub_inf();
    let sbtr = ld_sb_tr0();
    let ev = ld_sb_ev_proj().unwrap();
    let eb = ld_sb_eb_proj().unwrap();
    let et = ld_sb_et_proj().unwrap();
    let sbtp = sub_type();

    let _re = Regex::new(r"..._[0-9][0-9][VW]B01").unwrap();
    let mut prjs = vec![];
    let mut pbes = vec![];

    for p in &pv {
        let pp = p.to_string();
        //println!("pv:{}", p);

        let mut pv_prj = HashMap::<u32, f32>::new();
        for y in 0..12 {
            let yr = y + 2028;
            pv_prj.insert(yr, 0f32);
        }
        let mut pv_ben = HashMap::<u32, f32>::new();
        for b in 1..=27 {
            pv_ben.insert(b, 0f32);
        }

        let mut m1p_cnt = 0u32;
        let mut m3p_cnt = 0u32;
        let mut txp_cnt = 0u32;
        let mut txc_cnt = 0u32;
        let mut ess_mwh = 0u32;

        if let Some(sbv) = sbsl.get(&pp) {
            for sb in sbv {
                //println!("  sb:{}", sb.sbid);
                let sb = sb.sbid.clone();
                if let (Some(sbtr), Some(sf)) = (sbtr.get(&sb), sfb.get(&sb)) {
                    let sb_ev = sub_ratio_proj(sf, &ev, EV_UNIT_PRICE);
                    let sb_eb = sub_ratio_proj(sf, &eb, EB_UNIT_PRICE);
                    let sb_et = sub_ratio_proj(sf, &et, ET_UNIT_PRICE);
                    let va_p = tr_val(&sbtr.p_tx_cn_m);
                    let _va_c = tr_val(&sbtr.c_tx_cn_m);
                    let ben = ld_ben_bess1(&sb);

                    m1p_cnt += sbtr.mt_1_ph as u32;
                    m3p_cnt += sbtr.mt_3_ph as u32;
                    txp_cnt += sbtr.p_tx_cn_m.values().sum::<u32>();
                    txc_cnt += sbtr.c_tx_cn_m.values().sum::<u32>();

                    if let Some(tp) = sbtp.get(sb.as_str()) {
                        let mut bens = HashMap::<u32, Box<dyn AmtProj>>::new();
                        let mut emp = Vec::<(u32, f32)>::new();
                        for y in OP_YEAR_START..=OP_YEAR_END {
                            emp.push((y, 0f32));
                        }

                        //println!("p_en: {}", ben.p_en);
                        bens.insert(1, Box::new(sb_ev));
                        bens.insert(2, Box::new(sb_et));
                        bens.insert(3, Box::new(sb_eb));
                        let be1 = ben_trx(&va_p);
                        bens.insert(4, Box::new(BenProj { proj: emp.clone() }));
                        bens.insert(5, Box::new(be1));
                        let ben6 = ben_unbalan(sbtr);
                        bens.insert(6, Box::new(ben6));
                        let ben7 = ben_non_tech(sbtr, &ben);
                        bens.insert(7, Box::new(ben7));
                        let ben8 = ben_bill_accu(sbtr, &ben);
                        bens.insert(8, Box::new(ben8));
                        let ben9 = ben_cash_flow(sbtr, &ben);
                        bens.insert(9, Box::new(ben9));
                        let ben10 = ben_dr_save(sbtr, &ben);
                        bens.insert(10, Box::new(ben10));
                        if ben.mx_pw > 0f32
                            && ben.grw < 7f32
                            && ben.be_start <= 3
                            && ben.trlm > 40f32
                            && (*tp == "AIS" || *tp == "GIS")
                        {
                            let (be_sub_save, be_re_diff, be_svg_save, be_en_added) =
                                ben_amt_proj(&ben);
                            bens.insert(11, Box::new(be_sub_save));
                            bens.insert(12, Box::new(be_svg_save));
                            bens.insert(13, Box::new(be_en_added));
                            bens.insert(14, Box::new(be_re_diff));
                            ess_mwh += ben.bat_mwh as u32;
                        } else {
                            bens.insert(11, Box::new(BenProj { proj: emp.clone() }));
                            bens.insert(12, Box::new(BenProj { proj: emp.clone() }));
                            bens.insert(13, Box::new(BenProj { proj: emp.clone() }));
                            bens.insert(14, Box::new(BenProj { proj: emp.clone() }));
                        }
                        let ben15 = ben_boxline_save(sbtr, &ben);
                        bens.insert(15, Box::new(ben15));
                        let ben16 = ben_work_save(sbtr, &ben);
                        bens.insert(16, Box::new(ben16));
                        let ben17 = ben_sell_meter(sbtr, &ben);
                        bens.insert(17, Box::new(ben17));
                        let ben18 = ben_emeter(sbtr, &ben);
                        bens.insert(18, Box::new(ben18));
                        let ben19 = ben_mt_read(sbtr, &ben);
                        bens.insert(19, Box::new(ben19));
                        let ben20 = ben_mt_disconn(sbtr, &ben);
                        bens.insert(20, Box::new(ben20));
                        let ben21 = ben_tou_sell(sbtr, &ben);
                        bens.insert(21, Box::new(ben21));
                        let ben22 = ben_tou_read(sbtr, &ben);
                        bens.insert(22, Box::new(ben22));
                        let ben23 = ben_tou_update(sbtr, &ben);
                        bens.insert(23, Box::new(ben23));
                        let ben24 = ben_outage_labor(sbtr, &ben);
                        bens.insert(24, Box::new(ben24));
                        let ben25 = ben_reduce_complain(sbtr, &ben);
                        bens.insert(25, Box::new(ben25));
                        let ben26 = ben_asset_value(sbtr, &ben);
                        bens.insert(26, Box::new(ben26));
                        let ben27 = ben_model_entry(sbtr, &ben);
                        bens.insert(27, Box::new(ben27));
                        // sum by year
                        for y in 0..12 {
                            let yr = y + 2028;
                            let mut sm = 0f32;
                            for i in 1..=27 {
                                if let Some(be) = bens.get_mut(&i) {
                                    sm += be.get_amt(yr).unwrap();
                                }
                            }
                            if let Some(vv) = pv_prj.get_mut(&yr) {
                                *vv += sm;
                            }
                        } // end year

                        // sum by benefit
                        for b in 1..=27 {
                            let mut sm = 0f32;
                            for y in 0..12 {
                                let yr = y + 2028;
                                if let Some(be) = bens.get_mut(&b) {
                                    sm += be.get_amt(yr).unwrap();
                                }
                            }
                            if let Some(vv) = pv_ben.get_mut(&b) {
                                *vv += sm;
                            }
                        } // end benefit
                    } // end type
                } // end sub info
            } // end sub loop
        } // end if sub list
        println!("{pp} m1:{m1p_cnt} m3:{m3p_cnt} txp:{txp_cnt} txc:{txc_cnt} bat:{ess_mwh}");
        /*
        print!("{pp}");
        for y in 0..12 {
            let yr = y + 2028;
            let v = pv_prj.get(&yr).unwrap_or(&0f32);
            print!(" {v}");
        }
        println!();
        */
        prjs.push((pp.to_string(), pv_prj));
        pbes.push((pp.to_string(), pv_ben))
    } // end loop province
      //let mut wtr = csv::Writer::from_writer(std::io::stdout());
    use std::fmt::Write;

    // CSV by year
    let mut rs = String::new();
    write!(rs, "\"จังหวัด\"").expect("?");
    let mut sm = Vec::<f32>::new();
    for y in 0..12 {
        let yr = y + 2028 + 543;
        write!(rs, ",\"พ.ศ.{yr}\"").expect("?");
        sm.push(0f32);
    }
    writeln!(rs).expect("?");
    for (pv, prj) in &prjs {
        write!(rs, "\"{pv}\"").expect("?");
        for y in 0..12 {
            let yr = y + 2028;
            let dt = prj.get(&yr).unwrap_or(&0f32);
            write!(rs, ",{dt:.2}").expect("?");
            sm[y as usize] += dt;
        }
        writeln!(rs).expect("?");
    }
    let ss = sm.iter().sum::<f32>();
    write!(rs, "{ss}").expect("?");
    for y in 0..12 {
        write!(rs, ",{:.0}", sm[y as usize]).expect("?");
    }
    writeln!(rs).expect("?");
    std::fs::write("pea-ben-year.csv", rs).expect("?1");

    // CSV by benefit
    let mut rs = String::new();
    write!(rs, "\"จังหวัด\"").expect("?");
    let mut sm = Vec::<f32>::new();
    for b in 1..=27 {
        write!(rs, ",\"BEN:{b}\"").expect("?");
        sm.push(0f32);
    }
    writeln!(rs).expect("?");
    for (pv, pbe) in &pbes {
        write!(rs, "\"{pv}\"").expect("?");
        for b in 1..=27 {
            let dt = pbe.get(&b).unwrap_or(&0f32);
            write!(rs, ",{dt:.2}").expect("?");
            sm[(b - 1) as usize] += dt;
        }
        writeln!(rs).expect("?");
    }
    let ss = sm.iter().sum::<f32>();
    write!(rs, "{ss}").expect("?");
    for b in 1..=27 {
        write!(rs, ",{:.0}", sm[(b - 1) as usize]).expect("?");
    }
    writeln!(rs).expect("?");
    std::fs::write("pea-ben-ben.csv", rs).expect("?1");
    Ok(())
}

pub fn ben_calc4() -> Result<(), Box<dyn Error>> {
    let pv = grp1();
    let sbsl = ld_pv_sbv_m();
    let _sbif = ld_p3_sub_inf();
    let sfb = ld_p3_sub_inf();
    let sbtr = ld_sb_tr0();
    let ev = ld_sb_ev_proj().unwrap();
    let eb = ld_sb_eb_proj().unwrap();
    let et = ld_sb_et_proj().unwrap();
    let sbtp = sub_type();

    let _re = Regex::new(r"..._[0-9][0-9][VW]B01").unwrap();
    let mut prjs = vec![];
    let mut pbes = vec![];

    for p in &pv {
        let pp = p.to_string();
        //println!("pv:{}", p);

        let mut pv_prj = HashMap::<u32, f32>::new();
        for y in 0..12 {
            let yr = y + 2028;
            pv_prj.insert(yr, 0f32);
        }
        let mut pv_ben = HashMap::<u32, f32>::new();
        for b in 1..=27 {
            pv_ben.insert(b, 0f32);
        }

        let mut m1p_cnt = 0u32;
        let mut m3p_cnt = 0u32;
        let mut txp_cnt = 0u32;
        let mut txc_cnt = 0u32;
        let mut ess_mwh = 0u32;

        if let Some(sbv) = sbsl.get(&pp) {
            for sb in sbv {
                //println!("  sb:{}", sb.sbid);
                let sb = sb.sbid.clone();
                if let (Some(sbtr), Some(sf)) = (sbtr.get(&sb), sfb.get(&sb)) {
                    let sb_ev = sub_ratio_proj_2(sf, &ev, EV_UNIT_PRICE);
                    let sb_eb = sub_ratio_proj(sf, &eb, EB_UNIT_PRICE);
                    let sb_et = sub_ratio_proj(sf, &et, ET_UNIT_PRICE);
                    let va_p = tr_val(&sbtr.p_tx_cn_m);
                    let _va_c = tr_val(&sbtr.c_tx_cn_m);
                    let ben = ld_ben_bess1(&sb);

                    m1p_cnt += sbtr.mt_1_ph as u32;
                    m3p_cnt += sbtr.mt_3_ph as u32;
                    txp_cnt += sbtr.p_tx_cn_m.values().sum::<u32>();
                    txc_cnt += sbtr.c_tx_cn_m.values().sum::<u32>();

                    if let Some(tp) = sbtp.get(sb.as_str()) {
                        let mut bens = HashMap::<u32, Box<dyn AmtProj>>::new();
                        let mut emp = Vec::<(u32, f32)>::new();
                        for y in OP_YEAR_START..=OP_YEAR_END {
                            emp.push((y, 0f32));
                        }

                        //println!("p_en: {}", ben.p_en);
                        bens.insert(1, Box::new(sb_ev));
                        bens.insert(2, Box::new(sb_et));
                        bens.insert(3, Box::new(sb_eb));
                        let be1 = ben_trx(&va_p);
                        bens.insert(4, Box::new(BenProj { proj: emp.clone() }));
                        bens.insert(5, Box::new(be1));
                        let ben6 = ben_unbalan(sbtr);
                        bens.insert(6, Box::new(ben6));
                        let ben7 = ben_non_tech(sbtr, &ben);
                        bens.insert(7, Box::new(ben7));
                        let ben8 = ben_bill_accu(sbtr, &ben);
                        bens.insert(8, Box::new(ben8));
                        let ben9 = ben_cash_flow(sbtr, &ben);
                        bens.insert(9, Box::new(ben9));
                        let ben10 = ben_dr_save(sbtr, &ben);
                        bens.insert(10, Box::new(ben10));
                        if ben.mx_pw > 0f32
                            && ben.grw < 7f32
                            && ben.be_start <= 3
                            && ben.trlm > 40f32
                            && (*tp == "AIS" || *tp == "GIS")
                        {
                            let (be_sub_save, be_re_diff, be_svg_save, be_en_added) =
                                ben_amt_proj(&ben);
                            bens.insert(11, Box::new(be_sub_save));
                            bens.insert(12, Box::new(be_svg_save));
                            bens.insert(13, Box::new(be_en_added));
                            bens.insert(14, Box::new(be_re_diff));
                            ess_mwh += ben.bat_mwh as u32;
                        } else {
                            bens.insert(11, Box::new(BenProj { proj: emp.clone() }));
                            bens.insert(12, Box::new(BenProj { proj: emp.clone() }));
                            bens.insert(13, Box::new(BenProj { proj: emp.clone() }));
                            bens.insert(14, Box::new(BenProj { proj: emp.clone() }));
                        }
                        let ben15 = ben_boxline_save(sbtr, &ben);
                        bens.insert(15, Box::new(ben15));
                        let ben16 = ben_work_save(sbtr, &ben);
                        bens.insert(16, Box::new(ben16));
                        let ben17 = ben_sell_meter(sbtr, &ben);
                        bens.insert(17, Box::new(ben17));
                        let ben18 = ben_emeter(sbtr, &ben);
                        bens.insert(18, Box::new(ben18));
                        let ben19 = ben_mt_read(sbtr, &ben);
                        bens.insert(19, Box::new(ben19));
                        let ben20 = ben_mt_disconn(sbtr, &ben);
                        bens.insert(20, Box::new(ben20));
                        let ben21 = ben_tou_sell(sbtr, &ben);
                        bens.insert(21, Box::new(ben21));
                        let ben22 = ben_tou_read(sbtr, &ben);
                        bens.insert(22, Box::new(ben22));
                        let ben23 = ben_tou_update(sbtr, &ben);
                        bens.insert(23, Box::new(ben23));
                        let ben24 = ben_outage_labor(sbtr, &ben);
                        bens.insert(24, Box::new(ben24));
                        let ben25 = ben_reduce_complain(sbtr, &ben);
                        bens.insert(25, Box::new(ben25));
                        let ben26 = ben_asset_value(sbtr, &ben);
                        bens.insert(26, Box::new(ben26));
                        let ben27 = ben_model_entry(sbtr, &ben);
                        bens.insert(27, Box::new(ben27));
                        // sum by year
                        for y in 0..12 {
                            let yr = y + 2028;
                            let mut sm = 0f32;
                            for i in 1..=27 {
                                if let Some(be) = bens.get_mut(&i) {
                                    sm += be.get_amt(yr).unwrap();
                                }
                            }
                            if let Some(vv) = pv_prj.get_mut(&yr) {
                                *vv += sm;
                            }
                        } // end year

                        // sum by benefit
                        for b in 1..=27 {
                            let mut sm = 0f32;
                            for y in 0..12 {
                                let yr = y + 2028;
                                if let Some(be) = bens.get_mut(&b) {
                                    sm += be.get_amt(yr).unwrap();
                                }
                            }
                            if let Some(vv) = pv_ben.get_mut(&b) {
                                *vv += sm;
                            }
                        } // end benefit
                    } // end type
                } // end sub info
            } // end sub loop
        } // end if sub list
        println!("{pp} m1:{m1p_cnt} m3:{m3p_cnt} txp:{txp_cnt} txc:{txc_cnt} bat:{ess_mwh}");
        prjs.push((pp.to_string(), pv_prj));
        pbes.push((pp.to_string(), pv_ben))
    } // end loop province
      //let mut wtr = csv::Writer::from_writer(std::io::stdout());
    use std::fmt::Write;

    // CSV by year
    let mut rs = String::new();
    write!(rs, "\"จังหวัด\"").expect("?");
    let mut sm = Vec::<f32>::new();
    for y in 0..12 {
        let yr = y + 2028 + 543;
        write!(rs, ",\"พ.ศ.{yr}\"").expect("?");
        sm.push(0f32);
    }
    writeln!(rs).expect("?");
    for (pv, prj) in &prjs {
        write!(rs, "\"{pv}\"").expect("?");
        for y in 0..12 {
            let yr = y + 2028;
            let dt = prj.get(&yr).unwrap_or(&0f32);
            write!(rs, ",{dt:.2}").expect("?");
            sm[y as usize] += dt;
        }
        writeln!(rs).expect("?");
    }
    let ss = sm.iter().sum::<f32>();
    write!(rs, "{ss}").expect("?");
    for y in 0..12 {
        write!(rs, ",{:.0}", sm[y as usize]).expect("?");
    }
    writeln!(rs).expect("?");
    std::fs::write("pea-ben-year1.csv", rs).expect("?1");

    // CSV by benefit
    let mut rs = String::new();
    write!(rs, "\"จังหวัด\"").expect("?");
    let mut sm = Vec::<f32>::new();
    for b in 1..=27 {
        write!(rs, ",\"BEN:{b}\"").expect("?");
        sm.push(0f32);
    }
    writeln!(rs).expect("?");
    for (pv, pbe) in &pbes {
        write!(rs, "\"{pv}\"").expect("?");
        for b in 1..=27 {
            let dt = pbe.get(&b).unwrap_or(&0f32);
            write!(rs, ",{dt:.2}").expect("?");
            sm[(b - 1) as usize] += dt;
        }
        writeln!(rs).expect("?");
    }
    let ss = sm.iter().sum::<f32>();
    write!(rs, "{ss}").expect("?");
    for b in 1..=27 {
        write!(rs, ",{:.0}", sm[(b - 1) as usize]).expect("?");
    }
    writeln!(rs).expect("?");
    std::fs::write("pea-ben-ben1.csv", rs).expect("?1");
    Ok(())
}

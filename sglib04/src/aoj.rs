//use encoding_rs::WINDOWS_874;
use serde::{Deserialize, Serialize};
use sglab02_lib::sg::gis1::ar_list;
//use sglab02_lib::sg::gis1::DbfVal;
use geo::Contains;
use geo::{point, Polygon};
use geo_types::{coord, LineString};
use image::ImageReader;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbfData {
    Text(String),
    Int(i32),
    Real(f64),
    Bool(bool),
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BranchOffice {
    pub id: String,
    pub name: String,
    pub aid: String,
    pub gons: Vec<Vec<(f32, f32)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZoneUse {
    pub name: String,
    pub code: String,
    pub gons: Vec<Vec<(f32, f32)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BranchAccount {
    pub id: String,
    pub name: String,
    pub gons: Vec<Vec<(f32, f32)>>,
    pub trxs: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZoneAccount {
    pub name: String,
    pub code: String,
    pub gons: Vec<Vec<(f32, f32)>>,
    pub trxs: Vec<usize>,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transformer {
    pub id: String,
    pub own: String,
    pub fid: String,
    pub vol: String,
    pub loc: String,
    pub kva: f32,
    pub tap: String,
    pub tag: String,
    pub state: f32,
    pub enab: f32,
    pub typ: f32,
    pub x: f32,
    pub y: f32,
    pub offs: Vec<usize>,
    pub izns: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubstAreaZone {
    pub sbid: String,
    pub offs: HashSet<usize>,
    pub izns: HashSet<usize>,
    pub trxs: HashSet<usize>,
}

pub const DB2_DIR: &str = "/mnt/e/CHMBACK/PEADATA/db2";
pub const MP_MG: u32 = 72;
pub const MP_UPDW: u32 = 185;
pub const MP_WW: f32 = 1800_f32;
pub const MP_HH: f32 = 1733_f32 - 185_f32 * 2.0;

pub fn aoj_and_zone() {
    for x in ar_list() {
        println!("Area: {x}");
        let rg = format!("{DB2_DIR}/{x}_LB_AOJ.rg");
        let db = format!("{DB2_DIR}/{x}_LB_AOJ.at");
        let zg = format!("{DB2_DIR}/{x}_Zone_Use.rg");
        let za = format!("{DB2_DIR}/{x}_Zone_use.at");
        let tx = format!("{DB2_DIR}/{x}_DS_Transformer.pn");
        let ta = format!("{DB2_DIR}/{x}_DS_Transformer.at");
        if let (Ok(frg), Ok(fdb), Ok(fzg), Ok(fza), Ok(ftx), Ok(fta)) = (
            File::open(&rg),
            File::open(&db),
            File::open(&zg),
            File::open(&za),
            File::open(&tx),
            File::open(&ta),
        ) {
            let rdrg = BufReader::new(frg);
            let rddb = BufReader::new(fdb);
            let rdzg = BufReader::new(fzg);
            let rdza = BufReader::new(fza);
            let rdtx = BufReader::new(ftx);
            let rdta = BufReader::new(fta);
            if let (Ok(urg), Ok(udb), Ok(uzg), Ok(uza), Ok(utx), Ok(uta)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(rdrg),
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(rddb),
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(rdzg),
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(rdza),
                bincode::deserialize_from::<BufReader<File>, Vec<(f64, f64)>>(rdtx),
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(rdta),
            ) {
                // branch office
                let mut brns = Vec::<BranchOffice>::new();
                let mut aojrg = vec![];
                for (r, a) in urg.iter().zip(udb.iter()) {
                    let code = if let DbfData::Text(s) = a.get("CODE").unwrap() {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let name = if let DbfData::Text(s) = a.get("NAME").unwrap() {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let area = if let DbfData::Text(s) = a.get("AREA_CODE").unwrap() {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let mut pols = vec![];
                    let mut gons = vec![];
                    for s in r {
                        let mut lines = vec![];
                        let mut gon = vec![];
                        for (x, y) in s {
                            lines.push(coord! { x: *x as f32, y: *y as f32, });
                            gon.push((*x as f32, *y as f32));
                        }
                        let line_string = LineString::new(lines);
                        let polygon = Polygon::new(line_string.clone(), vec![]);
                        pols.push(polygon);
                        gons.push(gon);
                    }
                    let brn = BranchOffice {
                        id: code.clone(),
                        name: name.clone(),
                        aid: area.clone(),
                        gons,
                    };
                    brns.push(brn);
                    aojrg.push(pols);
                }

                //=== ZONE use
                let mut cdm = HashMap::<String, u32>::new();
                let mut zones = vec![];
                let mut zonrg = vec![];
                for (r, a) in uzg.iter().zip(uza.iter()) {
                    if let Some(DbfData::Text(code)) = a.get("ZONE_CODE") {
                        if let Some(cn) = cdm.get_mut(code) {
                            *cn += 1;
                        } else {
                            cdm.insert(code.to_string(), 1);
                        }
                    }
                    let code = if let Some(DbfData::Text(s)) = a.get("ZONE_CODE") {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let name = if let Some(DbfData::Text(s)) = a.get("ZONE_NAME") {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let mut pols = vec![];
                    let mut gons = vec![];
                    for s in r {
                        let mut gon = vec![];
                        let mut lines = vec![];
                        for (x, y) in s {
                            gon.push((*x as f32, *y as f32));
                            lines.push(coord! { x: *x as f32, y: *y as f32, });
                        }
                        let line_string = LineString::new(lines);
                        let polygon = Polygon::new(line_string.clone(), vec![]);
                        pols.push(polygon);
                        gons.push(gon);
                    }
                    let zone = ZoneUse {
                        code: code.clone(),
                        name: name.clone(),
                        gons,
                    };
                    zones.push(zone);
                    zonrg.push(pols);
                }
                println!("{cdm:?}");

                let mut subazm = HashMap::<String, SubstAreaZone>::new();
                //==== transformer
                //let mut sbtrx = vec![];
                let mut dtrxs = vec![];
                for (r, a) in utx.iter().zip(uta.iter()) {
                    let id = if let Some(DbfData::Text(s)) = a.get("FACILITYID") {
                        s.to_string()
                    } else {
                        String::new()
                    };
                    let own = if let Some(DbfData::Text(s)) = a.get("OWNER") {
                        s.to_string()
                    } else {
                        String::new()
                    };
                    let fid = if let Some(DbfData::Text(s)) = a.get("FEEDERID") {
                        s.to_string()
                    } else {
                        String::new()
                    };
                    let vol = if let Some(DbfData::Text(s)) = a.get("OP_VOLT") {
                        s.to_string()
                    } else {
                        String::new()
                    };
                    let loc = if let Some(DbfData::Text(s)) = a.get("LOCATION") {
                        s.to_string()
                    } else {
                        String::new()
                    };
                    let kva = if let Some(DbfData::Real(s)) = a.get("RATEKVA") {
                        *s as f32
                    } else {
                        0f32
                    };
                    let tap = if let Some(DbfData::Text(s)) = a.get("PRESENTTAP") {
                        s.to_string()
                    } else {
                        String::new()
                    };
                    let tag = if let Some(DbfData::Text(s)) = a.get("TAG") {
                        s.to_string()
                    } else {
                        String::new()
                    };
                    let state = if let Some(DbfData::Real(s)) = a.get("LOADSTATUS") {
                        *s as f32
                    } else {
                        0f32
                    };
                    let enab = if let Some(DbfData::Real(s)) = a.get("ENABLED") {
                        *s as f32
                    } else {
                        0f32
                    };
                    let typ = if let Some(DbfData::Real(s)) = a.get("SUBTYPECD") {
                        *s as f32
                    } else {
                        0f32
                    };
                    if fid.len() < 5 {
                        println!("ERROR trans feeder: id:{id} fid:{fid} en:{enab}");
                        continue;
                    }
                    let x = r.0 as f32;
                    let y = r.1 as f32;
                    let mut offs = Vec::<usize>::new();
                    let mut izns = Vec::<usize>::new();

                    let pn = point!(x: x, y: y);
                    for (i, po) in aojrg.iter().enumerate() {
                        for pp in po.iter() {
                            if pp.contains(&pn) {
                                offs.push(i);
                            }
                        }
                    }
                    for (i, po) in zonrg.iter().enumerate() {
                        for pp in po.iter() {
                            if pp.contains(&pn) {
                                izns.push(i);
                            }
                        }
                    }

                    let sbid = &fid[0..3];
                    if let Some(sbaz) = subazm.get_mut(sbid) {
                        for o in &offs {
                            sbaz.offs.insert(*o);
                        }
                        for o in &izns {
                            sbaz.izns.insert(*o);
                        }
                        sbaz.trxs.insert(dtrxs.len());
                    } else {
                        let mut sbaz = SubstAreaZone {
                            sbid: sbid.to_string(),
                            ..Default::default()
                        };
                        for o in &offs {
                            sbaz.offs.insert(*o);
                        }
                        for o in &izns {
                            sbaz.izns.insert(*o);
                        }
                        sbaz.trxs.insert(dtrxs.len());
                        subazm.insert(sbid.to_string(), sbaz);
                    }
                    let trx = Transformer {
                        id,
                        own,
                        fid,
                        vol,
                        loc,
                        kva,
                        tap,
                        tag,
                        state,
                        enab,
                        typ,
                        x,
                        y,
                        offs,
                        izns,
                    };
                    //let mut subazm = HashMap::<String, SubstAreaZone>::new();

                    dtrxs.push(trx);
                } // end transformer loop
                let odir = "../sgdata/trxoaj";
                std::fs::create_dir_all(odir).expect("?");

                let aojrgf = format!("{odir}/{x}_aojrg.bin");
                if let Ok(se) = bincode::serialize(&brns) {
                    std::fs::write(aojrgf, se).unwrap();
                }
                let zonrgf = format!("{odir}/{x}_zonrg.bin");
                if let Ok(se) = bincode::serialize(&zones) {
                    std::fs::write(zonrgf, se).unwrap();
                }
                let dtrxsf = format!("{odir}/{x}_dtrxs.bin");
                if let Ok(se) = bincode::serialize(&dtrxs) {
                    std::fs::write(dtrxsf, se).unwrap();
                }
                let subazmf = format!("{odir}/{x}_subazm.bin");
                if let Ok(se) = bincode::serialize(&subazm) {
                    std::fs::write(subazmf, se).unwrap();
                }
            } // end deserialized loop
        } // end open files
    } // end of area loop
}

use num_traits::Pow;
use std::error::Error;

pub fn zoom_to_meter_pixel_lat(z: u32, lat: f32) -> f32 {
    //156543.03392 * Math.cos(latLng.lat() * Math.PI / 180) / Math.pow(2, zoom);
    let d0 = 156_543.033_92_f64;
    let ra: f64 = lat as f64 / 180.0 * std::f64::consts::PI;
    let a2 = ra.cos();
    let pw = Pow::pow(2f64, z as f64);
    let d1 = d0 * a2 / pw;
    d1 as f32
}

pub fn meter_pixel_to_zoom_lat(dx: f32, px: u32, lat: f32) -> u32 {
    for z in (0u32..=24u32).rev() {
        let d1 = zoom_to_meter_pixel_lat(z, lat) * px as f32;
        //println!("  {z} {d1} {dx}");
        if d1 > dx {
            return z;
        }
    }
    0
}

pub fn zoom_to_meter_pixel(z: u32) -> f32 {
    zoom_to_meter_pixel_lat(z, 13_f32)
}

pub fn meter_pixel_to_zoom(dx: f32, px: u32) -> u32 {
    meter_pixel_to_zoom_lat(dx, px, 13_f32)
}

use sglab02_lib::sg::mvline::utm_latlong;

pub fn ld_aoj2() -> Result<(), Box<dyn Error>> {
    let trxs = ld_dtrans("NE3")?;
    println!("trx:{}", trxs.len());
    let brns = ld_brns("NE3")?;
    println!("brns: {}", brns.len());
    let zones = ld_zones("NE3")?;
    println!("zones: {}", zones.len());
    let subazm = ld_subazm("NE3")?;
    println!("subazm: {}", subazm.len());
    for (k, sb) in subazm {
        println!("{k}");
        let (mut x0, mut y0, mut x1, mut y1) = (trxs[0].x, trxs[0].y, trxs[0].x, trxs[0].y);
        for ti in sb.trxs {
            x0 = x0.min(trxs[ti].x);
            y0 = y0.min(trxs[ti].y);
            x1 = x1.max(trxs[ti].x);
            y1 = y1.max(trxs[ti].y);
        }
        let wd = x1 - x0;
        let (ld, ln) = utm_latlong(x0, y0);
        let zm = meter_pixel_to_zoom(wd, 3000);
        println!("  wd:{wd} zm:{zm} ld,ln:{ld},{ln}");
    }
    Ok(())
}

pub fn ld_dtrans(ar: &str) -> Result<Vec<Transformer>, Box<dyn Error>> {
    let odir = "../sgdata/trxoaj";
    let dtrxsf = format!("{odir}/{ar}_dtrxs.bin");
    let f = File::open(dtrxsf)?;
    Ok(bincode::deserialize_from::<
        BufReader<File>,
        Vec<Transformer>,
    >(BufReader::new(f))?)
}

pub fn ld_brns(ar: &str) -> Result<Vec<BranchOffice>, Box<dyn Error>> {
    let odir = "../sgdata/trxoaj";
    let aojrgf = format!("{odir}/{ar}_aojrg.bin");
    let f = File::open(aojrgf)?;
    Ok(bincode::deserialize_from::<
        BufReader<File>,
        Vec<BranchOffice>,
    >(BufReader::new(f))?)
}

pub fn ld_zones(ar: &str) -> Result<Vec<ZoneUse>, Box<dyn Error>> {
    let odir = "../sgdata/trxoaj";
    let aojrgf = format!("{odir}/{ar}_zonrg.bin");
    let f = File::open(aojrgf)?;
    Ok(bincode::deserialize_from::<BufReader<File>, Vec<ZoneUse>>(
        BufReader::new(f),
    )?)
}

pub fn ld_subazm(ar: &str) -> Result<HashMap<String, SubstAreaZone>, Box<dyn Error>> {
    let odir = "../sgdata/trxoaj";
    let aojrgf = format!("{odir}/{ar}_subazm.bin");
    let f = File::open(aojrgf)?;
    Ok(bincode::deserialize_from::<
        BufReader<File>,
        HashMap<String, SubstAreaZone>,
    >(BufReader::new(f))?)
}

pub fn read_aoj() {
    for x in ar_list() {
        if x != "NE3" {
            //continue;
        }
        println!("Area: {x}");
        let rg = format!("{DB2_DIR}/{x}_LB_AOJ.rg");
        let db = format!("{DB2_DIR}/{x}_LB_AOJ.at");
        let tx = format!("{DB2_DIR}/{x}_DS_Transformer.pn");
        if let (Ok(frg), Ok(fdb), Ok(ftx)) = (File::open(&rg), File::open(&db), File::open(&tx)) {
            let rdrg = BufReader::new(frg);
            let rddb = BufReader::new(fdb);
            let rdtx = BufReader::new(ftx);
            if let (Ok(urg), Ok(udb), Ok(utx)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(rdrg),
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(rddb),
                bincode::deserialize_from::<BufReader<File>, Vec<(f64, f64)>>(rdtx),
            ) {
                let mut names = vec![];
                let mut codes = vec![];
                let mut areas = vec![];
                let mut polys = vec![];
                for (r, a) in urg.iter().zip(udb.iter()) {
                    let code = if let DbfData::Text(s) = a.get("CODE").unwrap() {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let name = if let DbfData::Text(s) = a.get("NAME").unwrap() {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let area = if let DbfData::Text(s) = a.get("AREA_CODE").unwrap() {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let mut pols = vec![];
                    for s in r {
                        let mut lines = vec![];
                        for (x, y) in s {
                            lines.push(coord! { x: *x as f32, y: *y as f32, });
                        }
                        let line_string = LineString::new(lines);
                        let polygon = Polygon::new(line_string.clone(), vec![]);
                        pols.push(polygon);
                    }
                    polys.push(pols);
                    codes.push(code);
                    names.push(name);
                    areas.push(area);
                }
                let mut belo = vec![];
                let mut cntm = HashMap::<usize, usize>::new();
                for (x, y) in utx.iter() {
                    let mut cnv = Vec::<usize>::with_capacity(2);
                    let pn = point!(x: *x as f32, y: *y as f32);
                    for (i, po) in polys.iter().enumerate() {
                        for pp in po.iter() {
                            if pp.contains(&pn) {
                                cnv.push(i);
                            }
                        }
                    }
                    if let Some(cn) = cntm.get_mut(&cnv.len()) {
                        *cn += 1;
                    } else {
                        cntm.insert(cnv.len(), 1);
                    }
                    belo.push(cnv);
                }
                println!("txn: {}", belo.len());
                println!("cntm: {cntm:?}");
                /*
                if let Some(a) = uta.first() {
                    for (k, v) in a {
                        println!("{k} = {v:?}");
                    }
                }
                */
            }
        }
    }
}

pub fn read_zone() {
    for x in ar_list() {
        println!("Area: {x}");
        let rg = format!("{DB2_DIR}/{x}_Zone_Use.rg");
        let db = format!("{DB2_DIR}/{x}_Zone_use.at");
        let tx = format!("{DB2_DIR}/{x}_DS_Transformer.pn");
        if let (Ok(frg), Ok(fdb), Ok(ftx)) = (File::open(&rg), File::open(&db), File::open(&tx)) {
            let rdrg = BufReader::new(frg);
            let rddb = BufReader::new(fdb);
            let rdtx = BufReader::new(ftx);
            if let (Ok(urg), Ok(udb), Ok(utx)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(rdrg),
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(rddb),
                bincode::deserialize_from::<BufReader<File>, Vec<(f64, f64)>>(rdtx),
            ) {
                let mut cdm = HashMap::<String, u32>::new();
                let mut names = vec![];
                let mut codes = vec![];
                let mut polys = vec![];
                for (r, a) in urg.iter().zip(udb.iter()) {
                    if let Some(code) = a.get("ZONE_CODE") {
                        if let DbfData::Text(code) = code {
                            if let Some(cn) = cdm.get_mut(code) {
                                *cn += 1;
                            } else {
                                cdm.insert(code.to_string(), 1);
                            }
                        }
                    }
                    let code = if let DbfData::Text(s) = a.get("ZONE_CODE").unwrap() {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let name = if let DbfData::Text(s) = a.get("ZONE_NAME").unwrap() {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let mut pols = vec![];
                    for s in r {
                        let mut lines = vec![];
                        for (x, y) in s {
                            lines.push(coord! { x: *x as f32, y: *y as f32, });
                        }
                        let line_string = LineString::new(lines);
                        let polygon = Polygon::new(line_string.clone(), vec![]);
                        pols.push(polygon);
                    }
                    codes.push(code);
                    names.push(name);
                    polys.push(pols);
                }
                println!("{cdm:?}");
                let mut belo = vec![];
                let mut cntm = HashMap::<usize, usize>::new();
                for (x, y) in utx.iter() {
                    let mut cnv = Vec::<usize>::with_capacity(2);
                    let pn = point!(x: *x as f32, y: *y as f32);
                    for (i, po) in polys.iter().enumerate() {
                        for pp in po.iter() {
                            if pp.contains(&pn) {
                                cnv.push(i);
                            }
                        }
                    }
                    if let Some(cn) = cntm.get_mut(&cnv.len()) {
                        *cn += 1;
                    } else {
                        cntm.insert(cnv.len(), 1);
                    }
                    belo.push(cnv);
                }
                println!("txn: {}", belo.len());
                println!("cntm: {cntm:?}");
                //println!("{cn}. {code} {name} {area} = {}", s.len());
            }
        }
    }
}

use headless_chrome::protocol::cdp::Page;
use headless_chrome::types::Bounds;
use headless_chrome::Browser;
use image::GenericImage;
use image::GenericImageView;
use image::Rgba;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::drawing::draw_hollow_polygon_mut;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::pixelops::interpolate;
use imageproc::point::Point;
use regex::Regex;
use sglab02_lib::sg::mvline::latlong_utm;
use sglab02_lib::sg::prc3::ld_p3_sub_inf;
use sglib03::subtype::SUB_TYPES;
use std::{thread, time};

pub fn sub_latlong_adjust() -> HashMap<String, (f32, f32)> {
    let mut adjxy = HashMap::<String, (f32, f32)>::new();
    adjxy.insert("BJA".to_string(), (-0.0003, 0.0003));
    adjxy.insert("BKH".to_string(), (0.0000, -0.0003));
    adjxy.insert("BYA".to_string(), (0.0003, 0.0000));
    adjxy.insert("MBA".to_string(), (0.0003, -0.0003));
    adjxy.insert("NKP".to_string(), (-0.0004, 0.0000));
    adjxy.insert("APA".to_string(), (0.0003, 0.0004));
    adjxy.insert("BNF".to_string(), (0.0000, -0.0004));
    adjxy
}

pub fn zone_code() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("11", "นิคมอุตสาหกรรม"),
        ("12", "เขตอุตสาหกรรม"),
        ("13", "สวนอุตสาหกรรม"),
        ("14", "ย่านอุตสาหกรรม"),
        ("21", "เทศศบาลนคร"),
        ("22", "เทศบาลเมืองที่เป็นพื้นที่ธุรกิจ"),
        ("23", "เทศบาลเมืองที่เป็นพื้นที่สำคัญ"),
        ("24", "เทศบาลตำบลที่เป็นพื้นที่ธุรกิจ"),
        ("25", "เทศบาลตำบลที่เป็นพื้นที่สำคัญ"),
        ("31", "เทศบาลเมือง"),
        ("41", "เทศบาลตำบล"),
        ("41", "เทศบาลตำบล"),
        ("51", "ชนบท"),
    ])
}

pub fn map1() -> Result<(), Box<dyn Error>> {
    let odir1 = "../sgdata/sub_img1/";
    let odir2 = "../sgdata/sub_img2/";
    let odir3 = "../sgdata/sub_img3/";
    let odir4 = "../sgdata/sub_img4/";
    let odir5 = "../sgdata/sub_img5/";
    std::fs::create_dir_all(odir1).expect("?");
    std::fs::create_dir_all(odir2).expect("?");
    std::fs::create_dir_all(odir3).expect("?");
    std::fs::create_dir_all(odir4).expect("?");
    std::fs::create_dir_all(odir5).expect("?");

    let mg = MP_MG;
    let updw = MP_UPDW;
    let ww = MP_WW;
    let hh = MP_HH;

    let (w, h) = (mg as f32 + ww, updw as f32 * 2.0 + hh);

    let adjxy = sub_latlong_adjust();
    let sbif = ld_p3_sub_inf();

    let re = Regex::new(r"q=([0-9]+\.[0-9]+),([0-9]+\.[0-9]+)").unwrap();
    for (s, _c, g) in &SUB_TYPES {
        if let Some(cap) = re.captures_iter(g).next() {
            let x = &cap[1];
            let y = &cap[2];
            let mut xx = x.parse::<f32>().unwrap();
            let mut yy = y.parse::<f32>().unwrap();
            let sbid = s.to_string();
            if let Some((x1, y1)) = adjxy.get(&sbid) {
                xx += x1;
                yy += y1;
            }
            let ofs_x = 40f32;

            let (sb_x, sb_y) = latlong_utm(xx, yy);
            println!("=== {s} x:{xx} y:{yy} utm:{sb_x},{sb_y}");
            let sbid = s.to_string();

            if let Some(sbif) = sbif.get(&sbid) {
                let arid = sbif.arid.to_string();
                println!("{arid} - {s}");
                let trxs = ld_dtrans(&arid)?;
                let brns = ld_brns(&arid)?;
                let _zones = ld_zones(&arid)?;
                let subazm = ld_subazm(&arid)?;
                if let Some(sb) = subazm.get(&sbid) {
                    let (mut x0, mut y0, mut x1, mut y1) =
                        (trxs[0].x, trxs[0].y, trxs[0].x, trxs[0].y);
                    let mut offs = HashSet::<usize>::new();
                    let mut izns = HashSet::<usize>::new();
                    for ti in &sb.trxs {
                        x0 = x0.min(trxs[*ti].x);
                        y0 = y0.min(trxs[*ti].y);
                        x1 = x1.max(trxs[*ti].x);
                        y1 = y1.max(trxs[*ti].y);
                        for u in &trxs[*ti].offs {
                            offs.insert(*u);
                        }
                        for u in &trxs[*ti].izns {
                            izns.insert(*u);
                        }
                    }
                    let wd = x1 - x0;
                    let (ld, ln) = utm_latlong(x0, y0);
                    let zm = meter_pixel_to_zoom_lat(wd, ww as u32, ld);
                    let mtpx = zoom_to_meter_pixel_lat(zm, ld);
                    let ex_x = mtpx * ww;
                    let ex_y = mtpx * hh;
                    let or_x = sb_x - ex_x / 2.0;
                    let or_y = sb_y - ex_y / 2.0;
                    let (mut fg1, mut fg2) = (false, false);

                    let fimg1 = format!("{odir1}/{s}.jpeg");
                    let fimg2 = format!("{odir2}/{s}.jpeg");
                    let fimg3 = format!("{odir3}/{s}.jpeg");
                    let fimg4 = format!("{odir4}/{s}.jpeg");

                    let blk = Rgba([0u8, 0u8, 0u8, 0u8]);
                    let wht = Rgba([255u8, 255u8, 255u8, 0u8]);
                    let cols = [
                        Rgba([255u8, 0u8, 0u8, 0u8]),
                        Rgba([0u8, 255u8, 0u8, 0u8]),
                        Rgba([0u8, 0u8, 255u8, 0u8]),
                        Rgba([255u8, 255u8, 0u8, 0u8]),
                        Rgba([0u8, 255u8, 255u8, 0u8]),
                    ];

                    loop {
                        if !std::path::Path::new(fimg1.as_str()).exists() {
                            let bnd = Bounds::Normal {
                                left: None,
                                top: None,
                                width: Some(w.into()),
                                //height: Some(h.into()),
                                height: Some(w.into()),
                            };
                            let url = format!(
"https://www.google.com/maps/@?api=1&map_action=map&center={xx},{yy}&zoom=20&basemap=satellite");
                            println!("   img1: h:{h}");

                            let browser = Browser::default()?;
                            let tab = browser.new_tab()?;
                            if let Ok(_) = tab.navigate_to(&url) {
                            } else {
                                println!("!!! fail to navigate to");
                                continue;
                            }
                            if let Ok(_) = tab.set_bounds(bnd) {
                            } else {
                                println!("!!! fail to set bound");
                                continue;
                            }
                            if let Ok(_) = tab.wait_until_navigated() {
                            } else {
                                println!("!!! fail to wait");
                                continue;
                            }

                            let ten_millis = time::Duration::from_millis(2000);
                            thread::sleep(ten_millis);
                            let jpeg_data = tab.capture_screenshot(
                                Page::CaptureScreenshotFormatOption::Jpeg,
                                None,
                                None,
                                true,
                            )?;
                            std::fs::write(&fimg1, jpeg_data)?;
                            println!("img1 = {url}");
                        } else {
                            println!("{s} image 1 skipped");
                            fg1 = true;
                        }
                        break;
                    }

                    loop {
                        if !std::path::Path::new(fimg2.as_str()).exists() {
                            println!("  wd:{wd} zm:{zm} ld,ln:{ld},{ln}");
                            println!("    sb:{sb_x},{sb_y} dd:{ex_x},{ex_y}  or:{or_x},{or_y}");
                            println!("    offs:{} izns:{}", offs.len(), izns.len());

                            let bnd = Bounds::Normal {
                                left: None,
                                top: None,
                                width: Some(w.into()),
                                //height: Some(h.into()),
                                height: Some(w.into()),
                            };
                            let url = format!("https://www.google.pl/maps/@{xx},{yy},{zm}z");

                            let browser = Browser::default()?;
                            let tab = browser.new_tab()?;
                            if tab.navigate_to(&url).is_err() {
                                println!("!!! fail to navigate to");
                                continue;
                            }
                            if tab.set_bounds(bnd).is_err() {
                                println!("!!! fail to set bound");
                                continue;
                            }
                            if tab.wait_until_navigated().is_err() {
                                println!("!!! fail to wait");
                                continue;
                            }

                            let ten_millis = time::Duration::from_millis(2000);
                            thread::sleep(ten_millis);
                            let jpeg_data = tab.capture_screenshot(
                                Page::CaptureScreenshotFormatOption::Jpeg,
                                None,
                                None,
                                true,
                            )?;
                            std::fs::write(&fimg2, jpeg_data)?;
                            println!("img2 = {url}");
                        } else {
                            println!("{s} image 2 skipped");
                            fg2 = true;
                        }
                        break;
                    }

                    if !fg1 || !std::path::Path::new(fimg3.as_str()).exists() {
                        if let Ok(img) = ImageReader::open(&fimg1) {
                            if let Ok(mut img) = img.decode() {
                                let (w, h) = (img.width(), img.height());
                                println!(" hh:{hh} h:{h} updw:{updw}");
                                let mut img = img.crop(mg, updw, w - mg, h - updw * 2);
                                //let semi_red = Rgba([255u8, 0u8, 0u8, 0u8]);
                                let x = (sb_x - or_x) * ww / ex_x;
                                let y = (sb_y - or_y) * hh / ex_y;
                                println!(
                                    " {}/{} = {} hh:{hh} y:{y}",
                                    sb_y - or_y,
                                    ex_y,
                                    (sb_y - or_y) / ex_y
                                );
                                let y = hh - y;
                                draw_filled_rect_mut(
                                    &mut img,
                                    imageproc::rect::Rect::at(x as i32 - 15, y as i32 - 15)
                                        .of_size(30, 30),
                                    wht,
                                );
                                draw_hollow_rect_mut(
                                    &mut img,
                                    imageproc::rect::Rect::at(x as i32 - 15, y as i32 - 15)
                                        .of_size(30, 30),
                                    blk,
                                );
                                img.save(&fimg3).expect("?");
                            }
                            println!("fimg3: {fimg3}");
                        }
                    }

                    if !fg2 || !std::path::Path::new(fimg4.as_str()).exists() {
                        if let Ok(img) = ImageReader::open(&fimg2) {
                            if let Ok(mut img) = img.decode() {
                                let (w, h) = (img.width(), img.height());
                                println!(" hh:{hh} h:{h} updw:{updw}");
                                let mut img = img.crop(mg, updw, w - mg, h - updw * 2);
                                let semi_red = Rgba([255u8, 0u8, 0u8, 0u8]);
                                let x = (sb_x - or_x) * ww / ex_x - ofs_x;
                                let y = (sb_y - or_y) * hh / ex_y;
                                println!(
                                    " {}/{} = {} hh:{hh} y:{y}",
                                    sb_y - or_y,
                                    ex_y,
                                    (sb_y - or_y) / ex_y
                                );
                                let y = hh - y;
                                let mut off_pols = vec![];
                                for o in &offs {
                                    let gons = &brns[*o].gons;
                                    let mut pols = vec![];
                                    for gon in gons {
                                        let mut pnts = Vec::<Point<f32>>::new();
                                        let mut lines = vec![];
                                        println!("    o: {o} - {}", gon.len());
                                        let mut pn0 = Point::<f32>::new(0.0, 0.0);
                                        let mut pn1 = Point::<f32>::new(0.0, 0.0); // last point
                                        for (_i, pt) in gon.iter().enumerate() {
                                            let x = (pt.0 - or_x) * ww / ex_x - ofs_x;
                                            let y = hh - (pt.1 - or_y) * hh / ex_y;
                                            let pn2 = Point::<f32>::new(x, y);
                                            if pn2 != pn1 && pn2 != pn0 {
                                                pn1 = pn2;
                                                if pnts.is_empty() {
                                                    pn0 = pn2;
                                                }
                                                pnts.push(pn2);
                                                lines.push(coord! { x:x, y:y, });
                                                //lines.push(coord! { x: x as f32, y: y as f32, });
                                            }
                                        }
                                        let line_string = LineString::new(lines);
                                        let polygon = Polygon::new(line_string.clone(), vec![]);
                                        pols.push(polygon);
                                        draw_hollow_polygon_mut(&mut img, &pnts, semi_red);
                                    }
                                    off_pols.push(pols);
                                }
                                let (wd, hg) = (ww as u32, hh as u32);
                                for y in 0..hg {
                                    for x in 0..wd {
                                        let pn = point!(x: x as f32, y: y as f32);
                                        for (o, pols) in off_pols.iter().enumerate() {
                                            for pi in pols.iter() {
                                                if pi.contains(&pn) {
                                                    let oo = o % cols.len();
                                                    let pixel = img.get_pixel(x, y);
                                                    let pixel = interpolate(pixel, cols[oo], 0.8);
                                                    img.put_pixel(x, y, pixel);
                                                }
                                            }
                                        }
                                    }
                                }
                                draw_filled_rect_mut(
                                    &mut img,
                                    imageproc::rect::Rect::at(x as i32 - 15, y as i32 - 15)
                                        .of_size(30, 30),
                                    wht,
                                );
                                draw_hollow_rect_mut(
                                    &mut img,
                                    imageproc::rect::Rect::at(x as i32 - 15, y as i32 - 15)
                                        .of_size(30, 30),
                                    blk,
                                );
                                img.save(&fimg4).expect("?");
                            }
                            println!("fimg4: {fimg4}");
                        } // open image
                    } // check if fg
                } // end check get
            } // end sub info
        } // end regex cap
    } // end sub loop
    Ok(())
}

pub fn map2() -> Result<(), Box<dyn Error>> {
    let odir1 = "../sgdata/sub_img1/";
    let odir2 = "../sgdata/sub_img2/";
    let odir3 = "../sgdata/sub_img3/";
    let odir4 = "../sgdata/sub_img4/";
    let odir5 = "../sgdata/sub_img5/";
    let odir6 = "../sgdata/sub_img6/";
    std::fs::create_dir_all(odir1).expect("?");
    std::fs::create_dir_all(odir2).expect("?");
    std::fs::create_dir_all(odir3).expect("?");
    std::fs::create_dir_all(odir4).expect("?");
    std::fs::create_dir_all(odir5).expect("?");
    std::fs::create_dir_all(odir6).expect("?");

    let mg = MP_MG;
    let updw = MP_UPDW;
    let ww = MP_WW;
    let hh = MP_HH;

    let (w, h) = (mg as f32 + ww, updw as f32 * 2.0 + hh);

    let mut adjxy = HashMap::<&str, (f32, f32)>::new();
    adjxy.insert("BJA", (-0.0003, 0.0003));
    adjxy.insert("BKH", (0.0000, -0.0003));
    adjxy.insert("BYA", (0.0003, 0.0000));
    adjxy.insert("MBA", (0.0003, -0.0003));
    adjxy.insert("NKP", (-0.0004, 0.0000));
    adjxy.insert("APA", (0.0003, 0.0004));
    adjxy.insert("BNF", (0.0000, -0.0004));
    let sbif = ld_p3_sub_inf();

    //let browser = Browser::default()?;
    //let tab = browser.new_tab()?;

    let re = Regex::new(r"q=([0-9]+\.[0-9]+),([0-9]+\.[0-9]+)").unwrap();
    for (s, _c, g) in &SUB_TYPES {
        if let Some(cap) = re.captures_iter(g).next() {
            let x = &cap[1];
            let y = &cap[2];
            let mut xx = x.parse::<f32>().unwrap();
            let mut yy = y.parse::<f32>().unwrap();
            if let Some((x1, y1)) = adjxy.get(s) {
                xx += x1;
                yy += y1;
            }
            let ofs_x = 40f32;

            let (sb_x, sb_y) = latlong_utm(xx, yy);
            println!("=== {s} x:{xx} y:{yy} utm:{sb_x},{sb_y}");
            let sbid = s.to_string();

            //let mut zm = 11u32;
            //let (mut or_x, mut or_y) = (0.0, 0.0);
            //let (mut ex_x, mut ex_y) = (0.0, 0.0);
            if let Some(sbif) = sbif.get(&sbid) {
                let arid = sbif.arid.to_string();
                println!("{arid} - {s}");
                let trxs = ld_dtrans(&arid)?;
                let _brns = ld_brns(&arid)?;
                let zones = ld_zones(&arid)?;
                let subazm = ld_subazm(&arid)?;
                if let Some(sb) = subazm.get(&sbid) {
                    let mut offs = HashSet::<usize>::new();
                    let mut izns = HashSet::<usize>::new();
                    for ti in &sb.trxs {
                        for u in &trxs[*ti].offs {
                            offs.insert(*u);
                        }
                        for u in &trxs[*ti].izns {
                            izns.insert(*u);
                        }
                    }
                    let blk = Rgba([0u8, 0u8, 0u8, 0u8]);
                    let wht = Rgba([255u8, 255u8, 255u8, 0u8]);
                    let _cols = [
                        Rgba([255u8, 0u8, 0u8, 0u8]),
                        Rgba([0u8, 255u8, 0u8, 0u8]),
                        Rgba([0u8, 0u8, 255u8, 0u8]),
                        Rgba([255u8, 255u8, 0u8, 0u8]),
                        Rgba([0u8, 255u8, 255u8, 0u8]),
                    ];

                    //println!(" ===== {s} izn:{}", izns.len());
                    let mut fst = false;
                    let (mut x0, mut y0, mut x1, mut y1) = (0f32, 0f32, 0f32, 0f32);
                    for iz in &izns {
                        let zn = &zones[*iz];
                        for gn in &zn.gons {
                            for g in gn {
                                if !fst {
                                    (x0, y0, x1, y1) = (g.0, g.1, g.0, g.1);
                                    fst = true;
                                }
                                x0 = x0.min(g.0);
                                y0 = y0.min(g.1);
                                x1 = x1.max(g.0);
                                y1 = y1.max(g.1);
                            }
                        }
                    }

                    //println!("ext: {x0},{y0} - {x1},{y1}");
                    let (ox, oy) = ((x1 + x0) * 0.5f32, (y1 + y0) * 0.5f32);
                    let wd = x1 - x0;
                    let (o_ld, o_ln) = utm_latlong(ox, oy);
                    //println!("cent: {ox},{oy} => {o_ld},{o_ln}");
                    let zm = meter_pixel_to_zoom_lat(wd, ww as u32, o_ld);
                    let mtpx = zoom_to_meter_pixel_lat(zm, o_ld);
                    let ex_x = mtpx * ww;
                    let ex_y = mtpx * hh;
                    let or_x = ox - ex_x / 2.0;
                    let or_y = oy - ex_y / 2.0;

                    /*
                    println!("  wd:{wd} zm:{zm} ld,ln:{o_ld},{o_ln}");
                    println!("    sb:{sb_x},{sb_y} dd:{ex_x},{ex_y}  or:{or_x},{or_y}");
                    println!("    offs:{} izns:{}", offs.len(), izns.len());
                    */
                    //let (mut fg1, mut fg2) = (false, false);
                    //let fimg1 = format!("{odir1}/{s}.jpeg");
                    //let fimg2 = format!("{odir2}/{s}.jpeg");
                    //let fimg3 = format!("{odir3}/{s}.jpeg");
                    //let fimg4 = format!("{odir4}/{s}.jpeg");
                    let fimg5 = format!("{odir5}/{s}.jpeg");
                    let fimg6 = format!("{odir6}/{s}.jpeg");

                    let mut fg5 = false;
                    loop {
                        if !std::path::Path::new(fimg5.as_str()).exists() {
                            let url = format!(
                    "https://www.google.com/maps/@?api=1&map_action=map&center={o_ld},{o_ln}&zoom={zm}&basemap=satellite");
                            println!("   img5: h:{h}");
                            let bnd = Bounds::Normal {
                                left: None,
                                top: None,
                                width: Some(w.into()),
                                //height: Some(h.into()),
                                height: Some(w.into()),
                            };
                            let browser = Browser::default()?;
                            let tab = browser.new_tab()?;
                            if let Ok(_) = tab.navigate_to(&url) {
                            } else {
                                println!("!!! fail to navigate to");
                                continue;
                            }
                            if let Ok(_) = tab.set_bounds(bnd) {
                            } else {
                                println!("!!! fail to set bound");
                                continue;
                            }
                            if let Ok(_) = tab.wait_until_navigated() {
                            } else {
                                println!("!!! fail to wait");
                                continue;
                            }
                            let ten_millis = time::Duration::from_millis(2000);
                            thread::sleep(ten_millis);
                            let jpeg_data = tab.capture_screenshot(
                                Page::CaptureScreenshotFormatOption::Jpeg,
                                None,
                                None,
                                true,
                            )?;
                            std::fs::write(&fimg5, jpeg_data)?;
                            println!("img1 = {url}");

                            fg5 = true;
                        } else {
                            println!("{s} image 1 skipped");
                        }
                        break;
                    }

                    if fg5 || !std::path::Path::new(fimg6.as_str()).exists() {
                        if let Ok(img) = ImageReader::open(&fimg5) {
                            if let Ok(mut img) = img.decode() {
                                let (w, h) = (img.width(), img.height());
                                //println!(" hh:{hh} h:{h} updw:{updw}");
                                let mut img = img.crop(mg, updw, w - mg, h - updw * 2);
                                let x = (sb_x - or_x) * ww / ex_x - ofs_x;
                                let y = (sb_y - or_y) * hh / ex_y;
                                /*
                                println!(
                                    " {}/{} = {} hh:{hh} y:{y}",
                                    sb_y - or_y,
                                    ex_y,
                                    (sb_y - or_y) / ex_y
                                );
                                */
                                //let mut zon_pols = vec![];
                                for (_i, iz) in izns.iter().enumerate() {
                                    let zn = &zones[*iz];
                                    //println!(" n:{} c:{}", zn.name, zn.code);
                                    //println!("  z: {i} - {iz}");
                                    //let mut pols = vec![];
                                    for z in &zn.gons {
                                        let mut pnts = Vec::<Point<f32>>::new();
                                        //let mut lines = vec![];
                                        //println!("    z:  - {}", z.len());
                                        let mut pn0 = Point::<f32>::new(0.0, 0.0);
                                        let mut pn1 = Point::<f32>::new(0.0, 0.0); // last point
                                        for (i, pt) in z.iter().enumerate() {
                                            let x = (pt.0 - or_x) * ww / ex_x - ofs_x;
                                            let y = hh - (pt.1 - or_y) * hh / ex_y;
                                            let pn2 = Point::<f32>::new(x, y);
                                            if pn2 != pn1 && pn2 != pn0 {
                                                pn1 = pn2;
                                                pnts.push(pn2);
                                                //lines.push(coord! { x: x as f32, y: y as f32, });
                                                if i == 0 {
                                                    pn0 = pn2;
                                                }
                                            }
                                        }
                                        if pnts.len() > 1 {
                                            //println!("    draw {pnts:?}");
                                            draw_hollow_polygon_mut(&mut img, &pnts, wht);
                                            //let line_string = LineString::new(lines);
                                            //let polygon = Polygon::new(line_string.clone(), vec![]);
                                            //pols.push(polygon);
                                        }
                                    }
                                    //zon_pols.push(pols);
                                }

                                /*
                                let (wd, hg) = (ww as u32, hh as u32);
                                for y in 0..hg {
                                    for x in 0..wd {
                                        let pn = point!(x: x as f32, y: y as f32);
                                        for (o, pols) in zon_pols.iter().enumerate() {
                                            for pi in pols.iter() {
                                                if pi.contains(&pn) {
                                                    let oo = o % cols.len();
                                                    let pixel = img.get_pixel(x, y);
                                                    let pixel = interpolate(pixel, cols[oo], 0.8);
                                                    img.put_pixel(x, y, pixel);
                                                }
                                            }
                                        }
                                    }
                                }
                                */

                                let y = hh - y;
                                draw_filled_rect_mut(
                                    &mut img,
                                    imageproc::rect::Rect::at(x as i32 - 15, y as i32 - 15)
                                        .of_size(30, 30),
                                    wht,
                                );
                                draw_hollow_rect_mut(
                                    &mut img,
                                    imageproc::rect::Rect::at(x as i32 - 15, y as i32 - 15)
                                        .of_size(30, 30),
                                    blk,
                                );
                                img.save(&fimg6).expect("?");
                            }
                            println!("fimg6: {fimg6}");
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

use sglab02_lib::sg::prc1::SubstInfo;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct GridArea {
    pub arid: String,
    pub name: String,
    pub provs: HashSet<String>,
    pub subs: HashSet<String>,
    pub mark: bool,
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct GridProv {
    pub name: String,
    pub subs: HashSet<String>,
    pub mark: bool,
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct GridSubst {
    pub sbid: String,
    pub name: String,
    pub prov: String,
    pub utm: (f32, f32),
    pub latlon: (f32, f32),
    pub conf: String,
    pub brns: HashMap<String, BranchAccount>,
    pub zons: HashMap<String, ZoneAccount>,
    pub txs: Vec<Transformer>,
    pub tr_x_ex: f32,
    pub tr_zoom: u32,
    pub tr_mt_px: f32,
    pub mark: bool,
    pub sbif: SubstInfo,
}

use sglab02_lib::sg::prc4::ld_pv_sbv_m;

pub fn map3() -> Result<(), Box<dyn Error>> {
    let adjxy = sub_latlong_adjust();
    let sbif = ld_p3_sub_inf();
    let ww = MP_WW;
    //let pv = grp1();
    let sbsl = ld_pv_sbv_m();

    let mut ar_inf = HashMap::<String, GridArea>::new();
    let mut pv_inf = HashMap::<String, GridProv>::new();
    let mut sb_inf = HashMap::<String, GridSubst>::new();
    let mut mkcn = 0;
    let mut subs = Vec::<String>::new();
    for (sbid, sbif) in &sbif {
        let mut pvmk = false;
        let mut sbmk = false;
        if let Some(sbv) = sbsl.get(&sbif.prov) {
            pvmk = true;
            let c = sbv
                .iter()
                .map(|p41| if p41.sbid == *sbid { 1 } else { 0 })
                .sum::<u32>();
            if c > 0 {
                sbmk = true;
            }
        }
        if sbmk {
            mkcn += 1;
            subs.push(sbid.to_string());
        }
        if let Some(ga) = ar_inf.get_mut(&sbif.arid) {
            ga.provs.insert(sbif.prov.to_string());
            ga.subs.insert(sbid.to_string());
            ga.mark = pvmk;
        } else {
            let mut provs = HashSet::<String>::new();
            provs.insert(sbif.prov.to_string());
            let mut subs = HashSet::<String>::new();
            subs.insert(sbid.to_string());
            let ga = GridArea {
                arid: sbif.arid.to_string(),
                name: sbif.area.to_string(),
                provs,
                subs,
                mark: pvmk,
            };
            ar_inf.insert(sbif.arid.to_string(), ga);
        }
        if let Some(gp) = pv_inf.get_mut(&sbif.prov) {
            gp.subs.insert(sbid.to_string());
        } else {
            let mut subs = HashSet::<String>::new();
            subs.insert(sbid.to_string());
            let gp = GridProv {
                name: sbif.prov.to_string(),
                subs,
                mark: pvmk,
            };
            pv_inf.insert(sbif.prov.to_string(), gp);
        }
        let gs = GridSubst {
            sbid: sbid.to_string(),
            name: sbif.name.to_string(),
            prov: sbif.prov.to_string(),
            mark: sbmk,
            sbif: sbif.clone(),
            ..Default::default()
        };
        sb_inf.insert(sbid.to_string(), gs);
    }

    let zncd = zone_code();
    let mut txcn = 0usize;
    for (aid, a) in &ar_inf {
        println!("AREA {} - {}", aid, a.name);
        let trxs = ld_dtrans(aid)?;
        let brns = ld_brns(aid)?;
        let zones = ld_zones(aid)?;
        let subazm = ld_subazm(aid)?;
        for (s, sb) in &subazm {
            let sbid = s.to_string();
            if let Some(gs) = sb_inf.get_mut(&sbid) {
                let mut txs = vec![];
                for ti in &sb.trxs {
                    let tx = trxs[*ti].clone();
                    for u in &trxs[*ti].offs {
                        if let Some(brac) = gs.brns.get_mut(&brns[*u].id) {
                            brac.trxs.push(txs.len());
                        } else {
                            let brac = BranchAccount {
                                id: brns[*u].id.clone(),
                                name: brns[*u].name.clone(),
                                gons: brns[*u].gons.clone(),
                                trxs: vec![txs.len()],
                            };
                            gs.brns.insert(brac.id.clone(), brac);
                        }
                    }
                    for u in &trxs[*ti].izns {
                        if let Some(znac) = gs.zons.get_mut(&zones[*u].name) {
                            znac.trxs.push(txs.len());
                        } else {
                            let desc = zncd.get(zones[*u].code.as_str()).unwrap().to_string();
                            let znac = ZoneAccount {
                                name: zones[*u].name.clone(),
                                code: zones[*u].code.clone(),
                                gons: zones[*u].gons.clone(),
                                desc,
                                trxs: vec![txs.len()],
                            };
                            gs.zons.insert(znac.name.clone(), znac);
                        }
                    }
                    txs.push(tx);
                }
                /*
                let wd = x1 - x0;
                let (ld, ln) = utm_latlong(x0, y0);
                let zm = meter_pixel_to_zoom_lat(wd, ww as u32, ld);
                println!(
                    "  sub: {s} b:{} z:{} tx:{}={} wd:{wd} zm:{zm}",
                    gs.brns.len(),
                    gs.zons.len(),
                    txs.len(),
                    sb.trxs.len(),
                );
                */
                txcn += txs.len();
                gs.txs = txs;
            } else {
                println!("ERROR : {sbid}");
            }
        }
        println!("TX: {txcn}");
    }

    let re = Regex::new(r"q=([0-9]+\.[0-9]+),([0-9]+\.[0-9]+)").unwrap();
    for (s, c, g) in &SUB_TYPES {
        if let Some(cap) = re.captures_iter(g).next() {
            let sbid = s.to_string();
            let x = &cap[1];
            let y = &cap[2];
            let mut xx = x.parse::<f32>().unwrap();
            let mut yy = y.parse::<f32>().unwrap();
            if let Some((x1, y1)) = adjxy.get(&sbid) {
                xx += *x1;
                yy += *y1;
            }
            let sbid = s.to_string();
            let (sb_x, sb_y) = latlong_utm(xx, yy);
            if let Some(gs) = sb_inf.get_mut(&sbid) {
                let (mut x0, mut x1, mut y0, mut y1) = (0f32, 0f32, 0f32, 0f32);
                let mut fst = true;
                for tx in &gs.txs {
                    if fst {
                        x0 = tx.x;
                        x1 = tx.x;
                        y0 = tx.y;
                        y1 = tx.y;
                        fst = false;
                    }
                    x0 = x0.min(tx.x);
                    y0 = y0.min(tx.y);
                    x1 = x1.max(tx.x);
                    y1 = y1.max(tx.y);
                }
                let wd = x1 - x0;
                let wd2 = y1 - y0;
                let wd = if wd2 > wd { wd2 } else { wd };
                let zm = meter_pixel_to_zoom_lat(wd, ww as u32, xx);
                let mtpx = zoom_to_meter_pixel_lat(zm, xx);
                println!("SUB: {s} z:{zm} wd:{wd} mtpx:{mtpx}");

                gs.latlon = (xx, yy);
                gs.utm = (sb_x, sb_y);
                gs.conf = c.to_string();
                gs.tr_x_ex = wd;
                gs.tr_zoom = zm;
                gs.tr_mt_px = mtpx;
            } else {
                println!("ERROR {s}");
            }
        }
    }

    let odir = "../sgdata/trxoaj";
    std::fs::create_dir_all(odir).expect("?");

    let fnm = format!("{odir}/ar_inf.bin");
    if let Ok(se) = bincode::serialize(&ar_inf) {
        std::fs::write(fnm, se).unwrap();
    }
    let fnm = format!("{odir}/pv_inf.bin");
    if let Ok(se) = bincode::serialize(&pv_inf) {
        std::fs::write(fnm, se).unwrap();
    }
    let fnm = format!("{odir}/sb_inf.bin");
    if let Ok(se) = bincode::serialize(&sb_inf) {
        std::fs::write(fnm, se).unwrap();
    }
    println!("====== MARK CNT : {mkcn}");
    if let Ok(se) = bincode::serialize(&subs) {
        println!("write sele sub {}", subs.len());
        std::fs::write("/mnt/e/CHMBACK/pea-data/data1/sele_subs.bin", se)?;
    }

    Ok(())
}

pub fn ld_ar_inf() -> Result<HashMap<String, GridArea>, Box<dyn Error>> {
    let odir = "../sgdata/trxoaj";
    let f = File::open(format!("{odir}/ar_inf.bin"))?;
    Ok(bincode::deserialize_from::<
        BufReader<File>,
        HashMap<String, GridArea>,
    >(BufReader::new(f))?)
}

pub fn ld_pv_inf() -> Result<HashMap<String, GridProv>, Box<dyn Error>> {
    let odir = "../sgdata/trxoaj";
    let f = File::open(format!("{odir}/pv_inf.bin"))?;
    Ok(bincode::deserialize_from::<
        BufReader<File>,
        HashMap<String, GridProv>,
    >(BufReader::new(f))?)
}

pub fn ld_sb_inf() -> Result<HashMap<String, GridSubst>, Box<dyn Error>> {
    let odir = "../sgdata/trxoaj";
    let f = File::open(format!("{odir}/sb_inf.bin"))?;
    Ok(bincode::deserialize_from::<
        BufReader<File>,
        HashMap<String, GridSubst>,
    >(BufReader::new(f))?)
}

pub fn ld_map3() -> Result<(), Box<dyn Error>> {
    let ar_inf = ld_ar_inf()?;
    let pv_inf = ld_pv_inf()?;
    let sb_inf = ld_sb_inf()?;
    println!(
        "ar:{} pv:{} sb:{}",
        ar_inf.len(),
        pv_inf.len(),
        sb_inf.len()
    );
    Ok(())
}

use sglib03::drw::sb_dr5::SubDraw5;
use sglib03::prc2::SubGraphDraw5;
use sglib03::prc2::LP_PNG_DIR;
use sglib03::prc4::ld_ben_bess1;
use sglib03::prc4::SubBenInfo;

pub fn sub_lp_draw1() -> Result<(), Box<dyn Error>> {
    let sb_inf = ld_sb_inf()?;
    println!("draw lp");
    let mut cnt = 0;
    let mut cn1 = 0;
    let mut cn2 = 0;
    for (sbid, _gs) in sb_inf {
        //println!("=== {sbid}");

        let year = "2025";
        let fdir = format!("{}/{}/{}_dr5", LP_PNG_DIR, year, sbid);
        let _ = std::fs::create_dir_all(&fdir);
        let fnm = format!("{}/{}.png", fdir, sbid);
        //println!("fnm:{fnm}");

        let sbbe: SubBenInfo = ld_ben_bess1(&sbid);
        if sbbe.sub != sbid {
            //println!("==== {sbid} === {} ===", sbbe.sub);
            cn1 += 1;
            continue;
        }
        //println!("{sbbe:?}");

        let yr = year.parse::<usize>().unwrap();
        let mut lp = vec![0f32; 48];
        let mut ss = 0f32;
        for yb in &sbbe.yrben {
            if yb.year == yr {
                lp = yb.day_prof.clone();
                ss = lp.iter().filter(|n| n.is_nan()).map(|_| 1f32).sum::<f32>();
                break;
            }
        }
        if ss > 0f32 {
            println!(" NaN data in {sbid} ====== ");
            cn2 += 1;
            continue;
        }
        let mut rf = Vec::<(String, f32)>::new();
        rf.push(("trlm".to_string(), sbbe.trlm));
        rf.push(("trcr".to_string(), sbbe.trcr));
        let mut sld = SubGraphDraw5 {
            sub: sbid.to_string(),
            fnm: fnm.clone(),
            lp,
            rf,
            yr: format!("{}", yr),
            ..Default::default() //sz: (400, 300),
        };
        sld.sz = (400, 300);
        if let Ok(bb) = sld.draw_prof() {
            cnt += 1;
            println!("draw: {fnm} - {}", bb.len());
        }
        //break;
    }
    println!("CNT {cnt} {cn1} {cn2}");
    Ok(())
}

pub const OFS_X: f32 = 40f32;
pub const COL_BLK: Rgba<u8> = Rgba([0u8, 0u8, 0u8, 0u8]);
pub const COL_WHT: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 0u8]);
pub const COL_PCK1: [Rgba<u8>; 5] = [
    Rgba([0u8, 255u8, 0u8, 0u8]),
    Rgba([0u8, 0u8, 255u8, 0u8]),
    Rgba([255u8, 255u8, 0u8, 0u8]),
    Rgba([0u8, 255u8, 255u8, 0u8]),
    Rgba([255u8, 0u8, 0u8, 0u8]),
];

pub fn map1a() -> Result<(), Box<dyn Error>> {
    let odir7 = "../sgdata/sub_img7/";
    std::fs::create_dir_all(odir7).expect("?");
    let mg = MP_MG;
    let updw = MP_UPDW;
    let ww = MP_WW;
    let hh = MP_HH;
    let (w, _h) = (mg as f32 + ww, updw as f32 * 2.0 + hh);
    let sb_inf = ld_sb_inf()?;
    let keys = sb_inf.keys();
    for s in keys {
        let gs = sb_inf.get(s).unwrap();
        let ex_x = gs.tr_mt_px * ww;
        let ex_y = gs.tr_mt_px * hh;
        let or_x = gs.utm.0 - ex_x / 2.0;
        let or_y = gs.utm.1 - ex_y / 2.0;
        let fimg7 = format!("{odir7}/{s}.jpeg");

        if gs.tr_zoom == 0 {
            continue;
        }

        loop {
            if !std::path::Path::new(fimg7.as_str()).exists() {
                let url = format!(
                    "https://www.google.pl/maps/@{},{},{}z",
                    gs.latlon.0, gs.latlon.1, gs.tr_zoom
                );
                println!("{s} loading {url}");
                let bnd = Bounds::Normal {
                    left: None,
                    top: None,
                    width: Some(w.into()),
                    height: Some(w.into()),
                };
                let browser = Browser::default()?;
                let tab = browser.new_tab()?;
                if tab.navigate_to(&url).is_err() {
                    println!("!!! fail to navigate to");
                    continue;
                }
                if tab.set_bounds(bnd).is_err() {
                    println!("!!! fail to set bound");
                    continue;
                }
                if tab.wait_until_navigated().is_err() {
                    println!("!!! fail to wait");
                    continue;
                }
                let ten_millis = time::Duration::from_millis(1000);
                thread::sleep(ten_millis);
                let jpeg_data = tab.capture_screenshot(
                    Page::CaptureScreenshotFormatOption::Jpeg,
                    None,
                    None,
                    true,
                )?;
                std::fs::write(&fimg7, jpeg_data)?;
                if let Ok(img) = ImageReader::open(&fimg7) {
                    if let Ok(mut img) = img.decode() {
                        let (w, h) = (img.width(), img.height());
                        let mut img = img.crop(mg, updw, w - mg, h - updw * 2);
                        let x = (gs.utm.0 - or_x) * ww / ex_x;
                        let y = (gs.utm.1 - or_y) * hh / ex_y;
                        let y = hh - y;
                        draw_filled_rect_mut(
                            &mut img,
                            imageproc::rect::Rect::at(x as i32 - 15, y as i32 - 15).of_size(30, 30),
                            COL_WHT,
                        );
                        draw_hollow_rect_mut(
                            &mut img,
                            imageproc::rect::Rect::at(x as i32 - 15, y as i32 - 15).of_size(30, 30),
                            COL_BLK,
                        );
                        img.save(&fimg7).expect("?");
                    }
                }
            } else {
                println!("{s} skipped");
            }
            break;
        }
    }
    Ok(())
}

pub fn map4() -> Result<(), Box<dyn Error>> {
    Ok(())
}

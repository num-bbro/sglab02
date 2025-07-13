use geo::polygon;
use geo::Area;
use geo::ClosestPoint;
use geo_types::{point, Geometry, GeometryCollection, Point};
//use image::codecs::qoi;
use core::f32;
use sglab02_lib::sg::mvline::utm_latlong;
use std::error::Error;
//use std::io::BufRead;

pub fn chk1() -> Result<(), Box<dyn Error>> {
    println!("chk1");
    let mut pes = vec![];
    for i in 0..10 {
        let f = i as f32;
        let p = point!(x: f, y: f);
        let pe = Geometry::Point(p);
        pes.push(pe);
    }
    let gc = GeometryCollection::new_from(pes);

    let p: Point<f32> = Point::new(5.2, 5.3);
    let c = gc.closest_point(&p);
    println!("Closest: {c:?}");

    Ok(())
}

pub fn chk2() -> Result<(), Box<dyn Error>> {
    let mut polygon = polygon![
        (x: 0., y: 0.),
        (x: 5., y: 0.),
        (x: 5., y: 6.),
        (x: 0., y: 6.),
        (x: 0., y: 0.),
    ];

    assert_eq!(polygon.signed_area(), 30.);
    assert_eq!(polygon.unsigned_area(), 30.);

    polygon.exterior_mut(|line_string| {
        line_string.0.reverse();
    });

    assert_eq!(polygon.signed_area(), -30.);
    assert_eq!(polygon.unsigned_area(), 30.);

    Ok(())
}

use serde::{Deserialize, Serialize};
use sglab02_lib::sg::imp::xlsx_data;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmpPop {
    pub prv: String,
    pub amp: String,
    pub pop: i32,
    pub home: i32,
}

use regex::Regex;
use std::collections::HashMap;

pub async fn amp_popu() -> Result<(), Box<dyn Error>> {
    let flst = vec![format!("../inp/stat_a67_ampho.xlsx")];
    println!("{:?}", flst);
    let re = Regex::new(r"ท้องถิ่น(.+)").unwrap();
    if let Ok(xlsv) = xlsx_data(&flst).await {
        let mut ampos = vec![];
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
            for (i, r) in x.data.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                let prv = r[2].to_string();
                let amp = r[4].to_string();
                let pop = r[11].parse::<i32>()?;
                let home = r[12].parse::<i32>()?;
                let ampo = AmpPop {
                    prv,
                    amp,
                    pop,
                    home,
                };
                ampos.push(ampo);
            }
        }
        let mut muni = HashMap::<String, AmpPop>::new();
        let mut ampm = HashMap::<String, AmpPop>::new();
        let (mut c1, mut c2, mut c3) = (0, 0, 0);
        let mut prv = "".to_string();
        for ap in &ampos {
            if let Some(cap) = re.captures_iter(&ap.amp.as_str()).next() {
                c1 += 1;
                let x = &cap[1];
                if prv == "กรุงเทพมหานคร" {
                    ampm.insert(x.to_string(), ap.clone());
                } else {
                    muni.insert(x.to_string(), ap.clone());
                    println!("MU: {c1}.{x} in {prv}");
                }
            } else if ap.amp == "-" {
                prv = ap.prv.clone();
                c2 += 1;
            } else {
                ampm.insert(ap.amp.to_string(), ap.clone());
                c3 += 1;
            }
        }
        println!("c1:{c1} c2:{c2} c3:{c3}");
        if let Ok(ser) = bincode::serialize(&ampm) {
            std::fs::write("../sgdata/amp_pop.bin", ser).unwrap();
        }
        if let Ok(ser) = bincode::serialize(&muni) {
            std::fs::write("../sgdata/mun_pop.bin", ser).unwrap();
        }
    }
    Ok(())
}

const FILES: [&str; 3] = [
    "amphur_wgs84_z47",
    "municipal_wgs84_z47",
    "province_wgs84_z47",
];

use shapefile::dbase;

pub fn poly_read1() -> Result<(), Box<dyn Error>> {
    println!("poly read");
    for f in &FILES {
        let fnm = format!("../inp/{f}/{f}.shp");
        println!("f: {fnm}");

        if let Ok(mut reader) = shapefile::Reader::from_path(fnm.clone()) {
            let mut vrg = vec![];
            for result in reader.iter_shapes_and_records_as::<shapefile::Polygon, dbase::Record>() {
                if let Ok((gon, _rc)) = result {
                    let mut ringpnts = Vec::<Vec<(f64, f64)>>::new();
                    for ring in gon.into_inner() {
                        let mut pnts = Vec::<(f64, f64)>::new();
                        for pnt in ring.into_inner() {
                            pnts.push((pnt.x, pnt.y));
                        }
                        ringpnts.push(pnts);
                    }
                    vrg.push(ringpnts);
                }
            }
            println!("vrg: {}", vrg.len());
            let fou = format!("../sgdata/gis/{f}.rg");
            if let Ok(ser) = bincode::serialize(&vrg) {
                std::fs::write(fou, ser).unwrap();
            }
        }
    }
    Ok(())
}

use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MeterBill {
    pub trsg: String,
    pub pea: String,
    pub ca: String,
    pub inst: String,
    pub rate: String,
    pub volt: String,
    pub mru: String,
    pub mat: String,
    pub main: String,
    pub kwh15: f32,
    pub kwh18: f32,
    pub amt19: f32,
    pub ar: String,
    pub idx: usize,
    pub meth: i32,
}

// billing file to bin
pub async fn read_bill1() -> Result<(), Box<dyn std::error::Error>> {
    let fdir = "/mnt/e/CHMBACK/pea-data/20240801_กรอ";
    let fout = "/mnt/e/CHMBACK/pea-data/data1";
    let flst = vec!["202402", "202405"];
    //let _csv_v = Vec::<CSVFile>::new();
    let mut rate = HashMap::<String, usize>::new();
    let mut volt = HashMap::<String, usize>::new();
    //let mut cn = 0;
    for f in flst {
        let fln = format!("{fdir}/export_กรอ_bil013_{}.csv", f);
        let fou = format!("{fout}/{}_bil.bin", f);
        println!("start {}", &fln);
        if let Ok(mut rdr) = csv::Reader::from_path(&fln) {
            let mut bils = Vec::<MeterBill>::new();
            for rc in rdr.records().flatten() {
                // if the record exist
                if let (
                    Some(c0),
                    Some(c2),
                    Some(c3),
                    Some(c4),
                    Some(c5),
                    Some(c6),
                    Some(c7),
                    Some(c8),
                    Some(c9),
                    Some(c15),
                    Some(c18),
                    Some(c19),
                ) = (
                    rc.get(0),
                    rc.get(2),
                    rc.get(3),
                    rc.get(4),
                    rc.get(5),
                    rc.get(6),
                    rc.get(7),
                    rc.get(8),
                    rc.get(9),
                    rc.get(15),
                    rc.get(18),
                    rc.get(19),
                ) {
                    if let (Ok(n15), Ok(n18), Ok(n19)) =
                        (c15.parse::<f32>(), c18.parse::<f32>(), c19.parse::<f32>())
                    {
                        if let Some(rt) = rate.get_mut(c4) {
                            *rt += 1;
                        } else {
                            rate.insert(c4.to_string(), 1);
                        }
                        if let Some(vo) = volt.get_mut(c5) {
                            *vo += 1;
                        } else {
                            volt.insert(c5.to_string(), 1);
                        }
                        let mb0 = MeterBill {
                            trsg: c0.trim().to_string(),
                            pea: c7.trim().to_string(),
                            ca: c2.trim().to_string(),
                            inst: c3.trim().to_string(),
                            rate: c4.to_string(),
                            volt: c5.to_string(),
                            mru: c6.to_string(),
                            mat: c8.trim().to_string(),
                            main: c9.trim().to_string(),
                            kwh15: n15,
                            kwh18: n18,
                            amt19: n19,
                            ..Default::default()
                        };
                        if !c9.is_empty() {
                            //println!("c9: {c9} {} {}", mb0.kwh15, mb0.amt19);
                        }
                        bils.push(mb0);
                    }
                }
            } // loop all rec
            println!("rate: {rate:?}");
            println!("volt: {volt:?}");
            println!("write {fou}");
            if let Ok(ser) = bincode::serialize(&bils) {
                std::fs::write(&fou, ser)?;
            }
            println!("writen {fln}");
        }
    }
    println!("FINISHED");
    Ok(())
}

pub const DB2_DIR: &str = "/mnt/e/CHMBACK/dt-2025-05-29/db2";
use crate::aoj::DbfData;
use sglab02_lib::sg::gis1::ar_list;

//use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;

pub fn utm_2_nid(x: f32, y: f32) -> String {
    let x0: f32 = x * 1000f32;
    let x1 = x0 as u32;
    let x2: [u8; 4] = x1.to_be_bytes();
    let x3 = BASE64_STANDARD.encode(x2);
    let x4 = &x3[0..6];
    let y0: f32 = y * 1000f32;
    let y1 = y0 as u32;
    let y2: [u8; 4] = y1.to_be_bytes();
    let y3 = BASE64_STANDARD.encode(y2);
    let y4 = &y3[0..6];
    format!("{x4}-{y4}")
}

pub fn nid_2_utm(id: String) -> (f32, f32) {
    let x0 = format!("{}==", &id[0..6]);
    let x1 = BASE64_STANDARD.decode(x0).unwrap();
    let x2: [u8; 4] = [x1[0], x1[1], x1[2], x1[3]];
    let x3: u32 = u32::from_be_bytes(x2);
    let x4 = x3 as f32;
    let x4 = x4 / 1000f32;

    let y0 = format!("{}==", &id[7..]);
    let y1 = BASE64_STANDARD.decode(y0).unwrap();
    let y2: [u8; 4] = [y1[0], y1[1], y1[2], y1[3]];
    let y3: u32 = u32::from_be_bytes(y2);
    let y4 = y3 as f32;
    let y4 = y4 / 1000f32;

    (x4, y4)
}

pub fn chkfld() -> Result<(), Box<dyn Error>> {
    //let lys = ["GIS_HVMVCNL", "GIS_LVCNL"];
    //_Zone_use
    let db = format!("{DB2_DIR}/N1_Zone_use.at");
    //let db = format!("{DB2_DIR}/{x}_DS_HVPrimaryMeter.at");
    //let lys = ["DS_LowVoltageMeter", "DS_HVPrimaryMeter", "DS_PrimaryMeter"];
    //let lys = ["DS_HVTransformer", "DS_HVTransformer"];
    //let lys = ["GIS_EQ_SWITCH", "GIS_EQ_SOURCE", "GIS_EQ_ASSIST"];
    //let db = "/mnt/e/CHMBACK/pea-data/inp1/gis/LB_AOJ_Merge_Polygon.at".to_string();
    //let db = "/mnt/e/CHMBACK/pea-data/inp1/gis/amphur_wgs84_z47.at".to_string();
    //let db = "/mnt/e/CHMBACK/pea-data/inp1/gis/municipal_wgs84_z47.at".to_string();
    println!("{db}");
    if let Ok(fat) = File::open(&db) {
        let bat = BufReader::new(fat);
        if let Ok(at) =
            bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(bat)
        {
            for r in at {
                for (k, v) in r {
                    println!("{k} - {v:?}");
                }
                break;
            }
        }
    }
    /*
    //let lys = ["DS_CircuitBreaker"];
    for _a in ar_list() {
        let a = "N2";
        //let ly = "GIS_HVMVCNL";
        //for (ly, _) in GIS_EQ_SWITCH {
        //for (ly, _) in GIS_EQ_SOURCE {
        //for (ly, _) in GIS_EQ_ASSIST {
        //for ly in lys {
        for ly in gis_line_lays() {
            let db = format!("{DB2_DIR}/{a}_{ly}.at");
            println!();
            println!("=== {ly}");
            if let Ok(fat) = File::open(&db) {
                let bat = BufReader::new(fat);
                if let Ok(at) =
                    bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(bat)
                {
                    for r in at {
                        for (k, v) in r {
                            println!("{k} - {v:?}");
                        }
                        break;
                    }
                }
            }
        }
        //}
        break;
    }
    */
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NclData {
    pub inst: String,
    pub lv: bool,
    pub pea: String,
    pub fid: String,
    pub utm: (f32, f32),
    pub nid: String,
    pub n1d: u64,
    pub aoj: String,
    pub trx: String,
    pub ar: String,
    pub ly: String,
    pub ix: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CnlData {
    pub mt_ins: Option<String>,
    pub mt_pea: Option<String>,
    pub mt_tag: Option<String>,
    pub mt_phs: Option<String>,
    pub mt_x: Option<f32>,
    pub mt_y: Option<f32>,
    pub mt_lt: Option<f32>,
    pub mt_ln: Option<f32>,
    pub mt_aoj: Option<String>,
    pub tr_tag: Option<String>,
    pub tr_fid: Option<String>,
    pub tr_lt: Option<f32>,
    pub tr_ln: Option<f32>,
    pub tr_cd: Option<f32>,
    pub tr_aoj: Option<String>,
    pub tr_pea: Option<String>,
    pub tr_kva: Option<f32>,
    pub tr_own: Option<String>,
    pub tr_loc: Option<String>,
    pub tr_n1d: Option<u64>,
    pub mt_n1d: Option<u64>,
    pub ar: String,
    pub ly: String,
    pub ix: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transformer {
    pub elec: Option<f32>,
    pub en: Option<f32>,
    pub pea: Option<String>,
    pub fid: Option<String>,
    pub fid2: Option<String>,
    pub fif: Option<f32>,
    pub ldmva: Option<f32>,
    pub ldmw: Option<f32>,
    pub loc: Option<String>,
    pub opv: Option<String>,
    pub volt: Option<f32>,
    pub own: Option<String>,
    pub mva: Option<f32>,
    pub code: Option<f32>,
    pub tag: Option<String>,
    pub n1d: u64,
    pub ar: String,
    pub ly: String,
    pub ix: usize,
}

pub fn n1d_2_utm(n1d: u64) -> (f32, f32) {
    let mut c: u64 = 1;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    for i in 0..32 {
        let sb = i * 2; // 0, 2, 4, 6, .. 62
        let yb = n1d & c;
        let yb = yb >> (sb - i);
        y |= yb as u32;
        c <<= 1;

        let sb = sb + 1; // 1, 3, 5, 7,.. 63
        let xb = n1d & c;
        let xb = xb >> (sb - i);
        x |= xb as u32;
        c <<= 1;
    }
    let x = x as f32 / 1000_f32;
    let y = y as f32 / 1000_f32;
    (x, y)
}

pub fn chk3() -> Result<(), Box<dyn Error>> {
    let (x, y) = (13.123_f32, 100.456_f32);
    let u = utm_2_n1d(x, y);
    println!("utm: {x} {y}");
    println!("n1d: {u}");
    let (x, y) = n1d_2_utm(u);
    println!("utm: {x} {y}");
    Ok(())
}

pub fn utm_2_n1d(x: f32, y: f32) -> u64 {
    let x1 = x * 1000f32;
    let y1 = y * 1000f32;
    let mut a1: u64 = x1 as u64;
    let mut b1: u64 = y1 as u64;
    let mut a2: u64 = 0;
    let mut b2: u64 = 0;
    let mut c: u64 = 1;

    for _i in 1..=32 {
        a2 |= a1 & c;
        b2 |= b1 & c;
        a1 <<= 1;
        b1 <<= 1;
        c <<= 2;
    }
    a2 <<= 1;
    a2 | b2
}

pub fn cnl_ins_mp() -> Result<(), Box<dyn Error>> {
    let mut cnl_ins_mp = HashMap::<String, (String, usize)>::new();
    let mut cnl_pea_mp = HashMap::<String, (String, usize)>::new();
    let mut cn = 0;
    let mut cn2 = 0;
    for ar in ar_list() {
        let fin = format!("/mnt/e/CHMBACK/pea-data/data1/rdcnl2_{ar}.bin");
        println!("fin: {fin}");
        if let Ok(fin) = File::open(&fin) {
            let bat = BufReader::new(fin);
            if let Ok(ncls) = bincode::deserialize_from::<BufReader<File>, Vec<NclData>>(bat) {
                for (i, ncl) in ncls.iter().enumerate() {
                    let ins = ncl.inst.to_string();
                    let pea = ncl.pea.to_string();
                    if cnl_ins_mp.get(&ins).is_some() {
                        cn += 1;
                        //println!("i:{cn}.{ins}");
                    } else {
                        cnl_ins_mp.insert(ins, (ar.to_string(), i));
                    }
                    if cnl_pea_mp.get(&pea).is_some() {
                        cn2 += 1;
                        //println!("p:{cn2}.{pea}");
                    } else {
                        cnl_pea_mp.insert(pea, (ar.to_string(), i));
                    }
                }
            }
        }
    }
    if let Ok(ser) = bincode::serialize(&cnl_ins_mp) {
        std::fs::write("/mnt/e/CHMBACK/pea-data/data1/cnl_ins_mp.bin", ser).unwrap();
    }
    if let Ok(ser) = bincode::serialize(&cnl_pea_mp) {
        std::fs::write("/mnt/e/CHMBACK/pea-data/data1/cnl_pea_mp.bin", ser).unwrap();
    }
    println!(
        "finish {}+{cn} {}+{cn2}",
        cnl_ins_mp.len(),
        cnl_pea_mp.len(),
    );
    Ok(())
}

use sglab02_lib::sg::mvline::latlong_utm;

pub fn p2_read_cnl() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let lys = ["GIS_HVMVCNL", "GIS_LVCNL"];
        for ly in lys {
            let db = format!("{DB2_DIR}/{ar}_{ly}.at");
            println!("Area: {db} LV");
            if let Ok(fat) = File::open(&db) {
                let bat = BufReader::new(fat);
                if let Ok(at) =
                    bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(bat)
                {
                    let mut mts = Vec::<CnlData>::new();
                    let mut tr_ns = HashMap::<u64, CnlData>::new();
                    //let mut tr_ps = HashMap::<String, CnlData>::new();
                    //let mut tr_ts = HashMap::<String, CnlData>::new();
                    //let mut tr_x = Vec::<CnlData>::new();
                    for (ix, r) in at.iter().enumerate() {
                        let mt_ins = if let Some(DbfData::Text(v)) = r.get("METER_INST") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let mt_pea = if let Some(DbfData::Text(v)) = r.get("PEA_METER") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let mt_aoj = if let Some(DbfData::Text(v)) = r.get("METER_AOJ") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let mt_tag = if let Some(DbfData::Text(v)) = r.get("METER_TAG") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let mt_phs = if let Some(DbfData::Text(v)) = r.get("METER_PHAS") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let mt_x = if let Some(DbfData::Real(v)) = r.get("X_COOD") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let mt_y = if let Some(DbfData::Real(v)) = r.get("Y_COOD") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let mt_lt = if let Some(DbfData::Real(v)) = r.get("MT_LAT") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let mt_ln = if let Some(DbfData::Real(v)) = r.get("MT_LONG") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let tr_tag = if let Some(DbfData::Text(v)) = r.get("TRF_TAG") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let tr_fid = if let Some(DbfData::Text(v)) = r.get("TRF_FEEDER") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let tr_lt = if let Some(DbfData::Real(v)) = r.get("TRF_LAT") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let tr_ln = if let Some(DbfData::Real(v)) = r.get("TRF_LONG") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let tr_cd = if let Some(DbfData::Real(v)) = r.get("TRF_SUBTYP") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let tr_aoj = if let Some(DbfData::Text(v)) = r.get("TRF_AOJ") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let tr_pea = if let Some(DbfData::Text(v)) = r.get("TRF_PEA_NO") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let tr_kva = if let Some(DbfData::Real(v)) = r.get("TRF_KVA") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let tr_own = if let Some(DbfData::Text(v)) = r.get("TRF_OWNER") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let tr_loc = if let Some(DbfData::Text(v)) = r.get("TRF_LOCATI") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let mut mt_n1d = if let (Some(x), Some(y)) = (mt_x, mt_y) {
                            Some(utm_2_n1d(x, y))
                        } else {
                            None
                        };
                        let tr_n1d = if let (Some(lt), Some(ln)) = (tr_lt, tr_ln) {
                            let (x, y) = latlong_utm(lt, ln);
                            Some(utm_2_n1d(x, y))
                        } else {
                            None
                        };
                        if mt_n1d.is_none() {
                            if let Some(n1d) = tr_n1d {
                                mt_n1d = Some(n1d);
                            } else {
                                println!("no meter no trf");
                            }
                        } else if let (Some(mtn1d), Some(trn1d)) = (mt_n1d, tr_n1d) {
                            if mtn1d == 0u64 {
                                if trn1d == 0u64 {
                                    println!(" mt_n1d == 0 & tr_n1d == 0");
                                } else {
                                    mt_n1d = Some(trn1d);
                                }
                            }
                        }
                        if let None = mt_n1d {
                            println!("2. no meter location");
                        }
                        if let Some(mtn1d) = mt_n1d {
                            if mtn1d == 0u64 {
                                println!("4. no meter 0 n1d");
                            }
                        }
                        let ly = ly.to_string();
                        let ar = ar.to_string();
                        let mt = CnlData {
                            mt_ins,
                            mt_pea,
                            mt_tag,
                            mt_phs,
                            mt_x,
                            mt_y,
                            mt_lt,
                            mt_ln,
                            mt_aoj,
                            tr_tag,
                            tr_fid,
                            tr_lt,
                            tr_ln,
                            tr_cd,
                            tr_aoj,
                            tr_pea,
                            tr_kva,
                            tr_own,
                            tr_loc,
                            tr_n1d,
                            mt_n1d,
                            ar,
                            ly,
                            ix,
                        };
                        if let Some(ref tr_n1d) = mt.tr_n1d {
                            if let Some(tr) = tr_ns.get(tr_n1d) {
                                //println!("=== TR at {}", tr_n1d);
                                if tr.tr_pea != mt.tr_pea {
                                    println!(" ????... pea: {:?}-{:?}", tr.tr_pea, mt.tr_pea);
                                }
                                if tr.tr_tag != mt.tr_tag {
                                    println!(" ????... tag: {:?}-{:?}", tr.tr_tag, mt.tr_tag);
                                }
                                if tr.tr_fid != mt.tr_fid {
                                    println!(" ????... fid: {:?}-{:?}", tr.tr_fid, mt.tr_fid);
                                }
                            } else {
                                tr_ns.insert(*tr_n1d, mt.clone());
                            }
                        }
                        mts.push(mt);
                    } // end loop
                    println!("mt:{} tr_ns:{}", mts.len(), tr_ns.len(),);
                    let omt = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_mt.bin");
                    if let Ok(ser) = bincode::serialize(&mts) {
                        std::fs::write(omt, ser).unwrap();
                    }
                    let mut trs = Vec::<CnlData>::new();
                    for tr in tr_ns.values() {
                        trs.push(tr.clone());
                    }
                    let otr = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_tr.bin");
                    if let Ok(ser) = bincode::serialize(&trs) {
                        std::fs::write(otr, ser).unwrap();
                    }
                } // end dese
            } // file open
        } // layer loop
    } // area loop
    Ok(())
}

pub fn p4_check_cnl() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let fcnl = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_mt.bin");
        if let Ok(fcnl) = File::open(&fcnl) {
            let fcnl = BufReader::new(fcnl);
            if let Ok(cnls) = bincode::deserialize_from::<BufReader<File>, Vec<CnlData>>(fcnl) {
                let mut zcn = 0;
                for c in &cnls {
                    if let Some(u) = c.mt_n1d {
                        if u == 0u64 {
                            zcn += 1;
                        }
                    }
                }
                println!("CNL: {} {zcn}", cnls.len());
            }
        }
    }
    Ok(())
}

pub fn rdcnl2() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        let mut cnls = Vec::<NclData>::new();
        let lys = ["GIS_HVMVCNL", "GIS_LVCNL"];
        for ly in lys {
            let db = format!("{DB2_DIR}/{ar}_{ly}.at");
            println!("Area: {db} LV");
            if let Ok(fat) = File::open(&db) {
                let bat = BufReader::new(fat);
                if let Ok(at) =
                    bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(bat)
                {
                    let mut ix = 0;
                    for r in at {
                        let inst = if let Some(DbfData::Text(v)) = r.get("METER_INST") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let lv = true;
                        let pea = if let Some(DbfData::Text(v)) = r.get("PEA_METER") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let fid = if let Some(DbfData::Text(v)) = r.get("MT_FEEDER") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let aoj = if let Some(DbfData::Text(v)) = r.get("AOJ") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let x = if let Some(DbfData::Real(v)) = r.get("X_COOD") {
                            *v as f32
                        } else {
                            0_f32
                        };
                        let y = if let Some(DbfData::Real(v)) = r.get("Y_COOD") {
                            *v as f32
                        } else {
                            0_f32
                        };
                        let nid = utm_2_nid(x, y);
                        let n1d = utm_2_n1d(x, y);
                        let trx = if let Some(DbfData::Text(v)) = r.get("TRF_PEA_NO") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let ly = ly.to_string();
                        let ar = ar.to_string();
                        let cnl = NclData {
                            inst,
                            lv,
                            pea,
                            fid,
                            utm: (x, y),
                            nid,
                            n1d,
                            aoj,
                            trx,
                            ar,
                            ly,
                            ix,
                        };
                        ix += 1;
                        cnls.push(cnl);
                    }
                }
            }
        }
        let fout = format!("/mnt/e/CHMBACK/pea-data/data1/rdcnl2_{ar}.bin");
        println!("write to {fout}");
        if let Ok(ser) = bincode::serialize(&cnls) {
            std::fs::write(fout, ser).unwrap();
            println!("cnl {}", cnls.len());
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GisMeter {
    pub ca: String,
    pub tp: String,
    pub fid: String,
    pub pea: String,
    pub am: f32,
    pub rd: f32,
    pub rf: bool,
    pub cd: String,
    pub en: bool,
    pub inst: String,
    pub finf: f32,
    pub route: String,
    pub utm: (f32, f32),
    pub nid: String,
    pub n1d: u64,
    pub ar: String,
    pub ly: String,
    pub ix: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MeterData {
    pub ca: Option<String>,
    pub tp: Option<String>,
    pub fid: Option<String>,
    pub pea: Option<String>,
    pub am: Option<f32>,
    pub rd: Option<f32>,
    pub rf: Option<bool>,
    pub cd: Option<String>,
    pub en: Option<bool>,
    pub inst: Option<String>,
    pub finf: Option<f32>,
    pub route: Option<String>,
    pub fid2: Option<String>,
    pub own: Option<String>,
    pub opv: Option<String>,
    pub fdif: Option<f32>,
    pub loc: Option<String>,
    pub src: Option<f32>,
    pub elc: Option<f32>,
    pub tag: Option<String>,
    pub nos: Option<f32>,
    pub x: f32,
    pub y: f32,
    pub n1d: u64,
    pub ar: String,
    pub ly: String,
    pub ix: usize,
}

pub fn met_chk(tp: &str) -> HashMap<String, (String, usize)> {
    let mut cnl_ins = HashMap::<String, (String, usize)>::new();
    if let Ok(fin) = File::open(format!("/mnt/e/CHMBACK/pea-data/data1/{tp}.bin")) {
        let bat = BufReader::new(fin);
        if let Ok(cnl) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, (String, usize)>>(bat)
        {
            cnl_ins = cnl;
        }
    }
    cnl_ins
}

pub fn bill_to_area() -> Result<(), Box<dyn Error>> {
    let fout = "/mnt/e/CHMBACK/pea-data/data1";
    let flst = vec!["202402", "202405"];
    for ym in flst {
        let fbl = format!("{fout}/{ym}_bil.bin");
        let fbx = format!("{fout}/{ym}_bix.bin");
        if let (Ok(fbl), Ok(fbx)) = (File::open(&fbl), File::open(&fbx)) {
            let fbl = BufReader::new(fbl);
            let fbx = BufReader::new(fbx);
            if let (Ok(mbs), Ok(bxs)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<MeterBill>>(fbl),
                bincode::deserialize_from::<BufReader<File>, HashMap<String, Vec<usize>>>(fbx),
            ) {
                for (ar, bx) in bxs {
                    let ar = if ar.is_empty() { "xx" } else { ar.as_str() };
                    let fmb = format!("{fout}/{ym}_{ar}_bil.bin");
                    let mut arbls = Vec::<MeterBill>::new();
                    for b in bx {
                        let bl = mbs[b].clone();
                        arbls.push(bl);
                    }
                    println!("wr {fmb} - {}", arbls.len());
                    if let Ok(ser) = bincode::serialize(&arbls) {
                        std::fs::write(fmb, ser).unwrap();
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn bill_to_cnl() -> Result<(), Box<dyn Error>> {
    let flst = vec!["202402", "202405"];
    let fout = "/mnt/e/CHMBACK/pea-data/data1";
    let cnl_ins = met_chk("cnl_ins_mp");
    println!("cnl: {}", cnl_ins.len());
    let cnl_pea = met_chk("cnl_pea_mp");
    println!("pea: {}", cnl_pea.len());

    let mut cn = 0;
    for ym in flst {
        let fin = format!("{fout}/{ym}_bil.bin");
        println!("{ym} - {fin}");
        let mut vol_sm = HashMap::<String, (u32, f32)>::new();
        let (mut cc, mut am) = (0, 0f32);
        let mut ar_bil_mp = HashMap::<String, Vec<usize>>::new();
        if let Ok(fin) = File::open(&fin) {
            let bat = BufReader::new(fin);
            if let Ok(mut mbs) = bincode::deserialize_from::<BufReader<File>, Vec<MeterBill>>(bat) {
                for (i, ref mut mb) in mbs.iter_mut().enumerate() {
                    if let Some(m) = cnl_ins.get(&mb.inst) {
                        mb.ar = m.0.clone();
                        mb.idx = m.1;
                        mb.meth = 1;
                    } else if let Some(m) = cnl_pea.get(&mb.pea) {
                        mb.ar = m.0.clone();
                        mb.idx = m.1;
                        mb.meth = 2;
                    } else {
                        cn += 1;
                        if let Some((cn, am)) = vol_sm.get_mut(&mb.volt) {
                            *cn += 1;
                            *am += mb.amt19;
                        } else {
                            vol_sm.insert(mb.volt.to_string(), (1, mb.amt19));
                        }
                    }
                    if let Some(arbl) = ar_bil_mp.get_mut(&mb.ar) {
                        arbl.push(i);
                    } else {
                        ar_bil_mp.insert(mb.ar.to_string(), vec![i]);
                    }
                    cc += 1;
                    am += mb.amt19;
                }
            }
        }
        println!("bill ym: {ym}");
        let fou = format!("{fout}/{ym}_bix.bin");
        println!("wr to {fou}");
        if let Ok(ser) = bincode::serialize(&ar_bil_mp) {
            std::fs::write(fou, ser).unwrap();
        }
        let ccp = cn as f32 / cc as f32 * 100f32;
        println!("cc:{cc}:{ccp:.2}% am:{am}");
        for (k, (c, a)) in vol_sm {
            let cp = c as f32 / cc as f32 * 100f32;
            let ap = a / am * 100f32;
            println!("{k} {c}:{cp:.2}% {a}:{ap:.2}%");
        }
    }
    Ok(())
}

pub fn bill_to_met() -> Result<(), Box<dyn Error>> {
    let flst = vec!["202402", "202405"];
    let fout = "/mnt/e/CHMBACK/pea-data/data1";
    let met_ins = met_chk("met_ins_mp");
    println!("met ins: {}", met_ins.len());
    let met_cai = met_chk("met_cai_mp");
    println!("met cai: {}", met_cai.len());
    let mut cn = 0;
    for ym in flst {
        let fin = format!("{fout}/{ym}_bil.bin");
        println!("{ym} - {fin}");
        if let Ok(fin) = File::open(&fin) {
            let bat = BufReader::new(fin);
            if let Ok(mets) = bincode::deserialize_from::<BufReader<File>, Vec<MeterBill>>(bat) {
                for met in mets {
                    if let Some(_v) = met_ins.get(&met.inst) {
                    } else {
                        let cai = format!("{}-{}", met.ca, met.inst);
                        if let Some(_) = met_cai.get(&cai) {
                        } else {
                            cn += 1;
                            println!("{cn}.{}", met.inst);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn met_mp() -> Result<(), Box<dyn Error>> {
    let mut met_ins_mp = HashMap::<String, (String, usize)>::new();
    let mut met_cai_mp = HashMap::<String, (String, usize)>::new();
    let mut met_pea_mp = HashMap::<String, (String, usize)>::new();
    let (mut insn, mut cain, mut pean) = (0, 0, 0);
    for ar in ar_list() {
        let fin = format!("/mnt/e/CHMBACK/pea-data/data1/rd_met1_{ar}.bin");
        println!("fin: {fin}");
        if let Ok(fin) = File::open(&fin) {
            let bat = BufReader::new(fin);
            if let Ok(mets) = bincode::deserialize_from::<BufReader<File>, Vec<GisMeter>>(bat) {
                for (i, met) in mets.iter().enumerate() {
                    let cai = format!("{}-{}", met.ca, met.inst);
                    let ins = met.inst.to_string();
                    let pea = met.pea.to_string();
                    if met_ins_mp.get(&ins).is_some() {
                        insn += 1;
                        //println!("i:{insn}.{ins}");
                    } else {
                        met_ins_mp.insert(ins, (ar.to_string(), i));
                    }
                    if met_cai_mp.get(&cai).is_some() {
                        cain += 1;
                        //println!("i:{cain}.{cai}");
                    } else {
                        met_cai_mp.insert(cai, (ar.to_string(), i));
                    }
                    if met_pea_mp.get(&pea).is_some() {
                        pean += 1;
                        //println!("p:{pean}.{pea}");
                    } else {
                        met_pea_mp.insert(pea, (ar.to_string(), i));
                    }
                }
            }
        }
    }
    if let Ok(ser) = bincode::serialize(&met_ins_mp) {
        std::fs::write("/mnt/e/CHMBACK/pea-data/data1/met_ins_mp.bin", ser).unwrap();
    }
    if let Ok(ser) = bincode::serialize(&met_cai_mp) {
        std::fs::write("/mnt/e/CHMBACK/pea-data/data1/met_cai_mp.bin", ser).unwrap();
    }
    if let Ok(ser) = bincode::serialize(&met_pea_mp) {
        std::fs::write("/mnt/e/CHMBACK/pea-data/data1/met_pea_mp.bin", ser).unwrap();
    }
    println!(
        "finish {}+{insn} {}+{cain} {}+{pean}",
        met_ins_mp.len(),
        met_cai_mp.len(),
        met_pea_mp.len(),
    );
    Ok(())
}

pub fn met_vs_cnl() -> Result<(), Box<dyn Error>> {
    /*
        for ar in ar_list() {
            let met = format!("/mnt/e/CHMBACK/pea-data/data1/rd_met1_{ar}.bin");
            let cnl = format!("/mnt/e/CHMBACK/pea-data/data1/rdcnl2_{ar}.bin");
            let mut ndhs = HashMap::<u64, NodeInfo>::new();
            if let (Ok(met), Ok(cnl)) = (File::open(&met), File::open(&cnl)) {
                let met = BufReader::new(met);
                let cnl = BufReader::new(cnl);
                if let (Ok(met), Ok(cnl)) = (
                    bincode::deserialize_from::<BufReader<File>, Vec<GisMeter>>(met),
                    bincode::deserialize_from::<BufReader<File>, Vec<NclData>>(cnl),
                ) {
                    for (i, m) in met.iter().enumerate() {
                        if let Some(n) = ndhs.get_mut(&m.n1d) {
                            n.metixs.push(i);
                        } else {
                            let n = NodeInfo {
                                n1d: m.n1d,
                                metixs: vec![i],
                                cnlixs: vec![],
                                ..Default::default()
                            };
                            ndhs.insert(m.n1d, n);
                        }
                    }
                    for (i, c) in cnl.iter().enumerate() {
                        if let Some(n) = ndhs.get_mut(&c.n1d) {
                            n.cnlixs.push(i);
                        } else {
                            let n = NodeInfo {
                                n1d: c.n1d,
                                metixs: vec![],
                                cnlixs: vec![i],
                                ..Default::default()
                            };
                            ndhs.insert(c.n1d, n);
                        }
                    }
                    let mut mocn = 0;
                    let mut cocn = 0;
                    let mut mccn = 0;
                    let mut mcx1 = 0;
                    let mut mcx2 = 0;
                    let mut mcxx = 0;
                    let mut nds = Vec::<(u64, usize, usize, u64)>::new();
                    for (_k, n) in ndhs {
                        let mi = if n.metixs.is_empty() { 0 } else { n.metixs[0] };
                        let ci = if n.cnlixs.is_empty() { 0 } else { n.cnlixs[0] };
                        nds.push((n.n1d, mi, ci, 0));
                        if n.metixs.len() == 1 && n.cnlixs.len() == 0 {
                            mocn += 1;
                        } else if n.metixs.len() == 0 && n.cnlixs.len() == 1 {
                            cocn += 1;
                        } else if n.metixs.len() == 1 && n.cnlixs.len() == 1 {
                            mccn += 1;
                        } else if n.metixs.len() == 0 && n.cnlixs.len() > 1 {
                            mcx1 += 1;
                        } else if n.metixs.len() > 1 && n.cnlixs.len() == 0 {
                            mcx2 += 1;
                        } else {
                            println!("me:{} cn:{}", n.metixs.len(), n.cnlixs.len());
                            print!(" pea: m:[");
                            for i in n.metixs {
                                print!(" {}", met[i].pea);
                            }
                            print!(" ] c:[");
                            for i in n.cnlixs {
                                print!(" {}", cnl[i].pea);
                            }
                            println!("]");
                            mcxx += 1;
                        }
                    }
                    nds.sort_by(|a, b| a.0.cmp(&b.0));
                    let mut dx = u64::MAX;
                    for i in 0..(nds.len() - 1) {
                        nds[i].3 = nds[i + 1].0 - nds[i].0;
                        dx = dx.min(nds[i].3);
                    }
                    println!("{ar}:: mccn:{mccn} mocn:{mocn} cocn:{cocn} mcx2:{mcx2} mcx1:{mcx1} mcxx:{mcxx} dx:{dx}");
                }
            }
        }
    */
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NodeInfo {
    pub n1d: u64,
    pub nodes: Vec<GridNode>,
    pub dis: bool,
    pub fg1: bool,
    pub fg2: bool,
    //pub metixs: Vec<usize>,
    //pub cnlixs: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GridLine {
    pub lns: Vec<Vec<(f32, f32)>>,
    //pub ln: Vec<(f32, f32)>,
    pub dis: bool,
    pub opv: Option<String>,  //	OP_VOLT - Text(M2)
    pub volt: Option<f32>,    //	OPVOLTINT - Real(22000.0)
    pub tag: Option<String>,  //	TAG - Text(12BSBA000004531)
    pub en: Option<f32>,      //	ENABLED - Real(1.0)
    pub fid: Option<String>,  //	FEEDERID - None
    pub fid2: Option<String>, //2	FEEDERID2 - None
    pub code: Option<f32>,    //	SUBTYPECOD - Real(1.0)
    pub len: Option<f32>,     //	SHAPE_Leng - Real(102.85118865966797)
    pub phs: Option<f32>,     //	PHASEDESIG - Real(7.0)
    pub own: Option<String>,  //	OWNER - Text(PEA)
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GridNode {
    pub ar: String,
    pub ly: String,
    pub ix: usize,
    pub n1d: u64,
    pub ntp: NodeType,
    pub lix: usize,
    pub pix: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum NodeType {
    #[default]
    Meter,
    Source,
    Customer,
    Load,
    Bridge,
    LineEnd(u64),
    //LineHead(u64),
    //LineTail(u64),
}

use sglab02_lib::sg::gis1::gis_line_lays;

pub fn is_area(ar: &str) -> bool {
    if ar == "NE1" {
        return true;
    }
    true
}

pub const POINTS: [&str; 14] = [
    "DS_Capacitor",
    "DS_CircuitBreaker",
    "DS_Generator",
    "DS_HVCircuitbreaker",
    "DS_HVGenerator",
    "DS_HVSwitch",
    "DS_HVTransformer",
    "DS_LVCapacitor",
    "DS_LVGenerator",
    "DS_RECLOSER",
    "DS_Switch",
    "DS_SwitchingFacility",
    "DS_Transformer",
    "DS_VoltageRegulator",
];

use geo::Closest;

pub fn p2_adj_cnl() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let nds = format!("/mnt/e/CHMBACK/pea-data/data1/p2_{ar}_nodes.bin");
        let xmt = format!("/mnt/e/CHMBACK/pea-data/data1/p2_{ar}_ex_mt.bin");
        let xcs = format!("/mnt/e/CHMBACK/pea-data/data1/p2_{ar}_ex_cs.bin");
        if let (Ok(nds), Ok(xmt), Ok(xcs)) = (File::open(&nds), File::open(&xmt), File::open(&xcs))
        {
            let nds = BufReader::new(nds);
            let xmt = BufReader::new(xmt);
            let xcs = BufReader::new(xcs);
            if let (Ok(nds), Ok(xmt), Ok(xcs)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<GisMeter>>(xmt),
                bincode::deserialize_from::<BufReader<File>, Vec<NclData>>(xcs),
            ) {
                let mut ps = vec![];
                for (n1d, _) in nds {
                    let (x, y) = n1d_2_utm(n1d);
                    let p = Geometry::Point(point!(x:x, y:y));
                    ps.push(p);
                }
                let gc = GeometryCollection::new_from(ps);

                let mut cn = 0;
                for mt in xmt {
                    let (x, y) = n1d_2_utm(mt.n1d);
                    let p: Point<f32> = Point::new(x, y);
                    let c = gc.closest_point(&p);
                    cn += 1;
                    if let Closest::SinglePoint(c) = c {
                        //println!("closest {c:?}");
                        let dx = p.x() - c.x();
                        let dy = p.y() - c.y();
                        let dx = dx.abs();
                        let dy = dy.abs();
                        println!("M {cn}. {dx},{dy}");
                    } else {
                        println!(" ====== ERROR  closest: {c:?}");
                    }
                }
                let mut cn = 0;
                for cs in xcs {
                    let (x, y) = n1d_2_utm(cs.n1d);
                    let p: Point<f32> = Point::new(x, y);
                    let c = gc.closest_point(&p);
                    if let Closest::SinglePoint(c) = c {
                        //println!("closest {c:?}");
                        let dx = p.x() - c.x();
                        let dy = p.y() - c.y();
                        let dx = dx.abs();
                        let dy = dy.abs();
                        if dx + dy > 100f32 {
                            cn += 1;
                            let (lt, ln) = utm_latlong(x, y);
                            println!("C {cn}. {x},{y} -> {lt},{ln}");
                        }
                    } else {
                        println!(" ====== ERROR  closest: {c:?}");
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn p7_add_tr_nodes() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        println!("{ar}");
        //let fnds = format!("/mnt/e/CHMBACK/pea-data/data1/p6_{ar}_nodes.bin");
        let nds = format!("/mnt/e/CHMBACK/pea-data/data1/p6_{ar}_nodes.bin");
        let trs = format!("/mnt/e/CHMBACK/pea-data/data1/p7_{ar}_tr.bin");
        let cnl = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_tr.bin");
        if let (Ok(nds), Ok(trs), Ok(cnl)) = (File::open(&nds), File::open(&trs), File::open(&cnl))
        {
            let nds = BufReader::new(nds);
            let trs = BufReader::new(trs);
            let cnl = BufReader::new(cnl);
            if let (Ok(mut nds), Ok(trs), Ok(cnl)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<Transformer>>(trs),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlData>>(cnl),
            ) {
                //let mut ndcn0 = 0;
                //let mut ndcn1 = 0;
                let mut ps = vec![];
                for (n1d, _nd) in &mut nds {
                    let (x, y) = n1d_2_utm(*n1d);
                    let p = Geometry::Point(point!(x:x, y:y));
                    ps.push(p);
                    /*
                    ndcn0 += nd.nodes.len();
                    let nodes = nd.nodes.clone();
                    nd.nodes = vec![];
                    for n in nodes {
                        if let NodeType::Bridge = n.ntp {
                        } else {
                            nd.nodes.push(n);
                        }
                    }
                    ndcn1 += nd.nodes.len();
                    */
                }
                /*
                if let Ok(ser) = bincode::serialize(&nds) {
                    std::fs::write(fnds, ser).unwrap();
                }
                println!("ndcn0:{ndcn0} ndcn1:{ndcn1}");
                */
                let gc = GeometryCollection::new_from(ps.clone());

                let mut tps = vec![];
                let mut trnd = vec![];
                println!("nd:{} tr:{} cnl:{}", ps.len(), trs.len(), cnl.len());

                let mut mc = 0;
                for (_i, t) in trs.iter().enumerate() {
                    let mut tr = GridNode {
                        ar: ar.to_string(),
                        ly: t.ly.to_string(),
                        ix: t.ix,
                        n1d: t.n1d,
                        ntp: NodeType::Bridge,
                        ..Default::default()
                    };
                    if let Some(nd) = nds.get_mut(&t.n1d) {
                        nd.nodes.push(tr);
                        trnd.push(t.n1d);
                        let (x, y) = n1d_2_utm(t.n1d);
                        tps.push(Geometry::Point(point!(x:x,y:y)));
                    } else {
                        let (x, y) = n1d_2_utm(t.n1d);
                        let p: Point<f32> = Point::new(x, y);
                        if let Closest::SinglePoint(c) = gc.closest_point(&p) {
                            let cc = Geometry::Point(c);
                            if let Some(pos) = ps.iter().position(|p| *p == cc) {
                                mc += 1;
                                println!("M {mc}. fnd:{pos}");
                                if let Geometry::Point(pp) = &ps[pos] {
                                    tr.n1d = utm_2_n1d(pp.x(), pp.y());
                                    if let Some(nd) = nds.get_mut(&tr.n1d) {
                                        let (x, y) = n1d_2_utm(tr.n1d);
                                        tps.push(Geometry::Point(point!(x:x,y:y)));
                                        trnd.push(tr.n1d);
                                        nd.nodes.push(tr);
                                    } else {
                                        println!("!!! ERROR 4 - no good adjust {x},{y}");
                                    }
                                } else {
                                    println!("!!!=== ERROR 3 ");
                                }
                            } else {
                                println!(" ====== ERROR2  closest: {mc:?}");
                            }
                            //let pos = ps.iter().position(|p| *p == cc).unwrap();
                        } else {
                            println!(" ====== ERROR  closest: {mc:?}");
                        }
                    }
                }
                //let trgc = GeometryCollection::new_from(ps.clone());
                trnd.sort();

                //let mut cc = 0;
                for (_i, t) in cnl.iter().enumerate() {
                    let mut n1d = t.tr_n1d.unwrap();
                    let mut tr = GridNode {
                        ar: ar.to_string(),
                        ly: t.ly.to_string(),
                        ix: t.ix,
                        n1d,
                        ntp: NodeType::Bridge,
                        ..Default::default()
                    };
                    if let Some(nd) = nds.get_mut(&n1d) {
                        nd.nodes.push(tr);
                    } else {
                        //let n1d0 = n1d;
                        let mut xtp = trnd.len() - 1;
                        let mut xbt = 0;
                        if n1d > trnd[xtp] {
                            n1d = trnd[xtp];
                        } else if n1d < trnd[0] {
                            n1d = trnd[0];
                        } else {
                            let mut xi = (xtp + xbt) / 2;
                            loop {
                                if xi == 0 {
                                    n1d = trnd[0];
                                    break;
                                }
                                if trnd[xi] > n1d {
                                    xtp = xi;
                                    if xtp - xbt <= 1 {
                                        let tp = trnd[xtp] - n1d;
                                        let bt = n1d - trnd[xbt];
                                        if tp < bt {
                                            n1d = trnd[xtp];
                                        } else {
                                            n1d = trnd[xbt];
                                        }
                                        break;
                                    } else {
                                        xi = (xtp + xbt) / 2;
                                        continue;
                                    }
                                }
                                if trnd[xi] < n1d {
                                    xbt = xi;
                                    if xtp - xbt <= 1 {
                                        let tp = trnd[xtp] - n1d;
                                        let bt = n1d - trnd[xbt];
                                        if tp < bt {
                                            n1d = trnd[xtp];
                                        } else {
                                            n1d = trnd[xbt];
                                        }
                                        break;
                                    } else {
                                        xi = (xtp + xbt) / 2;
                                        continue;
                                    }
                                }
                                n1d = trnd[xi];
                                break;
                            }
                        }
                        tr.n1d = n1d;
                        if let Some(nd) = nds.get_mut(&tr.n1d) {
                            nd.nodes.push(tr);
                        } else {
                            print!(" N");
                        }
                    }
                } // end customer
                println!();
                let ond = format!("/mnt/e/CHMBACK/pea-data/data1/p8_{ar}_nodes.bin");
                println!(" trs:{} cnl:{} {ond}", trs.len(), cnl.len(),);
                if let Ok(ser) = bincode::serialize(&nds) {
                    std::fs::write(ond, ser).unwrap();
                }
            } // end deseri
        } // end file
    } // end area
    Ok(())
}

pub fn p7_add_tr_nodes2() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        println!("{ar}");
        let nds = format!("/mnt/e/CHMBACK/pea-data/data1/p6_{ar}_nodes.bin");
        let trs = format!("/mnt/e/CHMBACK/pea-data/data1/p7_{ar}_tr.bin");
        let cnl = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_tr.bin");
        if let (Ok(nds), Ok(trs), Ok(cnl)) = (File::open(&nds), File::open(&trs), File::open(&cnl))
        {
            let nds = BufReader::new(nds);
            let trs = BufReader::new(trs);
            let cnl = BufReader::new(cnl);
            if let (Ok(mut nds), Ok(trs), Ok(cnl)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<Transformer>>(trs),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlData>>(cnl),
            ) {
                let mut ndv = vec![];
                for (n1d, _nd) in &mut nds {
                    ndv.push(*n1d);
                }
                ndv.sort();

                let mut tps = vec![];
                let mut trnd = vec![];
                println!("nd:{} tr:{} cnl:{}", nds.len(), trs.len(), cnl.len());

                for (lix, t) in trs.iter().enumerate() {
                    let mut tr = GridNode {
                        ar: ar.to_string(),
                        ly: t.ly.to_string(),
                        ix: t.ix,
                        lix,
                        n1d: t.n1d,
                        ntp: NodeType::Bridge,
                        ..Default::default()
                    };
                    if let Some(nd) = nds.get_mut(&t.n1d) {
                        nd.nodes.push(tr);
                        trnd.push(t.n1d);
                        let (x, y) = n1d_2_utm(t.n1d);
                        tps.push(Geometry::Point(point!(x:x,y:y)));
                    } else {
                        let n1d0 = find_node(t.n1d, &ndv);
                        if let Some(nd) = nds.get_mut(&n1d0) {
                            tr.n1d = n1d0;
                            nd.nodes.push(tr);
                        } else {
                            println!("ERROR 5 NO found");
                        }
                    }
                }
                trnd.sort();

                for (lix, t) in cnl.iter().enumerate() {
                    let n1d = t.tr_n1d.unwrap();
                    let mut tr = GridNode {
                        ar: ar.to_string(),
                        ly: t.ly.to_string(),
                        ix: t.ix,
                        lix,
                        n1d,
                        ntp: NodeType::Bridge,
                        ..Default::default()
                    };
                    if let Some(nd) = nds.get_mut(&n1d) {
                        nd.nodes.push(tr);
                    } else {
                        tr.n1d = find_node(n1d, &trnd);
                        if let Some(nd) = nds.get_mut(&tr.n1d) {
                            nd.nodes.push(tr);
                        } else {
                            print!(" N");
                        }
                    }
                } // end customer
                println!();
                let ond = format!("/mnt/e/CHMBACK/pea-data/data1/p8_{ar}_nodes.bin");
                println!(" trs:{} cnl:{} {ond}", trs.len(), cnl.len(),);
                if let Ok(ser) = bincode::serialize(&nds) {
                    std::fs::write(ond, ser).unwrap();
                }
            } // end deseri
        } // end file
    } // end area
    Ok(())
}

pub fn p5_add_met_cnl() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let nds = format!("/mnt/e/CHMBACK/pea-data/data1/p1_{ar}_nodes.bin");
        let met = format!("/mnt/e/CHMBACK/pea-data/data1/p4_meter_{ar}.bin");
        let cnl = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_mt.bin");
        if let (Ok(nds), Ok(met), Ok(cnl)) = (File::open(&nds), File::open(&met), File::open(&cnl))
        {
            let nds = BufReader::new(nds);
            let met = BufReader::new(met);
            let cnl = BufReader::new(cnl);
            if let (Ok(mut nds), Ok(met), Ok(cnl)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<MeterData>>(met),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlData>>(cnl),
            ) {
                let mut ps = vec![];
                for (n1d, _) in &nds {
                    let (x, y) = n1d_2_utm(*n1d);
                    let p = Geometry::Point(point!(x:x, y:y));
                    ps.push(p);
                }
                let gc = GeometryCollection::new_from(ps.clone());

                println!("{ar} - nd:{}", nds.len());
                let mut mc = 0;
                for (_i, m) in met.iter().enumerate() {
                    let mut mt = GridNode {
                        ar: ar.to_string(),
                        ly: m.ly.to_string(),
                        ix: m.ix,
                        n1d: m.n1d,
                        ntp: NodeType::Meter,
                        ..Default::default()
                    };
                    if let Some(nd) = nds.get_mut(&m.n1d) {
                        nd.nodes.push(mt);
                    } else {
                        let (x, y) = n1d_2_utm(m.n1d);
                        let p: Point<f32> = Point::new(x, y);
                        if let Closest::SinglePoint(c) = gc.closest_point(&p) {
                            let cc = Geometry::Point(c);
                            if let Some(pos) = ps.iter().position(|p| *p == cc) {
                                mc += 1;
                                println!("M {mc}. fnd:{pos}");
                                if let Geometry::Point(pp) = &ps[pos] {
                                    mt.n1d = utm_2_n1d(pp.x(), pp.y());
                                    if let Some(nd) = nds.get_mut(&mt.n1d) {
                                        nd.nodes.push(mt);
                                    } else {
                                        println!("!!! no good adjust {x},{y}");
                                    }
                                }
                            } else {
                                println!(" ====== ERROR2  closest: {mc:?}");
                            }
                            //let pos = ps.iter().position(|p| *p == cc).unwrap();
                        } else {
                            println!(" ====== ERROR  closest: {mc:?}");
                        }
                    }
                }
                let mut cc = 0;
                for (_i, c) in cnl.iter().enumerate() {
                    let n1d = c.mt_n1d.unwrap();
                    let mut cs = GridNode {
                        ar: ar.to_string(),
                        ly: c.ly.to_string(),
                        ix: c.ix,
                        n1d,
                        ntp: NodeType::Customer,
                        ..Default::default()
                    };
                    if let Some(nd) = nds.get_mut(&n1d) {
                        nd.nodes.push(cs);
                    } else {
                        let (x, y) = n1d_2_utm(n1d);
                        let p: Point<f32> = Point::new(x, y);
                        if let Closest::SinglePoint(c) = gc.closest_point(&p) {
                            let ccc = Geometry::Point(c);
                            if let Some(pos) = ps.iter().position(|p| *p == ccc) {
                                cc += 1;
                                println!("CS {cc}. fnd:{pos}");
                                if let Geometry::Point(pp) = &ps[pos] {
                                    cs.n1d = utm_2_n1d(pp.x(), pp.y());
                                    if let Some(nd) = nds.get_mut(&cs.n1d) {
                                        nd.nodes.push(cs);
                                    } else {
                                        println!("!!! no good adjust {x},{y}");
                                    }
                                }
                            } else {
                                println!(" ====== ERROR2  closest: {mc:?}");
                            }
                        } else {
                            println!(" ====== ERROR  closest: {mc:?}");
                        }
                    }
                } // end customer
                let ond = format!("/mnt/e/CHMBACK/pea-data/data1/p6_{ar}_nodes.bin");
                println!(" met:{} cnl:{} {ond}", met.len(), cnl.len(),);
                if let Ok(ser) = bincode::serialize(&nds) {
                    std::fs::write(ond, ser).unwrap();
                }
            } // end deseri
        } // end file
    } // end area
    Ok(())
}

pub fn p6_read_transf() -> Result<(), Box<dyn Error>> {
    let lys = ["DS_HVTransformer", "DS_Transformer"];
    for ar in ar_list() {
        let mut trs = Vec::<Transformer>::new();
        for ly in lys {
            let sh = format!("{DB2_DIR}/{ar}_{ly}.pn");
            let db = format!("{DB2_DIR}/{ar}_{ly}.at");
            println!("Area: {db}");
            if let (Ok(fsh), Ok(fat)) = (File::open(&sh), File::open(&db)) {
                println!("open file");
                let bsh = BufReader::new(fsh);
                let bat = BufReader::new(fat);
                if let (Ok(dsh), Ok(dat)) = (
                    bincode::deserialize_from::<BufReader<File>, Vec<(f64, f64)>>(bsh),
                    bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(
                        bat,
                    ),
                ) {
                    println!("deserialize file");
                    println!("sh:{} at:{}", dsh.len(), dat.len());
                    for (ix, ((x, y), r)) in dsh.iter().zip(dat.iter()).enumerate() {
                        //elec	ELECTRICTR - Real(16384.0)
                        let elec = if let Some(DbfData::Real(v)) = r.get("ELECTRICTR") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //en	ENABLED - Real(1.0)
                        let en = if let Some(DbfData::Real(v)) = r.get("ENABLED") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //pea	FACILITYID - Text("48-800044")
                        let pea = if let Some(DbfData::Text(v)) = r.get("FACILITYID") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //fid	FEEDERID - Text("MTG-L1")
                        let fid = if let Some(DbfData::Text(v)) = r.get("FEEDERID") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //fid2	FEEDERID2 - None
                        let fid2 = if let Some(DbfData::Text(v)) = r.get("FEEDERID2") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //fif	FEEDERINFO - Real(7.0)
                        let fif = if let Some(DbfData::Real(v)) = r.get("FEEDERINFO") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //ldmva	LOADMVAR - Real(0.0)
                        let ldmva = if let Some(DbfData::Real(v)) = r.get("LOADMVAR") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //ldmw	LOADMW - Real(0.0)
                        let ldmw = if let Some(DbfData::Real(v)) = r.get("LOADMW") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //loc	LOCATION - Text("สถาน\u{e35}ไฟฟ\u{e49}าเช\u{e35}ยงดาว")
                        let loc = if let Some(DbfData::Text(v)) = r.get("LOCATION") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //opv	OP_VOLT - Text("H1")
                        let opv = if let Some(DbfData::Text(v)) = r.get("OP_VOLT") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //volt	OPVOLTINT - Real(115000.0)
                        let volt = if let Some(DbfData::Real(v)) = r.get("OPVOLTINT") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //own	OWNER - Text("PEA")
                        let own = if let Some(DbfData::Text(v)) = r.get("OWNER") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //mva	RATEMVA - Real(50.0)
                        let mva = if let Some(DbfData::Real(v)) = r.get("RATEMVA") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //code	SUBTYPECOD - Real(3.0)
                        let code = if let Some(DbfData::Real(v)) = r.get("SUBTYPECOD") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //tag	TAG - Text("11HFAA000000099")
                        let tag = if let Some(DbfData::Text(v)) = r.get("TAG") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let n1d = utm_2_n1d(*x as f32, *y as f32);
                        let ar = format!("{ar}");
                        let ly = format!("{ly}");
                        let tr = Transformer {
                            elec,
                            en,
                            pea,
                            fid,
                            fid2,
                            fif,
                            ldmva,
                            ldmw,
                            loc,
                            opv,
                            volt,
                            own,
                            mva,
                            code,
                            tag,
                            n1d,
                            ar,
                            ly,
                            ix,
                        };
                        trs.push(tr);
                    } // end loop elem
                } // end deserial
            } // end file open
        } // end lys
        let ftrs = format!("/mnt/e/CHMBACK/pea-data/data1/p7_{ar}_tr.bin");
        println!("tr {ftrs}");
        if let Ok(ser) = bincode::serialize(&trs) {
            std::fs::write(ftrs, ser).unwrap();
        }
    } // end area
    Ok(())
}

pub fn p6_check_lines() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let nds = format!("/mnt/e/CHMBACK/pea-data/data1/p6_{ar}_nodes.bin");
        if let Ok(nds) = File::open(&nds) {
            let nds = BufReader::new(nds);
            if let Ok(nds) =
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds)
            {
                let (mut mtc, mut csc, mut lec, mut brc, mut cnt) = (0, 0, 0, 0, 0);
                for nd in nds.values() {
                    for n in &nd.nodes {
                        cnt += 1;
                        match n.ntp {
                            NodeType::Meter => {
                                mtc += 1;
                            }
                            NodeType::Customer => {
                                csc += 1;
                            }
                            NodeType::LineEnd(_) => {
                                lec += 1;
                            }
                            NodeType::Bridge => {
                                brc += 1;
                            }
                            _ => {}
                        }
                    }
                }
                println!("{ar} a:{cnt} m:{mtc} c:{csc} l:{lec} b:{brc}");
            }
        }
    }
    Ok(())
}

pub const GIS_EQ_SOURCE: [(&str, NodeType); 3] = [
    ("DS_Generator", NodeType::Source),
    ("DS_HVGenerator", NodeType::Source),
    ("DS_LVGenerator", NodeType::Source),
];
pub const GIS_EQ_ASSIST: [(&str, NodeType); 3] = [
    ("DS_Capacitor", NodeType::Load),
    ("DS_LVCapacitor", NodeType::Load),
    ("DS_SwitchingFacility", NodeType::Load),
];
pub const GIS_EQ_SWITCH: [(&str, NodeType); 6] = [
    ("DS_HVCircuitbreaker", NodeType::Bridge),
    ("DS_CircuitBreaker", NodeType::Bridge),
    ("DS_HVSwitch", NodeType::Bridge),
    ("DS_Switch", NodeType::Bridge),
    ("DS_RECLOSER", NodeType::Bridge),
    ("DS_VoltageRegulator", NodeType::Bridge),
];

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ElecEquip {
    pub elec: Option<f32>,    //	ELECTRICTR - Real(0.0)
    pub en: Option<f32>,      //	ENABLED - Real(1.0)
    pub pea: Option<String>,  //	FACILITYID - Text("CDO05VA-101")
    pub fid: Option<String>,  //	FEEDERID - Text("CDO05")
    pub fid2: Option<String>, //	FEEDERID2 - None
    pub fif: Option<f32>,     //	FEEDERINFO - Real(7.0)
    pub loc: Option<String>,  //	LOCATION - Text("เช\u{e35}ยงใหม\u{e48}-เช\u{e35}ยงดาว สบคาบ")
    pub norm: Option<f32>,    // NORMALSTAT - Real(0.0)
    pub opv: Option<String>,  //	OP_VOLT - Text("M2")
    pub volt: Option<f32>,    //	OPVOLTINT - Real(22000.0)
    pub own: Option<String>,  //	OWNER - Text("PEA")
    pub phs: Option<f32>,     //	PHASEDESIG - Real(7.0)
    pub code: Option<f32>,    //  SUBTYPECOD - Real(3.0)
    pub tag: Option<String>,  //	TAG - Text("1150VR000000007")
    pub n1d: u64,
    pub ntp: NodeType,
    pub ar: String,
    pub ly: String,
    pub ix: usize,
}

pub fn p8_read_sw() -> Result<(), Box<dyn Error>> {
    let mut lys = vec![];
    lys.append(&mut GIS_EQ_SOURCE.to_vec());
    lys.append(&mut GIS_EQ_SWITCH.to_vec());
    lys.append(&mut GIS_EQ_ASSIST.to_vec());
    for ar in ar_list() {
        let mut sws = Vec::<ElecEquip>::new();
        for (ly, ntp) in &lys {
            let sh = format!("{DB2_DIR}/{ar}_{ly}.pn");
            let db = format!("{DB2_DIR}/{ar}_{ly}.at");
            println!("Area: {db}");
            if let (Ok(fsh), Ok(fat)) = (File::open(&sh), File::open(&db)) {
                println!("open file");
                let bsh = BufReader::new(fsh);
                let bat = BufReader::new(fat);
                if let (Ok(dsh), Ok(dat)) = (
                    bincode::deserialize_from::<BufReader<File>, Vec<(f64, f64)>>(bsh),
                    bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(
                        bat,
                    ),
                ) {
                    println!("deserialize file");
                    println!("sh:{} at:{}", dsh.len(), dat.len());
                    for (ix, ((x, y), r)) in dsh.iter().zip(dat.iter()).enumerate() {
                        //pub elec: Option<f32>,    //	ELECTRICTR - Real(0.0)
                        let elec = if let Some(DbfData::Real(v)) = r.get("ELECTRICTR") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //pub en: Option<f32>,      //	ENABLED - Real(1.0)
                        let en = if let Some(DbfData::Real(v)) = r.get("ENABLED") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //pub pea: Option<String>,  //	FACILITYID - Text("CDO05VA-101")
                        let pea = if let Some(DbfData::Text(v)) = r.get("FACILITYID") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //pub fid: Option<String>,  //	FEEDERID - Text("CDO05")
                        let fid = if let Some(DbfData::Text(v)) = r.get("FEEDERID") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //pub fid2: Option<String>, //	FEEDERID2 - None
                        let fid2 = if let Some(DbfData::Text(v)) = r.get("FEEDERID2") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //pub fif: Option<f32>,     //	FEEDERINFO - Real(7.0)
                        let fif = if let Some(DbfData::Real(v)) = r.get("FEEDERINFO") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //pub loc: Option<String>,  //	LOCATION - Text("เช\u{e35}ยงใหม\u{e48}-เช\u{e35}ยงดาว สบคาบ")
                        let loc = if let Some(DbfData::Text(v)) = r.get("loc") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //pub norm: Option<f32>,    // NORMALSTAT - Real(0.0)
                        let norm = if let Some(DbfData::Real(v)) = r.get("NORMALSTAT") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //pub opv: Option<String>,  //	OP_VOLT - Text("M2")
                        let opv = if let Some(DbfData::Text(v)) = r.get("OP_VOLT") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //pub volt: Option<f32>,    //	OPVOLTINT - Real(22000.0)
                        let volt = if let Some(DbfData::Real(v)) = r.get("OPVOLTINT") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //pub own: Option<String>,  //	OWNER - Text("PEA")
                        let own = if let Some(DbfData::Text(v)) = r.get("OWNER") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //pub phs: Option<f32>,     //	PHASEDESIG - Real(7.0)
                        let phs = if let Some(DbfData::Real(v)) = r.get("PHASEDESIG") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //pub code: Option<f32>,    //  SUBTYPECOD - Real(3.0)
                        let code = if let Some(DbfData::Real(v)) = r.get("SUBTYPECOD") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //pub tag: Option<String>,  //	TAG - Text("1150VR000000007")
                        let tag = if let Some(DbfData::Text(v)) = r.get("tag") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let x = *x as f32;
                        let y = *y as f32;
                        let n1d = utm_2_n1d(x, y);
                        let ar = ar.to_string();
                        let ly = ly.to_string();
                        let ntp = ntp.clone();
                        let sw = ElecEquip {
                            elec,
                            en,
                            pea,
                            fid,
                            fid2,
                            fif,
                            loc,
                            norm,
                            opv,
                            volt,
                            own,
                            phs,
                            code,
                            tag,
                            ntp,
                            n1d,
                            ar,
                            ly,
                            ix,
                        };
                        sws.push(sw);
                    }
                }
            }
        }
        let fsws = format!("/mnt/e/CHMBACK/pea-data/data1/p8_{ar}_sw.bin");
        println!("tr {fsws}");
        if let Ok(ser) = bincode::serialize(&sws) {
            std::fs::write(fsws, ser).unwrap();
        }
    }
    Ok(())
}

pub fn p8_add_sw() -> Result<(), Box<dyn Error>> {
    let mut eqs = vec![];
    eqs.append(&mut GIS_EQ_SOURCE.to_vec());
    eqs.append(&mut GIS_EQ_SWITCH.to_vec());
    eqs.append(&mut GIS_EQ_ASSIST.to_vec());
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        println!("{ar}");
        let fnds = format!("/mnt/e/CHMBACK/pea-data/data1/p8_{ar}_nodes.bin");
        let fsws = format!("/mnt/e/CHMBACK/pea-data/data1/p8_{ar}_sw.bin");
        if let (Ok(nds), Ok(sws)) = (File::open(&fnds), File::open(&fsws)) {
            let nds = BufReader::new(nds);
            let sws = BufReader::new(sws);
            if let (Ok(mut nds), Ok(sws)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<ElecEquip>>(sws),
            ) {
                let mut n1ds = vec![];
                for n1d in nds.keys() {
                    n1ds.push(*n1d);
                }
                n1ds.sort();

                println!("nds:{} sws:{}", nds.len(), sws.len());
                let mut sc = 0;
                for s in sws.iter() {
                    let mut sw = GridNode {
                        ar: ar.to_string(),
                        ly: s.ly.to_string(),
                        ix: s.ix,
                        n1d: s.n1d,
                        ntp: s.ntp.clone(),
                        ..Default::default()
                    };
                    if let Some(nd) = nds.get_mut(&s.n1d) {
                        nd.nodes.push(sw);
                    } else {
                        sw.n1d = find_node(s.n1d, &n1ds);
                        if let Some(nd) = nds.get_mut(&sw.n1d) {
                            sc += 1;
                            let (x0, y0) = n1d_2_utm(s.n1d);
                            let (x1, y1) = n1d_2_utm(sw.n1d);
                            println!("S {sc}. ly:{} ix:{} [{x0},{y0}] -> [{x1},{y1}]", s.ly, s.ix);
                            nd.nodes.push(sw);
                        } else {
                            println!("=== ERROR 1 sws");
                        }
                    }
                }
                let ond = format!("/mnt/e/CHMBACK/pea-data/data1/p9_{ar}_nodes.bin");
                println!(" sws:{}", sws.len(),);
                if let Ok(ser) = bincode::serialize(&nds) {
                    std::fs::write(ond, ser).unwrap();
                }
            } // end deseri
        } // end file
    } // end area

    println!("ly:{}", eqs.len());
    Ok(())
}

pub fn find_node(n1d0: u64, n1ds: &Vec<u64>) -> u64 {
    let mut n1d = n1d0;
    let mut xtp = n1ds.len() - 1;
    let mut xbt = 0;
    if n1d > n1ds[xtp] {
        n1d = n1ds[xtp];
    } else if n1d < n1ds[0] {
        n1d = n1ds[0];
    } else {
        let mut xi = (xtp + xbt) / 2;
        loop {
            if xi == 0 {
                n1d = n1ds[0];
                break;
            }
            if n1ds[xi] > n1d {
                xtp = xi;
                if xtp - xbt <= 1 {
                    let tp = n1ds[xtp] - n1d;
                    let bt = n1d - n1ds[xbt];
                    if tp < bt {
                        n1d = n1ds[xtp];
                    } else {
                        n1d = n1ds[xbt];
                    }
                    break;
                } else {
                    xi = (xtp + xbt) / 2;
                    continue;
                }
            }
            if n1ds[xi] < n1d {
                xbt = xi;
                if xtp - xbt <= 1 {
                    let tp = n1ds[xtp] - n1d;
                    let bt = n1d - n1ds[xbt];
                    if tp < bt {
                        n1d = n1ds[xtp];
                    } else {
                        n1d = n1ds[xbt];
                    }
                    break;
                } else {
                    xi = (xtp + xbt) / 2;
                    continue;
                }
            }
            n1d = n1ds[xi];
            break;
        }
    }
    n1d
}

pub fn p9_check_lines() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let nds = format!("/mnt/e/CHMBACK/pea-data/data1/p9_{ar}_nodes.bin");
        if let Ok(nds) = File::open(&nds) {
            let nds = BufReader::new(nds);
            if let Ok(nds) =
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds)
            {
                let (mut mtc, mut csc, mut lec, mut brc, mut cnt, mut src, mut lod) =
                    (0, 0, 0, 0, 0, 0, 0);
                for nd in nds.values() {
                    cnt += 1;
                    for n in &nd.nodes {
                        match n.ntp {
                            NodeType::Meter => {
                                mtc += 1;
                            }
                            NodeType::Customer => {
                                csc += 1;
                            }
                            NodeType::LineEnd(_) => {
                                lec += 1;
                            }
                            NodeType::Bridge => {
                                brc += 1;
                            }
                            NodeType::Source => {
                                src += 1;
                            }
                            NodeType::Load => {
                                lod += 1;
                            }
                        }
                    }
                }
                println!("{ar} a:{cnt} m:{mtc} c:{csc} l:{lec} b:{brc} s:{src} l:{lod}");
            }
        }
    }
    Ok(())
}

pub fn p8_check_lines() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let nds = format!("/mnt/e/CHMBACK/pea-data/data1/p8_{ar}_nodes.bin");
        if let Ok(nds) = File::open(&nds) {
            let nds = BufReader::new(nds);
            if let Ok(nds) =
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds)
            {
                let (mut mtc, mut csc, mut lec, mut brc, mut cnt) = (0, 0, 0, 0, 0);
                for nd in nds.values() {
                    cnt += 1;
                    for n in &nd.nodes {
                        match n.ntp {
                            NodeType::Meter => {
                                mtc += 1;
                            }
                            NodeType::Customer => {
                                csc += 1;
                            }
                            NodeType::LineEnd(_) => {
                                lec += 1;
                            }
                            NodeType::Bridge => {
                                brc += 1;
                            }
                            _ => {}
                        }
                    }
                }
                println!("{ar} a:{cnt} m:{mtc} c:{csc} l:{lec} b:{brc}");
            }
        }
    }
    Ok(())
}

pub fn p2_met_cnl_read() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let nds = format!("/mnt/e/CHMBACK/pea-data/data1/p1_{ar}_nodes.bin");
        let met = format!("/mnt/e/CHMBACK/pea-data/data1/rd_met1_{ar}.bin");
        let cnl = format!("/mnt/e/CHMBACK/pea-data/data1/rdcnl2_{ar}.bin");
        let ond = format!("/mnt/e/CHMBACK/pea-data/data1/p2_{ar}_nodes.bin");
        let omt = format!("/mnt/e/CHMBACK/pea-data/data1/p2_{ar}_ex_mt.bin");
        let ocs = format!("/mnt/e/CHMBACK/pea-data/data1/p2_{ar}_ex_cs.bin");
        let mut ex_mt = Vec::<GisMeter>::new();
        let mut ex_cs = Vec::<NclData>::new();
        if let (Ok(nds), Ok(met), Ok(cnl)) = (File::open(&nds), File::open(&met), File::open(&cnl))
        {
            let nds = BufReader::new(nds);
            let met = BufReader::new(met);
            let cnl = BufReader::new(cnl);
            if let (Ok(mut nds), Ok(met), Ok(cnl)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<GisMeter>>(met),
                bincode::deserialize_from::<BufReader<File>, Vec<NclData>>(cnl),
            ) {
                println!("{ar} - nd:{}", nds.len());
                //let mut mc = 0;
                for (_i, m) in met.iter().enumerate() {
                    if let Some(nd) = nds.get_mut(&m.n1d) {
                        let mt = GridNode {
                            ar: ar.to_string(),
                            ly: m.ly.to_string(),
                            ix: m.ix,
                            n1d: m.n1d,
                            ntp: NodeType::Meter,
                            ..Default::default()
                        };
                        nd.nodes.push(mt);
                    } else {
                        //mc += 1;
                        ex_mt.push(m.clone());
                        //println!("{mc}.meter {}", m.n1d);
                    }
                }
                //let mut cc = 0;
                for (_i, c) in cnl.iter().enumerate() {
                    if let Some(nd) = nds.get_mut(&c.n1d) {
                        let cs = GridNode {
                            ar: ar.to_string(),
                            ly: c.ly.to_string(),
                            ix: c.ix,
                            n1d: c.n1d,
                            ntp: NodeType::Customer,
                            ..Default::default()
                        };
                        nd.nodes.push(cs);
                    } else {
                        //cc += 1;
                        ex_cs.push(c.clone());
                        //println!("{cc}.customer {}", c.n1d);
                    }
                } // end customer
                println!(
                    " met:{}-{} cnl:{}-{}",
                    met.len(),
                    ex_mt.len(),
                    cnl.len(),
                    ex_cs.len()
                );
                println!(" ond:{ond} {omt} {ocs}");
                if let Ok(ser) = bincode::serialize(&nds) {
                    std::fs::write(ond, ser).unwrap();
                }
                if let Ok(ser) = bincode::serialize(&ex_mt) {
                    std::fs::write(omt, ser).unwrap();
                }
                if let Ok(ser) = bincode::serialize(&ex_cs) {
                    std::fs::write(ocs, ser).unwrap();
                }
            } // end deseri
        } // end file
    } // end area
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineData {
    pub opv: Option<String>,  //	OP_VOLT - Text(M2)
    pub volt: Option<f32>,    //	OPVOLTINT - Real(22000.0)
    pub tag: Option<String>,  //	TAG - Text(12BSBA000004531)
    pub en: Option<f32>,      //	ENABLED - Real(1.0)
    pub fid: Option<String>,  //	FEEDERID - None
    pub fid2: Option<String>, //2	FEEDERID2 - None
    pub code: Option<f32>,    //	SUBTYPECOD - Real(1.0)
    pub len: Option<f32>,     //	SHAPE_Leng - Real(102.85118865966797)
    pub phs: Option<f32>,     //	PHASEDESIG - Real(7.0)
    pub own: Option<String>,  //	OWNER - Text(PEA)
}

pub fn p1_line_node() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        let mut nds = HashMap::<u64, NodeInfo>::new();
        let mut gls = Vec::<GridLine>::new();
        for ly in gis_line_lays() {
            println!("ar:{ar} ly:{ly}");
            let ln = format!("{DB2_DIR}/{ar}_{ly}.ln");
            let at = format!("{DB2_DIR}/{ar}_{ly}.at");
            println!("  ln:{ln} at:{at}");
            if let (Ok(ln), Ok(at)) = (File::open(&ln), File::open(&at)) {
                let ln = BufReader::new(ln);
                let at = BufReader::new(at);
                if let (Ok(ln), Ok(at)) = (
                    bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(ln),
                    bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(at),
                ) {
                    for (ix, (lss, at)) in ln.iter().zip(at.iter()).enumerate() {
                        //opv	OP_VOLT - Text("M2")
                        let opv = if let Some(DbfData::Text(v)) = at.get("OP_VOLT") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //volt	OPVOLTINT - Real(22000.0)
                        let volt = if let Some(DbfData::Real(v)) = at.get("OPVOLTINT") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //tag	TAG - Text("12BSBA000004531")
                        let tag = if let Some(DbfData::Text(v)) = at.get("TAG") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //en	ENABLED - Real(1.0)
                        let en = if let Some(DbfData::Real(v)) = at.get("ENABLED") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //fid	FEEDERID - None
                        let fid = if let Some(DbfData::Text(v)) = at.get("FEEDERID") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //fid2	FEEDERID2 - None
                        let fid2 = if let Some(DbfData::Text(v)) = at.get("FEEDERID2") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        //code	SUBTYPECOD - Real(1.0)
                        let code = if let Some(DbfData::Real(v)) = at.get("SUBTYPECOD") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //len	SHAPE_Leng - Real(102.85118865966797)
                        let len = if let Some(DbfData::Real(v)) = at.get("SHAPE_Leng") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //phs	PHASEDESIG - Real(7.0)
                        let phs = if let Some(DbfData::Real(v)) = at.get("PHASEDESIG") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        //own	OWNER - Text("PEA")
                        let own = if let Some(DbfData::Text(v)) = at.get("OWNER") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let mut lns = Vec::<Vec<(f32, f32)>>::new();
                        //let mut fst = true;
                        //let mut p0 = &ln.ln[0];
                        //let mut p1 = &ln.ln[ln.ln.len() - 1];
                        //let mut p0;
                        //let mut p1;
                        for ls in lss {
                            let mut ln = Vec::<(f32, f32)>::new();
                            for l in ls {
                                ln.push((l.0 as f32, l.1 as f32));
                            }
                            lns.push(ln);
                        }
                        //let lns = lns.clone();
                        let p0 = &lns[0][0];
                        let ls = lns.len() - 1;
                        let lt = lns[ls].len() - 1;
                        let p1 = &lns[ls][lt];
                        let n1d0 = utm_2_n1d(p0.0 as f32, p0.1 as f32);
                        let n1d1 = utm_2_n1d(p1.0 as f32, p1.1 as f32);

                        let dis = false;
                        let ln = GridLine {
                            opv,
                            volt,
                            tag,
                            en,
                            fid,
                            fid2,
                            code,
                            len,
                            phs,
                            own,
                            lns,
                            dis,
                            //..Default::default()
                        };
                        //let mut ln = GridLine::default();
                        /*
                        for ls in lss {
                            for l in ls {
                                ln.ln.push((l.0 as f32, l.1 as f32));
                            }
                        }
                        */
                        //let p0 = &ln.ln[0];
                        //let p1 = &ln.ln[ln.ln.len() - 1];
                        let hd = GridNode {
                            ar: ar.to_string(),
                            ly: ly.to_string(),
                            ix,
                            n1d: n1d0,
                            ntp: NodeType::LineEnd(n1d1),
                            lix: gls.len(),
                            ..Default::default()
                        };
                        if let Some(nd) = nds.get_mut(&n1d0) {
                            nd.nodes.push(hd);
                        } else {
                            let nd = NodeInfo {
                                n1d: n1d0,
                                nodes: vec![hd],
                                ..Default::default()
                            };
                            nds.insert(n1d0, nd);
                        }

                        let tl = GridNode {
                            ar: ar.to_string(),
                            ly: ly.to_string(),
                            ix,
                            n1d: n1d1,
                            ntp: NodeType::LineEnd(n1d0),
                            lix: gls.len(),
                            ..Default::default()
                        };
                        if let Some(nd) = nds.get_mut(&n1d1) {
                            nd.nodes.push(tl);
                        } else {
                            let nd = NodeInfo {
                                n1d: n1d1,
                                nodes: vec![tl],
                                ..Default::default()
                            };
                            nds.insert(n1d1, nd);
                        }

                        gls.push(ln);
                    } // end of loop
                } // end of deseriled
            } // end of file
        } // end of layer
        use std::collections::HashSet;
        let mut nd_sz = HashMap::<usize, usize>::new();
        let mut nd_ds = HashSet::<u64>::new();
        for (n1d, nd) in &nds {
            if nd.nodes.len() == 1 {
                let nd0 = &nd.nodes[0];
                if let NodeType::LineEnd(ed) = nd0.ntp {
                    if let Some(end) = nds.get(&ed) {
                        if end.nodes.len() == 1 {
                            nd_ds.insert(*n1d);
                            nd_ds.insert(ed);
                            gls[nd0.lix].dis = true;
                            //print!(" ..{ed}");
                        }
                    }
                }
            }
            let sz = nd.nodes.len();
            if let Some(cn) = nd_sz.get_mut(&sz) {
                *cn += 1;
            } else {
                nd_sz.insert(sz, 1);
            }
        }
        println!();
        for ds in &nd_ds {
            if let Some(nd) = nds.get_mut(&ds) {
                nd.dis = true;
            }
        }
        let dis = gls.iter().filter(|a| a.dis).map(|_| 1).sum::<i32>();
        let mut keys = Vec::from_iter(nd_sz.keys());
        keys.sort();
        for k in keys {
            let s = nd_sz.get(k).unwrap();
            println!(" {k} - {s}");
        }
        println!("==== nodes: {} nd:{} ld:{dis}", nds.len(), nd_ds.len());
        let fnds = format!("/mnt/e/CHMBACK/pea-data/data1/p1_{ar}_nodes.bin");
        let flns = format!("/mnt/e/CHMBACK/pea-data/data1/p1_{ar}_lines.bin");
        println!("{fnds} {flns}");
        if let Ok(ser) = bincode::serialize(&nds) {
            std::fs::write(fnds, ser).unwrap();
        }
        if let Ok(ser) = bincode::serialize(&gls) {
            std::fs::write(flns, ser).unwrap();
        }
    } // end of area
    Ok(())
}

pub fn p3_read_meter() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        let lys = ["DS_LowVoltageMeter", "DS_HVPrimaryMeter", "DS_PrimaryMeter"];
        let mut mts = Vec::<MeterData>::new();
        for ly in lys {
            let sh = format!("{DB2_DIR}/{ar}_{ly}.pn");
            let db = format!("{DB2_DIR}/{ar}_{ly}.at");
            println!("Area: {db}");
            if let (Ok(fsh), Ok(fat)) = (File::open(&sh), File::open(&db)) {
                println!("open file");
                let bsh = BufReader::new(fsh);
                let bat = BufReader::new(fat);
                if let (Ok(dsh), Ok(dat)) = (
                    bincode::deserialize_from::<BufReader<File>, Vec<(f64, f64)>>(bsh),
                    bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(
                        bat,
                    ),
                ) {
                    println!("deserialize file");
                    println!("sh:{} at:{}", dsh.len(), dat.len());
                    let mut ix = 0;
                    for ((x, y), r) in dsh.iter().zip(dat.iter()) {
                        let ca = if let Some(DbfData::Text(v)) = r.get("ACCOUNTNUM") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let tp = if let Some(DbfData::Text(v)) = r.get("SUBTYPECOD") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let fid = if let Some(DbfData::Text(v)) = r.get("FEEDERID") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let pea = if let Some(DbfData::Text(v)) = r.get("PEANO") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let am = if let Some(DbfData::Real(v)) = r.get("AMP") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let rd = if let Some(DbfData::Real(v)) = r.get("STREETLIGH") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let rf = if let Some(DbfData::Text(v)) = r.get("ROOFTOP") {
                            Some(v != "N")
                        } else {
                            None
                        };
                        let cd = if let Some(DbfData::Text(v)) = r.get("CODE") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let en = if let Some(DbfData::Real(v)) = r.get("ENABLED") {
                            Some(*v == 1.0)
                        } else {
                            None
                        };
                        let inst = if let Some(DbfData::Text(v)) = r.get("INSTALLATI") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let finf = if let Some(DbfData::Real(v)) = r.get("PHASEDESIG") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let route = if let Some(DbfData::Text(v)) = r.get("ROUTE") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let fid2 = if let Some(DbfData::Text(v)) = r.get("FEEDERID2") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let own = if let Some(DbfData::Text(v)) = r.get("OWNER") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let opv = if let Some(DbfData::Text(v)) = r.get("OPVOLTINT") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let fdif = if let Some(DbfData::Real(v)) = r.get("FEEDERINFO") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let loc = if let Some(DbfData::Text(v)) = r.get("LOCATION") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let src = if let Some(DbfData::Real(v)) = r.get("SOURCETYPE") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let elc = if let Some(DbfData::Real(v)) = r.get("ELECTRICTR") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let tag = if let Some(DbfData::Text(v)) = r.get("TAG") {
                            Some(v.to_string())
                        } else {
                            None
                        };
                        let nos = if let Some(DbfData::Real(v)) = r.get("NUMBEROFST") {
                            Some(*v as f32)
                        } else {
                            None
                        };
                        let x = *x as f32;
                        let y = *y as f32;
                        let n1d = utm_2_n1d(x, y);
                        let ar = ar.to_string();
                        let ly = ly.to_string();
                        let gm = MeterData {
                            ca,
                            tp,
                            fid,
                            pea,
                            am,
                            rd,
                            rf,
                            cd,
                            en,
                            inst,
                            finf,
                            route,
                            x,
                            y,
                            n1d,
                            fid2,
                            own,
                            fdif,
                            loc,
                            src,
                            elc,
                            tag,
                            nos,
                            opv,
                            ar,
                            ly,
                            ix,
                        };
                        mts.push(gm);
                        ix += 1;
                    }
                    println!("  end {ix}");
                }
            }
        }
        let fout = format!("/mnt/e/CHMBACK/pea-data/data1/p4_meter_{ar}.bin");
        println!("write to {fout}");
        if let Ok(ser) = bincode::serialize(&mts) {
            std::fs::write(fout, ser).unwrap();
            println!("cnl {}", mts.len());
        }
    }
    Ok(())
}

pub fn rd_met1() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        let lys = ["DS_LowVoltageMeter", "DS_HVPrimaryMeter", "DS_PrimaryMeter"];
        let mut gms = Vec::<GisMeter>::new();
        for ly in lys {
            let sh = format!("{DB2_DIR}/{ar}_{ly}.pn");
            let db = format!("{DB2_DIR}/{ar}_{ly}.at");
            println!("Area: {db}");
            if let (Ok(fsh), Ok(fat)) = (File::open(&sh), File::open(&db)) {
                println!("open file");
                let bsh = BufReader::new(fsh);
                let bat = BufReader::new(fat);
                if let (Ok(dsh), Ok(dat)) = (
                    bincode::deserialize_from::<BufReader<File>, Vec<(f64, f64)>>(bsh),
                    bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(
                        bat,
                    ),
                ) {
                    println!("deserialize file");
                    println!("sh:{} at:{}", dsh.len(), dat.len());
                    let mut ix = 0;
                    for ((x, y), r) in dsh.iter().zip(dat.iter()) {
                        let ca = if let Some(DbfData::Text(v)) = r.get("ACCOUNTNUM") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let tp = if let Some(DbfData::Text(v)) = r.get("SUBTYPECOD") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let fid = if let Some(DbfData::Text(v)) = r.get("FEEDERID") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let pea = if let Some(DbfData::Text(v)) = r.get("PEANO") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let am = if let Some(DbfData::Real(v)) = r.get("AMP") {
                            *v as f32
                        } else {
                            0.0f32
                        };
                        let rd = if let Some(DbfData::Real(v)) = r.get("STREETLIGH") {
                            *v as f32
                        } else {
                            0.0f32
                        };
                        let rf = if let Some(DbfData::Text(v)) = r.get("ROOFTOP") {
                            v != "N"
                        } else {
                            false
                        };
                        let cd = if let Some(DbfData::Text(v)) = r.get("CODE") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let en = if let Some(DbfData::Real(v)) = r.get("ENABLED") {
                            *v == 1.0
                        } else {
                            false
                        };
                        let inst = if let Some(DbfData::Text(v)) = r.get("INSTALLATI") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let finf = if let Some(DbfData::Real(v)) = r.get("PHASEDESIG") {
                            *v as f32
                        } else {
                            0.0f32
                        };
                        let route = if let Some(DbfData::Text(v)) = r.get("ROUTE") {
                            v.to_string()
                        } else {
                            "".to_string()
                        };
                        let utm = (*x as f32, *y as f32);
                        let nid = utm_2_nid(utm.0, utm.1);
                        let n1d = utm_2_n1d(utm.0, utm.1);
                        let ar = ar.to_string();
                        let ly = ly.to_string();
                        let gm = GisMeter {
                            ca,
                            tp,
                            fid,
                            pea,
                            am,
                            rd,
                            rf,
                            cd,
                            en,
                            inst,
                            finf,
                            route,
                            utm,
                            nid,
                            n1d,
                            ar,
                            ly,
                            ix,
                        };
                        gms.push(gm);
                        ix += 1;
                    }
                    println!("  end {ix}");
                }
            }
        }
        let fout = format!("/mnt/e/CHMBACK/pea-data/data1/rd_met1_{ar}.bin");
        println!("write to {fout}");
        if let Ok(ser) = bincode::serialize(&gms) {
            std::fs::write(fout, ser).unwrap();
            println!("cnl {}", gms.len());
        }
    }
    Ok(())
}

/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NclMeter {
    pub inst: String,
    pub lv: bool,
    pub pea: String,
    pub fid: String,
    pub utm: (f32, f32),
    pub nodeid: String,
    pub aoj: String,
    pub trx: String,
    pub inst_n: String,
    pub pea_n: String,
    pub nodeid_i: String,
}

pub fn rdcnl1() -> Result<(), Box<dyn Error>> {
    let mut cn = 0;
    for x in ar_list() {
        let mut cnls = Vec::<NclMeter>::new();
        let mut cnl_ins = HashMap::<String, String>::new();
        let mut cnl_pea = HashMap::<String, String>::new();

        let db = format!("{DB2_DIR}/{x}_GIS_HVMVCNL.at");
        println!("Area: {db} HVMV");
        if let Ok(fat) = File::open(&db) {
            let bat = BufReader::new(fat);
            if let Ok(at) =
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(bat)
            {
                for r in at {
                    let inst = if let Some(DbfData::Text(v)) = r.get("METER_INST") {
                        v.to_string()
                    } else {
                        "".to_string()
                    };
                    let lv = false;
                    let pea = if let Some(DbfData::Text(v)) = r.get("PEA_METER") {
                        v.to_string()
                    } else {
                        "".to_string()
                    };
                    let fid = if let Some(DbfData::Text(v)) = r.get("MT_FEEDER") {
                        v.to_string()
                    } else {
                        "".to_string()
                    };
                    let aoj = if let Some(DbfData::Text(v)) = r.get("AOJ") {
                        v.to_string()
                    } else {
                        "".to_string()
                    };
                    let x = if let Some(DbfData::Real(v)) = r.get("X_COOD") {
                        *v as f32
                    } else {
                        0_f32
                    };
                    let y = if let Some(DbfData::Real(v)) = r.get("Y_COOD") {
                        *v as f32
                    } else {
                        0_f32
                    };
                    let nodeid = utm_nodeid(x, y);
                    let trx = "".to_string();
                    let mut cnl = NclMeter {
                        inst,
                        lv,
                        pea,
                        fid,
                        utm: (x, y),
                        nodeid,
                        aoj,
                        trx,
                        inst_n: "".to_string(),
                        pea_n: "".to_string(),
                        nodeid_i: "".to_string(),
                    };
                    if let Some(nid) = cnl_ins.get_mut(&cnl.inst) {
                        cn += 1;
                        cnl.inst_n = nid.to_string();
                        println!("ncl1:{cn}. i:{}", cnl.inst);
                    } else {
                        cnl_ins.insert(cnl.inst.to_string(), cnl.nodeid.clone());
                    }
                    if let Some(nid) = cnl_pea.get_mut(&cnl.pea) {
                        cnl.pea_n = nid.to_string();
                    } else {
                        cnl_pea.insert(cnl.pea.to_string(), cnl.nodeid.clone());
                    }
                    cnls.push(cnl);
                }
            }
        }
        let db = format!("{DB2_DIR}/{x}_GIS_LVCNL.at");
        println!("Area: {db} LV");
        if let Ok(fat) = File::open(&db) {
            let bat = BufReader::new(fat);
            if let Ok(at) =
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(bat)
            {
                for r in at {
                    let inst = if let Some(DbfData::Text(v)) = r.get("METER_INST") {
                        v.to_string()
                    } else {
                        "".to_string()
                    };
                    let lv = true;
                    let pea = if let Some(DbfData::Text(v)) = r.get("PEA_METER") {
                        v.to_string()
                    } else {
                        "".to_string()
                    };
                    let fid = if let Some(DbfData::Text(v)) = r.get("MT_FEEDER") {
                        v.to_string()
                    } else {
                        "".to_string()
                    };
                    let aoj = if let Some(DbfData::Text(v)) = r.get("AOJ") {
                        v.to_string()
                    } else {
                        "".to_string()
                    };
                    let x = if let Some(DbfData::Real(v)) = r.get("X_COOD") {
                        *v as f32
                    } else {
                        0_f32
                    };
                    let y = if let Some(DbfData::Real(v)) = r.get("Y_COOD") {
                        *v as f32
                    } else {
                        0_f32
                    };
                    let nodeid = utm_nodeid(x, y);
                    let trx = if let Some(DbfData::Text(v)) = r.get("TRF_PEA_NO") {
                        v.to_string()
                    } else {
                        "".to_string()
                    };
                    let mut cnl = NclMeter {
                        inst,
                        lv,
                        pea,
                        fid,
                        utm: (x, y),
                        nodeid,
                        aoj,
                        trx,
                        inst_n: "".to_string(),
                        pea_n: "".to_string(),
                        nodeid_i: "".to_string(),
                    };
                    if let Some(nid) = cnl_ins.get_mut(&cnl.inst) {
                        cn += 1;
                        cnl.inst_n = nid.to_string();
                        println!("ncl1:{cn}. i:{}", cnl.inst);
                    } else {
                        cnl_ins.insert(cnl.inst.to_string(), cnl.nodeid.clone());
                    }
                    if let Some(nid) = cnl_pea.get_mut(&cnl.pea) {
                        cnl.pea_n = nid.to_string();
                    } else {
                        cnl_pea.insert(cnl.pea.to_string(), cnl.nodeid.clone());
                    }
                    cnls.push(cnl);
                }
            }
        }
        let fout = format!("/mnt/e/CHMBACK/pea-data/data1/rdcnl1_{x}.bin");
        println!("write {cn} to {fout}");
        if let Ok(ser) = bincode::serialize(&cnls) {
            std::fs::write(fout, ser).unwrap();
            println!("cnl {}", cnls.len());
        }
    }
    Ok(())
}

use sglab02_lib::sg::ldp::TranxInfo;
use sglab02_lib::sg::prc2::Transformer;
// billing to trans+mete
pub async fn bill_to_txmt1() -> Result<(), Box<dyn std::error::Error>> {
    //let base = base();
    let mut mt202405 = HashMap::<String, f64>::new();
    if let Ok(file) = File::open(sglab02_lib::sg::ldp::res("mt-202405.bin")) {
        let rd = BufReader::new(file);
        if let Ok(mt) = bincode::deserialize_from::<BufReader<File>, HashMap<String, f64>>(rd) {
            mt202405 = mt;
        }
    }
    let mut mt202402 = HashMap::<String, f64>::new();
    if let Ok(file) = File::open(sglab02_lib::sg::ldp::res("mt-202402.bin")) {
        if let Ok(mt) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, f64>>(BufReader::new(file))
        {
            mt202402 = mt;
        }
    }
    let txmtno = HashMap::<usize,(usize,usize,usize)>::new();
    if let Ok(file) = File::open(sglab02_lib::sg::ldp::res("txmtmp.bin")) {
        //let (mut t30, mut t50, mut t100, mut t160, mut t250, mut t300, mut t500, mut t2000) = (0,0,0,0,0,0,0,0);
        let rd = BufReader::new(file);
        if let Ok(txmtmp) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, TranxInfo>>(rd)
        {
            let mut fd_trs = HashMap::<String, Vec<Transformer>>::new();
            //let mut fdtxmp = HashMap::<String, Vec<FeederTranx>>::new();
            for (_k, tx) in txmtmp {
                // for each tx
                if tx.trans_feed.len() < 5 {
                    continue;
                }
                let fd0 = &tx.trans_feed[3..5];
                let fd_id = format!("{}{}", tx.trans_sub, fd0);

                let tx_id = tx.trans_id.to_string();
                let tx_power = tx.trans_power;
                let tx_own = tx.trans_own;
                //println!("tx:{} pw:{} ow:{}", tx_id, tx_power, tx_own);

                let (
                    mut mt_ph_a,
                    mut mt_ph_b,
                    mut mt_ph_c,
                    mut mt_1_ph,
                    mut mt_3_ph,
                    mut mt_else,
                    _mt_sm,
                ) = (0, 0, 0, 0, 0, 0, tx.meters.len());
                let (mut eg5_a, mut eg5_b, mut eg5_c, mut eg5_1p, mut eg5_3p, mut eg5_sm) =
                    (0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
                let (mut eg2_a, mut eg2_b, mut eg2_c, mut eg2_1p, mut eg2_3p, mut eg2_sm) =
                    (0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
                let mt_cnt = tx.meters.len();
                let cc = tx.meters.len();
                let mut c5 = 0;
                let mut c2 = 0;
                for mt in &tx.meters {
                    let /*mut*/ eg05 = if let Some(eg) = mt202405.get(&mt.meter_id) { *eg } else { c5+=1; 0f64 };
                    let /*mut*/ eg02 = if let Some(eg) = mt202402.get(&mt.meter_id) { *eg } else { c2+=1; 0f64 };
                    eg5_sm += eg05;
                    if mt.meter_phase == "A" {
                        mt_ph_a += 1;
                        mt_1_ph += 1;
                        eg5_a += eg05;
                        eg5_1p += eg05;
                        eg2_a += eg02;
                        eg2_1p += eg02;
                    } else if mt.meter_phase == "B" {
                        mt_ph_b += 1;
                        mt_1_ph += 1;
                        eg5_b += eg05;
                        eg5_1p += eg05;
                        eg2_b += eg02;
                        eg2_1p += eg02;
                    } else if mt.meter_phase == "C" {
                        mt_ph_c += 1;
                        mt_1_ph += 1;
                        eg5_c += eg05;
                        eg5_1p += eg05;
                        eg2_c += eg02;
                        eg2_1p += eg02;
                    } else if mt.meter_phase == "ABC" {
                        mt_3_ph += 1;
                        eg5_3p += eg05;
                        eg2_3p += eg05;
                    } else {
                        mt_else += 1;
                    }
                }
                println!("fd:{fd_id} tx:{tx_id} cc:{cc} = c5:{c5} c2:{c2}");
                let trn = Transformer {
                    fd_id,
                    tx_id,
                    tx_power,
                    tx_own,
                    mt_ph_a,
                    mt_ph_b,
                    mt_ph_c,
                    mt_1_ph,
                    mt_3_ph,
                    mt_else,
                    mt_cnt,
                    eg5_a,
                    eg5_b,
                    eg5_c,
                    eg5_1p,
                    eg5_3p,
                    eg5_sm,
                    eg2_a,
                    eg2_b,
                    eg2_c,
                    eg2_1p,
                    eg2_3p,
                    eg2_sm,
                };
                if let Some(/*mut*/ trnv) = fd_trs.get_mut(&trn.fd_id) {
                    trnv.push(trn);
                } else {
                    fd_trs.insert(trn.fd_id.to_string(), vec![trn]);
                }
            }
            println!("fd trs {}", fd_trs.len());
            let file = format!("{}/fd_trs.bin", sglab02_lib::sg::imp::data_dir());
            if let Ok(ser) = bincode::serialize(&fd_trs) {
                std::fs::write(file, ser).unwrap();
            }
        } // read txmtmp.bin
    } // end open file
    println!("{:?}", txmtno);

    Ok(())
}
pub async fn read_bill2() -> Result<(), Box<dyn std::error::Error>> {
    let fdir = "/mnt/d/CHMBACK/pea-data/20240801_กรอ";
    let fout = "/mnt/e/CHMBACK/pea-data/bill";
    let flst = vec!["202402", "202405"];
    //let _csv_v = Vec::<CSVFile>::new();
    let mut rate = HashMap::<String, usize>::new();
    let mut volt = HashMap::<String, usize>::new();
    let mut cn = 0;
    for f in flst {
        let fln = format!("{fdir}/export_กรอ_bil013_{}.csv", f);
        let fou = format!("{fout}/{}_2.bin", f);
        println!("start {}", &fln);
        if let Ok(mut rdr) = csv::Reader::from_path(&fln) {
            //let mut mb_pea = HashMap::<String, MeterBill>::new();
            let mut mb_cai = HashMap::<String, MeterBill>::new();
            for rs in rdr.records() {
                if let Ok(rc) = rs {
                    // if the record exist
                    if let (
                        Some(c2),
                        Some(c3),
                        Some(c4),
                        Some(c5),
                        Some(c7),
                        Some(c15),
                        Some(c18),
                        Some(c19),
                    ) = (
                        rc.get(2),
                        rc.get(3),
                        rc.get(4),
                        rc.get(5),
                        rc.get(7),
                        rc.get(15),
                        rc.get(18),
                        rc.get(19),
                    ) {
                        if let (Ok(_n15), Ok(_n18), Ok(n19)) =
                            (c15.parse::<f32>(), c18.parse::<f32>(), c19.parse::<f32>())
                        {
                            if let Some(rt) = rate.get_mut(c4) {
                                *rt += 1;
                            } else {
                                rate.insert(c4.to_string(), 1);
                            }
                            if let Some(vo) = volt.get_mut(c5) {
                                *vo += 1;
                            } else {
                                volt.insert(c5.to_string(), 1);
                            }
                            let mb0 = MeterBill {
                                pea: c7.trim().to_string(),
                                ca: c2.trim().to_string(),
                                inst: c3.trim().to_string(),
                                rate: c4.to_string(),
                                volt: c5.to_string(),
                                kwh: n19,
                            };
                            let cai = format!("{}", mb0.ca);
                            if let Some(mb) = mb_cai.get(&cai) {
                                cn += 1;
                                println!(
                                    "2. !!== {cn}.CAI ca:{}-{} i:{}-{}",
                                    mb0.ca, mb.ca, mb0.inst, mb.inst
                                );
                            } else {
                                mb_cai.insert(cai, mb0.clone());
                            }
                        }
                    }
                } // fi record
            } // loop all rec
            println!("rate: {rate:?}");
            println!("volt: {volt:?}");
            println!("write {fou}");
            if let Ok(ser) = bincode::serialize(&mb_cai) {
                std::fs::write(&fou, ser)?;
            }
            println!("writen {fln}");
        }
    }
    println!("FINISHED");
    Ok(())
}

pub async fn read_bill3() -> Result<(), Box<dyn std::error::Error>> {
    let fdir = "/mnt/d/CHMBACK/pea-data/20240801_กรอ";
    let fout = "/mnt/e/CHMBACK/pea-data/bill";
    let flst = vec!["202402", "202405"];
    //let _csv_v = Vec::<CSVFile>::new();
    let mut rate = HashMap::<String, usize>::new();
    let mut volt = HashMap::<String, usize>::new();
    for f in flst {
        let mut cn = 0;
        let fln = format!("{fdir}/export_กรอ_bil013_{}.csv", f);
        let fou = format!("{fout}/{}_3.bin", f);
        println!("start {}", &fln);
        if let Ok(mut rdr) = csv::Reader::from_path(&fln) {
            //let mut mb_pea = HashMap::<String, MeterBill>::new();
            let mut mb_cai = HashMap::<String, MeterBill>::new();
            for rs in rdr.records() {
                if let Ok(rc) = rs {
                    // if the record exist
                    if let (
                        Some(c2),
                        Some(c3),
                        Some(c4),
                        Some(c5),
                        Some(c7),
                        Some(c15),
                        Some(c18),
                        Some(c19),
                    ) = (
                        rc.get(2),
                        rc.get(3),
                        rc.get(4),
                        rc.get(5),
                        rc.get(7),
                        rc.get(15),
                        rc.get(18),
                        rc.get(19),
                    ) {
                        if let (Ok(_n15), Ok(_n18), Ok(n19)) =
                            (c15.parse::<f32>(), c18.parse::<f32>(), c19.parse::<f32>())
                        {
                            if let Some(rt) = rate.get_mut(c4) {
                                *rt += 1;
                            } else {
                                rate.insert(c4.to_string(), 1);
                            }
                            if let Some(vo) = volt.get_mut(c5) {
                                *vo += 1;
                            } else {
                                volt.insert(c5.to_string(), 1);
                            }
                            let mb0 = MeterBill {
                                pea: c7.trim().to_string(),
                                ca: c2.trim().to_string(),
                                inst: c3.trim().to_string(),
                                rate: c4.to_string(),
                                volt: c5.to_string(),
                                kwh: n19,
                            };
                            let cai = format!("{}", mb0.inst);
                            if let Some(mb) = mb_cai.get(&cai) {
                                cn += 1;
                                println!(
                                    "3. === {cn} CAI ca:{}-{} i:{}-{}",
                                    mb0.ca, mb.ca, mb0.inst, mb.inst
                                );
                            } else {
                                mb_cai.insert(cai, mb0.clone());
                            }
                        }
                    }
                } // fi record
            } // loop all rec
            println!("rate: {rate:?}");
            println!("volt: {volt:?}");
            println!("write {fou}");
            if let Ok(ser) = bincode::serialize(&mb_cai) {
                std::fs::write(&fou, ser)?;
            }
            println!("writen {fln}");
        }
    }
    println!("FINISHED");
    Ok(())
}

pub async fn read_bill4() -> Result<(), Box<dyn std::error::Error>> {
    let fdir = "/mnt/d/CHMBACK/pea-data/20240801_กรอ";
    let fout = "/mnt/e/CHMBACK/pea-data/bill";
    let flst = vec!["202402", "202405"];
    //let _csv_v = Vec::<CSVFile>::new();
    let mut rate = HashMap::<String, usize>::new();
    let mut volt = HashMap::<String, usize>::new();
    for f in flst {
        let mut cn = 0;
        let fln = format!("{fdir}/export_กรอ_bil013_{}.csv", f);
        let fou = format!("{fout}/{}_4.bin", f);
        println!("start {}", &fln);
        if let Ok(mut rdr) = csv::Reader::from_path(&fln) {
            //let mut mb_pea = HashMap::<String, MeterBill>::new();
            let mut mb_cai = HashMap::<String, MeterBill>::new();
            for rs in rdr.records() {
                if let Ok(rc) = rs {
                    // if the record exist
                    if let (
                        Some(c2),
                        Some(c3),
                        Some(c4),
                        Some(c5),
                        Some(c7),
                        Some(c15),
                        Some(c18),
                        Some(c19),
                    ) = (
                        rc.get(2),
                        rc.get(3),
                        rc.get(4),
                        rc.get(5),
                        rc.get(7),
                        rc.get(15),
                        rc.get(18),
                        rc.get(19),
                    ) {
                        if let (Ok(_n15), Ok(_n18), Ok(n19)) =
                            (c15.parse::<f32>(), c18.parse::<f32>(), c19.parse::<f32>())
                        {
                            if let Some(rt) = rate.get_mut(c4) {
                                *rt += 1;
                            } else {
                                rate.insert(c4.to_string(), 1);
                            }
                            if let Some(vo) = volt.get_mut(c5) {
                                *vo += 1;
                            } else {
                                volt.insert(c5.to_string(), 1);
                            }
                            let mb0 = MeterBill {
                                pea: c7.trim().to_string(),
                                ca: c2.trim().to_string(),
                                inst: c3.trim().to_string(),
                                rate: c4.to_string(),
                                volt: c5.to_string(),
                                kwh: n19,
                            };
                            let uid = format!("{}", mb0.pea);
                            if let Some(mb) = mb_cai.get(&uid) {
                                cn += 1;
                                println!(
                                    "4. === {cn} CAI ca:{}-{} i:{}-{}",
                                    mb0.ca, mb.ca, mb0.inst, mb.inst
                                );
                            } else {
                                mb_cai.insert(uid, mb0.clone());
                            }
                        }
                    }
                } // fi record
            } // loop all rec
            println!("rate: {rate:?}");
            println!("volt: {volt:?}");
            println!("write {fou}");
            if let Ok(ser) = bincode::serialize(&mb_cai) {
                std::fs::write(&fou, ser)?;
            }
            println!("writen {fln}");
        }
    }
    println!("FINISHED");
    Ok(())
}

pub async fn read_bill5() -> Result<(), Box<dyn std::error::Error>> {
    let fdir = "/mnt/d/CHMBACK/pea-data/20240801_กรอ";
    let flst = vec!["202402", "202405"];
    for f in flst {
        let fln = format!("{fdir}/export_กรอ_bil013_{}.csv", f);
        println!("start {}", &fln);
        if let Ok(mut rdr) = csv::Reader::from_path(&fln) {
            for rs in rdr.records() {
                if let Ok(rc) = rs {
                    // if the record exist
                    if let (
                        Some(c2),
                        Some(c3),
                        Some(c4),
                        Some(c5),
                        Some(c7),
                        Some(c15),
                        Some(c18),
                        Some(c19),
                    ) = (
                        rc.get(2),
                        rc.get(3),
                        rc.get(4),
                        rc.get(5),
                        rc.get(7),
                        rc.get(15),
                        rc.get(18),
                        rc.get(19),
                    ) {
                        if let (Ok(_n15), Ok(_n18), Ok(n19)) =
                            (c15.parse::<f32>(), c18.parse::<f32>(), c19.parse::<f32>())
                        {
                            if c2.len() <= 5 {
                                let ca = c2;
                                let inst = c3;
                                println!("ca:{ca} in:{inst}");
                            }
                            /*
                            let mb0 = MeterBill {
                                pea: c7.trim().to_string(),
                                ca: c2.trim().to_string(),
                                inst: c3.trim().to_string(),
                                rate: c4.to_string(),
                                volt: c5.to_string(),
                                kwh: n19,
                            };
                            */
                        }
                    }
                } // fi record
            } // loop all rec
        }
    }
    println!("FINISHED");
    Ok(())
}
*/

//use crate::sg::mvline::{utm_latlong, latlong_utm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
//use tis620::decode;

//use encoding_rs::*;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbfVal {
    Character(Option<String>),
    Numeric(Option<f64>),
    Logical(Option<bool>),
    Float(Option<f32>),
    Integer(i32),
    Currency(f64),
    Double(f64),
    Memo(String),
    None,
    //Date(Option<Date>),
    //DateTime(dbase::DateTime),
}

fn db_rec(rc: dbase::Record) -> HashMap<String, DbfVal> {
    let mut rec = HashMap::new();
    for (nm, va) in rc {
        let v = match &va {
            dbase::FieldValue::Character(op) => DbfVal::Character(op.clone()),
            dbase::FieldValue::Numeric(op) => DbfVal::Numeric(op.clone()),
            dbase::FieldValue::Logical(op) => DbfVal::Logical(op.clone()),
            dbase::FieldValue::Float(op) => DbfVal::Float(op.clone()),
            dbase::FieldValue::Integer(i) => DbfVal::Integer(*i),
            dbase::FieldValue::Currency(c) => DbfVal::Currency(*c),
            dbase::FieldValue::Double(v) => DbfVal::Double(*v),
            dbase::FieldValue::Memo(s) => DbfVal::Memo(s.to_string()),
            _ => DbfVal::None,
        };
        rec.insert(nm, v);
    }
    rec
}

pub fn ar_list() -> [&'static str; 12] {
    [
        "N1", "N2", "N3", "C1", "C2", "C3", "NE1", "NE2", "NE3", "S1", "S2", "S3",
    ]
}

#[allow(dead_code)]
pub fn gis_line_lays() -> [&'static str; 6] {
    [
        "DS_BusBar",
        "DS_EserviceLine",
        "DS_HVBusBar",
        "DS_HVConductor",
        "DS_LVConductor",
        "DS_MVConductor",
    ]
}

#[allow(dead_code)]
pub fn gis_pnt_lays() -> [&'static str; 17] {
    [
        "DS_Capacitor",
        "DS_CircuitBreaker",
        "DS_Generator",
        "DS_HVCircuitbreaker",
        "DS_HVGenerator",
        "DS_HVPrimaryMeter",
        "DS_HVSwitch",
        "DS_HVTransformer",
        "DS_LowVoltageMeter",
        "DS_LVCapacitor",
        "DS_LVGenerator",
        "DS_PrimaryMeter",
        "DS_RECLOSER",
        "DS_Switch",
        "DS_SwitchingFacility",
        "DS_Transformer",
        "DS_VoltageRegulator",
    ]
}

#[allow(dead_code)]
pub fn gis_data_lays() -> [&'static str; 3] {
    ["DS_GroupMeter_Detail", "GIS_HVMVCNL", "GIS_LVCNL"]
}

#[allow(dead_code)]
pub fn gis_plg_lays() -> [&'static str; 5] {
    [
        "LB_Amphoe",
        "LB_AOJ",
        "LB_Changwat",
        "LB_Tambol",
        "Zone_Use",
    ]
}

#[allow(dead_code)]
pub fn gis_dir() -> &'static str {
    //"../sgdata/OneDrive_2567-08-17/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูล GIS/GIS_Data",
    //"../sgdata/OneDrive_2567-08-17/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูล GIS/data12092567"
    //"../sgdata/OneDrive_2567-08-17/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูล GIS/GIS-2024-08-10"
    "/mnt/d/CHMBACK/pea-data/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูล GIS/GIS-2024-08-10"
}

pub fn gis2_dir() -> &'static str {
    "/mnt/d/CHMBACK/pea-data/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูล GIS/GIS-2024-09-12"
}
pub fn gis1_dir() -> &'static str {
    "/mnt/d/CHMBACK/pea-data/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูล GIS/GIS-2024-08-10"
}
pub fn db1_dir() -> &'static str {
    "../sgdata/db1"
}
pub fn db2_dir() -> &'static str {
    "../sgdata/db2"
}

pub async fn read_aoj() {
    let mut prov = HashMap::<String, Vec<Vec<Vec<(f64, f64)>>>>::new();
    for x in ar_list() {
        let rg = format!("{}/{}_LB_AOJ.rg", db1_dir(), x);
        let db = format!("{}/{}_LB_AOJ.db", db1_dir(), x);
        if let (Ok(frg), Ok(fdb)) = (File::open(&rg), File::open(&db)) {
            let rdrg = BufReader::new(frg);
            let rddb = BufReader::new(fdb);
            if let (Ok(urg), Ok(udb)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(rdrg),
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfVal>>>(rddb),
            ) {
                print!("======= : {}\n", x);
                //let ll = udb.len();
                for i in 0..udb.len() {
                    if let Some(DbfVal::Character(Some(nm))) = udb[i].get("CODE") {
                        if let Some(ls) = prov.get_mut(nm) {
                            ls.push(urg[i].clone());
                        } else {
                            prov.insert(nm.to_string(), vec![urg[i].clone()]);
                        }
                    }
                }
            }
        }
    }
    let mut cn = 0;
    for (prv, lst) in prov.into_iter() {
        cn += 1;
        print!("{:02}.{} - {}\n", cn, prv, lst.len());
    }
}

pub async fn read_prov() {
    let mut prov = HashMap::<String, Vec<Vec<Vec<(f64, f64)>>>>::new();
    for x in ar_list() {
        let rg = format!("{}/{}_LB_Changwat.rg", db1_dir(), x);
        let db = format!("{}/{}_LB_Changwat.db", db1_dir(), x);
        if let (Ok(frg), Ok(fdb)) = (File::open(&rg), File::open(&db)) {
            let rdrg = BufReader::new(frg);
            let rddb = BufReader::new(fdb);
            if let (Ok(urg), Ok(udb)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(rdrg),
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfVal>>>(rddb),
            ) {
                print!("======= : {}\n", x);
                //let ll = udb.len();
                for i in 0..udb.len() {
                    if let Some(DbfVal::Character(Some(nm))) = udb[i].get("CHANGWAT_1") {
                        if let Some(ls) = prov.get_mut(nm) {
                            ls.push(urg[i].clone());
                        } else {
                            prov.insert(nm.to_string(), vec![urg[i].clone()]);
                        }
                        //print!("  {}\n", nm);
                    }
                }
            }
        }
    }
    let mut cn = 0;
    for (prv, lst) in prov.into_iter() {
        cn += 1;
        print!("{:02}.{} - {}\n", cn, prv, lst.len());
    }
}

#[allow(dead_code)]
pub async fn read_gis_0810() {
    read_shp_0(gis1_dir(), db1_dir()).await;
}

#[allow(dead_code)]
pub async fn read_gis_0912() {
    read_shp_0(gis2_dir(), db2_dir()).await;
}

pub async fn read_shp() {
    println!("read shp 1");
    read_gis_0912().await;
    println!("read shp 2");
}

pub async fn read_shp_0(gisdir: &str, wdir: &str) {
    println!("read shp 1.1");

    //    let wdir = db1_dir();
    //let gisdir = gis_dir();
    //let lys = [
    //"N1", "N2", "N3", "C1", "C2", "C3", "NE1",
    //"NE2", "NE3", "S1",
    //"S2",
    //"S3",
    //];
    let lys = ar_list();
    let lns = [
        "DS_BusBar",
        "DS_EserviceLine",
        "DS_HVBusBar",
        "DS_HVConductor",
        "DS_LVConductor",
        "DS_MVConductor",
    ];
    let pns = [
        "DS_Capacitor",
        "DS_CircuitBreaker",
        "DS_Generator",
        "DS_HVCircuitbreaker",
        "DS_HVGenerator",
        "DS_HVPrimaryMeter",
        "DS_HVSwitch",
        "DS_HVTransformer",
        "DS_LowVoltageMeter",
        "DS_LVCapacitor",
        "DS_LVGenerator",
        "DS_PrimaryMeter",
        "DS_RECLOSER",
        "DS_Switch",
        "DS_SwitchingFacility",
        "DS_Transformer",
        "DS_VoltageRegulator",
    ];
    let dbs = ["DS_GroupMeter_Detail", "GIS_HVMVCNL", "GIS_LVCNL"];
    let rgs = [
        "LB_Amphoe",
        "LB_AOJ",
        "LB_Changwat",
        "LB_Tambol",
        "Zone_Use",
    ];

    //let gisdir = format!("../sgdata/OneDrive_2567-08-17/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูล GIS/GIS_Data");
    for r in lys {
        let agisdir = format!("{}/{}", gisdir, r);
        //let wdir = format!("../sgdata/db1");
        std::fs::create_dir_all(&wdir).expect("ERR");

        // POLYGON
        for rg in rgs {
            let rgf = format!("{}/{}.shp", agisdir, rg);
            println!("rgf {}", rgf);
            let mut cnt = 0;
            let mut cnu = 0;
            if let Ok(mut reader) = shapefile::Reader::from_path(rgf.clone()) {
                let mut vrg = vec![];
                let mut vdb = vec![];
                for result in
                    reader.iter_shapes_and_records_as::<shapefile::Polygon, dbase::Record>()
                {
                    if let Ok((gon, rc)) = result {
                        let mut ringpnts = Vec::<Vec<(f64, f64)>>::new();
                        for ring in gon.into_inner() {
                            let mut pnts = Vec::<(f64, f64)>::new();
                            for pnt in ring.into_inner() {
                                pnts.push((pnt.x, pnt.y));
                                //cnt += 1;
                            }
                            ringpnts.push(pnts);
                            cnt += 1;
                        }
                        cnu += 1;
                        vrg.push(ringpnts);
                        let r = db_rec(rc);
                        vdb.push(r);
                    }
                }
                print!("rg: {} cnu:{} cnt:{}\n", rgf, cnu, cnt);

                let dbw = format!("{}/{}_{}.db", wdir, r, rg);
                if let Ok(ser) = bincode::serialize(&vdb) {
                    std::fs::write(dbw, ser).unwrap();
                }
                let rgw = format!("{}/{}_{}.rg", wdir, r, rg);
                if let Ok(ser) = bincode::serialize(&vrg) {
                    std::fs::write(rgw, ser).unwrap();
                }
            }
        }

        // POINT FILE
        for pn in pns {
            let pnf = format!("{}/{}.shp", agisdir, pn);
            let mut cnt = 0;

            if let Ok(mut reader) = shapefile::Reader::from_path(pnf.clone()) {
                let mut vpn = vec![];
                let mut vdb = vec![];
                for result in reader.iter_shapes_and_records_as::<shapefile::Point, dbase::Record>()
                {
                    if let Ok((pnt, rc)) = result {
                        vpn.push((pnt.x, pnt.y));
                        let r = db_rec(rc);
                        vdb.push(r);
                        cnt += 1;
                    }
                }
                print!("pn {} {}\n", pnf, cnt);

                let dbw = format!("{}/{}_{}.db", wdir, r, pn);
                if let Ok(ser) = bincode::serialize(&vdb) {
                    std::fs::write(dbw, ser).unwrap();
                }
                let pnw = format!("{}/{}_{}.pn", wdir, r, pn);
                if let Ok(ser) = bincode::serialize(&vpn) {
                    std::fs::write(pnw, ser).unwrap();
                }
            }
        }

        // LINE FILE
        for ln in lns {
            let lnf = format!("{}/{}.shp", agisdir, ln);
            let mut cnt = 0;
            let mut vdb = vec![];
            let mut vln = vec![];
            if let Ok(mut reader) = shapefile::Reader::from_path(&lnf) {
                for result in
                    reader.iter_shapes_and_records_as::<shapefile::Polyline, dbase::Record>()
                {
                    if let Ok((line, rc)) = result {
                        let mut lines = vec![];
                        for vpnts in line.into_inner() {
                            let mut line = vec![];
                            for pnt in vpnts {
                                line.push((pnt.x, pnt.y));
                            }
                            lines.push(line);
                        }
                        vln.push(lines);
                        let r = db_rec(rc);
                        vdb.push(r);
                        cnt += 1;
                    }
                }
                print!("ln: {} : {}\n", lnf, cnt);

                let dbw = format!("{}/{}_{}.db", wdir, r, ln);
                if let Ok(ser) = bincode::serialize(&vdb) {
                    std::fs::write(dbw, ser).unwrap();
                }
                let lnw = format!("{}/{}_{}.ln", wdir, r, ln);
                if let Ok(ser) = bincode::serialize(&vln) {
                    std::fs::write(lnw, ser).unwrap();
                }
            }
        }

        // DBASE FILE
        for db in dbs {
            let dbf = format!("{}/{}.dbf", agisdir, db);
            let mut cnt = 0;
            let mut vdb = vec![];
            if let Ok(records) = dbase::read(dbf.clone()) {
                for rc in records {
                    let r = db_rec(rc);
                    vdb.push(r);
                    cnt += 1;
                }
            }
            print!("db: {} {}\n", dbf, cnt);
            let dbw = format!("{}/{}_{}.db", wdir, r, db);
            if let Ok(ser) = bincode::serialize(&vdb) {
                std::fs::write(dbw, ser).unwrap();
            }
        }
    }
}

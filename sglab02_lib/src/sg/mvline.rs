use calamine::{deserialize_as_f64_or_none, open_workbook, /*RangeDeserializerBuilder*/ Reader, Xlsx};
use micromath::F32Ext;
//use shapefile::{Point, PolygonRing};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

pub fn utm_latlong(x: f32, y: f32) -> (f32, f32) {
    //fn utm_latlong(x: f64, y: f64) -> (f64, f64) {
    let e5 = x;
    let f5 = y;
    let c12 = 6378137.0_f32;
    let c13 = 6356752.31424518_f32;
    let _c15 = (c12 * c12 - c13 * c13).sqrt() / c12;
    let c16 = (c12 * c12 - c13 * c13).sqrt() / c13;
    let c17 = c16 * c16;
    let c18 = c12.powf(2.0) / c13;
    //System.out.println("C17: "+C17+" C18:"+C18);
    let c21 = 47.0;
    let c22 = 'N';
    let o5 = if c22 == 'S' { f5 - 10000000.0 } else { f5 };
    let k5 = o5 / (6366197.724 * 0.9996);
    // $C$17*(COS($K$5))^2
    let l7 = c17 * k5.cos().powf(2.0);
    let l8 = (1.0 + l7).powf(0.5);
    //=(1+L7)^(1/2)
    let l9 = c18 / l8 * 0.9996;
    //System.out.println("L7:"+L7+" L8:"+L8+" L9:"+L9);
    let l5 = l9;
    let p5 = (e5 - 500000.0) / l5;
    let aa5 = ((c17 * p5.powf(2.0)) / 2.0) * k5.cos().powf(2.0);
    let ab5 = p5 * (1.0 - (aa5 / 3.0));
    let ad5 = (ab5.exp() - (-ab5).exp()) / 2.0;
    let q5 = (2.0 * k5).sin();
    let r5 = q5 * k5.cos().powf(2.0);
    let s5 = k5 + (q5 / 2.0);
    let t5 = (3.0 * s5 + r5) / 4.0;
    //System.out.println("Q5: "+ Q5+" R5:"+R5+" S5:"+S5+" T5:"+T5);
    let u5 = (5.0 * t5 + r5 * k5.cos().powf(2.0)) / 3.0;
    let v5 = (0.75) * c17;
    let w5 = (5.0 / 3.0) * v5.powf(2.0);
    let x5 = (35.0 / 27.0) * v5.powf(3.0);
    //System.out.println("U5: "+ U5+" V5:"+V5+" W5:"+W5+" X5:"+X5);
    let y5 = 0.9996 * c18 * (k5 - (v5 * s5) + (w5 * t5) - (x5 * u5));
    let z5 = (o5 - y5) / l5;
    let ac5 = z5 * (1.0 - aa5) + k5;
    let ae5 = (ad5 / ac5.cos()).atan();
    //System.out.println("AA5:"+ AA5+" AB5:"+AB5+" AD5:"+AD5+" AC5:"+AC5+" AE5:"+AE5);
    let af5 = (ae5.cos() * ac5.tan()).atan();
    let m5 = k5
        + (1.0 + c17 * k5.cos().powf(2.0) - (3.0 / 2.0) * c17 * k5.sin() * k5.cos() * (af5 - k5))
            * (af5 - k5);
    let n5 = 6.0 * c21 - 183.0;
    let ag5 = m5 / std::f32::consts::PI * 180.0;
    let ah5 = ae5 / std::f32::consts::PI * 180.0 + n5;
    (ag5, ah5)
}

pub fn latlong_utm(e5: f32, f5: f32) -> (f32, f32) {
    let c12 = 6378137.0;
    let c13 = 6356752.31424518;
    let _c15 = (c12 * c12 - c13 * c13).sqrt() / c12;
    let c16 = (c12 * c12 - c13 * c13).sqrt() / c13;
    let c17 = c16 * c16;
    let c18 = c12.powf(2.0) / c13;
    let pi = 3.14159f32;
    let g5 = f5 * pi / 180.0;
    let h5 = e5 * pi / 180.0;
    let i5 = 47.0;
    let j5 = 6.0 * i5 - 183.0;
    let k5 = g5 - ((j5 * pi) / 180.0);
    let n5 = (h5.tan() / k5.cos()).atan() - h5;
    let l5 = h5.cos() * k5.sin(); //Math.cos(H5)*Math.sin(K5);
    let m5 = 0.5 * ((1.0 + l5) / (1.0 - l5)).ln();
    let o5 = c18 / (1.0 + c17 * h5.cos().powf(2.0)).powf(0.5) * 0.9996;
    let p5 = (c17 / 2.0) * m5.powf(2.0) * h5.cos().powf(2.0);
    let v5 = (3.0 / 4.0) * c17;
    let q5 = (2.0 * h5).sin();
    let s5 = h5 + (q5 / 2.0);
    let r5 = q5 * h5.cos().powf(2.0);
    let w5 = (5.0 / 3.0) * v5.powf(2.0);
    let t5 = ((3.0 * s5) + r5) / 4.0;
    let x5 = (35.0 / 27.0) * v5.powf(3.0);
    let u5 = 5.0 * t5 + r5 * h5.cos().powf(2.0) / 3.0;
    let y5 = 0.9996 * c18 * (h5 - (v5 * s5) + (w5 * t5) - (x5 * u5));
    let ad5 = n5 * o5 * (1.0 + p5) + y5;
    let ac5 = m5 * o5 * (1.0 + p5 / 3.0) + 500000.0;
    (ac5, ad5)
}

#[derive(Deserialize)]
struct Record {
    #[allow(dead_code)]
    metric: String,
    #[serde(deserialize_with = "deserialize_as_f64_or_none")]
    #[allow(dead_code)]
    value: Option<f64>,
}

pub async fn excel() -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("../sgdata/bess-plan-2024-07-10.xlsx");
    let mut excel: Xlsx<_> = open_workbook(path)?;
    let reg = [
        "ภาคกลาง",
        "ภาคใต้",
        "ภาคเหนือ",
        "ภาคตะวันออกเฉียงเหนือ",
    ];
    for r in reg {
        let range = excel
            .worksheet_range(r)
            .map_err(|_| calamine::Error::Msg("Cannot find ภาคกลาง"))?;
        for row in range.rows() {
            println!("{}", row[1]);
        }
    }
    Ok(())
}

pub async fn read() {
    print!("READ\n");
    let lys = [
        "N1", "N2", "N3", "C1", "C2", "C3", "NE1", "NE2", "NE3", "S1", "S2", "S3",
    ];
    for r in lys {
        let mut sbgismp = HashMap::new();
        let f = format!("../sgdata/ShpMV/Shp{}/DS_T_Station.shp", r);
        print!("LY: {}\n", f);
        if let Ok(mut reader) = shapefile::Reader::from_path(&f) {
            print!(" OK\n");
            let (mut abbr, mut thnm, mut name) = ("".to_string(), "".to_string(), "".to_string());
            let (mut sub, mut own, mut btp) = ("".to_string(), "".to_string(), "".to_string());
            for result in reader.iter_shapes_and_records_as::<shapefile::Point, dbase::Record>() {
                if let Ok((pnt, rc)) = result {
                    for (nm, va) in rc {
                        /*
                        if let dbase::FieldValue::Character(Some(s)) = &va {
                            print!("{} = {}\n", nm, va);
                        }
                        */
                        if nm == "ABBRNAME" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                abbr = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "NAME_THAI" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                thnm = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "STATIONNAM" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                name = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "SUBSTATION" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                sub = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "OWNER" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                own = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "BUSTYPE" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                btp = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        }
                    }
                    let (x, y) = utm_latlong(pnt.x as f32, pnt.y as f32);
                    print!(
                        "({},{}) {},{},{},{},{},{}\n",
                        x, y, abbr, thnm, name, sub, own, btp
                    );
                    sbgismp.insert(
                        abbr.to_string(),
                        (
                            x,
                            y,
                            thnm.to_string(),
                            name.to_string(),
                            sub.to_string(),
                            own.to_string(),
                            btp.to_string(),
                        ),
                    );
                }
            }
        }
        if let Ok(se) = bincode::serialize(&sbgismp) {
            std::fs::write(crate::sg::ldp::res("sbgismp.bin"), se).unwrap();
        }
    }
}

pub async fn read_lv_line() {
    let lys = [
        "N1", "N2", "N3", "C1", "C2", "C3", "NE1", "NE2", "NE3", "S1", "S2", "S3",
    ];
    for r in lys {
        let mut cnt = 0;
        let f = format!("../sgdata/LV_conductor_12_area/DS_LVConductor_{}.shp", r);
        if let Ok(mut reader) = shapefile::Reader::from_path(&f) {
            print!("READ: {}\n", f);
            for result in reader.iter_shapes_and_records_as::<shapefile::Polyline, dbase::Record>()
            {
                if let Ok((line, _rc)) = result {
                    for vpnts in line.into_inner() {
                        for pnt in vpnts {
                            let (lat, lng) = utm_latlong(pnt.x as f32, pnt.y as f32);
                            let (tx, ty) = latlong_utm(lat, lng);
                            print!(
                                "pnt {},{} - {},{} - {},{}\n",
                                pnt.x, pnt.y, lat, lng, tx, ty
                            );
                        }
                    }
                    cnt += 1;
                }
            }
            print!("  lines: {}\n", cnt);
        }
    }
}

pub async fn read_trans_lv() {
    print!("READ\n");
    let lys = [
        "N1", "N2", "N3", "C1", "C2", "C3", "NE1", "NE2", "NE3", "S1", "S2", "S3",
    ];
    let /*mut*/ _fst = true;
    let mut mt_400 = 0;
    let mut mt_lng = 0;
    for r in lys {
        let z1 = format!("../sgdata/ShpMV/Shp{}/GIS_LVCNL.dbf", r);
        print!("FILE {}\n", z1);
        if let Ok(rc) = dbase::read(z1) {
            for r in rc {
                //let mut mt = "".to_string();
                let mut fd = "".to_string();
                //let mut ph = "".to_string();
                //let mut of = "".to_string();
                //let mut tx = "".to_string();
                //let mut sb = "".to_string();
                //let mut tp = "".to_string();
                //let mut ow = "".to_string();
                //let mut pw = 0f64;
                let mut txlat = 0f64;
                let mut txlng = 0f64;
                let mut mtlat = 0f64;
                let mut mtlng = 0f64;
                for (nm, va) in r {
                    let nms = nm.to_string();
                    let v = if let dbase::FieldValue::Character(Some(s)) = &va {
                        s.to_string()
                    } else {
                        "".to_string()
                    };
                    let n = if let dbase::FieldValue::Numeric(Some(n)) = &va {
                        n.clone()
                    } else {
                        0f64
                    };
                    match nms.as_str() {
                        "PEA_METER" => /*mt = v*/ {},
                        "METER_PHAS" => /*ph = v*/ {},
                        "TRF_PEA_NO" => /*tx = v*/ {},
                        "TRF_KVA" => /*pw = n*/ {},
                        "METER_AOJ" => /*of = v*/ {},
                        "TRF_FEEDER" => fd = v,
                        "TRF_SUBTYP" => {}/*tp = v*/,
                        "TRF_OWNER" => {}/*ow = v*/,
                        "TRF_LAT" => txlat = n,
                        "TRF_LONG" => txlng = n,
                        "MT_LAT" => mtlat = n,
                        "MT_LONG" => mtlng = n,
                        _ => {}
                    }
                    //print!("k: {}\n", nms);
                }
                if fd.len() > 3 {
                    //sb = fd[0..3].to_string();
                }
                let (txx, txy) = latlong_utm(txlat as f32, txlng as f32);
                let (mtx, mty) = latlong_utm(mtlat as f32, mtlng as f32);
                let (dx, dy) = ((mtx - txx), (mty - txy));
                let len = (dx * dx + dy * dy).sqrt();
                if len < 400.0 {
                    mt_400 += 1;
                } else {
                    mt_lng += 1;
                }
            }
        }
        print!("meter {}/{}\n", mt_400, mt_lng);
    }
    print!("meter {}/{}\n", mt_400, mt_lng);
    /*
    let f = format!("../sgdata/LVConductor/DS_LVConductor.shp");
    let mut all = 0;
    if let Ok(mut reader) = shapefile::Reader::from_path(&f) {
        for result in reader.iter_shapes_and_records_as::<shapefile::Polyline, dbase::Record>() {
            if let Ok((pnt, rc)) = result {
                all += 1;
            }
        }
    }
    print!("all {}\n", all);
    */
    /*
    let mut fst = true;
    let mut all = 0;
    for r in lys {
        let mut cnt = 0;
        let f = format!("../sgdata/ShpMV/Shp{}/DS_Transformer.shp", r);
        if let Ok(mut reader) = shapefile::Reader::from_path(&f) {
            print!(" OK\n");
            for result in reader.iter_shapes_and_records_as::<shapefile::Point, dbase::Record>() {
                if let Ok((pnt, rc)) = result {
                    if fst {
                        for (nm, va) in rc {
                            print!("{}\n", nm);
                        }
                        fst = false;
                    }
                    cnt += 1;
                }
            }
        }
        all += cnt;
        print!("LY: {} cnt:{}\n", f, cnt);
    }
    print!("all trx {}\n", all);
    */
}

pub fn sub_xls() -> &'static str {
    //"../sgdata/OneDrive_2567-08-17/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูล GIS/GIS_Data",
    //"../sgdata/OneDrive_2567-08-17/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูล GIS/data12092567"
    "../sgdata/OneDrive_2567-08-17/ข้อมูลส่งให้อาจารย์ มธ/ข้อมูลสถานีไฟฟ้าทุกเขต/GOC Substation & Another Detail.xlsx"
}

#[derive(Debug,Serialize,Deserialize)]
pub struct SubInfo {
    pub ord: String,
    pub area: String,
    pub name: String,
    pub sbid: String,
    pub volt: String,
    pub cate: String,
    pub egat: String,
    pub state: String,
    pub conf: String,
    pub trax: String,
    pub mvax: String,
    pub feed: String,
    pub cnt: usize,
}

#[allow(dead_code)]
pub fn db1_dir() -> &'static str {
    "../sgdata/db1"
}

pub async fn pea_sub_excel() -> Result<(), Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook(sub_xls())?;
    let range = excel.worksheet_range("ส่ง กรอ.").map_err(|_| calamine::Error::Msg("Cannot find ภาคกลาง"))?;
    let mut sbhs = HashMap::<String,SubInfo>::new();
    for row in range.rows() {
        let ord = row[0].to_string();
        let area = row[1].to_string();
        let name = row[2].to_string();
        let sbid = row[3].to_string();
        let volt = row[4].to_string();
        let cate = row[5].to_string();
        let egat = row[6].to_string();
        let state = row[7].to_string();
        let conf = row[8].to_string();
        let trax = row[9].to_string();
        let mvax = row[10].to_string();
        let feed = row[11].to_string();
    
        if let Some(/*mut*/ sbdt) = sbhs.get_mut(&sbid) {
            sbdt.cnt += 1;
        } else {
            let sbid0 = sbid.clone();
            let sbinf = SubInfo { ord, area, name, sbid, volt, cate, egat, state, conf, trax, mvax, feed, cnt: 1 };
            sbhs.insert(sbid0, sbinf);
        }
    }
    let sbinfo = format!("{}/sb_info.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&sbhs) {
        std::fs::write(sbinfo, ser).unwrap();
    }
    println!("sub no:{}", sbhs.len());
    Ok(())
}

pub async fn pea_sub_read() -> Result<(), Box<dyn std::error::Error>> {
    let psbinfo = format!("{}/sb_info.bin", crate::sg::imp::data_dir());

    if let Ok(fsbinfo) = File::open(&psbinfo) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(sbinfo_hs) = bincode::deserialize_from::<BufReader<File>,HashMap::<String,SubInfo>>(rsbinfo) {
            println!("SUB: {}", sbinfo_hs.len());
            if let Some(sbinfo) = sbinfo_hs.get("CMU") {
                println!("{:?}", sbinfo);
                println!("state:{}", sbinfo.state);
            }
        }
    }
    Ok(())
}

//use crate::sg::ldp::base;
use calamine::{
    /*deserialize_as_f64_or_none*/ open_workbook, Data,
    /*RangeDeserializerBuilder*/ Reader as XlsReader, Xlsx,
};
use csv;
use quick_xml::events::{/*BytesStart*/ Event};
use quick_xml::reader::Reader as XmlReader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_dir;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn src_dir() -> &'static str {
    //"E:\\CHMBACK\\pea-data\\ข้อมูลส่งให้อาจารย์ มธ"
    //"/mnt/e/CHMBACK/pea-data/ข้อมูลส่งให้อาจารย์ มธ"
    "/mnt/d/CHMBACK/pea-data/data"
}

pub fn data_dir() -> &'static str {
    "../sgdata"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SPPLoadProfile {
    pub substation: String,
    pub feeder: String,
    pub date: String,
    pub time: String,
    pub mw: String,
    pub pf: String,
    pub mvar: String,
    pub vab: String,
    pub vbc: String,
    pub vca: String,
    pub ia: String,
    pub ib: String,
    pub ic: String,
}

pub async fn pea_sub_xml_read() -> Result<(), Box<dyn std::error::Error>> {
    let fd = format!(
        "{}\\ผัง single line diagram สถานีไฟฟ้า\\CIM_PEA_Substation\\Exports",
        src_dir()
    );
    println!("{}", fd);
    //let mut cn = 0;
    if let Ok(paths) = read_dir(fd) {
        // if paths
        let mut subxv = vec![];
        for pt in paths {
            // for pt
            if let Ok(pt) = pt {
                // if pt
                let mut subhs = HashMap::new();
                let mut xml_file = String::from("?");
                if let Some(f) = pt.path().file_name() {
                    if let Some(f) = f.to_str() {
                        xml_file = f.to_string();
                    }
                }
                subhs.insert("xml_file".to_string(), xml_file.clone());
                //println!("file name {}", xml_file);
                let xmlf = pt.path().display().to_string();
                if let Ok(mut xrd) = XmlReader::from_file(xmlf) {
                    let mut xbuf = Vec::new();
                    //let mut _lv = 0;
                    let mut tagx = Vec::<String>::new();
                    let mut txts = "".to_string();
                    //cn += 1;
                    //println!("xml OK {}", cn);
                    loop {
                        // xml
                        match xrd.read_event_into(&mut xbuf) {
                            // xml match
                            Ok(Event::Eof) => break,
                            Ok(Event::Start(e)) => {
                                // e
                                txts = "".to_string();
                                //lv += 1;
                                let mut el = String::from("?");
                                if let Ok(q) = String::from_utf8(e.name().0.to_vec()) {
                                    el = format!("{}", q);
                                }
                                tagx.push(el);
                            } // e
                            Ok(Event::End(e)) => {
                                // e
                                //lv -= 1;
                                //let mut el = String::from("?");
                                if let Ok(_q) = String::from_utf8(e.name().0.to_vec()) {
                                    //el = format!("{}", q);
                                }
                                if tagx.len() == 3 && txts.len() > 0 && tagx[1] == "cim:Substation"
                                {
                                    let tg = format!("/{}/{}", tagx[1], tagx[2]);
                                    subhs.insert(tagx[2].to_string(), txts.clone());
                                    if tg == "/cim:Substation/cim:IdentifiedObject.description" {
                                        println!("{}: {}", tg, txts);
                                    }
                                }
                                tagx.pop();
                                //println!("gat-{}: {}",lv,el);
                            }
                            Ok(Event::Text(tx)) => {
                                // text
                                //let mut txt = String::from("?");
                                if let Ok(tx) = String::from_utf8(tx.to_vec()) {
                                    if tx.len() > 0 {
                                        txts.push_str(&tx);
                                        //txt = format!("{}", tx);
                                        //println!("txt: {}",txt);
                                    }
                                }
                            } // text
                            _ => (),
                        } // xml match
                        xbuf.clear();
                    } // xml
                      //println!("sub: {:?}", subhs);
                } // read xml
                subxv.push(subhs);
            } // fi pt
        } // rof pt
        if let Ok(ser) = bincode::serialize(&subxv) {
            // if serialize
            let subxv = format!("{}/sub_xml.bin", data_dir());
            std::fs::write(subxv, ser).unwrap();
            println!("write sub xml list");
        } // end serialize
    } // endif paths
    Ok(())
}

pub async fn pea_spp_lp_read() -> Result<(), Box<dyn std::error::Error>> {
    let yrs = vec!["2022", "2023"];
    for yr in yrs {
        let lp = format!("{}\\ข้อมูล P, Q VSPP\\SPP {}\\SPP {}.csv", src_dir(), yr, yr);
        println!("{}", lp);
        if let Ok(mut rdr) = csv::Reader::from_path(&lp) {
            // if read file
            let mut lpv = Vec::<SPPLoadProfile>::new();
            print!("read \n");
            for rs in rdr.records() {
                // loop all record
                if let Ok(rc) = rs {
                    // if the record exist
                    if let (
                        // if all data exist
                        Some(substation),
                        Some(feeder),
                        Some(date),
                        Some(time),
                        Some(mw),
                        Some(pf),
                        Some(mvar),
                        Some(vab),
                        Some(vbc),
                        Some(vca),
                        Some(ia),
                        Some(ib),
                        Some(ic),
                    ) = (
                        rc.get(0),
                        rc.get(1),
                        rc.get(2),
                        rc.get(3),
                        rc.get(4),
                        rc.get(5),
                        rc.get(6),
                        rc.get(7),
                        rc.get(8),
                        rc.get(9),
                        rc.get(10),
                        rc.get(11),
                        rc.get(12),
                    ) {
                        let substation = substation.to_string();
                        let feeder = feeder.to_string();
                        let date = date.to_string();
                        let time = time.to_string();
                        let mw = mw.to_string();
                        let pf = pf.to_string();
                        let mvar = mvar.to_string();
                        let vab = vab.to_string();
                        let vbc = vbc.to_string();
                        let vca = vca.to_string();
                        let ia = ia.to_string();
                        let ib = ib.to_string();
                        let ic = ic.to_string();

                        let lpf = SPPLoadProfile {
                            substation,
                            feeder,
                            date,
                            time,
                            mw,
                            pf,
                            mvar,
                            vab,
                            vbc,
                            vca,
                            ia,
                            ib,
                            ic,
                        };

                        lpv.push(lpf);
                    } // all data exists
                } // if record exist
            } // end loop record
            if let Ok(ser) = bincode::serialize(&lpv) {
                // if serialize
                let lpf = format!("{}/spp-{}.bin", data_dir(), yr);
                std::fs::write(lpf, ser).unwrap();
            } // end serialize
        } // end read file
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
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

pub async fn pea_sub_excel() -> Result<(), Box<dyn std::error::Error>> {
    let file = format!(
        "{}/ข้อมูลสถานีไฟฟ้าทุกเขต/GOC Substation & Another Detail.xlsx",
        src_dir()
    );

    let mut excel: Xlsx<_> = open_workbook(file)?;
    let range = excel
        .worksheet_range("ส่ง กรอ.")
        .map_err(|_| calamine::Error::Msg("Cannot find ภาคกลาง"))?;
    let mut sbhs = HashMap::<String, SubInfo>::new();
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
            let sbinf = SubInfo {
                ord,
                area,
                name,
                sbid,
                volt,
                cate,
                egat,
                state,
                conf,
                trax,
                mvax,
                feed,
                cnt: 1,
            };
            sbhs.insert(sbid0, sbinf);
        }
    }
    let sbinfo = format!("{}/sub_xlsx.bin", crate::sg::imp::data_dir());
    if let Ok(ser) = bincode::serialize(&sbhs) {
        std::fs::write(sbinfo, ser).unwrap();
    }
    println!("sub xlsx no:{}", sbhs.len());
    Ok(())
}

pub async fn pea_sub_read() -> Result<(), Box<dyn std::error::Error>> {
    let psbinfo = format!("{}/sub_xlsx.bin", crate::sg::imp::data_dir());

    if let Ok(fsbinfo) = File::open(&psbinfo) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(sbinfo_hs) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, SubInfo>>(rsbinfo)
        {
            println!("SUB: {}", sbinfo_hs.len());
            if let Some(sbinfo) = sbinfo_hs.get("CMU") {
                println!("{:?}", sbinfo);
                println!("state:{}", sbinfo.state);
                println!("{:?}", sbinfo);
            }
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VSPPLoadProfile {
    pub plant_code: String,
    pub date: String,
    pub time: String,
    pub mw: String,
}

pub async fn pea_vspp_lp_read() -> Result<(), Box<dyn std::error::Error>> {
    let yrs = vec!["2022", "2023"];
    for yr in yrs {
        let lp = format!(
            "{}\\ข้อมูล P, Q VSPP\\VSPP {}\\VSPP {}.csv",
            src_dir(),
            yr,
            yr
        );
        println!("{}", lp);
        if let Ok(mut rdr) = csv::Reader::from_path(&lp) {
            // if read file
            let mut lpv = Vec::<VSPPLoadProfile>::new();
            print!("read \n");
            for rs in rdr.records() {
                // loop all record
                if let Ok(rc) = rs {
                    // if the record exist
                    if let (
                        // if all data exist
                        Some(plant_code),
                        Some(date),
                        Some(time),
                        Some(mw),
                    ) = (rc.get(0), rc.get(1), rc.get(2), rc.get(3))
                    {
                        let plant_code = plant_code.to_string();
                        let date = date.to_string();
                        let time = time.to_string();
                        let mw = mw.to_string();
                        let lpf = VSPPLoadProfile {
                            plant_code,
                            date,
                            time,
                            mw,
                        };

                        lpv.push(lpf);
                    }
                }
            }
            if let Ok(ser) = bincode::serialize(&lpv) {
                // if serialize
                let lpf = format!("{}/vspp-{}.bin", data_dir(), yr);
                std::fs::write(lpf, ser).unwrap();
            } // end serialize
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SPPInfo {
    pub ord: String,
    pub code: String,
    pub tamb: String,
    pub amp: String,
    pub prov: String,
    pub abbr: String,
    pub fuel: String,
    pub area: String,
    pub kv: String,
    pub conn: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VSPPInfo {
    pub ord: String,
    pub code: String,
    pub area: String,
    pub tamb: String,
    pub amp: String,
    pub prov: String,
    pub fuel: String,
    pub kv: String,
    pub conn: String,
    pub fdno: String,
    pub circ: String,
    pub ppid: String,
}

pub async fn pea_vspp_excel() -> Result<(), Box<dyn std::error::Error>> {
    let file = format!(
        "{}/ข้อมูลSPP, VSPP ที่เชื่อมต่อ กฟภ/สถานะ VSPP + SPP (กรอ.) - updated.xlsx",
        src_dir()
    );
    println!("FILE {}", file);
    let mut excel: Xlsx<_> = open_workbook(file)?;
    let mut spp_info = vec![];
    if let Ok(range) = excel.worksheet_range("SPP") {
        for row in range.rows() {
            let ord = row[0].to_string();
            let code = row[1].to_string();
            let tamb = row[2].to_string();
            let amp = row[3].to_string();
            let prov = row[4].to_string();
            let abbr = row[5].to_string();
            let fuel = row[6].to_string();
            let area = row[7].to_string();
            let kv = row[8].to_string();
            let conn = row[9].to_string();
            let sppinf = SPPInfo {
                ord,
                code,
                tamb,
                amp,
                prov,
                abbr,
                fuel,
                area,
                kv,
                conn,
            };
            spp_info.push(sppinf);
        }
        if let Ok(ser) = bincode::serialize(&spp_info) {
            // if serialize
            let lpf = format!("{}/spp_info.bin", data_dir());
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    let mut vspp_info = vec![];
    if let Ok(range) = excel.worksheet_range("VSPP") {
        println!("YES2");
        for row in range.rows() {
            let ord = row[0].to_string();
            let code = row[1].to_string();
            let area = row[2].to_string();
            let tamb = row[3].to_string();
            let amp = row[4].to_string();
            let prov = row[5].to_string();
            let fuel = row[6].to_string();
            let kv = row[7].to_string();
            let conn = row[8].to_string();
            let fdno = row[9].to_string();
            let circ = row[10].to_string();
            let ppid = row[11].to_string();
            let vsppinf = VSPPInfo {
                ord,
                code,
                area,
                tamb,
                amp,
                prov,
                fuel,
                kv,
                conn,
                fdno,
                circ,
                ppid,
            };
            vspp_info.push(vsppinf);
        }
        if let Ok(ser) = bincode::serialize(&vspp_info) {
            // if serialize
            let lpf = format!("{}/vspp_info.bin", data_dir());
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlsSheet {
    path: String,
    name: String,
    shnm: String,
    rcnt: usize,
    ccnt: usize,
    data: Vec<Vec<String>>,
}

pub async fn xlsx_info(flst: &Vec<String>) -> Result<Vec<XlsSheet>, Box<dyn std::error::Error>> {
    let mut xlsv = Vec::<XlsSheet>::new();
    for fl in flst {
        let pt = PathBuf::from(fl.clone());
        let mut excel: Xlsx<_> = open_workbook(fl.clone())?;
        let ff = pt.file_name().unwrap().to_str().unwrap();
        let sheets = excel.sheet_names().to_owned();
        for sh in &sheets {
            if let Ok(range) = excel.worksheet_range(sh) {
                let path = fl.to_string();
                let name = ff.to_string();
                let shnm = sh.to_string();
                let rcnt = range.get_size().0;
                let ccnt = range.get_size().1;
                let mut data = Vec::<Vec<String>>::new();

                for row in range.rows() {
                    let mut rw = Vec::<String>::new();
                    for c in 0..row.len() {
                        rw.push(row[c].to_string());
                    }
                    data.push(rw);
                }

                let xls_info = XlsSheet {
                    path,
                    name,
                    shnm,
                    rcnt,
                    ccnt,
                    data,
                };
                xlsv.push(xls_info);
            }
        }
    }
    Ok(xlsv)
}

pub async fn pea_lv_solar_xlsx() -> Result<(), Box<dyn std::error::Error>> {
    let fd = format!("{}\\ข้อมูลSolar แรงต่ำ 12 เขต", src_dir());
    let mut vdir = vec![fd];
    let mut flst = vec![];
    while let Some(dr) = vdir.pop() {
        if let Ok(paths) = read_dir(dr) {
            // if paths
            for pt in paths {
                // for pt
                if let Ok(pt) = pt {
                    // if pt
                    let pt = pt.path();
                    let pn = pt.display().to_string();
                    if pt.is_dir() {
                        vdir.push(pn);
                    } else {
                        if pn.ends_with(".xlsx") {
                            flst.push(pn);
                        }
                    }
                }
            }
        }
    }
    println!("FILE - {}", flst.len());
    if let Ok(xlsv) = xlsx_info(&flst).await {
        println!("xlsv: {}", xlsv.len());
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
        }
        if let Ok(ser) = bincode::serialize(&xlsv) {
            // if serialize
            let lpf = format!("{}/solar_xlsx.bin", data_dir());
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    Ok(())
}

pub async fn pea_der_xlsx() -> Result<(), Box<dyn std::error::Error>> {
    let fd = format!("{}\\ข้อมูล DERS", src_dir());
    let mut vdir = vec![fd];
    let mut flst = vec![];
    while let Some(dr) = vdir.pop() {
        if let Ok(paths) = read_dir(dr) {
            // if paths
            for pt in paths {
                // for pt
                if let Ok(pt) = pt {
                    // if pt
                    let pt = pt.path();
                    let pn = pt.display().to_string();
                    if pt.is_dir() {
                        vdir.push(pn);
                    } else {
                        if pn.ends_with(".xlsx") {
                            flst.push(pn);
                        }
                    }
                }
            }
        }
    }
    println!("FILE - {}", flst.len());
    if let Ok(xlsv) = xlsx_info(&flst).await {
        println!("xlsv: {}", xlsv.len());
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
        }
        if let Ok(ser) = bincode::serialize(&xlsv) {
            // if serialize
            let lpf = format!("{}/der_xlsx.bin", data_dir());
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    Ok(())
}

pub async fn pea_bizme() -> Result<(), Box<dyn std::error::Error>> {
    //let flst = vec![format!("{}\\ข้อมูลหม้อแปลงยกลง ก.ย.66-ก.ย.67\\zmmr034 261 ปี 2566.xlsx",src_dir())];
    let flst = vec![format!(
        "{}/ข้อมูลหม้อแปลงยกลง ก.ย.66-ก.ย.67/zmmr034 261 ปี 2566.xlsx",
        src_dir()
    )];
    println!("{:?}", flst);

    if let Ok(xlsv) = xlsx_info(&flst).await {
        println!("xlsv: {}", xlsv.len());
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
        }
        if let Ok(ser) = bincode::serialize(&xlsv) {
            // if serialize
            let lpf = format!("{}/bizme.bin", data_dir());
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    Ok(())
}

#[allow(dead_code)]
pub fn ld_bisze() {
    if let Ok(f) = File::open(format!("{}/bizme.bin", crate::sg::imp::data_dir())) {
        let br = BufReader::new(f);
        if let Ok(shv) = bincode::deserialize_from::<BufReader<File>, Vec<XlsSheet>>(br) {
            println!("biz sheet {}", shv.len());
            for sh in shv {
                let mut cn = 0;
                println!("sh: {}", sh.data.len());
                for rw in sh.data {
                    cn += 1;
                    if cn > 5 {
                        break;
                    }
                    print!("r:{} ", rw.len());
                    for cl in rw {
                        print!(" {}", cl);
                    }
                    println!("");
                }
                /*
                let rc = sh.rcnt;
                for r in 0..sh.rcnt {
                    print!("{}:", r);
                    if r>10 { break; }
                    let rw = &sh.data[r];
                    for c in 0..sh.ccnt {
                        let vv = &sh.data[r][c];
                        print!(" {}", vv);
                    }
                    println!("");
                }
                */
            }
        }
    }
}

pub async fn pea_sub_plan() -> Result<(), Box<dyn std::error::Error>> {
    let flst = vec![format!(
        "{}\\ข้อมูลแผนงานก่อสร้างสถานีไฟฟ้า\\Report Substation Load Forecast - Report ม.ค. 2566.xlsx",
        src_dir()
    )];
    println!("{:?}", flst);

    if let Ok(xlsv) = xlsx_info(&flst).await {
        println!("xlsv: {}", xlsv.len());
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
        }
        if let Ok(ser) = bincode::serialize(&xlsv) {
            // if serialize
            let lpf = format!("{}/sub_plan.bin", data_dir());
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    Ok(())
}

pub async fn pea_load_fore() -> Result<(), Box<dyn std::error::Error>> {
    let flst = vec![format!(
        "{}\\ข้อมูลพยากรณ์โหลด\\Report Detail Substation Load Forecast_กวร. ม.ค. 66.xlsx",
        src_dir()
    )];
    println!("{:?}", flst);
    if let Ok(xlsv) = xlsx_info(&flst).await {
        println!("xlsv: {}", xlsv.len());
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
        }
        if let Ok(ser) = bincode::serialize(&xlsv) {
            // if serialize
            let lpf = format!("{}/load_fore.bin", data_dir());
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    Ok(())
}

pub async fn pea_evcs() -> Result<(), Box<dyn std::error::Error>> {
    //let flst = vec![format!("{}\\ข้อมูลสถานีชาร์จEV\\PEA VOLTA May 2024.xlsx", src_dir())];
    let flst = vec![format!(
        "{}/ข้อมูลสถานีชาร์จEV/PEA VOLTA May 2024.xlsx",
        src_dir()
    )];
    println!("{:?}", flst);
    if let Ok(xlsv) = xlsx_info(&flst).await {
        println!("xlsv: {}", xlsv.len());
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
        }
        if let Ok(ser) = bincode::serialize(&xlsv) {
            // if serialize
            let lpf = format!("{}/pea_evcs.bin", data_dir());
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    Ok(())
}

pub async fn pea_bess_plan() -> Result<(), Box<dyn std::error::Error>> {
    //let flst = vec![format!("{}\\bess-plan-2024-07-10.xlsx", src_dir())];
    let flst = vec![format!("{}/bess-plan-2024-07-10.xlsx", src_dir())];
    println!("{:?}", flst);
    if let Ok(xlsv) = xlsx_data(&flst).await {
        println!("xlsv: {}", xlsv.len());
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
        }
        if let Ok(ser) = bincode::serialize(&xlsv) {
            // if serialize
            let lpf = format!("{}/bess_plan.bin", data_dir());
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CSVFile {
    pub path: String,
    pub name: String,
    pub rcnt: usize,
    pub ccnt: usize,
    pub data: Vec<Vec<String>>,
    //arry: Vec<[String;23]>,
}

pub async fn pea_meter_read() -> Result<(), Box<dyn std::error::Error>> {
    let flst = vec![
        //format!("{}\\20240801_กรอ\\export_กรอ_bil013_202402.csv", src_dir()),
        //format!("{}\\20240801_กรอ\\export_กรอ_bil013_202405.csv", src_dir()),
        format!("202402"),
        format!("202405"),
    ];
    let mut csv_v = Vec::<CSVFile>::new();
    for fl in flst {
        //let fl = format!("{}\\20240801_กรอ\\export_กรอ_bil013_{}.csv", src_dir(), fl);
        let fl = format!("{}/20240801_กรอ/export_กรอ_bil013_{}.csv", src_dir(), fl);
        println!("start {}", &fl);
        if let Ok(mut rdr) = csv::Reader::from_path(&fl) {
            // if read file
            let path = PathBuf::from(fl.clone());
            let name = path.file_name().unwrap().to_str().unwrap();
            let name = name.to_string();
            let path = path.to_str().unwrap().to_string();
            let mut rcnt = 0;
            let mut ccnt = 0;
            let mut data = Vec::<Vec<String>>::new();
            //let mut arry = Vec::<[&String]>::new();
            for rs in rdr.records() {
                // loop all record
                let mut row = Vec::<String>::new();
                //let mut ary = [String; 23];
                if let Ok(rc) = rs {
                    // if the record exist
                    rcnt += 1;
                    ccnt = rc.len();
                    for cno in 0..ccnt {
                        let cell = if let Some(s) = rc.get(cno) {
                            String::from(s)
                        } else {
                            String::new()
                        };
                        //ary[cno] = cell;
                        row.push(cell.clone());
                    }
                    for x in &row {
                        print!(" '{}'", x);
                    }
                    println!("");
                    data.push(row);
                    //arry.push(ary);
                } // fi record
                if data.len() > 10 {
                    break;
                }
            } // loop all rec
            let csv = CSVFile {
                path,
                name,
                rcnt,
                ccnt,
                data,
            };
            csv_v.push(csv);
            print!("read '{}' r:{} c:{}\n", fl, rcnt, ccnt);
        }
    }
    println!("start saving");
    if let Ok(ser) = bincode::serialize(&csv_v) {
        // if serialize
        let lpf = format!("{}/meter_read.bin", data_dir());
        std::fs::write(lpf, ser).unwrap();
    } // end serialize
    println!("finished saving");
    Ok(())
}

pub async fn pea_re_plan() -> Result<(), Box<dyn std::error::Error>> {
    let flst = vec![format!("{}/re-plan-2024-0.xlsx", src_dir())];
    println!("{:?}", flst);

    if let Ok(xlsv) = xlsx_data(&flst).await {
        println!("xlsv: {}", xlsv.len());
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
        }
        if let Ok(ser) = bincode::serialize(&xlsv) {
            // if serialize
            let lpf = format!("{}/re_plan.bin", data_dir());
            println!("file: {}", lpf);
            std::fs::write(lpf, ser).unwrap();
        } // end serialize
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct REPlan {
    pub year: String,
    pub apid: String,
    pub proj: String,
    pub feed: String,
    pub sub: String,
    pub pwmw: String,
    pub cate: String,
    pub sbid: String,
}

pub async fn pea_sub_do() -> Result<(), Box<dyn std::error::Error>> {
    let mut subxml = Vec::<HashMap<String, String>>::new();
    if let Ok(fsbinfo) = File::open(format!("{}/sub_xml.bin", crate::sg::imp::data_dir())) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(sub) =
            bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, String>>>(rsbinfo)
        {
            subxml = sub;
        }
    }
    println!("XML: {}", subxml.len());

    let mut subxls = HashMap::<String, SubInfo>::new();
    if let Ok(fsbinfo) = File::open(format!("{}/sub_xlsx.bin", crate::sg::imp::data_dir())) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(sub) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, SubInfo>>(rsbinfo)
        {
            subxls = sub;
        }
    }
    println!("XLS: {}", subxls.len());

    let mut replan = Vec::<XlsSheet>::new();
    if let Ok(fsbinfo) = File::open(&format!("{}/re_plan.bin", crate::sg::imp::data_dir())) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(re) = bincode::deserialize_from::<BufReader<File>, Vec<XlsSheet>>(rsbinfo) {
            replan = re;
        }
    }
    let tgid = "cim:IdentifiedObject.description".to_string();
    let tgnm = "cim:IdentifiedObject.name".to_string();
    let mut thnm = HashMap::<String, String>::new();
    let mut ennm = HashMap::<String, String>::new();
    for sb in subxml {
        if let (Some(sbid), Some(sbnm)) = (sb.get(&tgid), sb.get(&tgnm)) {
            ennm.insert(sbnm.to_string(), sbid.to_string());
        }
    }
    for (_key, sbif) in &subxls {
        thnm.insert(sbif.name.to_string(), sbif.sbid.to_string());
    }

    let mut newre = Vec::<REPlan>::new();

    let repl = &replan[0];
    let mut pwsum = 0f32;
    let mut cn = 0;
    println!("replan {}", repl.data.len());
    for rw in &repl.data {
        let yr = rw[2].to_string();
        let id = if rw[5].len() > 0 {
            rw[5][0..3].to_string()
        } else {
            "?".to_string()
        };
        let sb = if rw[6].len() > 0 {
            rw[6].to_string()
        } else {
            "?".to_string()
        };
        let _sz = rw[7].to_string();
        let _nm = if let Some(sbif) = subxls.get(&id) {
            sbif.name.to_string()
        } else {
            "?".to_string()
        };
        let th = if let Some(thid) = thnm.get(&sb) {
            thid.to_string()
        } else {
            "?".to_string()
        };
        let tp = if th == "?" { "1" } else { "2" };
        let mut fd = rw[5].to_string();
        if tp == "2" {
            fd = th.to_string();
        }
        let year = yr.to_string();
        let apid = rw[3].to_string();
        let proj = rw[4].to_string();
        let feed = rw[5].to_string();
        let sub = rw[6].to_string();
        let pwmw = rw[7].to_string();
        let cate = rw[8].to_string();
        let mut sbid = if tp == "1" {
            th.to_string()
        } else {
            fd.to_string()
        };
        if sbid.len() != 3 && feed.len() == 5 {
            sbid = feed[0..3].to_string();
        }

        if sbid.len() == 3 {
            cn += 1;
            if let Ok(d) = pwmw.parse::<f32>() {
                pwsum += d;
            } else {
                println!("ERROR parsing MW");
            }
            println!("{}. Y:{} SB:{} P:{} CA:{}", cn, year, sbid, pwmw, cate);
            /*
            println!(
                "Y:{} ID:'{}' NM:'{}' TH:'{}' IX:'{}' = {} - {}.",
                yr, id, sb, nm, th, tp, fd
            );
            */
            let repln = REPlan {
                year,
                apid,
                proj,
                feed,
                sub,
                pwmw,
                cate,
                sbid,
            };
            newre.push(repln);
        } else {
            println!(
                "============= '{}' '{}' '{}' '{}' '{}' '{}'",
                rw[3], rw[4], rw[5], rw[6], rw[7], sbid
            );
            //println!("?? '{}' '{}' '{}' '{}'", rw[4], rw[5], rw[6], rw[7]);
        }
    }
    println!("{}", pwsum);
    if let Ok(ser) = bincode::serialize(&newre) {
        // if serialize
        let lpf = format!("{}/newre.bin", data_dir());
        println!("file: {}", lpf);
        std::fs::write(lpf, ser).unwrap();
    } // end serialize

    Ok(())
}

pub fn ld_replan() -> Vec<REPlan> {
    if let Ok(fsbinfo) = File::open(&format!("{}/newre.bin", crate::sg::imp::data_dir())) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(re) = bincode::deserialize_from::<BufReader<File>, Vec<REPlan>>(rsbinfo) {
            return re;
        }
    }
    Vec::<REPlan>::new()
}

pub async fn xlsx_data(flst: &Vec<String>) -> Result<Vec<XlsSheet>, Box<dyn std::error::Error>> {
    let mut xlsv = Vec::<XlsSheet>::new();
    for fl in flst {
        let pt = PathBuf::from(fl.clone());
        let mut excel: Xlsx<_> = open_workbook(fl.clone())?;
        let ff = pt.file_name().unwrap().to_str().unwrap();
        let sheets = excel.sheet_names().to_owned();
        for sh in &sheets {
            if let Ok(range) = excel.worksheet_range(sh) {
                let path = fl.to_string();
                let name = ff.to_string();
                let shnm = sh.to_string();
                let rcnt = range.get_size().0;
                let ccnt = range.get_size().1;
                let mut data = Vec::<Vec<String>>::new();
                for r in range.rows() {
                    let mut rr = Vec::<String>::new();
                    for (_i, c) in r.iter().enumerate() {
                        let s = match c {
                            Data::Empty => "".to_string(),
                            Data::String(ref s)
                            | Data::DateTimeIso(ref s)
                            | Data::DurationIso(ref s) => s.to_string(),
                            v => v.to_string(),
                        };
                        rr.push(s);
                    }
                    data.push(rr);
                }
                let xls_info = XlsSheet {
                    path,
                    name,
                    shnm,
                    rcnt,
                    ccnt,
                    data,
                };
                xlsv.push(xls_info);
            }
        }
    }
    Ok(xlsv)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragePlan {
    sbid: String,
    sbnm: String,
    pwmw: String,
    emwh: String,
    year: String,
    yrno: String,
}

pub async fn pea_bess_ana() -> Result<(), Box<dyn std::error::Error>> {
    let mut subxml = Vec::<HashMap<String, String>>::new();
    if let Ok(fsbinfo) = File::open(format!("{}/sub_xml.bin", crate::sg::imp::data_dir())) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(sub) =
            bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, String>>>(rsbinfo)
        {
            subxml = sub;
        }
    }
    println!("XML: {}", subxml.len());

    let tgid = "cim:IdentifiedObject.description".to_string();
    let tgnm = "cim:IdentifiedObject.name".to_string();
    let mut ennm = HashMap::<String, String>::new();
    for sb in subxml {
        if let (Some(sbid), Some(sbnm)) = (sb.get(&tgid), sb.get(&tgnm)) {
            ennm.insert(sbnm.to_string(), sbid.to_string());
        }
    }

    let mut bess = Vec::<XlsSheet>::new();
    if let Ok(fsbinfo) = File::open(&format!("{}/bess_plan.bin", crate::sg::imp::data_dir())) {
        let rsbinfo = BufReader::new(fsbinfo);
        if let Ok(re) = bincode::deserialize_from::<BufReader<File>, Vec<XlsSheet>>(rsbinfo) {
            bess = re;
        }
    }
    let mut newsto = Vec::<StoragePlan>::new();
    while let Some(be) = bess.pop() {
        for rw in &be.data {
            if let Some(s) = ennm.get(&rw[1]) {
                let sbid = s.to_string();
                let sbnm = rw[1].to_string();
                let pwmw = rw[2].to_string();
                let emwh = rw[3].to_string();
                let year = rw[5].to_string();
                let yrno = rw[8].to_string();
                let stor = StoragePlan {
                    sbid,
                    sbnm,
                    pwmw,
                    emwh,
                    year,
                    yrno,
                };
                println!("sto: {:?}", stor);
                newsto.push(stor);
            }
        }
    }
    Ok(())
}

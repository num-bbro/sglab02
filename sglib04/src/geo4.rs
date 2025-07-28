use crate::geo1::CnlData;
use crate::geo1::MeterBill;
use crate::geo1::NodeInfo;
use crate::geo2::CnlTrans;
use crate::geo2::SppData;
use crate::geo2::SubFeedTrans;
use crate::geo2::VoltaStation;
use crate::geo2::VsppData;
use crate::geo3::GisAoj;
use crate::geo3::GisZone;
use crate::geo3::PopuDenseSave;
use sglab02_lib::sg::gis1::ar_list;
use sglab02_lib::sg::prc4::ld_pv_sbv_m;
use sglab02_lib::sg::prc4::Proc41Item;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn p13_ana_test1() -> Result<(), Box<dyn Error>> {
    let ym = "202405";
    let sbsl = ld_pv_sbv_m();
    //println!("sbsl: {}", sbsl.len());
    let mut sbm = HashMap::<String, Proc41Item>::new();
    for pvsb in sbsl.values() {
        for sb in pvsb {
            sbm.insert(sb.sbid.to_string(), sb.clone());
        }
    }
    //println!("sb: {}", sbm.len());

    //pub fn ld_pv_sbv_m() -> HashMap::<String,Vec::<Proc41Item>> {
    for ar in ar_list() {
        println!("{ar}");
        let fsbf = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ar}_sb_fd_tr_hm.bin");
        let fctr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
        let fcmt = format!("/mnt/e/CHMBACK/pea-data/data1/p3_{ar}_cnl_mt.bin");
        let fbil = format!("/mnt/e/CHMBACK/pea-data/data1/{ym}_{ar}_bil.bin");
        let fm2b = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ym}_{ar}_m2b.bin");
        let fnds = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_nodes.bin");
        let fvol = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_volta.bin");
        let fvsp = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_vspp.bin");
        let fspp = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_spp.bin");
        let fzns = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_tr_in_zn.bin");
        let faoj = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_tr_in_aoj.bin");
        let famp = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_tr_in_amp.bin");
        let fmun = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_tr_in_mun.bin");
        let fao = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_aoj.bin");
        let fzn = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_zone.bin");
        let fam = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_am_po_de.bin");
        let fmu = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_mu_po_de.bin");
        let fsb = "/mnt/e/CHMBACK/pea-data/data1/y_sb_lp.bin".to_string();
        let ffd = "/mnt/e/CHMBACK/pea-data/data1/y_fd_lp.bin".to_string();
        if let (
            Ok(fsbf),
            Ok(fctr),
            Ok(fcmt),
            Ok(fbil),
            Ok(fm2b),
            Ok(nds),
            Ok(vol),
            Ok(vsp),
            Ok(spp),
            Ok(fzns),
            Ok(faoj),
            Ok(famp),
            Ok(fmun),
            Ok(fao),
            Ok(fzn),
            Ok(fam),
            Ok(fmu),
            Ok(fsb),
            Ok(ffd),
        ) = (
            File::open(&fsbf),
            File::open(&fctr),
            File::open(&fcmt),
            File::open(&fbil),
            File::open(fm2b),
            File::open(&fnds),
            File::open(&fvol),
            File::open(&fvsp),
            File::open(&fspp),
            File::open(&fzns),
            File::open(&faoj),
            File::open(&famp),
            File::open(&fmun),
            File::open(&fao),
            File::open(&fzn),
            File::open(&fam),
            File::open(&fmu),
            File::open(&fsb),
            File::open(&ffd),
        ) {
            let fsbf = BufReader::new(fsbf);
            let fctr = BufReader::new(fctr);
            let fcmt = BufReader::new(fcmt);
            let fbil = BufReader::new(fbil);
            let fm2b = BufReader::new(fm2b);
            let nds = BufReader::new(nds);
            let vol = BufReader::new(vol);
            let vsp = BufReader::new(vsp);
            let spp = BufReader::new(spp);
            let fzns = BufReader::new(fzns);
            let faoj = BufReader::new(faoj);
            let famp = BufReader::new(famp);
            let fmun = BufReader::new(fmun);
            let fao = BufReader::new(fao);
            let fzn = BufReader::new(fzn);
            let fam = BufReader::new(fam);
            let fmu = BufReader::new(fmu);
            let fsb = BufReader::new(fsb);
            let ffd = BufReader::new(ffd);
            if let (
                Ok(fsbf),
                Ok(ctrs),
                Ok(fcmt),
                Ok(fbil),
                Ok(fm2b),
                Ok(nds),
                Ok(vol),
                Ok(vsp),
                Ok(spp),
                Ok(zns),
                Ok(aoj),
                Ok(amp),
                Ok(mun),
                Ok(ao),
                Ok(zn),
                Ok(am),
                Ok(mu),
                Ok(sb),
                Ok(fd),
            ) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<String, SubFeedTrans>>(fsbf),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(fctr),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlData>>(fcmt),
                bincode::deserialize_from::<BufReader<File>, Vec<MeterBill>>(fbil),
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<usize>>>(fm2b),
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<VoltaStation>>(vol),
                bincode::deserialize_from::<BufReader<File>, Vec<VsppData>>(vsp),
                bincode::deserialize_from::<BufReader<File>, Vec<SppData>>(spp),
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<usize>>>(fzns),
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<usize>>>(faoj),
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<usize>>>(famp),
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<usize>>>(fmun),
                bincode::deserialize_from::<BufReader<File>, Vec<GisAoj>>(fao),
                bincode::deserialize_from::<BufReader<File>, Vec<GisZone>>(fzn),
                bincode::deserialize_from::<BufReader<File>, Vec<PopuDenseSave>>(fam),
                bincode::deserialize_from::<BufReader<File>, Vec<PopuDenseSave>>(fmu),
                bincode::deserialize_from::<
                    BufReader<File>,
                    HashMap<String, HashMap<String, Vec<f32>>>,
                >(fsb),
                bincode::deserialize_from::<
                    BufReader<File>,
                    HashMap<String, HashMap<String, Vec<f32>>>,
                >(ffd),
            ) {
                println!("READ !!!");
                println!(
                    "{ar} fsbf:{} ctr:{} cmt:{} bil:{} m2b:{}",
                    fsbf.len(),
                    ctrs.len(),
                    fcmt.len(),
                    fbil.len(),
                    fm2b.len(),
                );
                println!(
                    "   nds:{} vol:{} vsp:{} spp:{}",
                    nds.len(),
                    vol.len(),
                    vsp.len(),
                    spp.len(),
                );
                println!(
                    "   zn:{} aoj:{} amp:{} mun:{}",
                    zns.len(),
                    aoj.len(),
                    amp.len(),
                    mun.len(),
                );
                println!(
                    "   zn0:{} aoj0:{} amp0:{} mun0:{}",
                    zn.len(),
                    ao.len(),
                    am.len(),
                    mu.len(),
                );
                println!("   sb:{} fd:{} ", sb.len(), fd.len());
                let mut ctrm = HashMap::<String, CnlTrans>::new();
                for ctr in &ctrs {
                    ctrm.insert(ctr.trid.to_string(), ctr.clone());
                }
                for (sb, fdtr) in &fsbf {
                    println!("{ar} - {sb} - fd:{}", fdtr.feed.len());
                    let mut fds: Vec<String> =
                        fdtr.feed.clone().into_iter().map(|(k, _)| k).collect();
                    fds.sort();
                    for fd in &fds {
                        let trids = fdtr.feed.get(fd).unwrap();
                        //for (fd, trids) in &fdtr.feed {
                        let mut cn = 0;
                        let mut mttp = HashMap::<String, (u32, f32)>::new();
                        for ti in trids {
                            let ctr = &ctrs[*ti];
                            //if let Some(ctr) = fctr.get(id) {
                            //if let Some(ctr) = ctrm.get(id) {
                            cn += 1;
                            for mi in &ctr.mts {
                                let _mt = &fcmt[*mi];
                                let m2b = &fm2b[*mi];
                                for mi in m2b {
                                    let bil: &MeterBill = &fbil[*mi];
                                    let volt = bil.volt.to_string();
                                    if let Some((cn, en)) = mttp.get_mut(&volt) {
                                        *cn += 1;
                                        *en += bil.kwh15;
                                    } else {
                                        mttp.insert(volt, (1, 0f32));
                                    }
                                }
                                //bcn += m2b.len();
                                //println!("  mt: {:?} bil:{}", mt.mt_pea, m2b.len());
                            }
                            //}
                        }
                        if mttp.len() > 0 {
                            println!("  {fd} trx:{cn} mt:{mttp:?}");
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

//use crate::geo1::xlsx_data;
//use sglab02_lib::sg::imp::ld_replan;
use serde::Deserialize;
use serde::Serialize;
use sglab02_lib::sg::imp::xlsx_data;
use sglab02_lib::sg::prc5::sub_inf;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum PowerProdType {
    IPP,
    SPP,
    VSPP,
    ISP,
    #[default]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct REPlan {
    pub ar: String,
    pub pptp: PowerProdType,
    pub year: String,
    pub apid: String,
    pub prj_name: String,
    pub fid: Option<String>,
    pub sub_th: Option<String>,
    pub pwmw: Option<f32>,
    pub grp: Option<String>,
    pub sbid: Option<String>,
}

pub async fn p13_re_plan() -> Result<(), Box<dyn Error>> {
    let flst = vec!["/mnt/e/CHMBACK/pea-data/re-plan-2024-0.xlsx".to_string()];
    println!("{:?}", flst);
    let sbif = sub_inf();
    let mut thsub = HashMap::<String, String>::new();
    for (sb, sf) in sbif {
        thsub.insert(sf.name.to_string(), sb.to_string());
    }
    if let Ok(repls) = xlsx_data(&flst).await {
        let repl = &repls[0];
        println!("replan {}", repl.data.len());
        let mut ar_re_plan = HashMap::<String, Vec<REPlan>>::new();
        for rw in &repl.data {
            let year = rw[2].to_string();
            let pptp = if rw[5].len() >= 5 {
                PowerProdType::VSPP
            } else if !rw[6].is_empty() {
                PowerProdType::SPP
            } else {
                PowerProdType::Unknown
            };
            let apid = rw[3].to_string();
            let prj_name = rw[4].to_string();
            let fid = if !rw[5].is_empty() {
                Some(rw[5].to_string())
            } else {
                None
            };
            let mut sbth = "?".to_string();
            let sub_th = if !rw[6].is_empty() {
                sbth = rw[6].to_string();
                Some(rw[6].to_string())
            } else {
                None
            };
            let pwmw = if let Ok(v) = rw[7].parse::<f32>() {
                Some(v)
            } else {
                None
            };
            let sbid = match pptp {
                PowerProdType::SPP => thsub.get(&sbth).map(|id| id.to_string()),
                PowerProdType::VSPP => fid.clone().map(|ref id| id[0..3].to_string()),
                _ => None,
            };
            let grp = if !rw[8].is_empty() {
                Some(rw[8].to_string())
            } else {
                None
            };
            let sid = if let Some(ref id) = sbid {
                id.to_string()
            } else {
                "???".to_string()
            };
            if let Some(sf) = sbif.get(&sid) {
                let ar = sf.arid.to_string();
                let repln = REPlan {
                    ar,
                    pptp,
                    year,
                    apid,
                    sub_th,
                    prj_name,
                    fid,
                    pwmw,
                    grp,
                    sbid,
                };
                println!("{repln:?}");
                if let Some(reps) = ar_re_plan.get_mut(&repln.ar) {
                    reps.push(repln);
                } else {
                    ar_re_plan.insert(repln.ar.to_string(), vec![repln]);
                }
                //re_plan.push(repln);
            }
        }
        for (ar, mut reps) in ar_re_plan {
            reps.sort_by(|a, b| a.sbid.cmp(&b.sbid));
            let fsub = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ar}_sb_fd_tr.bin");
            println!("{ar} {}", reps.len());
            if let Ok(fsub) = File::open(&fsub) {
                let fsub = BufReader::new(fsub);
                if let Ok(sub) =
                    bincode::deserialize_from::<BufReader<File>, Vec<SubFeedTrans>>(fsub)
                {
                    println!("  sub feed trans :{}", sub.len());
                    let mut sb_rep = vec![Vec::<usize>::new(); sub.len()];
                    for (sbrep, sub) in sb_rep.iter_mut().zip(sub.iter()) {
                        for (ix, rep) in reps.iter().enumerate() {
                            if let Some(sid) = &rep.sbid {
                                if *sid == sub.sbid {
                                    sbrep.push(ix);
                                }
                            }
                        }
                        println!("   {} = {}", sub.sbid, sbrep.len());
                        for ix in sbrep {
                            println!(
                                "      {:?} - {:?} - {:?}",
                                reps[*ix].pptp, reps[*ix].pwmw, reps[*ix].sub_th
                            );
                        }
                    }
                    let fsre = format!("/mnt/e/CHMBACK/pea-data/data1/p14_{ar}_sb_in_re.bin");
                    if let Ok(ser) = bincode::serialize(&sb_rep) {
                        println!("  write to {fsre}");
                        std::fs::write(fsre, ser)?;
                    }
                    let fre = format!("/mnt/e/CHMBACK/pea-data/data1/p14_{ar}_re_plan.bin");
                    if let Ok(ser) = bincode::serialize(&reps) {
                        println!("  write to {fre}");
                        std::fs::write(fre, ser)?;
                    }
                }
            }
        }
    }
    Ok(())
}

use phf::phf_map;

pub static GPPS: phf::Map<&'static str, u32> = phf_map! {
"ระยอง" => 942205,
"กรุงเทพมหานคร" => 675979,
"ชลบุรี" => 592335,
"ฉะเชิงเทรา" => 490005,
"พระนครศรีอยุธยา" => 428870,
"ปราจีนบุรี" => 388559,
"สมุทรสาคร" => 374056,
"สระบุรี" => 344734,
"สมุทรปราการ" => 320294,
"นครปฐม" => 316636,
"ภูเก็ต" => 314921,
"จันทบุรี" => 253522,
"ปทุมธานี" => 246463,
"ลำพูน" => 236619,
"ราชบุรี" => 231516,
"ชุมพร" => 230319,
"พังงา" => 229213,
"ประจวบคีรีขันธ์" => 221151,
"นนทบุรี" => 214515,
"สุราษฎร์ธานี" => 188181,
"กระบี่" => 174058,
"สมุทรสงคราม" => 167164,
"ตราด" => 164835,
"ชัยนาท" => 157159,
"เพชรบุรี" => 156719,
"กำแพงเพชร" => 155404,
"เชียงใหม่" => 154925,
"กาญจนบุรี" => 153662,
"ลพบุรี" => 152831,
"สิงห์บุรี" => 151750,
"สงขลา" => 147790,
"นครสวรรค์" => 139184,
"นครราชสีมา" => 137864,
"อ่างทอง" => 135248,
"ขอนแก่น" => 131987,
"นครศรีธรรมราช" => 127405,
"นครนายก" => 126435,
"พิษณุโลก" => 124884,
"สุพรรณบุรี" => 124482,
"อุทัยธานี" => 123946,
"ตาก" => 121537,
"อุตรดิตถ์" => 120720,
"เลย" => 117624,
"ตรัง" => 111746,
"สตูล" => 110312,
"พะเยา" => 109275,
"ยะลา" => 108108,
"ลำปาง" => 107732,
"หนองคาย" => 107589,
"พิจิตร" => 105054,
"เชียงราย" => 102988,
"เพชรบูรณ์" => 100936,
"อุดรธานี" => 100005,
"ระนอง" => 99331,
"นครพนม" => 96731,
"สุโขทัย" => 93208,
"บุรีรัมย์" => 91636,
"แพร่" => 91324,
"ศรีสะเกษ" => 91060,
"มหาสารคาม" => 90996,
"สุรินทร์" => 89852,
"น่าน" => 89515,
"พัทลุง" => 87098,
"ชัยภูมิ" => 85951,
"อำนาจเจริญ" => 85707,
"กาฬสินธุ์" => 84785,
"บึงกาฬ" => 84021,
"ปัตตานี" => 83369,
"อุบลราชธานี" => 82895,
"สระแก้ว" => 82526,
"ร้อยเอ็ด" => 82491,
"สกลนคร" => 78895,
"ยโสธร" => 77376,
"มุกดาหาร" => 72251,
"แม่ฮ่องสอน" => 69828,
"หนองบัวลำภู" => 69008,
"นราธิวาส" => 64005,
};

use regex::Regex;
use sglab02_lib::sg::imp::xlsx_info2;
use std::fs::read_dir;

pub enum XlsxSolarType {
    Type2, // 3 key: pw,kv,co
    Type3, // 4 key: pw,kv,sb,ci
    Type4, // 4 key: pw,tr,sb,ci
    Type5, // 2 key: tr,pw
    Type6, // 2 key: tr,pw
    Type7, // 2 key: tr,pw
    Type8, // 2 key: tr,pw
    Type9, // 2 key: tr,pw
    TypeA, // 2 key: tr,pw
    TypeB, // 2 key: tr,pw
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LowVoltSolar {
    pub ar: String,
    pub fil: String,
    pub sht: String,
    pub row: usize,
    pub pow: Option<f32>,
    pub trx: Option<String>,
    pub con: Option<String>,
    pub sub: Option<String>,
    pub vol: Option<f32>,
}

use crate::geo2::AR_TH_CODE;
use sglab02_lib::sg::imp::XlsSheet;

pub fn read_low_volt_xls_3(
    x: &XlsSheet,
    r: usize,
    p: usize,
    v: usize,
    c: usize,
) -> Result<Vec<LowVoltSolar>, Box<dyn Error>> {
    let dt: &Vec<Vec<String>> = &x.data;
    let pat = x.path.to_string();
    let pre = Regex::new(r"/กวว. (.?\.[0-9])/").unwrap();
    //let pre = Regex::new(r"/กวว. (ก.1)/").unwrap();
    let ar = if let Some(cap) = pre.captures_iter(pat.as_str()).next() {
        let ath = (&cap[1]).to_string();
        if let Some(a) = AR_TH_CODE.get(&ath) {
            a.to_string()
        } else {
            return Err("OK".into());
        }
    } else {
        return Err("OK".into());
    };
    //println!("AR {ar}");
    let fil = x.name.to_string();
    let sht = x.shnm.to_string();
    //println!("sht:'{sht}'");
    //println!("fnm:'{fil}'");
    let mut lvs = Vec::<LowVoltSolar>::new();
    for ri in r + 1..dt.len() {
        let row = ri;
        let pow = if let Ok(v) = dt[ri][p].parse::<f32>() {
            Some(v)
        } else {
            None
        };
        let vol = if let Ok(v) = dt[ri][v].parse::<f32>() {
            Some(v)
        } else {
            None
        };
        let sub = if dt[ri][c].is_empty() {
            None
        } else {
            Some(dt[ri][c].to_string())
        };
        let ok = if pow.is_some() && vol.is_some() && sub.is_some() {
            true
        } else {
            false
        };
        if !ok {
            continue;
        }
        let ar = ar.to_string();
        let fil = fil.to_string();
        let sht = sht.to_string();
        let trx = None;
        let con = None;
        let lv = LowVoltSolar {
            ar,
            fil,
            sht,
            row,
            pow,
            trx,
            con,
            sub,
            vol,
        };
        lvs.push(lv);
        //println!("{pow:?} {vol:?} {sub:?}");
    }
    Ok(lvs)
}

pub fn read_low_volt_xls_4(
    x: &XlsSheet,
    r: usize,
    p: usize,
    v: usize,
    s: usize,
    c: usize,
) -> Result<Vec<LowVoltSolar>, Box<dyn Error>> {
    let dt: &Vec<Vec<String>> = &x.data;
    let pat = x.path.to_string();
    let pre = Regex::new(r"/กวว. (.?\.[0-9])/").unwrap();
    let ar = if let Some(cap) = pre.captures_iter(pat.as_str()).next() {
        let ath = (&cap[1]).to_string();
        if let Some(a) = AR_TH_CODE.get(&ath) {
            a.to_string()
        } else {
            return Err("OK".into());
        }
    } else {
        return Err("OK".into());
    };
    //println!("AR {ar}");
    let fil = x.name.to_string();
    let sht = x.shnm.to_string();
    //println!("sht:'{sht}'");
    //println!("fnm:'{fil}'");
    let mut lvs = Vec::<LowVoltSolar>::new();
    for ri in r + 1..dt.len() {
        let row = ri;
        let pow = if let Ok(v) = dt[ri][p].parse::<f32>() {
            Some(v)
        } else {
            None
        };
        let vol = if let Ok(v) = dt[ri][v].parse::<f32>() {
            Some(v)
        } else {
            None
        };
        let sub = if dt[ri][s].is_empty() {
            None
        } else {
            Some(dt[ri][s].to_string())
        };
        let con = if dt[ri][c].is_empty() {
            None
        } else {
            Some(dt[ri][c].to_string())
        };
        let ok = if pow.is_some() && vol.is_some() && sub.is_some() {
            true
        } else {
            false
        };
        if !ok {
            continue;
        }
        let ar = ar.to_string();
        let fil = fil.to_string();
        let sht = sht.to_string();
        let trx = None;
        let lv = LowVoltSolar {
            ar,
            fil,
            sht,
            row,
            pow,
            trx,
            con,
            sub,
            vol,
        };
        lvs.push(lv);
        //println!("{pow:?} {vol:?} {sub:?}");
    }
    Ok(lvs)
}

pub fn read_lv_tr_xls_4(
    x: &XlsSheet,
    r: usize,
    p: usize,
    t: usize,
    s: usize,
    c: usize,
) -> Result<Vec<LowVoltSolar>, Box<dyn Error>> {
    let dt: &Vec<Vec<String>> = &x.data;
    let pat = x.path.to_string();
    let pre = Regex::new(r"/กวว. (.?\.[0-9])/").unwrap();
    //let pre = Regex::new(r"/กวว. (ก.1)/").unwrap();
    let ar = if let Some(cap) = pre.captures_iter(pat.as_str()).next() {
        let ath = (&cap[1]).to_string();
        if let Some(a) = AR_TH_CODE.get(&ath) {
            a.to_string()
        } else {
            return Err("OK".into());
        }
    } else {
        return Err("OK".into());
    };
    //println!("AR {ar}");
    let fil = x.name.to_string();
    let sht = x.shnm.to_string();
    //println!("sht:'{sht}'");
    //println!("fnm:'{fil}'");
    let mut lvs = Vec::<LowVoltSolar>::new();
    for ri in r + 1..dt.len() {
        let row = ri;
        let pow = if let Ok(v) = dt[ri][p].parse::<f32>() {
            Some(v)
        } else {
            None
        };
        let trx = if !dt[ri][t].is_empty() {
            Some(dt[ri][t].to_string())
        } else {
            None
        };
        let sub = if !dt[ri][s].is_empty() {
            Some(dt[ri][s].to_string())
        } else {
            None
        };
        let con = if !dt[ri][c].is_empty() {
            Some(dt[ri][c].to_string())
        } else {
            None
        };
        let ok = if pow.is_some() && trx.is_some() && sub.is_some() && con.is_some() {
            true
        } else {
            false
        };
        if !ok {
            continue;
        }
        let ar = ar.to_string();
        let fil = fil.to_string();
        let sht = sht.to_string();
        let vol = None;
        let lv = LowVoltSolar {
            ar,
            fil,
            sht,
            row,
            pow,
            trx,
            con,
            sub,
            vol,
        };
        lvs.push(lv);
        //println!("{pow:?} {vol:?} {sub:?}");
    }
    Ok(lvs)
}

pub fn read_lv_tr_xls_2(
    x: &XlsSheet,
    r: usize,
    p: usize,
    t: usize,
) -> Result<Vec<LowVoltSolar>, Box<dyn Error>> {
    let dt: &Vec<Vec<String>> = &x.data;
    let pat = x.path.to_string();
    let pre = Regex::new(r"/กวว. (.?\.[0-9])/").unwrap();
    //let pre = Regex::new(r"/กวว. (ก.1)/").unwrap();
    let ar = if let Some(cap) = pre.captures_iter(pat.as_str()).next() {
        let ath = (&cap[1]).to_string();
        if let Some(a) = AR_TH_CODE.get(&ath) {
            a.to_string()
        } else {
            return Err("OK".into());
        }
    } else {
        return Err("OK".into());
    };
    //println!("AR {ar}");
    let fil = x.name.to_string();
    let sht = x.shnm.to_string();
    //println!("sht:'{sht}'");
    //println!("fnm:'{fil}'");
    let mut lvs = Vec::<LowVoltSolar>::new();
    for ri in r + 1..dt.len() {
        let row = ri;
        let pow = if let Ok(v) = dt[ri][p].parse::<f32>() {
            Some(v)
        } else {
            None
        };
        let trx = if !dt[ri][t].is_empty() {
            Some(dt[ri][t].to_string())
        } else {
            None
        };
        println!("{ar} s:{sht} tr:{trx:?}");
        let sub = None;
        let con = None;
        let ok = if pow.is_some() && trx.is_some() {
            true
        } else {
            false
        };
        if !ok {
            continue;
        }
        let ar = ar.to_string();
        let fil = fil.to_string();
        let sht = sht.to_string();
        let vol = None;
        let lv = LowVoltSolar {
            ar,
            fil,
            sht,
            row,
            pow,
            trx,
            con,
            sub,
            vol,
        };
        lvs.push(lv);
        //println!("{pow:?} {vol:?} {sub:?}");
    }
    Ok(lvs)
}

pub async fn p13_pea_lv_solar() -> Result<(), Box<dyn std::error::Error>> {
    let fd = "/mnt/e/CHMBACK/pea-data/ข้อมูลSolar แรงต่ำ 12 เขต".to_string();
    let mut vdir = vec![fd];
    let mut flst = vec![];
    while let Some(dr) = vdir.pop() {
        if let Ok(paths) = read_dir(dr) {
            for pt in paths.flatten() {
                let pt = pt.path();
                let pn = pt.display().to_string();
                if pt.is_dir() {
                    vdir.push(pn);
                } else if pn.ends_with(".xlsx") {
                    flst.push(pn);
                }
            }
        }
    }
    // type 1,
    let k_p2 = Regex::new(r"(MW)").unwrap();
    let k_v2 = Regex::new(r"(kV)").unwrap();
    let k_c2 = Regex::new(r"ที่เชื่อมโยง").unwrap();
    //let k_x = Regex::new(r"X").unwrap();
    //let k_y = Regex::new(r"Y").unwrap();
    // type 3
    let k_p3 = Regex::new(r"กำลังการผลิตติดตั้ง\s\(MW\)").unwrap();
    let k_v3 = Regex::new(r"แรงดันที่เชื่อมโยง\s\(\skV\s\)").unwrap();
    let k_s3 = Regex::new(r"สถานีไฟฟ้า").unwrap();
    let k_c3 = Regex::new(r"วงจร").unwrap();
    // type 4
    let k_p4 = Regex::new(r"กำลังผลิตติดตั้งตามสัญญา\s\(kWp\)").unwrap();
    let k_t4 = Regex::new(r"หมายเลขหม้อแปลงจำหน่ายที่เชื่อมโยง").unwrap();
    let k_s4 = Regex::new(r"สถานีไฟฟ้า").unwrap();
    let k_c4 = Regex::new(r"หม้อแปลงจำหน่ายรับไฟ\s22\skV\sจากวงจรที่").unwrap();
    // type 5
    let k_t5 = Regex::new(r"หมายเลขหม้อแปลงจำหน่ายที่เชื่อมโยง").unwrap();
    let k_p5 = Regex::new(r"ปริมาณรวม\s\(kW\)").unwrap();
    // type 6
    let k_t6 = Regex::new(r"รหัสวงจร").unwrap();
    let k_p6 = Regex::new(r"Total\sPV\sCapacity\s\(kWp\)").unwrap();
    // type 7
    let k_t7 = Regex::new(r"รหัสหม้อแปลง").unwrap();
    let k_p7 = Regex::new(r"รวมกำลังการผลิตติดตั้ง\s\([kK]Wp\)").unwrap();
    // type 8
    let k_t8 = Regex::new(r"รหัสวงจร.*").unwrap();
    let k_p8 = Regex::new(r"\(kWp?\)").unwrap();
    // type 9
    let k_p9 = Regex::new(r"\(\skW\s\)").unwrap();
    let k_t9 = Regex::new(r"หม้อแปลง\sPEA\sNO\.").unwrap();
    // type a
    let k_pa = Regex::new(r"kWp").unwrap();
    let k_ta = Regex::new(r"หม้อแปลง").unwrap();
    // type b
    let k_pb = Regex::new(r"สัญญา").unwrap();
    let k_tb = Regex::new(r"PEA No. หม้อแปลง").unwrap();

    let mut lvs = Vec::<LowVoltSolar>::new();
    println!("FILE - {}", flst.len());
    let mut cn = 0;
    let (mut cn2, mut cn3, mut cn4, mut cn5, mut cn6, mut cn7, mut cn8, mut cn9, mut cna, mut cnb) =
        (0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    if let Ok(mut xlsv) = xlsx_info2(&flst).await {
        xlsv.sort_by(|a, b| a.path.cmp(&b.path));
        println!("xlsv: {}", xlsv.len());
        for x in &xlsv {
            cn += 1;
            let (mut c_p2, mut c_v2, mut c_c2) = (None, None, None);
            let (mut c_p3, mut c_v3, mut c_s3, mut c_c3) = (None, None, None, None);
            let (mut c_p4, mut c_t4, mut c_s4, mut c_c4) = (None, None, None, None);
            let (mut c_p5, mut c_t5) = (None, None);
            let (mut c_p6, mut c_t6) = (None, None);
            let (mut c_p7, mut c_t7) = (None, None);
            let (mut c_p8, mut c_t8) = (None, None);
            let (mut c_p9, mut c_t9) = (None, None);
            let (mut c_pa, mut c_ta) = (None, None);
            let (mut c_pb, mut c_tb) = (None, None);
            for (j, rw) in x.data.iter().enumerate().take(10) {
                for (i, cv) in rw.iter().enumerate() {
                    // type1,2
                    if let Some(_cap) = k_p2.captures_iter(cv.as_str()).next() {
                        c_p2 = Some((j, i));
                    } else if let Some(_cap) = k_v2.captures_iter(cv.as_str()).next() {
                        c_v2 = Some((j, i));
                    } else if let Some(_cap) = k_c2.captures_iter(cv.as_str()).next() {
                        c_c2 = Some((j, i));
                    }
                    // type 3
                    if let Some(_cap) = k_p3.captures_iter(cv.as_str()).next() {
                        c_p3 = Some((j, i));
                        //println!("c_p3:{c_p3:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_v3.captures_iter(cv.as_str()).next() {
                        c_v3 = Some((j, i));
                        //println!("c_v3:{c_v3:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_s3.captures_iter(cv.as_str()).next() {
                        c_s3 = Some((j, i));
                        //println!("c_s3:{c_s3:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_c3.captures_iter(cv.as_str()).next() {
                        c_c3 = Some((j, i));
                        //println!("c_c3:{c_c3:?} <= [{}]", x.shnm);
                    }
                    // type 4
                    if let Some(_cap) = k_p4.captures_iter(cv.as_str()).next() {
                        c_p4 = Some((j, i));
                        //println!("c_p4:{c_p4:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_t4.captures_iter(cv.as_str()).next() {
                        c_t4 = Some((j, i));
                        //println!("c_t4:{c_t4:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_s4.captures_iter(cv.as_str()).next() {
                        c_s4 = Some((j, i));
                        //println!("c_s4:{c_s4:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_c4.captures_iter(cv.as_str()).next() {
                        c_c4 = Some((j, i));
                        //println!("c_c4:{c_c4:?} <= [{}]", x.shnm);
                    }
                    // type 5
                    if let Some(_cap) = k_p5.captures_iter(cv.as_str()).next() {
                        c_p5 = Some((j, i));
                        //println!("c_p5:{c_p5:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_t5.captures_iter(cv.as_str()).next() {
                        c_t5 = Some((j, i));
                        //println!("c_t5:{c_t5:?} <= [{}]", x.shnm);
                    }
                    // type 6
                    if let Some(_cap) = k_p6.captures_iter(cv.as_str()).next() {
                        c_p6 = Some((j, i));
                        //println!("c_p6:{c_p6:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_t6.captures_iter(cv.as_str()).next() {
                        c_t6 = Some((j, i));
                        //println!("c_t6:{c_t6:?} <= [{}]", x.shnm);
                    }
                    // type 7
                    if let Some(_cap) = k_p7.captures_iter(cv.as_str()).next() {
                        c_p7 = Some((j, i));
                        //println!("c_p7:{c_p7:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_t7.captures_iter(cv.as_str()).next() {
                        c_t7 = Some((j, i));
                        //println!("c_t7:{c_t7:?} <= [{}]", x.shnm);
                    }
                    // type 8
                    if let Some(_cap) = k_p8.captures_iter(cv.as_str()).next() {
                        c_p8 = Some((j, i));
                        //println!("c_p8:{c_p8:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_t8.captures_iter(cv.as_str()).next() {
                        c_t8 = Some((j, i));
                        //println!("c_t8:{c_t8:?} <= [{}]", x.shnm);
                    }
                    // type 9
                    if let Some(_cap) = k_p9.captures_iter(cv.as_str()).next() {
                        c_p9 = Some((j, i));
                        //println!("c_p9:{c_p9:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_t9.captures_iter(cv.as_str()).next() {
                        c_t9 = Some((j, i));
                        //println!("c_t9:{c_t9:?} <= [{}]", x.shnm);
                    }
                    // type A
                    if let Some(_cap) = k_pa.captures_iter(cv.as_str()).next() {
                        c_pa = Some((j, i));
                        //println!("c_pa:{c_pa:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_ta.captures_iter(cv.as_str()).next() {
                        c_ta = Some((j, i));
                        //println!("c_ta:{c_ta:?} <= [{}]", x.shnm);
                    }
                    // type b
                    if let Some(_cap) = k_pb.captures_iter(cv.as_str()).next() {
                        c_pb = Some((j, i));
                        //println!("c_pb:{c_pb:?} <= [{}]", x.shnm);
                    } else if let Some(_cap) = k_tb.captures_iter(cv.as_str()).next() {
                        c_tb = Some((j, i));
                        //println!("c_tb:{c_tb:?} <= [{}]", x.shnm);
                    }
                }
            }
            let mut xtp = XlsxSolarType::None;
            if let (Some(mw), Some(kv), Some(co)) = (c_p2, c_v2, c_c2) {
                if mw.0 == kv.0 && kv.0 == co.0 {
                    if let Ok(mut v) = read_low_volt_xls_3(x, mw.0, mw.1, kv.1, co.1) {
                        println!("type2 :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::Type2;
                    cn2 += 1;
                } else {
                    println!("   2 >>> {c_p2:?} {c_v2:?} {c_c2:?}");
                }
            } else if let (Some(p3), Some(v3), Some(s3), Some(c3)) = (c_p3, c_v3, c_s3, c_c3) {
                //println!("GOT TYPE 3 : {p3:?} {v3:?} {s3:?} {c3:?}");
                if p3.0 == v3.0 && v3.0 == s3.0 - 2 && s3.0 == c3.0 {
                    if let Ok(mut v) = read_low_volt_xls_4(x, s3.0, p3.1, v3.1, s3.1, c3.1) {
                        println!("type3 :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::Type3;
                    cn3 += 1;
                } else {
                    println!("   3 >>> {c_p2:?} {c_v2:?} {c_c2:?}");
                }
            } else if let (Some(p4), Some(t4), Some(s4), Some(c4)) = (c_p4, c_t4, c_s4, c_c4) {
                //println!("GOT TYPE 4 : {p4:?} {t4:?} {s4:?} {c4:?}");
                if p4.0 == t4.0 - 1 && t4.0 == s4.0 && s4.0 == c4.0 {
                    if let Ok(mut v) = read_lv_tr_xls_4(x, t4.0, p4.1, t4.1, s4.1, c4.1) {
                        println!("type4 :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::Type4;
                    cn4 += 1;
                } else {
                    println!("   4 >>> {c_p2:?} {c_v2:?} {c_c2:?}");
                }
            } else if let (Some(p5), Some(t5)) = (c_p5, c_t5) {
                //println!("GOT TYPE 5 : {p5:?} {t5:?}");
                if p5.0 == t5.0 {
                    if let Ok(mut v) = read_lv_tr_xls_2(x, t5.0, p5.1, t5.1) {
                        println!("type5 :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::Type5;
                    cn5 += 1;
                } else {
                    println!("   5 >>> {c_p2:?} {c_v2:?} {c_c2:?}");
                }
            } else if let (Some(p6), Some(t6)) = (c_p6, c_t6) {
                //println!("GOT TYPE 6 : {p6:?} {t6:?}");
                if p6.0 == t6.0 {
                    if let Ok(mut v) = read_lv_tr_xls_2(x, t6.0, p6.1, t6.1) {
                        println!("type6 :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::Type6;
                    cn6 += 1;
                } else {
                    println!("   6 >>> {c_p2:?} {c_v2:?} {c_c2:?}");
                }
            } else if let (Some(p7), Some(t7)) = (c_p7, c_t7) {
                //println!("GOT TYPE 7 : {p7:?} {t7:?}");
                if p7.0 == t7.0 {
                    if let Ok(mut v) = read_lv_tr_xls_2(x, t7.0, p7.1, t7.1) {
                        println!("type7 :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::Type7;
                    cn7 += 1;
                } else {
                    println!("   7 >>> {c_p2:?} {c_v2:?} {c_c2:?}");
                }
            } else if let (Some(p8), Some(t8)) = (c_p8, c_t8) {
                //println!("GOT TYPE 8 : {p8:?} {t8:?}");
                if p8.0 == t8.0 {
                    if let Ok(mut v) = read_lv_tr_xls_2(x, t8.0, p8.1, t8.1) {
                        println!("type8 :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::Type8;
                    cn8 += 1;
                } else {
                    println!("   8 >>> {c_p2:?} {c_v2:?} {c_c2:?}");
                }
            } else if let (Some(p9), Some(t9)) = (c_p9, c_t9) {
                //println!("GOT TYPE 9 : {p9:?} {t9:?}");
                if p9.0 == t9.0 {
                    if let Ok(mut v) = read_lv_tr_xls_2(x, t9.0, p9.1, t9.1) {
                        println!("type9 :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::Type9;
                    cn9 += 1;
                } else {
                    println!("   9 >>> {c_p2:?} {c_v2:?} {c_c2:?}");
                }
            } else if let (Some(pa), Some(ta)) = (c_pa, c_ta) {
                //println!("GOT TYPE a : {pa:?} {ta:?}");
                if pa.0 == ta.0 {
                    if let Ok(mut v) = read_lv_tr_xls_2(x, ta.0, pa.1, ta.1) {
                        println!("typeA :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::TypeA;
                    cna += 1;
                } else {
                    println!("   a >>> {c_p2:?} {c_v2:?} {c_c2:?}");
                }
            } else if let (Some(pb), Some(tb)) = (c_pb, c_tb) {
                //println!("GOT TYPE b : {pb:?} {tb:?}");
                if pb.0 == tb.0 {
                    if let Ok(mut v) = read_lv_tr_xls_2(x, tb.0, pb.1, tb.1) {
                        println!("typeB :{}", v.len());
                        lvs.append(&mut v);
                    }
                    xtp = XlsxSolarType::TypeB;
                    cnb += 1;
                } else {
                    println!("   b >>> {pb:?} {tb:?}");
                }
            }
            if let XlsxSolarType::None = xtp {
                println!("{cn} '{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
            }
        }
        println!("lv: {}", lvs.len());
        cn -= cn2 + cn3 + cn4 + cn5 + cn6 + cn7 + cn8 + cn9 + cna + cnb;
        println!("{cn} - {cn2} {cn3} {cn4} {cn5} {cn6} {cn7} {cn8} {cn9} {cna} {cnb}");
    }
    let (mut cn1, mut cn2) = (0, 0);
    let mut ar_lv_solar = HashMap::<String, Vec<LowVoltSolar>>::new();
    for lv in lvs {
        if let Some(_t) = &lv.trx {
            cn1 += 1;
        }
        if let Some(_s) = &lv.sub {
            cn2 += 1;
        }
        if let Some(lvs) = ar_lv_solar.get_mut(&lv.ar) {
            lvs.push(lv.clone());
        } else {
            ar_lv_solar.insert(lv.ar.to_string(), vec![lv.clone()]);
        }
    }
    for (ar, mut lvs) in ar_lv_solar {
        lvs.sort_by(|a, b| {
            let aa = format!(
                "{}-{}",
                a.sub.clone().unwrap_or("?".to_string()),
                a.trx.clone().unwrap_or("?".to_string())
            );
            let bb = format!(
                "{}-{}",
                b.sub.clone().unwrap_or("?".to_string()),
                b.trx.clone().unwrap_or("?".to_string())
            );
            aa.cmp(&bb)
        });
        let flv = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_lv_solar.bin");
        if let Ok(ser) = bincode::serialize(&lvs) {
            println!("{ar} write {} to {flv}", lvs.len());
            std::fs::write(flv, ser)?;
        }
    }
    println!("cn: {cn1} {cn2}");
    Ok(())
}

pub const EV_SCURVE_30: [f32; 16] = [
    0.21, 0.31, 0.46, 0.67, 0.95, 1.33, 1.8, 2.35, 2.96, 3.57, 4.13, 4.62, 5.01, 5.3, 5.52, 5.67,
];

pub const EV_SCURVE_32: [f32; 16] = [
    0.21, 0.28, 0.38, 0.50, 0.65, 0.85, 1.09, 1.39, 1.73, 2.13, 2.55, 3.00, 3.45, 3.87, 4.27, 4.61,
];

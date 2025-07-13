use crate::aoj::ld_sb_inf;
use crate::aoj::GridSubst;
use crate::prc41::ld_sb_tr0;
use crate::prc41::SubCalc;
use crate::web1::ben_amt_proj;
use crate::web1::ben_asset_value;
use crate::web1::ben_bill_accu;
use crate::web1::ben_boxline_save;
use crate::web1::ben_cash_flow;
use crate::web1::ben_dr_save;
use crate::web1::ben_emeter;
use crate::web1::ben_model_entry;
use crate::web1::ben_mt_disconn;
use crate::web1::ben_mt_read;
use crate::web1::ben_non_tech;
use crate::web1::ben_outage_labor;
use crate::web1::ben_reduce_complain;
use crate::web1::ben_sell_meter;
use crate::web1::ben_tou_read;
use crate::web1::ben_tou_sell;
use crate::web1::ben_tou_update;
use crate::web1::ben_trx;
use crate::web1::ben_unbalan;
use crate::web1::ben_work_save;
use crate::web1::tr_val;
use crate::web1::AmtProj;
use crate::web1::BenProj;
use crate::web1::SubRatioProj;
use crate::web1::EB_UNIT_PRICE;
use crate::web1::ET_UNIT_PRICE;
use crate::web1::EV_UNIT_PRICE;
use crate::web1::OP_YEAR_END;
use crate::web1::OP_YEAR_START;
use sglab02_lib::sg::prc3::ld_p3_sub_inf;
use sglab02_lib::sg::prc4::grp1;
use sglab02_lib::sg::prc4::ld_pv_sbv_m;
use sglib03::p_31::ld_sb_eb_proj;
use sglib03::p_31::ld_sb_et_proj;
use sglib03::p_31::ld_sb_ev_proj;
use sglib03::p_31::AreaRatio;
use sglib03::prc4::ld_ben_bess1;
use std::collections::HashMap;
use std::error::Error;

pub struct EnergyProfile {
    ev: Vec<Vec<AreaRatio>>,
    et: Vec<Vec<AreaRatio>>,
    eb: Vec<Vec<AreaRatio>>,
}

pub fn calc3() -> Result<(), Box<dyn Error>> {
    let sb_inf = ld_sb_inf()?;
    let sbtr = ld_sb_tr0();
    let ev = ld_sb_ev_proj().unwrap();
    let eb = ld_sb_eb_proj().unwrap();
    let et = ld_sb_et_proj().unwrap();
    let enpf = EnergyProfile { ev, et, eb };
    for (s, gs) in &sb_inf {
        let sb = s.to_string();
        if !gs.mark {
            continue;
        }
        if let Some(sbtr) = sbtr.get(&sb) {
            calc1_a(sbtr, gs, &enpf)?;
        }
    }
    Ok(())
}

pub fn calc2() -> Result<(), Box<dyn Error>> {
    let sb_inf = ld_sb_inf()?;
    let sbtr = ld_sb_tr0();
    let ev = ld_sb_ev_proj().unwrap();
    let eb = ld_sb_eb_proj().unwrap();
    let et = ld_sb_et_proj().unwrap();
    let enpf = EnergyProfile { ev, et, eb };
    for (s, gs) in &sb_inf {
        let sb = s.to_string();
        if sb != "BYA" && sb != "BJA" {
            continue;
        }
        if !gs.mark {
            continue;
        }
        if let Some(sbtr) = sbtr.get(&sb) {
            calc1_a(sbtr, gs, &enpf)?;
        }
    }
    Ok(())
}

pub fn calc1() -> Result<(), Box<dyn Error>> {
    let pv = grp1();
    let sbsl = ld_pv_sbv_m();
    let _sbif = ld_p3_sub_inf();
    let sbtr = ld_sb_tr0();
    let ev = ld_sb_ev_proj().unwrap();
    let eb = ld_sb_eb_proj().unwrap();
    let et = ld_sb_et_proj().unwrap();
    let enpf = EnergyProfile { ev, et, eb };
    let sb_inf = ld_sb_inf()?;
    for p in &pv {
        let pp = p.to_string();
        if let Some(sbv) = sbsl.get(&pp) {
            for sb in sbv {
                let sb = sb.sbid.clone();
                if sb == "BYA" || sb == "BJA" {
                    println!("=== SUB: {sb}");
                    if let (Some(sbtr), Some(gs)) = (sbtr.get(&sb), sb_inf.get(&sb)) {
                        if gs.mark {
                            continue;
                        }
                        calc1_a(sbtr, gs, &enpf)?;
                    }
                }
            } // end for subst
        } // end chosen substation
    }
    Ok(())
}

pub struct SubstReport<'a> {
    pub sbtr: &'a SubCalc,
    pub gs: &'a GridSubst,
    pub enpf: &'a EnergyProfile,
    pub ben: SubBenInfo,
    pub ben1: Rc<dyn AmtProj>,
    pub ben2: Rc<dyn AmtProj>,
    pub ben3: Rc<dyn AmtProj>,
    pub ben4: Rc<dyn AmtProj>,
    pub ben5: Rc<dyn AmtProj>,
    pub ben6: Rc<dyn AmtProj>,
    pub ben7: Rc<dyn AmtProj>,
    pub ben8: Rc<dyn AmtProj>,
    pub ben9: Rc<dyn AmtProj>,
    pub ben10: Rc<dyn AmtProj>,
    pub ben11: Rc<dyn AmtProj>,
    pub ben12: Rc<dyn AmtProj>,
    pub ben13: Rc<dyn AmtProj>,
    pub ben14: Rc<dyn AmtProj>,
    pub ben15: Rc<dyn AmtProj>,
    pub ben16: Rc<dyn AmtProj>,
    pub ben17: Rc<dyn AmtProj>,
    pub ben18: Rc<dyn AmtProj>,
    pub ben19: Rc<dyn AmtProj>,
    pub ben20: Rc<dyn AmtProj>,
    pub ben21: Rc<dyn AmtProj>,
    pub ben22: Rc<dyn AmtProj>,
    pub ben23: Rc<dyn AmtProj>,
    pub ben24: Rc<dyn AmtProj>,
    pub ben25: Rc<dyn AmtProj>,
    pub ben26: Rc<dyn AmtProj>,
    pub ben27: Rc<dyn AmtProj>,
}

use crate::prc43::BENET;
use crate::web1::docx_adj;
use crate::web1::page_h2;
use crate::web1::para_n1;
use crate::web1::para_nm;
use crate::web1::tr_tab_ben3;
use crate::web1::DOCX0_PATH;
use crate::web1::DOCX_PATH;
use crate::web1::PDF_PATH;
use docx_rs::Docx;
use sglib03::prc4::SubBenInfo;
use std::rc::Rc;

pub fn calc1_a(sbtr: &SubCalc, gs: &GridSubst, enpf: &EnergyProfile) -> Result<(), Box<dyn Error>> {
    let sb = sbtr.sb.to_string();
    let va_p = tr_val(&sbtr.p_tx_cn_m);
    let _va_c = tr_val(&sbtr.c_tx_cn_m);
    let ben = ld_ben_bess1(&sb);
    let mut emp = Vec::<(u32, f32)>::new();
    for y in OP_YEAR_START..=OP_YEAR_END {
        emp.push((y, 0f32));
    }

    let ben1 = Rc::new(sub_proj_rate(&sb, &enpf.ev, EV_UNIT_PRICE));
    let ben2 = Rc::new(sub_proj_rate(&sb, &enpf.eb, EB_UNIT_PRICE));
    let ben3 = Rc::new(sub_proj_rate(&sb, &enpf.et, ET_UNIT_PRICE));
    let ben4 = Rc::new(BenProj { proj: emp.clone() });
    let ben5 = Rc::new(ben_trx(&va_p));
    let ben6 = Rc::new(ben_unbalan(sbtr));
    let ben7 = Rc::new(ben_non_tech(sbtr, &ben));
    let ben8 = Rc::new(ben_bill_accu(sbtr, &ben));
    let ben9 = Rc::new(ben_cash_flow(sbtr, &ben));
    let ben10 = Rc::new(ben_dr_save(sbtr, &ben));
    let mut ben11 = Rc::new(BenProj { proj: emp.clone() });
    let mut ben12 = Rc::new(BenProj { proj: emp.clone() });
    let mut ben13 = Rc::new(BenProj { proj: emp.clone() });
    let mut ben14 = Rc::new(BenProj { proj: emp.clone() });
    if ben.mx_pw > 0f32
        && ben.grw < 7f32
        && ben.be_start <= 3
        && ben.trlm > 40f32
        && (gs.conf == "AIS" || gs.conf == "GIS")
    {
        let (be_sub_save, be_re_diff, be_svg_save, be_en_added) = ben_amt_proj(&ben);
        ben11 = Rc::new(be_sub_save);
        ben12 = Rc::new(be_svg_save);
        ben13 = Rc::new(be_en_added);
        ben14 = Rc::new(be_re_diff);
    }
    let ben15 = Rc::new(ben_boxline_save(sbtr, &ben));
    let ben16 = Rc::new(ben_work_save(sbtr, &ben));
    let ben17 = Rc::new(ben_sell_meter(sbtr, &ben));
    let ben18 = Rc::new(ben_emeter(sbtr, &ben));
    let ben19 = Rc::new(ben_mt_read(sbtr, &ben));
    let ben20 = Rc::new(ben_mt_disconn(sbtr, &ben));
    let ben21 = Rc::new(ben_tou_sell(sbtr, &ben));
    let ben22 = Rc::new(ben_tou_read(sbtr, &ben));
    let ben23 = Rc::new(ben_tou_update(sbtr, &ben));
    let ben24 = Rc::new(ben_outage_labor(sbtr, &ben));
    let ben25 = Rc::new(ben_reduce_complain(sbtr, &ben));
    let ben26 = Rc::new(ben_asset_value(sbtr, &ben));
    let ben27 = Rc::new(ben_model_entry(sbtr, &ben));

    let sbrep = SubstReport {
        sbtr,
        gs,
        enpf,
        ben,
        ben1,
        ben2,
        ben3,
        ben4,
        ben5,
        ben6,
        ben7,
        ben8,
        ben9,
        ben10,
        ben11,
        ben12,
        ben13,
        ben14,
        ben15,
        ben16,
        ben17,
        ben18,
        ben19,
        ben20,
        ben21,
        ben22,
        ben23,
        ben24,
        ben25,
        ben26,
        ben27,
    };

    let _ = std::fs::create_dir_all(DOCX0_PATH);
    let _ = std::fs::create_dir_all(DOCX_PATH);
    let _ = std::fs::create_dir_all(PDF_PATH);
    let fnm = format!("{DOCX0_PATH}/sub-repo-{sb}0.docx");
    let fnm2 = format!("{DOCX_PATH}/sub-repo-{sb}.docx");
    let path = std::path::Path::new(&fnm);
    let file = std::fs::File::create(path).unwrap();
    let mut docx = Docx::new();
    docx = sub_gen_repo(docx, &sbrep, enpf);
    docx.build().pack(file)?;
    docx_adj(&fnm, &fnm2);
    println!("f1:{fnm} f2:{fnm2}");

    Ok(())
}

pub struct SubstCapCost {
    pub m1p: f32,
    pub m3p: f32,
    pub trx: f32,
    pub ess: f32,
    pub plf: f32,
}

use crate::web1::ESS_COST;
use crate::web1::M1P_COST;
use crate::web1::M1P_IMP_COST;
use crate::web1::M3P_COST;
use crate::web1::M3P_IMP_COST;
use crate::web1::PLATFORM_COST;
use crate::web1::TRX_COST;
use crate::web1::TRX_IMP_COST;

pub fn cost_capex(sbtr: &SubCalc, ben: &SubBenInfo) -> SubstCapCost {
    let m1p = sbtr.mt_1_ph as f32 * (M1P_COST + M1P_IMP_COST);
    let m3p = sbtr.mt_3_ph as f32 * (M3P_COST + M3P_IMP_COST);
    let mut trx = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>() as f32;
    trx += sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>() as f32;
    let plf = (sbtr.mt_1_ph as f32 + sbtr.mt_3_ph as f32 + trx) * PLATFORM_COST;
    let mut ess = 0f32;
    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32
    //&& (gs.conf == "AIS" || gs.conf == "GIS")
    {
        //
    }
    ess *= ESS_COST;
    trx *= TRX_COST + TRX_IMP_COST;
    SubstCapCost {
        m1p: m1p as f32,
        m3p: m3p as f32,
        trx: trx as f32,
        ess: ess as f32,
        plf,
    }
}

use crate::prc43::ECONOMICS;
use crate::web1::sub_ev_docx;
use crate::web1::sub_info_p;
use crate::web1::sub_ratio_proj;

pub fn sub_gen_repo(mut docx: Docx, sbrep: &SubstReport, enpf: &EnergyProfile) -> Docx {
    let mut benr = HashMap::<u32, Rc<dyn AmtProj>>::new();
    benr.insert(1, sbrep.ben1.clone());
    benr.insert(2, sbrep.ben2.clone());
    benr.insert(3, sbrep.ben3.clone());
    benr.insert(4, sbrep.ben4.clone());
    benr.insert(5, sbrep.ben5.clone());
    benr.insert(6, sbrep.ben6.clone());
    benr.insert(7, sbrep.ben7.clone());
    benr.insert(8, sbrep.ben8.clone());
    benr.insert(9, sbrep.ben9.clone());
    benr.insert(10, sbrep.ben10.clone());
    benr.insert(11, sbrep.ben11.clone());
    benr.insert(12, sbrep.ben12.clone());
    benr.insert(13, sbrep.ben13.clone());
    benr.insert(14, sbrep.ben14.clone());
    benr.insert(15, sbrep.ben15.clone());
    benr.insert(16, sbrep.ben16.clone());
    benr.insert(17, sbrep.ben17.clone());
    benr.insert(18, sbrep.ben18.clone());
    benr.insert(19, sbrep.ben19.clone());
    benr.insert(20, sbrep.ben20.clone());
    benr.insert(21, sbrep.ben21.clone());
    benr.insert(22, sbrep.ben22.clone());
    benr.insert(23, sbrep.ben23.clone());
    benr.insert(24, sbrep.ben24.clone());
    benr.insert(25, sbrep.ben25.clone());
    benr.insert(26, sbrep.ben26.clone());
    benr.insert(27, sbrep.ben27.clone());

    let econs = HashMap::<u32, Rc<dyn AmtProj>>::new();

    let sb_ev = sub_ratio_proj(&sbrep.gs.sbif, &enpf.ev, EV_UNIT_PRICE);
    let sb_eb = sub_ratio_proj(&sbrep.gs.sbif, &enpf.eb, EB_UNIT_PRICE);
    let sb_et = sub_ratio_proj(&sbrep.gs.sbif, &enpf.et, ET_UNIT_PRICE);
    let va_p = tr_val(&sbrep.sbtr.p_tx_cn_m);
    let va_c = tr_val(&sbrep.sbtr.c_tx_cn_m);

    docx = sub_info3(sbrep, docx);

    docx = sub_info4(sbrep, docx);

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

    docx = sub_capex(sbrep, docx);

    docx = sub_opex(sbrep, docx);

    let tx = "ผลตอบแทนทางการเงินจากการดำเนินโครงการ".to_string();
    docx = docx.add_paragraph(page_h2(&tx));
    let mut ii = 0;
    for (h, _c) in &BENET {
        ii += 1;
        let hh = format!("{ii}. {h}");
        docx = docx.add_paragraph(para_n1(&hh));
    }
    let tx = "ผลตอบแทนทางเศรษฐศาสตร์จากการดำเนินโครงการ".to_string();
    docx = docx.add_paragraph(page_h2(&tx));
    let mut ii = 0;
    for (h, _c) in &ECONOMICS {
        ii += 1;
        let hh = format!("{ii}. {h}");
        docx = docx.add_paragraph(para_n1(&hh));
    }

    let mut ii = 0;
    for (h, c) in &BENET {
        ii += 1;
        let hh = format!("{ii}. {h}");
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
        if let Some(ampj) = benr.get(&ii) {
            //println!("{ii}: found");
            let tb = tr_tab_ben3(ampj);
            docx = docx.add_table(tb);
            docx = docx.add_paragraph(para_nm(""));
        }
    }

    let mut ii = 0;
    for (h, c) in &ECONOMICS {
        ii += 1;
        let hh = format!("{ii}. {h}");
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
        if let Some(ampj) = econs.get(&ii) {
            //println!("{ii}: found");
            let tb = tr_tab_ben3(ampj);
            docx = docx.add_table(tb);
            docx = docx.add_paragraph(para_nm(""));
        }
    }

    docx
}

use crate::web1::page_h1;
use crate::web1::tb2_gen;
use crate::web1::tb2_tab;
use crate::web1::DOC_TB_INDENT;
use docx_rs::Paragraph;
use docx_rs::Pic;
use docx_rs::Run;
use std::io::Read;

pub const DOC_MULTI_FACTOR: u32 = 9525;
use crate::aoj::zoom_to_meter_pixel_lat;
use crate::aoj::MP_WW;
use crate::web1::CELL_MARGIN;
use docx_rs::AlignmentType;

pub fn sub_info3(sr: &SubstReport, mut docx: Docx) -> Docx {
    let tb2 = tb2_gen(sr.sbtr);
    let tb2 = tb2_tab(&tb2);
    let tb2 = tb2.indent(DOC_TB_INDENT);
    let tt = format!("สถานีไฟฟ้าย่อย {}", sr.gs.sbif.name);
    let sbid = sr.gs.sbif.sbid.to_string();

    docx = docx.add_paragraph(page_h1(&tt));

    docx = docx.add_paragraph(para_n1(" "));

    let tx = format!("หน่วยงานการไฟฟ้าเขต   :  {}", sr.gs.sbif.area);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("รหัสหน่วยงานการไฟฟ้าเขต   :  {}", sr.gs.sbif.arid);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("ระดับแรงดันไฟฟ้า  :   {} kV", sr.gs.sbif.volt);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("ประเภทสถานีไฟฟ้า: {}", sr.gs.sbif.cate);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("สถานะการจ่ายไฟฟ้า: {}", sr.gs.sbif.state);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("โครงสร้างวงจร: {}", sr.gs.sbif.conf);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("จำนวนหม้อแปลง: {} ตัว", sr.gs.sbif.trax);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("กำลังไฟฟ้าสูงสุด: {} MVA", sr.gs.sbif.mvax);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("จำนวนสายป้อน: {}", sr.gs.sbif.feno);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("เขตจังหวัด: {}", sr.gs.sbif.prov);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("ชื่อภาษาอังกฤษสถานีไฟฟ้าย่อย :  {}", sr.gs.sbif.enam);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("ชื่อภาษาไทยสถานีไฟฟ้าย่อย :  {}", sr.gs.sbif.name);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!("รหัสสถานีไฟฟ้าย่อย {}", sr.gs.sbif.sbid);
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!(
        "พิกัดภูมิศาสตร์ : lat: {},  long: {}",
        sr.gs.latlon.0, sr.gs.latlon.1
    );
    docx = docx.add_paragraph(para_n1(&tx));

    let tx = format!(
        "พิกัดภูมิศาสตร์ GIS : EAST {:.2} km,  N: {:.2} km",
        sr.gs.utm.0 / 1000f32,
        sr.gs.utm.1 / 1000f32
    );
    docx = docx.add_paragraph(para_n1(&tx));
    let cnf = match sr.gs.conf.as_str() {
        "AIS" => "Air Insulated Switchgear",
        "GIS" => "Gas Insulated Switchgear",
        _ => "N/A",
    };
    let tt = format!("ประเภทของสถานีไฟฟ้า  :  {}", cnf);
    docx = docx.add_paragraph(para_n1(&tt));

    let pic1 = format!("../sgdata/sub_img3/{}.jpeg", sbid);
    //println!("pic: {pic1}");
    if let Ok(mut img) = std::fs::File::open(pic1) {
        let mut buf = Vec::new();
        let _ = img.read_to_end(&mut buf).unwrap();
        let (w, h) = (500, 350);
        let pic = Pic::new(&buf).size(w * DOC_MULTI_FACTOR, h * DOC_MULTI_FACTOR);
        let ppic = Paragraph::new().add_run(Run::new().add_text(" ").add_image(pic.clone()));
        let ppic = ppic.align(AlignmentType::Center);
        let ppic = ppic.indent(Some(CELL_MARGIN), None, Some(CELL_MARGIN), None);
        docx = docx.add_paragraph(ppic);

        let pa1 = Paragraph::new().add_run(Run::new().add_text(" "));
        docx = docx.add_paragraph(pa1);
    }

    let tt = format!("ภาพถ่ายดาวเทียม google map ที่ค่า ซูม :  {}", 20);
    docx = docx.add_paragraph(para_n1(&tt));

    let dd = zoom_to_meter_pixel_lat(20, sr.gs.latlon.0) * MP_WW;
    let tt = format!("ภาพถ่ายดาวเทียม google map ระยะทางแนวนอน :  {:.2} m", dd);
    docx = docx.add_paragraph(para_n1(&tt));

    let tt = format!(
        "ลิงค์แผนที่ : https://maps.google.com?q={:4},{:.4}&zoom={}",
        sr.gs.latlon.0, sr.gs.latlon.1, 20
    );
    docx = docx.add_paragraph(para_n1(&tt));

    docx = docx.add_paragraph(para_nm(""));
    docx = docx.add_paragraph(para_nm(""));

    docx = docx.add_paragraph(para_n1("ข้อมูลมิเตอร์และการใช้ไฟฟา"));
    docx = docx.add_table(tb2);
    docx = docx.add_paragraph(para_nm(""));

    let pic1 = format!("../sgdata/sub_img7/{}.jpeg", sbid);
    //println!("pic: {pic1}");
    if let Ok(mut img) = std::fs::File::open(pic1) {
        let mut buf = Vec::new();
        let _ = img.read_to_end(&mut buf).unwrap();
        let (w, h) = (500, 350);
        let pic = Pic::new(&buf).size(w * DOC_MULTI_FACTOR, h * DOC_MULTI_FACTOR);
        let ppic = Paragraph::new().add_run(Run::new().add_text(" ").add_image(pic.clone()));
        let ppic = ppic.align(AlignmentType::Center);
        let ppic = ppic.indent(Some(CELL_MARGIN), None, Some(CELL_MARGIN), None);
        docx = docx.add_paragraph(ppic);

        let pa1 = Paragraph::new().add_run(Run::new().add_text(" "));
        docx = docx.add_paragraph(pa1);
    }
    let tt = format!("ภาพถ่ายดาวเทียม google map ที่ค่า ซูม :  {}", sr.gs.tr_zoom);
    docx = docx.add_paragraph(para_n1(&tt));

    let dd = zoom_to_meter_pixel_lat(sr.gs.tr_zoom, sr.gs.latlon.0) * MP_WW;
    let tt = format!("ครอบคลุมระยะทาง :  {:.3} km", dd / 1000f32);
    docx = docx.add_paragraph(para_n1(&tt));

    let tt = format!(
        "ลิงค์แผนที่ : https://maps.google.com?q={:4},{:.4}&zoom={}'",
        sr.gs.latlon.0, sr.gs.latlon.1, sr.gs.tr_zoom
    );
    docx = docx.add_paragraph(para_n1(&tt));

    docx = docx.add_paragraph(para_nm(""));
    docx = docx.add_paragraph(para_nm(""));

    let tt = format!("หม้อแปลงจำหน่ายกระจายครอบคลุมพื้นที่ {} การไฟฟ้า", sr.gs.brns.len());
    docx = docx.add_paragraph(para_n1(&tt));

    let keys = sr.gs.brns.keys().cloned().collect::<Vec<String>>();
    for (i, k) in keys.iter().enumerate() {
        let br = sr.gs.brns.get(k).unwrap();
        let tt = format!(
            "{}. {} ({}) หม้อแปลง: {} ตัว",
            i + 1,
            br.name,
            k,
            br.trxs.len()
        );
        docx = docx.add_paragraph(para_n1(&tt));
    }

    let pic1 = format!("../sgdata/sub_img4/{}.jpeg", sbid);
    //println!("pic: {pic1}");
    if let Ok(mut img) = std::fs::File::open(pic1) {
        let mut buf = Vec::new();
        let _ = img.read_to_end(&mut buf).unwrap();
        let (w, h) = (500, 350);
        let pic = Pic::new(&buf).size(w * DOC_MULTI_FACTOR, h * DOC_MULTI_FACTOR);
        let ppic = Paragraph::new().add_run(Run::new().add_text(" ").add_image(pic.clone()));
        let ppic = ppic.align(AlignmentType::Center);
        let ppic = ppic.indent(Some(CELL_MARGIN), None, Some(CELL_MARGIN), None);
        docx = docx.add_paragraph(ppic);

        let pa1 = Paragraph::new().add_run(Run::new().add_text(" "));
        docx = docx.add_paragraph(pa1);
    }
    docx = docx.add_paragraph(para_nm(""));
    docx = docx.add_paragraph(para_nm(""));

    use crate::aoj::zone_code;
    let zcm = zone_code();

    let tt = format!("หม้อแปลงจำหน่ายกระจายครอบคลุมพื้นที่ {} ประเภท", sr.gs.zons.len());
    docx = docx.add_paragraph(para_n1(&tt));

    let keys = sr.gs.zons.keys().cloned().collect::<Vec<String>>();
    for (i, k) in keys.iter().enumerate() {
        let zn = sr.gs.zons.get(k).unwrap();
        let ztp = if let Some(zn) = zcm.get(zn.code.as_str()) {
            zn
        } else {
            "?"
        };
        let tt = format!(
            "{}. {} ({}:{}) หม้อแปลง: {} ตัว",
            i + 1,
            zn.name,
            zn.code,
            ztp,
            zn.trxs.len()
        );
        docx = docx.add_paragraph(para_n1(&tt));
    }

    let pic1 = format!("../sgdata/sub_img6/{}.jpeg", sbid);
    //println!("pic: {pic1}");
    if let Ok(mut img) = std::fs::File::open(pic1) {
        let mut buf = Vec::new();
        let _ = img.read_to_end(&mut buf).unwrap();
        let (w, h) = (500, 350);
        let pic = Pic::new(&buf).size(w * DOC_MULTI_FACTOR, h * DOC_MULTI_FACTOR);
        let ppic = Paragraph::new().add_run(Run::new().add_text(" ").add_image(pic.clone()));
        let ppic = ppic.align(AlignmentType::Center);
        let ppic = ppic.indent(Some(CELL_MARGIN), None, Some(CELL_MARGIN), None);
        docx = docx.add_paragraph(ppic);

        let pa1 = Paragraph::new().add_run(Run::new().add_text(" "));
        docx = docx.add_paragraph(pa1);
    }
    docx = docx.add_paragraph(para_nm(""));
    docx = docx.add_paragraph(para_nm(""));

    docx
}

use sglib03::drw::sb_dr5::SubDraw5;
use sglib03::prc2::SubGraphDraw5;
use sglib03::prc2::LP_PNG_DIR;

pub fn sub_draw_load_prof(sr: &SubstReport, year: &str) -> Result<String, Box<dyn Error>> {
    let sbid = sr.gs.sbid.to_string();

    let fdir = format!("{}/{}/{}_dr5", LP_PNG_DIR, year, sbid);
    let _ = std::fs::create_dir_all(&fdir);
    let fnm = format!("{}/{}.png", fdir, sbid);

    let sbbe: SubBenInfo = ld_ben_bess1(&sbid);
    if sbbe.sub != sbid {
        return Err("ERROR 1".into());
    }

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
        return Err("ERROR 2 : NaN data".into());
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
    sld.draw_prof()?;
    Ok(fnm)
}

use crate::web1::para_h2;

pub fn sub_capex(sr: &SubstReport, mut docx: Docx) -> Docx {
    let sbtr = sr.sbtr;
    let ben = &sr.ben;
    let capex = cost_capex(sbtr, &ben);

    let mut cn = 0;
    let tx = format!("ค่าใช้จ่ายในการลงทุนโครงการ CAPEX");
    docx = docx.add_paragraph(page_h2(&tx));

    cn += 1;
    let tx = format!(
        "{cn}. ค่าใช้จ่ายเกี่ยวกับการติดตั้ง มิเตอร์ 1 เฟส:  {:.2} ล้านบาท",
        capex.m1p / 1_000_000_f32
    );
    docx = docx.add_paragraph(para_n1(&tx));

    cn += 1;
    let tx = format!(
        "{cn}. ค่าใช้จ่ายเกี่ยวกับการติดตั้ง มิเตอร์ 3 เฟส:  {:.2} ล้านบาท",
        capex.m3p / 1_000_000_f32
    );
    docx = docx.add_paragraph(para_n1(&tx));

    cn += 1;
    let tx = format!(
        "{cn}. ค่าใช้จ่ายเกี่ยวกับการจัดหาและติดตั้งหม้อแปลง:  {:.2} ล้านบาท",
        capex.trx / 1_000_000_f32
    );
    docx = docx.add_paragraph(para_n1(&tx));

    if capex.ess > 0f32 {
        cn += 1;
        let tx = format!(
            "{cn}. ค่าใช้จ่ายเกี่ยวกับการติดตั้งระบบกับเก็บพลังงาน:  {:.2} ล้านบาท",
            capex.ess / 1_000_000_f32
        );
        docx = docx.add_paragraph(para_n1(&tx));
    }

    cn += 1;
    let tx = format!(
        "{cn}. ค่าใช้จ่ายเกี่ยวกับการติดตั้งแพลทฟอร์ม:  {:.2} ล้านบาท",
        capex.m1p / 1_000_000f32
    );
    docx = docx.add_paragraph(para_n1(&tx));

    docx
}

pub fn sub_opex(sr: &SubstReport, mut docx: Docx) -> Docx {
    let sbtr = sr.sbtr;
    let ben = &sr.ben;
    let capex = cost_capex(sbtr, &ben);

    let mut cn = 0;
    let tx = format!("ค่าใช้จ่ายในการดำเนินการโครงการ OPEX");
    docx = docx.add_paragraph(page_h2(&tx));

    cn += 1;
    let ben: Rc<dyn AmtProj> = Rc::new(opx_m1p(sr));
    let tb = tr_tab_ben3(&ben);
    let h = "ค่าใช้จ่ายในการปฏิบัติการมิเตอร์ 1 เฟส";
    let hh = format!("{cn}.{h}");
    docx = docx.add_paragraph(para_h2(&hh));
    docx = docx.add_paragraph(para_nm(" "));
    docx = docx.add_table(tb);
    docx = docx.add_paragraph(para_nm(""));

    cn += 1;
    let ben: Rc<dyn AmtProj> = Rc::new(opx_m3p(sr));
    let tb = tr_tab_ben3(&ben);
    let h = "ค่าใช้จ่ายในการปฏิบัติการมิเตอร์ 3 เฟส";
    let hh = format!("{cn}.{h}");
    docx = docx.add_paragraph(para_h2(&hh));
    docx = docx.add_paragraph(para_nm(" "));
    docx = docx.add_table(tb);
    docx = docx.add_paragraph(para_nm(""));

    cn += 1;
    let ben: Rc<dyn AmtProj> = Rc::new(opx_trx(sr));
    let tb = tr_tab_ben3(&ben);
    let h = "ค่าใช้จ่ายในการปฏิบัติการหม้อแปลงจำหน่าย";
    let hh = format!("{cn}.{h}");
    docx = docx.add_paragraph(para_h2(&hh));
    docx = docx.add_paragraph(para_nm(" "));
    docx = docx.add_table(tb);
    docx = docx.add_paragraph(para_nm(""));

    if capex.ess > 0f32 {
        cn += 1;
        let ben: Rc<dyn AmtProj> = Rc::new(opx_ess(sr));
        let tb = tr_tab_ben3(&ben);
        let h = "ค่าใช้จ่ายในการปฏิบัติการระบบกักเก็บพลังงาน";
        let hh = format!("{cn}.{h}");
        docx = docx.add_paragraph(para_h2(&hh));
        docx = docx.add_paragraph(para_nm(" "));
        docx = docx.add_table(tb);
        docx = docx.add_paragraph(para_nm(""));
    }

    cn += 1;
    let ben: Rc<dyn AmtProj> = Rc::new(opx_comm(sr));
    let tb = tr_tab_ben3(&ben);
    let h = "ค่าใช้จ่ายในการสื่อสาร";
    let hh = format!("{cn}.{h}");
    docx = docx.add_paragraph(para_h2(&hh));
    docx = docx.add_paragraph(para_nm(" "));
    docx = docx.add_table(tb);
    docx = docx.add_paragraph(para_nm(""));

    docx
}

use num_traits::Pow;

use crate::web1::COMM_COST;
use crate::web1::ESS_OP_COST;
use crate::web1::M1P_OP_COST;
use crate::web1::M3P_OP_COST;
use crate::web1::TRX_OP_COST;

pub fn opx_ess(_sr: &SubstReport) -> BenProj {
    //let sbtr = sr.sbtr;
    let ex = ESS_OP_COST as f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + DISCON_COST_UP as f64, y as f64);
        proj.push((y + 2028, be as f32));
    }
    BenProj { proj }
}

pub fn opx_m1p(sr: &SubstReport) -> BenProj {
    let sbtr = sr.sbtr;
    let ex = sbtr.mt_1_ph as f64 * M1P_OP_COST as f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + DISCON_COST_UP as f64, y as f64);
        proj.push((y + 2028, be as f32));
    }
    BenProj { proj }
}

pub fn opx_m3p(sr: &SubstReport) -> BenProj {
    let sbtr = sr.sbtr;
    let ex = sbtr.mt_3_ph as f64 * M3P_OP_COST as f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + DISCON_COST_UP as f64, y as f64);
        proj.push((y + 2028, be as f32));
    }
    BenProj { proj }
}

pub fn opx_trx(sr: &SubstReport) -> BenProj {
    let sbtr = sr.sbtr;
    let mut trx = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>() as f32;
    trx += sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>() as f32;
    let ex = trx as f64 * TRX_OP_COST as f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + DISCON_COST_UP as f64, y as f64);
        proj.push((y + 2028, be as f32));
    }
    BenProj { proj }
}

use crate::web1::DISCON_COST_UP;

pub fn opx_comm(sr: &SubstReport) -> BenProj {
    let sbtr = sr.sbtr;
    let ex = (sbtr.mt_1_ph + sbtr.mt_3_ph) as f64 * COMM_COST as f64;
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + DISCON_COST_UP as f64, y as f64);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub fn sub_info4(sr: &SubstReport, mut docx: Docx) -> Docx {
    if let Ok(fnm) = sub_draw_load_prof(sr, "2024") {
        //println!("fnm {fnm}");
        if let Ok(mut img) = std::fs::File::open(fnm) {
            docx = docx.add_paragraph(para_n1(" "));

            let tx = format!("พฤติกรรมโหลดโปรไฟล์");
            docx = docx.add_paragraph(page_h2(&tx));

            let mut buf = Vec::new();
            let _ = img.read_to_end(&mut buf).unwrap();
            let (w, h) = (500, 350);
            let pic = Pic::new(&buf).size(w * DOC_MULTI_FACTOR, h * DOC_MULTI_FACTOR);
            let ppic = Paragraph::new().add_run(Run::new().add_text(" ").add_image(pic.clone()));
            let ppic = ppic.align(AlignmentType::Center);
            let ppic = ppic.indent(Some(CELL_MARGIN), None, Some(CELL_MARGIN), None);
            docx = docx.add_paragraph(ppic);

            let pa1 = Paragraph::new().add_run(Run::new().add_text(" "));
            docx = docx.add_paragraph(pa1);
        }
    }

    let tx = format!("การคำนวณพฤติกรรมโหลดโปรไฟล์");
    docx = docx.add_paragraph(page_h2(&tx));

    let mut cn = 0;
    for y in 2025..=2039 {
        let yr = format!("{}", y);
        if let Ok(fnm) = sub_draw_load_prof(sr, &yr) {
            //println!("fnm {fnm}");
            if let Ok(mut img) = std::fs::File::open(fnm) {
                docx = docx.add_paragraph(para_n1(" "));

                cn += 1;
                let tx = format!("{cn}. ประมาณการโหลดโปรไฟล์ ปี {yr}");
                docx = docx.add_paragraph(para_h2(&tx));

                let mut buf = Vec::new();
                let _ = img.read_to_end(&mut buf).unwrap();
                let (w, h) = (500, 350);
                let pic = Pic::new(&buf).size(w * DOC_MULTI_FACTOR, h * DOC_MULTI_FACTOR);
                let ppic =
                    Paragraph::new().add_run(Run::new().add_text(" ").add_image(pic.clone()));
                let ppic = ppic.align(AlignmentType::Center);
                let ppic = ppic.indent(Some(CELL_MARGIN), None, Some(CELL_MARGIN), None);
                docx = docx.add_paragraph(ppic);

                let pa1 = Paragraph::new().add_run(Run::new().add_text(" "));
                docx = docx.add_paragraph(pa1);
            }
        }
    }
    docx
}

pub fn sub_proj_rate(sb: &str, sb_ev: &Vec<Vec<AreaRatio>>, up: f32) -> SubRatioProj {
    let mut proj = Vec::<(u32, u32, f32, f32)>::new();
    for v in sb_ev {
        let v1 = &v[0];
        if v1.sb == sb {
            let v2 = &v[1..];
            for ar in v2 {
                proj.push((ar.yr, ar.no as u32, ar.mwh, ar.mwh * up * 1000f32));
            }
        }
    }
    SubRatioProj { proj }
}

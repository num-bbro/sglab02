use crate::geo1::find_node;
use crate::geo1::is_area;
use crate::geo1::n1d_2_utm;
use crate::geo1::GridNode;
use crate::geo1::NodeInfo;
use crate::geo1::NodeType;
use crate::geo2::SppData;
use crate::geo2::SubFeedTrans;
use crate::geo2::VoltaStation;
use crate::geo2::VsppData;
use phf::phf_map;
use sglab02_lib::sg::gis1::ar_list;
use sglab02_lib::sg::prc1::SubstInfo;
use sglab02_lib::sg::prc5::sub_inf;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub static SB_TH_ADJ1: phf::Map<&'static str, &'static str> = phf_map! {
    "แรงสูงเทิง - จุน" => "เทิง",
    "บริษัท สุริยาพัฒน์ จำกัด" => "อุตรดิตถ์ 1",
    "บริษัท แสงดี คลีน เพาเวอร์ จำกัด" => "พิจิตร",
    "ลำปาง 3 (ชั่วคราว) - แจ้ห่ม" => "ลำปาง 3 (ช)",
    "พิษณุโลก 4 / F2" => "พิษณุโลก 4",
    "ศรีสัชนาลัย (Add bay)" => "ศรีสัชนาลัย",
    "คลองขลุง - กำแพงเพชร 2" => "คลองขลุง",
    "ศรีสัชนาลัย - บ.แสงพัฒน์พลังงาน" => "ศรีสัชนาลัย",
    "ตะพานหิน - บริษัท ซันเรย์ รีนิวอเบิล เอ็นเนอร์จี จำกัด" => "ตะพานหิน",
    "มโนรมย์ / F1" => "มโนรมย์",
    "แรงสูงหล่มสัก - เพชรบูรณ์ 2 (ชั่วคราว)" => "เพชรบูรณ์ 2 (ช)",
    "แรงสูงท่าตะโก (เพิ่ม Bay)" => "ท่าตะโก",
    "หันคา -  บริษัท ตะวันแดง 1999 จำกัด" => "หันคา",
    "บึงสามพัน (ลานไก) - วิเชียรบุรี" => "บึงสามพัน",
    "ชนแดน - หนองไผ่" => "หนองไผ่",
    "แรงสูงท่าตะโก - หนองบัว" => "ท่าตะโก",
    "อ่าวไผ่ 2 - บึง" => "อ่าวไผ่ 2",
    "อู่ทอง - ดอนเจดีย์" => "อู่ทอง 1",
    "อ่างทอง 2 - บางปลาม้า" => "อ่างทอง 2",
    "บ่อพลอย - อู่ทอง" => "บ่อพลอย 1",
    "ระยะ 1 : แรงสูงเดิมบางนางบวช - ด่านช้าง" => "เดิมบางนางบวช",
    "ระยะ 2 : เดิมบางนางบวช(ลานไก) - ด่านช้าง" => "ด่านช้าง",
    "เลาขวัญ - ดอนเจดีย์" => "เลาขวัญ",
    "ดอนเจดีย์ (เพิ่ม Bay)" => "ดอนเจดีย์",
    "ด่านมะขามเตี้ย (เพิ่ม Bay)" => "ด่านมะขามเตี้ย",
    "ระยะ 1 : แรงสูงอุดรธานี 1 - บ้านผือ" => "อุดรธานี 1",
    "ระยะ 2 : อุดรธานี 5 - บ้านผือ" => "อุดรธานี 5 (ช)",
    "อุดรธานี 3 (เพิ่ม Bay)" => "อุดรธานี 3",
    "วานรนิวาส - เซกา" => "วานรนิวาส",
    "โซ่พิสัย (เพิ่ม Bay)" => "โซ่พิสัย",
    "โซ่พิสัย - โพนพิสัย" => "โพนพิสัย",
    "หนองคาย 3 (เพิ่ม Bay)" => "หนองคาย 3",
    "สว่างแดนดิน - บ้านดุง 2" => "บ้านดุง 2",
    "แรงสูงประโคนชัย - นางรอง" => "นางรอง",
    "บุรีรัมย์ 2 (เพิ่ม Bay)" => "บุรีรัมย์ 2",
    "สำโรงทาบ (เพิ่ม Bay)" => "สำโรงทาบ",
    "แรงสูงสุรินทร์ 2 (เพิ่ม Bay)" => "สุรินทร์ 2",
    "เพชรบุรี 2 (เพิ่ม Bay)" => "เพชรบุรี 2",
    "แรงสูงเพชรบุรี - VSPP บริษัท เอ็นเนอร์จี เซิฟ จำกัด" => "เพชรบุรี 1",
    "แรงสูงราชบุรี 2 (เพิ่ม Bay)" => "ราชบุรี 2",
    "ราชบุรี 3 - จอมบึง" => "ราชบุรี 3",
    "จอมบึง - สวนผึ้ง" => "จอมบึง",
    "ทุ่งใหญ่ (Add bay)" => "ทุ่งใหญ่",
    "แรงสูงสตูล - ละงู" => "สตูล",
    "ปัตตานี 2 (เพิ่ม Bay)" => "ปัตตานี 2",
    "นาทวี (เพิ่ม Bay)" => "นาทวี",
    "แรงสูงนราธิวาส - บริษัท กันกุล วัน เอ็นเนอร์ยี่ 2 จำกัด (REP-EG-SF-00346.00)" => "นราธิวาส",
    "คลองแงะ (ลานไก) - สะเดา 2 (ชั่วคราว)" => "คลองแงะ",
};

pub static SB_TH_ADJ2: phf::Map<&'static str, &'static str> = phf_map! {
    "188 ม.17 ต.วะตะแบก อ.เทพสถิต จ.ชัยภูมิ 36230" => "ชัยภูมิ 1",
    "อ.ระโนด จ.สงขลา" => "สงขลา 1",
    "อ.หัวไทร จ.นครศรีธรรมราช" => "นครศรีธรรมราช 1",
    "อ.ปากพนัง จ.นครศรีธรรมราช" => "นครศรีธรรมราช 1",
    "333 ม.7 ต.หนองน้ำใส อ.สีคิ้ว จ.นครราชสีมา 30140" => "นครราชสีมา 1",
    "209 บ.เทพภูทอง 18 ต.วะตะแบก อ.เทพสถิต จ.ชัยภูมิ 36230" => "ชัยภูมิ 1",
    "207 ม.6 ต.หนองแวง อ.เทพารักษ์ จ.นครราชสีมา 30210" => "นครราชสีมา 1",
    "122 1 ต.กฤษณา อ.สีคิ้ว จ.นครราชสีมา 30140" => "นครราชสีมา 1",
    "ต.นายางกลัก อ.เทพสถิต จ.ชัยภูมิ 36230" => "ชัยภูมิ 1",
    "ต.ห้วยยายจิ๋ว อ.เทพสถิต จ.ชัยภูมิ 36230" => "ชัยภูมิ 1",
    "ต.วะตะแบก อ.เทพสถิต จ.ชัยภูมิ 36230" => "ชัยภูมิ 1",
    "ต.โคกเพชรพัฒนา อ.บำเหน็จณรงค์ จ.ชัยภูมิ 36160" => "ชัยภูมิ 1",
    "เลขที่ 333 หมู่ 14 ต.หนองโพ อ.ตาคลี จ.นครสวรรค์" => "นครสวรรค์ 1",
    "ต.ปูโย๊ะ อ.สุไหงโก-ลก จ.นราธิวาส" => "นราธิวาส",
    "ต.โคกเคียน อ.เมืองนราธิวาส จ.นราธิวาส" => "นราธิวาส",
    "ต.นาข่า และ ต.บ้านขาว อ.เมืองอุดรธานี จ.อุดรธานี" => "อุดรธานี 5",
    "แรงสูงสุไหงโกลก" => "นราธิวาส",
    "ต.บ่อกรุ อ.เดิมบางนางบวช จ.สุพรรณบุรี" => "เดิมบางนางบวช",
    "ต.หนองกระทุ่ม และ ต.บ่อกรุ อ.เดิมบางนางบวช จ.สุพรรณบุรี" => "เดิมบางนางบวช",
};

pub static SB_TH_VSPP: phf::Map<&'static str, &'static str> = phf_map! {
    "แรงต่ำ ลำพูน 2" => "ลำพูน 2",
    "แรงต่ำ เถิน" => "เถิน",
    "แม่แตง" => "แม่แตง 1 (ช)",
    "ลำปาง1" => "ลำปาง 1",
    "อุตรดิตถ์ 2 (ชั่วคราว)" => "อุตรดิตถ์ 2 (ช)",
    "พิษณุโลก 2" => "พิษณุโลก 1",
    "วชิรบารมี (ถาวร)" => "วชิรบารมี",
    "เขื่อนภูมิพล" => "เขื่อนภูมิพล (ช)",
    "โพธิ์ไทรงาม" => "โพธิ์ไทรงาม (ช)",
    "พรหมพิราม (ชั่วคราว)" => "พรหมพิราม",
    "กำแพงเพชร" => "กำแพงเพชร 1",
    "พัฒนานิคม" => "พัฒนานิคม 1",
    "ชนแดน" => "เพชรบูรณ์ 1",
    "นครสวรรค์ 4 (ชั่วคราว)" => "นครสวรรค์ 3",
    "เพชรบูรณ์" => "เพชรบูรณ์ 1",
    "ลพบุรี 3 (ชั่วคราว)" => "ลพบุรี 3 (ช)",
    "ชัยบาดาล2" => "ชัยบาดาล 2",
    "นโนรมย์" => "ชัยนาท",
    "ศรีมหาโพธิ์" => "ปราจีนบุรี 1",
    "หนองแค" => "หนองแค 1",
    "บ้านใหม่" => "บ้านใหม่ 1",
    "ลาดหลุมแก้ว (ชั่วคราว/F2)" => "ลาดหลุมแก้ว",
    "สระบุรี 6 (ถาวร)" => "สระบุรี 6",

    "บ่อทอง (ชั่วคราว)" => "บ่อทอง",
    "โป่งน้ำร้อน" => "โป่งน้ำร้อน 1",
    "หนองใหญ่" => "หนองใหญ่ 1",
    "บ้านบึง 3 (ชั่วคราว)" => "บ้านบึง 3 (ช)",
    "อีสเทิร์นซีบอร์ด" => "ปลวกแดง 1",
    "เหมราช" => "เหมราช 1",
    "พัทยาเหนือ" => "พัทยาเหนือ 1",
    "บางแสน" => "บางแสน 1",
    "คลองขวาง" => "คลองขวาง 1",
    "จอมเทียน" => "จอมเทียน 1",
    "เพ" => "บ้านเพ",
    "บ้านค่าย" => "บ้านค่าย 1",
    "ปลวกแดง" => "ปลวกแดง 1",
    "ระยอง" => "ระยอง 1",
    "ศรีราชา" => "ศรีราชา 1",
    "บ้านบึง 3" => "บ้านบึง 3",
    "ฉะเชิงเทรา" => "ฉะเชิงเทรา 1",
    "แกลง" => "แกลง 1",
    "พัทยาใต้" => "พัทยาใต้ 1",
    "จันทบุรี" => "จันทบุรี 1",
    "แหลมฉบัง" => "แหลมฉบัง 1",
    "ชัยเขต" => "สนามชัยเขต",
    "อู่ทอง" => "อู่ทอง 1",
    "สองพี่น้อง" => "สองพี่น้อง 1",
    "บางเลน" => "บางเลน 1",
    "ท่ามะกา" => "ท่ามะกา 1",
    "ท่ามะกา 2 (ชั่วคราว)" => "ท่ามะกา 2 (ช)",
    "ท่าม่วง" => "ท่าม่วง 1",
    "เลาขวัญ (ชั่วคราว)" => "เลาขวัญ",
    "บ่อพลอย" => "บ่อพลอย 1",
    "ท่าม่วง 2 (ชั่วคราว)" => "ท่าม่วง 2 (ช)",
    "นครพนม" => "นครพนม 1",
    "บึงกาฬ" => "บึงกาฬ 1",
    "หนองคาย 2" => "หนองคาย 2 (ช)",
    "สกลนคร" => "สกลนคร 1",
    "กุฉินารายณ์ (ชั่วคราว)" => "กุฉินารายณ์ ",
    "กาฬสินธุ์" => "กาฬสินธุ์ 1",
    "ศรีสะเกษ" => "ศรีสะเกษ 1",
    "กันทรลักษ์" => "กันทรลักษ์ 1",
    "อุบลราชธานี 3" => "อุบลราชธานี 2",
    "มหาสารคาม" => "มหาสารคาม 1",
    "บุรีรัมย์" => "บุรีรัมย์ 1",
    "ปราสาท" => "ปราสาท 1",
    "ด่านขุนทด" => "ด่านขุนทด 1",
    "สีคิ้ว" => "สีคิ้ว 1",
    "โชคชัย" => "โชคชัย 1",
    "ปักธงชัย (ชั่วคราว)" => "ปักธงชัย",
    "โคกกรวด 2 (ชั่วคราว)" => "โคกกรวด 2 (ช)",
    "เกษตรสมบูรณ์" => "เกษตรสมบูรณ์ 1",
    "ชัยภูมิ 2 (ชั่วคราว)" => "ชัยภูมิ 2",
    "โชคชัย 1 " => "โชคชัย 1",
    "อ่าวไผ่" => "อ่าวไผ่",
    "มาบตาพุด" => "มาบตาพุด 1",
    "วชิราลงกรณ์" => "วกาญจนบุรี 1",
    "บ่อพลอย 2 (ชั่วคราว)" => "บ่อพลอย 2",
    "ศรีประจันต์" => "ศรีประจันต์ (ช)",
    "อู่ทอง1" => "อู่ทอง 1",
    "หนองคาย1" => "หนองคาย 1",
    "นครราชสีมา 9 (ชั่วคราว)" => "นครราชสีมา 7 (ช)",
    "สวนผึ้ง (ชั่วคราว)" => "สวนผึ้ง",
    "ชะอำ2 - หัวหิน3 (ใช้ Terminal ร่วมกัน บจก.อิเควเตอร์ โซลาร์)" => "ชะอำ 2",
    "นิคมอุตสาหกรรมราชบุรี (ชั่งคราว)" => "นิคมอุตสาหกรรมราชบุรี (ช)",
    "จอมบึง 2" => "จอมบึง",
    "ประจวบฯ" => "ประจวบคีรีขันธ์",
    "หัวหิน" => "หัวหิน",
    "ศรีมหาโพธิ" => "ศรีมหาโพธิ 1",
    "KDA" => "เขื่อนแก่งกระจาน",
    "คลองท่อม(ชั่วคราว)" => "คลองท่อม",
    "พุนพิน2" => "พุนพิน 2",
    "ขนอม" => "นครศรีธรรมราช",
    "เขื่อนเชี่ยวหลาน" => "เขื่อนเชี่ยวหลาน 1",
    "เวียงสระ1" => "เวียงสระ",
    "เวียงสระ 1" => "เวียงสระ",
    "คลองท่อม (ชั่วคราว)" => "คลองท่อม",
    "พังงา" => "พังงา",
    "สงขลา 3 (ชั่วคราว)" => "สงขลา 3 (ช)",
    "แรงต่ำ 380 V สงขลา 1" => "สงขลา 1",
    "แรงต่ำ 380 V หาดใหญ่ 1" => "หาดใหญ่ 1",
    "แรงต่ำ 380 V หาดใหญ่ 3" => "หาดใหญ่ 3",
    "สุไหงโกลก" => "สุไหงโก-ลก",
    "เขื่อนบางลาง" => "ยะลา 1",
    "สงขลา 3" => "สงขลา 3 (ช)",
};

pub fn p12_add_der() -> Result<(), Box<dyn Error>> {
    //sub_inf() -> &'static HashMap<String, SubstInfo>
    let sbif = sub_inf();
    let mut sbls = String::new();
    //let mut sbenm = HashMap::<String, SubstInfo>::new();
    let mut sbtnm = HashMap::<String, SubstInfo>::new();
    for (_, sf) in sbif {
        use std::fmt::Write;
        writeln!(sbls, "'{}','{}','{}'", sf.sbid, sf.name, sf.enam)?;
        /*
        if let Some(_) = sbenm.get(&sf.enam) {
            //println!("dup sub:{} => {} => {}", sf.sbid, sf.name, sf.enam);
        } else {
            sbenm.insert(sf.enam.to_string(), sf.clone());
        }
        */
        if let Some(_) = sbtnm.get(&sf.name) {
            //println!("dup sub:{} => {} => {}", sf.sbid, sf.name, sf.enam);
        } else {
            sbtnm.insert(sf.name.to_string(), sf.clone());
        }
    }
    //let mut sbnotfo = String::new();
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        println!("{ar}");
        let fnds = format!("/mnt/e/CHMBACK/pea-data/data1/p9_{ar}_nodes.bin");
        let fvsp = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_vspp.bin");
        let fspp = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_spp.bin");
        let fsub = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ar}_sb_fd_tr.bin");
        if let (Ok(nds), Ok(vsp), Ok(spp), Ok(fsub)) = (
            File::open(&fnds),
            File::open(&fvsp),
            File::open(&fspp),
            File::open(&fsub),
        ) {
            let nds = BufReader::new(nds);
            let vsp = BufReader::new(vsp);
            let spp = BufReader::new(spp);
            let fsub = BufReader::new(fsub);
            //let mut sb_fd_tr_hm = HashMap::<String, SubFeedTrans>::new();
            if let (Ok(mut nds), Ok(vsp), Ok(spp), Ok(fsub)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<VsppData>>(vsp),
                bincode::deserialize_from::<BufReader<File>, Vec<SppData>>(spp),
                bincode::deserialize_from::<BufReader<File>, Vec<SubFeedTrans>>(fsub),
            ) {
                let mut n1ds = vec![];
                for n1d in nds.keys() {
                    n1ds.push(*n1d);
                }
                n1ds.sort();

                println!("spp: {} vspp:{}", spp.len(), vsp.len());
                //let mut sb_spp = Vec::<usize>::capacity(fsub.len());
                let mut sbid_m = HashMap::<String, usize>::new();
                for (ix, sft) in fsub.iter().enumerate() {
                    //println!("sub {}", sft.sbid);
                    sbid_m.insert(sft.sbid.clone(), ix);
                }

                let mut sb_spp = vec![Vec::<usize>::new(); fsub.len()];
                let mut sc = 0;
                for (ix, s) in spp.iter().enumerate() {
                    let sbid = if let (Some(sb), Some(lo)) = (&s.sub, &s.loc) {
                        let sbnm = sb.to_string();
                        if let Some(sf) = sbtnm.get(&sbnm) {
                            sf.sbid.to_string()
                        } else if let Some(sb) = SB_TH_ADJ1.get(sbnm.as_str()) {
                            let sbnm = sb.to_string();
                            if let Some(sf) = sbtnm.get(&sbnm) {
                                sf.sbid.to_string()
                            } else {
                                //println!("ERROR#1.6 SPP : sb:[{sb}] lo:[{lo}]");
                                "___".to_string()
                            }
                        } else if let Some(_) = SB_TH_ADJ2.get(lo) {
                            if let Some(sf) = sbtnm.get(&sbnm) {
                                sf.sbid.to_string()
                            } else {
                                //println!("ERROR#1.6 SPP : sb:[{sb}] lo:[{lo}]");
                                "___".to_string()
                            }
                        } else {
                            //println!("ERROR#1.6 SPP : sb:[{sb}] lo:[{lo}]");
                            "___".to_string()
                        }
                    } else {
                        //println!("ERROR#1.2 SPP : {:?}", s.sub);
                        "___".to_string()
                    };
                    if let Some(i) = sbid_m.get(&sbid) {
                        sb_spp[*i].push(ix);
                    }
                    if let Some(n1d) = s.n1d {
                        let mut sp = GridNode {
                            ar: ar.to_string(),
                            ly: format!("p12_{ar}_spp"),
                            ix,
                            ntp: NodeType::Source,
                            ..Default::default()
                        };
                        if let Some(nd) = nds.get_mut(&n1d) {
                            nd.nodes.push(sp);
                        } else {
                            sp.n1d = find_node(n1d, &n1ds);
                            if let Some(nd) = nds.get_mut(&sp.n1d) {
                                sc += 1;
                                let (x0, y0) = n1d_2_utm(n1d);
                                let (x1, y1) = n1d_2_utm(sp.n1d);
                                println!("S {sc}. ix:{ix} [{x0},{y0}] -> [{x1},{y1}]");
                                nd.nodes.push(sp);
                            } else {
                                println!("=== ERROR 1 panic");
                            }
                        }
                    }
                }

                let mut sb_vsp = vec![Vec::<usize>::new(); fsub.len()];
                let mut sc = 0;
                for (ix, s) in vsp.iter().enumerate() {
                    let sbid = if let (Some(sb), Some(_fd)) = (&s.sbnm, &s.fdno) {
                        let sbnm = sb.to_string();
                        if let Some(_v) = sbid_m.get(&sbnm) {
                            sbnm.to_string()
                        } else if let Some(sf) = sbtnm.get(&sbnm) {
                            sf.sbid.to_string()
                        } else if let Some(sb) = SB_TH_VSPP.get(sbnm.as_str()) {
                            let sbnm = sb.to_string();
                            if let Some(sf) = sbtnm.get(&sbnm) {
                                sf.sbid.to_string()
                            } else {
                                //println!("ERROR#1.6 SPP : sb:[{sb}] lo:[{lo}]");
                                "___".to_string()
                            }
                        } else {
                            sc += 1;
                            println!("\"{sb}\" => \"{sb}\",");
                            "___".to_string()
                        }
                    } else {
                        "___".to_string()
                    };
                    if let Some(i) = sbid_m.get(&sbid) {
                        sb_vsp[*i].push(ix);
                    }
                    if let Some(n1d) = s.n1d {
                        let mut sp = GridNode {
                            ar: ar.to_string(),
                            ly: format!("p12_{ar}_vspp"),
                            ix,
                            ntp: NodeType::Source,
                            n1d,
                            ..Default::default()
                        };
                        if let Some(nd) = nds.get_mut(&n1d) {
                            nd.nodes.push(sp);
                        } else {
                            sp.n1d = find_node(n1d, &n1ds);
                            if let Some(nd) = nds.get_mut(&sp.n1d) {
                                sc += 1;
                                let (x0, y0) = n1d_2_utm(n1d);
                                let (x1, y1) = n1d_2_utm(sp.n1d);
                                println!("VS {sc}. ix:{ix} [{x0},{y0}] -> [{x1},{y1}]");
                                nd.nodes.push(sp);
                            } else {
                                println!("=== ERROR 1 sws");
                            }
                        }
                    }
                }
                //let mut sb_spp = vec![Vec::<usize>::new(); fsub.len()];
                let fspp = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_spp.bin");
                if let Ok(ser) = bincode::serialize(&sb_spp) {
                    println!("write {ar} write to {fspp}");
                    std::fs::write(fspp, ser).unwrap();
                }
                let fvsp = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_vsp.bin");
                if let Ok(ser) = bincode::serialize(&sb_vsp) {
                    println!("write {ar} write to {fvsp}");
                    std::fs::write(fvsp, ser).unwrap();
                }
                let ond = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_nodes.bin");
                if let Ok(ser) = bincode::serialize(&nds) {
                    println!("write {ar} write to {ond}");
                    std::fs::write(ond, ser).unwrap();
                }
            } // end deseri
        } // end file
    } // end area

    Ok(())
}

//use crate::geo1::CnlData;
use crate::geo2::CnlTrans;
//use regex::Regex;

pub fn p13_add_vol() -> Result<(), Box<dyn Error>> {
    //let re = Regex::new(r"([0-9]+)-.").unwrap();
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        println!("{ar}");
        let (mut c1, mut c2) = (0, 0);
        let fnds = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_nodes.bin");
        let fvol = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_volta.bin");
        let fsub = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ar}_sb_fd_tr.bin");
        let fctr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
        if let (Ok(nds), Ok(vol), Ok(fsub), Ok(fctr)) = (
            File::open(&fnds),
            File::open(&fvol),
            File::open(&fsub),
            File::open(&fctr),
        ) {
            let nds = BufReader::new(nds);
            let vol = BufReader::new(vol);
            let fsub = BufReader::new(fsub);
            let fctr = BufReader::new(fctr);
            //let mut sb_fd_tr_hm = HashMap::<String, SubFeedTrans>::new();
            if let (Ok(mut nds), Ok(vol), Ok(fsub), Ok(ctrs)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<VoltaStation>>(vol),
                bincode::deserialize_from::<BufReader<File>, Vec<SubFeedTrans>>(fsub),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(fctr),
            ) {
                let mut ctrm = HashMap::<String, usize>::new();
                for (ix, ctr) in ctrs.iter().enumerate() {
                    ctrm.insert(ctr.trid.to_string(), ix);
                }
                let mut n1ds = vec![];
                for sft in &fsub {
                    for trids in sft.feed.values() {
                        for ti in trids {
                            n1ds.push(ctrs[*ti].n1d_f);
                        }
                    }
                }
                println!("trx: {}", n1ds.len());
                n1ds.sort();
                let mut sc = 0;
                let mut sc0 = 0;
                for (ix, s) in vol.iter().enumerate() {
                    if let Some(n1d) = s.n1d {
                        let mut n1d0 = n1d;
                        let mut vo = GridNode {
                            ar: ar.to_string(),
                            ly: format!("p13_{ar}_volta"),
                            ix,
                            lix: ix,
                            ntp: NodeType::Load,
                            n1d,
                            ..Default::default()
                        };
                        if let Some(nd) = nds.get_mut(&n1d) {
                            sc0 += 1;
                            nd.nodes.push(vo);
                        } else {
                            vo.n1d = find_node(n1d, &n1ds);
                            n1d0 = vo.n1d;
                            if let Some(nd) = nds.get_mut(&vo.n1d) {
                                let (x0, y0) = n1d_2_utm(n1d);
                                let (x1, y1) = n1d_2_utm(vo.n1d);
                                let dxy = (x0 - x1).abs() + (y0 - y1).abs();
                                if dxy > 50.0 {
                                    sc += 1;
                                    println!(
                                        "V-dif {sc}. dxy:{dxy} ix:{ix} [{x0},{y0}] -> [{x1},{y1}]"
                                    );
                                }
                                nd.nodes.push(vo);
                            } else {
                                println!("=== ERROR 1 vol n1d:{}", vo.n1d);
                            }
                        }
                        if n1ds.contains(&n1d0) {
                            c1 += 1;
                        } else {
                            c2 += 1;
                            println!("VOLTA NOT IN TRNS")
                        }
                    } else {
                        println!("ERROR #3 VOLTA");
                    }
                }
                let ond = format!("/mnt/e/CHMBACK/pea-data/data1/p14_{ar}_nodes.bin");
                if let Ok(ser) = bincode::serialize(&nds) {
                    println!("write {sc}+{sc0} {ar} write to {ond} = {c1}-{c2}");
                    std::fs::write(ond, ser).unwrap();
                }
            } // end deseri
        } // end file
    } // end area

    Ok(())
}

pub fn p13_add_vol2() -> Result<(), Box<dyn Error>> {
    //let re = Regex::new(r"([0-9]+)-.").unwrap();
    for ar in ar_list() {
        if !is_area(ar) {
            continue;
        }
        println!("{ar}");
        let (mut c1, mut c2) = (0, 0);
        let fnds = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_nodes.bin");
        let fvol = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_volta.bin");
        let fsub = format!("/mnt/e/CHMBACK/pea-data/data1/p11_{ar}_sb_fd_tr.bin");
        let fctr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
        if let (Ok(nds), Ok(vol), Ok(fsub), Ok(fctr)) = (
            File::open(&fnds),
            File::open(&fvol),
            File::open(&fsub),
            File::open(&fctr),
        ) {
            let nds = BufReader::new(nds);
            let vol = BufReader::new(vol);
            let fsub = BufReader::new(fsub);
            let fctr = BufReader::new(fctr);
            //let mut sb_fd_tr_hm = HashMap::<String, SubFeedTrans>::new();
            if let (Ok(mut nds), Ok(vol), Ok(_fsub), Ok(ctrs)) = (
                bincode::deserialize_from::<BufReader<File>, HashMap<u64, NodeInfo>>(nds),
                bincode::deserialize_from::<BufReader<File>, Vec<VoltaStation>>(vol),
                bincode::deserialize_from::<BufReader<File>, Vec<SubFeedTrans>>(fsub),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(fctr),
            ) {
                let mut n1ds = vec![];
                /*
                let mut ctrm = HashMap::<String, usize>::new();
                for (ix, ctr) in ctrs.iter().enumerate() {
                    ctrm.insert(ctr.trid.to_string(), ix);
                }
                for sft in &fsub {
                    for trids in sft.feed.values() {
                        for trid in trids {
                            if let Some(ci) = ctrm.get(trid) {
                                n1ds.push(ctrs[*ci].n1d_f);
                            }
                        }
                    }
                }
                */
                /*
                for (n1d, nd) in &nds {
                    for n in &nd.nodes {
                        //if n.ly == "DS_MVConductor" {
                        if n.ly == "DS_Transformer" {
                            n1ds.push(*n1d);
                            break;
                        }
                    }
                }
                */
                let mut tr_in_vo = vec![Vec::<usize>::new(); ctrs.len()];
                let mut n1d_at = HashMap::<u64, usize>::new();
                for (ti, ctr) in ctrs.iter().enumerate() {
                    n1ds.push(ctr.n1d_f);
                    n1d_at.insert(ctr.n1d_f, ti);
                }
                println!("trx: {}", n1ds.len());
                n1ds.sort();
                let mut sc = 0;
                let mut sc0 = 0;
                for (ix, s) in vol.iter().enumerate() {
                    if let Some(n1d) = s.n1d {
                        let mut n1d0 = n1d;
                        let mut vo = GridNode {
                            ar: ar.to_string(),
                            ly: format!("p13_{ar}_volta"),
                            ix,
                            lix: ix,
                            ntp: NodeType::Load,
                            n1d,
                            ..Default::default()
                        };
                        if let Some(nd) = nds.get_mut(&n1d) {
                            sc0 += 1;
                            nd.nodes.push(vo);
                        } else {
                            vo.n1d = find_node(n1d, &n1ds);
                            n1d0 = vo.n1d;
                            if let Some(nd) = nds.get_mut(&vo.n1d) {
                                let (x0, y0) = n1d_2_utm(n1d);
                                let (x1, y1) = n1d_2_utm(vo.n1d);
                                let dxy = (x0 - x1).abs() + (y0 - y1).abs();
                                if dxy > 50.0 {
                                    sc += 1;
                                    println!(
                                        "V-dif {sc}. dxy:{dxy} ix:{ix} [{x0},{y0}] -> [{x1},{y1}]"
                                    );
                                }
                                nd.nodes.push(vo);
                            } else {
                                println!("=== ERROR 1 vol n1d:{}", vo.n1d);
                            }
                        }
                        if let Some(ti) = n1d_at.get(&n1d0) {
                            tr_in_vo[*ti].push(ix);
                        } else {
                            println!("Not found {ix}");
                        }
                        if n1ds.contains(&n1d0) {
                            c1 += 1;
                        } else {
                            c2 += 1;
                            println!("VOLTA NOT IN TRNS")
                        }
                    } else {
                        println!("ERROR #3 VOLTA");
                    }
                }
                let ond = format!("/mnt/e/CHMBACK/pea-data/data1/p14_{ar}_nodes.bin");
                if let Ok(ser) = bincode::serialize(&nds) {
                    println!("write {sc}+{sc0} {ar} write to {ond} == {c1}={c2}");
                    std::fs::write(ond, ser).unwrap();
                }
                let fnm = format!("/mnt/e/CHMBACK/pea-data/data1/p14_{ar}_tr_in_vo.bin");
                if let Ok(ser) = bincode::serialize(&tr_in_vo) {
                    println!("write {} to {fnm}", tr_in_vo.len());
                    std::fs::write(fnm, ser).unwrap();
                }
            } // end deseri
        } // end file
    } // end area

    Ok(())
}

use shapefile::dbase;
pub fn p12_read_gis1() -> Result<(), Box<dyn Error>> {
    let fdir = "/mnt/e/CHMBACK/pea-data/inp1";
    let lys = [
        //"LB_AOJ_Merge_Polygon",
        "DS_GroupMeter_AOJ_Merge",
        "DS_GroupMeter_Detail_Merge_Point",
    ];
    for ly in lys {
        let fsh = format!("{fdir}/{ly}/{ly}.shp");
        let fat = format!("{fdir}/gis/{ly}.pn");
        println!("{ly}");
        if let Ok(mut reader) = shapefile::Reader::from_path(fsh.clone()) {
            let mut vpn = vec![];
            for result in reader.iter_shapes_and_records_as::<shapefile::Point, dbase::Record>() {
                if let Ok((pnt, _rc)) = result {
                    vpn.push((pnt.x, pnt.y));
                }
            }
            print!("pn {}\n", vpn.len());
            if let Ok(ser) = bincode::serialize(&vpn) {
                std::fs::write(fat, ser).unwrap();
            }
        }
    }

    Ok(())
}

pub fn p12_read_gis2() -> Result<(), Box<dyn Error>> {
    let fdir = "/mnt/e/CHMBACK/pea-data/inp1";
    let lys = [
        "LB_AOJ_Merge_Polygon",
        //"DS_GroupMeter_AOJ_Merge",
        //"DS_GroupMeter_Detail_Merge_Point",
    ];
    for ly in lys {
        let fsh = format!("{fdir}/{ly}/{ly}.shp");
        let frg = format!("{fdir}/gis/{ly}.rg");
        println!("{ly}");
        if let Ok(mut reader) = shapefile::Reader::from_path(fsh.clone()) {
            let mut vrg = vec![];
            for result in reader.iter_shapes_and_records_as::<shapefile::Polygon, dbase::Record>() {
                if let Ok((gon, _rc)) = result {
                    let mut ringpnts = Vec::<Vec<(f64, f64)>>::new();
                    for ring in gon.into_inner() {
                        let mut pnts = Vec::<(f64, f64)>::new();
                        for pnt in ring.into_inner() {
                            pnts.push((pnt.x, pnt.y));
                            //cnt += 1;
                        }
                        ringpnts.push(pnts);
                    }
                    vrg.push(ringpnts);
                }
            }
            print!("vrg {}\n", vrg.len());
            if let Ok(ser) = bincode::serialize(&vrg) {
                std::fs::write(frg, ser).unwrap();
            }
        }
    }
    Ok(())
}

use crate::aoj::DbfData;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GisAoj {
    pub ar: String,
    pub xmin: Option<f32>,
    pub ymin: Option<f32>,
    pub xmax: Option<f32>,
    pub ymax: Option<f32>,
    pub level: Option<f32>,
    pub center_x: Option<f32>,
    pub center_y: Option<f32>,
    pub code: Option<String>,
    pub sht_name: Option<String>,
    pub shp_len: Option<f32>,
    pub office: Option<String>,
    pub parent1: Option<String>,
    pub parent2: Option<String>,
    pub pea: Option<String>,
    pub ar_cd: Option<String>,
    pub shp_area: Option<f32>,
    pub prv_cd: Option<String>,
    pub aoj_sz: Option<String>,
    pub reg: Option<String>,
    pub name: Option<String>,
    pub gons: Vec<Vec<(f32, f32)>>,
}

pub static PEA_AR_CD: phf::Map<&'static str, &'static str> = phf_map! {
    "11" => "N1",
    "12" => "N2",
    "13" => "N3",
    "21" => "NE1",
    "22" => "NE2",
    "23" => "NE3",
    "31" => "C1",
    "32" => "C2",
    "33" => "C3",
    "41" => "S1",
    "42" => "S2",
    "43" => "S3",
};

pub static PEA_AR_CD2: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "N1",
    "B" => "N2",
    "C" => "N3",
    "D" => "NE1",
    "E" => "NE2",
    "F" => "NE3",
    "G" => "C1",
    "H" => "C2",
    "I" => "C3",
    "J" => "S1",
    "K" => "S2",
    "L" => "S3",
};

pub fn p12_read_aoj() -> Result<(), Box<dyn Error>> {
    let fdir = "/mnt/e/CHMBACK/pea-data/inp1";
    let ly = "LB_AOJ_Merge_Polygon";
    let frg = format!("{fdir}/gis/{ly}.rg");
    let fat = format!("{fdir}/gis/{ly}.at");
    println!("{frg} - {fat}");
    let mut a1 = HashMap::<String, usize>::new();
    let mut a2 = HashMap::<String, usize>::new();
    let mut a3 = HashMap::<String, usize>::new();
    if let (Ok(frg), Ok(fat)) = (File::open(&frg), File::open(&fat)) {
        let frg = BufReader::new(frg);
        let fat = BufReader::new(fat);
        if let (Ok(frg), Ok(fat)) = (
            bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(frg),
            bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(fat),
        ) {
            let mut cn = 0;
            let mut ar_aojs = HashMap::<String, Vec<GisAoj>>::new();
            for (_i, (rg, at)) in frg.iter().zip(fat.iter()).enumerate() {
                cn += 1;

                let xmin = if let Some(DbfData::Real(v)) = at.get("XMIN") {
                    Some(*v as f32)
                } else {
                    None
                };
                let ymin = if let Some(DbfData::Real(v)) = at.get("YMIN") {
                    Some(*v as f32)
                } else {
                    None
                };
                let xmax = if let Some(DbfData::Real(v)) = at.get("XMAX") {
                    Some(*v as f32)
                } else {
                    None
                };
                let ymax = if let Some(DbfData::Real(v)) = at.get("YMAX") {
                    Some(*v as f32)
                } else {
                    None
                };
                //pub aoj_lv: f32, AOJ_LEVEL - Real(1.0)
                let level = if let Some(DbfData::Real(v)) = at.get("AOJ_LEVEL") {
                    Some(*v as f32)
                } else {
                    None
                };
                //pub center_x: f32, CENTROID_X - Real(11195412.0)
                let center_x = if let Some(DbfData::Real(v)) = at.get("CENTER_X") {
                    Some(*v as f32)
                } else {
                    None
                };

                //pub center_y: f32, CENTROID_Y - Real(1599200.75)
                let center_y = if let Some(DbfData::Real(v)) = at.get("CENTER_Y") {
                    Some(*v as f32)
                } else {
                    None
                };
                //pub code: String, CODE - Text(0714101)
                let code = if let Some(DbfData::Text(s)) = at.get("CODE") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub sht_name: String, SHORT_NAME - Text(กฟส.)
                let sht_name = if let Some(DbfData::Text(s)) = at.get("SHORT_NAME") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub shp_len: f32, Shape_Leng - Real(96850.34375)
                let shp_len = if let Some(DbfData::Real(v)) = at.get("Shape_Leng") {
                    Some(*v as f32)
                } else {
                    None
                };
                //pub office: String, OFFICE - Text(GBIN)
                let office = if let Some(DbfData::Text(s)) = at.get("OFFICE") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub parent1: String, PARENT_1 - Text(G
                let parent1 = if let Some(DbfData::Text(s)) = at.get("PARENT_1") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub parent2: String, PARENT_2 - None
                let parent2 = if let Some(DbfData::Text(s)) = at.get("PARENT_2") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub pea: String, PEACODE - Text(G14101)
                let pea = if let Some(DbfData::Text(s)) = at.get("PEACODE") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub ar_cd: String, AREA_CODE - Text(31)
                let ar_cd = if let Some(DbfData::Text(s)) = at.get("AREA_CODE") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub shp_area: f32, Shape_Area - Real(198580080.0)
                let shp_area = if let Some(DbfData::Real(v)) = at.get("Shape_Area") {
                    Some(*v as f32)
                } else {
                    None
                };
                //pub prv_cd: String, PROV_CODE - Text(14)
                let prv_cd = if let Some(DbfData::Text(s)) = at.get("PROV_CODE") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub aoj_sz: String, AOJ_SIZE - Text(L)
                let aoj_sz = if let Some(DbfData::Text(s)) = at.get("AOJ_SIZE") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub reg: String, REGION - Text(G)
                let reg = if let Some(DbfData::Text(s)) = at.get("REGION") {
                    Some(s.to_string())
                } else {
                    None
                };
                //pub name: String, NAME - Text(กฟส)
                let name = if let Some(DbfData::Text(s)) = at.get("NAME") {
                    Some(s.to_string())
                } else {
                    None
                };
                let mut gons = Vec::<Vec<(f32, f32)>>::new();
                for rg1 in rg {
                    let mut gon = Vec::<(f32, f32)>::new();
                    for rg2 in rg1 {
                        gon.push((rg2.0 as f32, rg2.1 as f32));
                    }
                    gons.push(gon);
                }
                //==== area
                let off = office.clone().unwrap().to_string();
                let off = (&off[0..1]).to_string();
                if let Some(cn) = a1.get_mut(&off) {
                    *cn += 1;
                } else {
                    a1.insert(off.to_string(), 1);
                }
                let arcd = ar_cd.clone().unwrap().to_string();
                let arcd = (&arcd[0..2]).to_string();
                if let Some(cn) = a2.get_mut(&arcd) {
                    *cn += 1;
                } else {
                    a2.insert(arcd.to_string(), 1);
                }
                let peacd = pea.clone().unwrap().to_string();
                let peacd = (&peacd[0..1]).to_string();
                if let Some(cn) = a3.get_mut(&peacd) {
                    *cn += 1;
                } else {
                    a3.insert(peacd.to_string(), 1);
                }
                let ar1 = PEA_AR_CD.get(&arcd).unwrap_or(&"XX");
                let ar2 = PEA_AR_CD2.get(&peacd).unwrap_or(&"XX");
                let ar = if ar1 == ar2 {
                    ar1.to_string()
                } else if ar1 != &"XX" {
                    println!("ERROR1 {ar_cd:?}");
                    ar1.to_string()
                } else {
                    println!("ERROR2 {office:?}");
                    ar2.to_string()
                };
                let aoj = GisAoj {
                    ar,
                    xmin,
                    ymin,
                    xmax,
                    ymax,
                    level,
                    center_x,
                    center_y,
                    code,
                    sht_name,
                    shp_len,
                    office,
                    parent1,
                    parent2,
                    pea,
                    ar_cd,
                    shp_area,
                    prv_cd,
                    aoj_sz,
                    reg,
                    name,
                    gons,
                };
                if let Some(aojs) = ar_aojs.get_mut(&aoj.ar) {
                    aojs.push(aoj);
                } else {
                    ar_aojs.insert(aoj.ar.to_string(), vec![aoj]);
                }
            } // end loop
            println!("cn: {cn}");
            println!("office === {a1:?}");
            println!("ar_cd === {a2:?}");
            println!("pea === {a3:?}");
            for (ar, aojs) in &mut ar_aojs {
                aojs.sort_by(|a, b| a.ar_cd.cmp(&b.ar_cd));
                let fout = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_aoj.bin");
                println!("{ar} write to {fout}");
                if let Ok(ser) = bincode::serialize(&aojs) {
                    std::fs::write(fout, ser)?;
                }
            }
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GisAmp {
    pub ar: String,
    pub amp: Option<String>,     //AMPHOE_T - Text("อ.แม\u{e48}สาย")
    pub prov: Option<String>,    //PROV_NAM_T - Text("จ.เช\u{e35}ยงราย")
    pub perilen: Option<f32>,    //	PERIMETER - Real(100747.2)
    pub male: Option<f32>,       //	TOT_MALE - Real(27878.0)
    pub female: Option<f32>,     //	TOT_FEMALE - Real(28773.0)
    pub amcd: Option<String>,    //	AMP_CODE - Text("09")
    pub ampor: Option<f32>,      //	AMPHOE_ID - Real(1.0)
    pub prov_cd: Option<String>, //	PROV_CODE - Text("57")
    pub prove: Option<String>,   //	PROV_NAM_E - Text("Changwat Chiangrai")
    pub area: Option<f32>,       //	AREA - Real(564000.0)
    pub ampe: Option<String>,    //	AMPHOE_E - Text("Amphoe Mae Sai")
    pub prvcd: Option<String>,   //	P_CODE - Text("CHR")
    pub ampid: Option<String>,   //	AMPHOE_IDN - Text("5709")
    pub gons: Vec<Vec<(f32, f32)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GisMuni {
    pub ar: String,
    pub prov: Option<String>,
    pub mucd: Option<String>,
    pub amcd: Option<String>,
    pub prcd: Option<String>,
    pub perilen: Option<f32>,
    pub muni: Option<String>,
    pub amp: Option<String>,
    pub area: Option<f32>,
    pub gons: Vec<Vec<(f32, f32)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GisZone {
    pub ar: String,
    pub zncd: Option<String>,
    pub zone: Option<String>,
    pub area: Option<f32>,
    pub leng: Option<f32>,
    pub gons: Vec<Vec<(f32, f32)>>,
}

use std::collections::HashSet;
use substring::Substring;

pub fn p12_read_amp() -> Result<(), Box<dyn Error>> {
    let fdir = "/mnt/e/CHMBACK/pea-data/inp1";
    let ly = "amphur_wgs84_z47";
    let frg = format!("{fdir}/gis/{ly}.rg");
    let fat = format!("{fdir}/gis/{ly}.at");
    let mut prvh = HashSet::<String>::new();
    println!("{frg} - {fat}");
    let sbif = sub_inf();
    let mut prv2ar = HashMap::<String, String>::new();
    for sf in sbif.values() {
        if let Some(_) = prv2ar.get(&sf.prov) {
        } else {
            prv2ar.insert(sf.prov.to_string(), sf.arid.to_string());
        }
    }
    let mut ar_amps = HashMap::<String, Vec<GisAmp>>::new();
    if let (Ok(frg), Ok(fat)) = (File::open(&frg), File::open(&fat)) {
        let frg = BufReader::new(frg);
        let fat = BufReader::new(fat);
        if let (Ok(frg), Ok(fat)) = (
            bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(frg),
            bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(fat),
        ) {
            println!("read {}", frg.len());
            for (_i, (rg, at)) in frg.iter().zip(fat.iter()).enumerate() {
                //  amp	AMPHOE_T - Text("อ.แม\u{e48}สาย")
                let amp = if let Some(DbfData::Text(v)) = at.get("AMPHOE_T") {
                    Some(v.to_string())
                } else {
                    None
                };
                let mut prv = String::from("---");

                //	PROV_NAM_T - Text("จ.เช\u{e35}ยงราย")
                let prov = if let Some(DbfData::Text(v)) = at.get("PROV_NAM_T") {
                    let vv = v.substring(2, v.len());
                    prv = vv.to_string();
                    Some(vv.to_string())
                    //Some(v[2..].to_string())
                } else {
                    None
                };
                //	PERIMETER - Real(100747.2)
                let perilen = if let Some(DbfData::Real(v)) = at.get("PERIMETER") {
                    Some(*v as f32)
                } else {
                    None
                };
                //	TOT_MALE - Real(27878.0)
                let male = if let Some(DbfData::Real(v)) = at.get("TOT_MALE") {
                    Some(*v as f32)
                } else {
                    None
                };
                //	TOT_FEMALE - Real(28773.0)
                let female = if let Some(DbfData::Real(v)) = at.get("TOT_FEMALE") {
                    Some(*v as f32)
                } else {
                    None
                };
                //	AMP_CODE - Text("09")
                let amcd = if let Some(DbfData::Text(v)) = at.get("AMP_CODE") {
                    Some(v.to_string())
                } else {
                    None
                };
                //	AMPHOE_ID - Real(1.0)
                let ampor = if let Some(DbfData::Real(v)) = at.get("AMPHOE_ID") {
                    Some(*v as f32)
                } else {
                    None
                };
                //	PROV_CODE - Text("57")
                let prov_cd = if let Some(DbfData::Text(v)) = at.get("PROV_CODE") {
                    Some(v.to_string())
                } else {
                    None
                };
                //	PROV_NAM_E - Text("Changwat Chiangrai")
                let prove = if let Some(DbfData::Text(v)) = at.get("PROV_NAM_E") {
                    Some(v.to_string())
                } else {
                    None
                };
                //	AREA - Real(564000.0)
                let area = if let Some(DbfData::Real(v)) = at.get("AREA") {
                    Some(*v as f32)
                } else {
                    None
                };
                //	AMPHOE_E - Text("Amphoe Mae Sai")
                let ampe = if let Some(DbfData::Text(v)) = at.get("AMPHOE_E") {
                    Some(v.to_string())
                } else {
                    None
                };
                //	P_CODE - Text("CHR")
                let prvcd = if let Some(DbfData::Text(v)) = at.get("P_CODE") {
                    Some(v.to_string())
                } else {
                    None
                };
                //	AMPHOE_IDN - Text("5709")
                let ampid = if let Some(DbfData::Text(v)) = at.get("AMPHOE_IDN") {
                    Some(v.to_string())
                } else {
                    None
                };
                let mut gons = Vec::<Vec<(f32, f32)>>::new();
                for rg1 in rg {
                    let mut gon = Vec::<(f32, f32)>::new();
                    for rg2 in rg1 {
                        gon.push((rg2.0 as f32, rg2.1 as f32));
                    }
                    gons.push(gon);
                }
                if !prvh.contains(&prv) {
                    prvh.insert(prv.to_string());
                    //println!("{prv:?}");
                }
                let ar = prv2ar.get(&prv).unwrap_or(&"--".to_string()).to_string();
                if ar == "--" {
                    continue;
                    //println!("prv: '{prv}'");
                }
                let amp = GisAmp {
                    ar,
                    amp,
                    prov,
                    perilen,
                    male,
                    female,
                    amcd,
                    ampor,
                    prov_cd,
                    prove,
                    area,
                    ampe,
                    prvcd,
                    ampid,
                    gons,
                };
                if let Some(amps) = ar_amps.get_mut(&amp.ar) {
                    amps.push(amp);
                } else {
                    ar_amps.insert(amp.ar.to_string(), vec![amp]);
                }
            }
            for (ar, aojs) in &mut ar_amps {
                aojs.sort_by(|a, b| a.ar.cmp(&b.ar));
                let fout = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_amp.bin");
                println!("{ar} write to {fout}");
                if let Ok(ser) = bincode::serialize(&aojs) {
                    std::fs::write(fout, ser)?;
                }
            }
        }
    }
    Ok(())
}

pub fn p12_chk_muni() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        let fmun = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_muni.bin");
        if let Ok(fmun) = File::open(&fmun) {
            let fmun = BufReader::new(fmun);
            if let Ok(fmun) = bincode::deserialize_from::<BufReader<File>, Vec<GisMuni>>(fmun) {
                println!("{ar} = {}", fmun.len());
                for mu in fmun {
                    println!("{:?} in {:?}", mu.amp, mu.prov);
                }
            }
        }
    }
    Ok(())
}

pub fn p12_read_muni() -> Result<(), Box<dyn Error>> {
    let fdir = "/mnt/e/CHMBACK/pea-data/inp1";
    let ly = "municipal_wgs84_z47";
    let frg = format!("{fdir}/gis/{ly}.rg");
    let fat = format!("{fdir}/gis/{ly}.at");
    let mut prvh = HashSet::<String>::new();
    println!("{frg} - {fat}");
    let sbif = sub_inf();
    let mut prv2ar = HashMap::<String, String>::new();
    for sf in sbif.values() {
        if let Some(_) = prv2ar.get(&sf.prov) {
        } else {
            prv2ar.insert(sf.prov.to_string(), sf.arid.to_string());
        }
    }
    let mut ar_munis = HashMap::<String, Vec<GisMuni>>::new();
    if let (Ok(frg), Ok(fat)) = (File::open(&frg), File::open(&fat)) {
        let frg = BufReader::new(frg);
        let fat = BufReader::new(fat);
        if let (Ok(frg), Ok(fat)) = (
            bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(frg),
            bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(fat),
        ) {
            println!("read {}", frg.len());
            for (_i, (rg, at)) in frg.iter().zip(fat.iter()).enumerate() {
                let mut prv = String::from("---");

                //prov	PROV_NAM_T - Text
                let prov = if let Some(DbfData::Text(v)) = at.get("PROV_NAM_T") {
                    let vv = v.substring(2, v.len());
                    prv = vv.to_string();
                    Some(prv.to_string())
                } else {
                    None
                };
                //mucd	MUNI_CODE - Text
                let mucd = if let Some(DbfData::Text(v)) = at.get("MUNI_CODE") {
                    Some(v.to_string())
                } else {
                    None
                };
                //amcd	AMP_CODE - Text
                let amcd = if let Some(DbfData::Text(v)) = at.get("AMP_CODE") {
                    Some(v.to_string())
                } else {
                    None
                };
                //prcd	PROV_CODE - Text
                let prcd = if let Some(DbfData::Text(v)) = at.get("PROV_CODE") {
                    Some(v.to_string())
                } else {
                    None
                };
                //perilen	PERIMETER - Real
                let perilen = if let Some(DbfData::Real(v)) = at.get("PERIMETER") {
                    Some(*v as f32)
                } else {
                    None
                };
                //muni	MUNI_NAME - Text
                let muni = if let Some(DbfData::Text(v)) = at.get("MUNI_NAME") {
                    Some(v.to_string())
                } else {
                    None
                };
                //amp	AMPHOE_T - Text
                let amp = if let Some(DbfData::Text(v)) = at.get("AMPHOE_T") {
                    Some(v.to_string())
                } else {
                    None
                };
                //area	AREA - Real(488880.0)
                let area = if let Some(DbfData::Real(v)) = at.get("AREA") {
                    Some(*v as f32)
                } else {
                    None
                };
                let mut gons = Vec::<Vec<(f32, f32)>>::new();
                for rg1 in rg {
                    let mut gon = Vec::<(f32, f32)>::new();
                    for rg2 in rg1 {
                        gon.push((rg2.0 as f32, rg2.1 as f32));
                    }
                    gons.push(gon);
                }
                if !prvh.contains(&prv) {
                    prvh.insert(prv.to_string());
                    //println!("{prv:?}");
                }
                let ar = prv2ar.get(&prv).unwrap_or(&"--".to_string()).to_string();
                if ar == "--" {
                    continue;
                    //println!("prv: '{prv}'");
                }
                let muni = GisMuni {
                    ar,
                    prov,
                    mucd,
                    amcd,
                    prcd,
                    perilen,
                    muni,
                    amp,
                    area,
                    gons,
                };
                if let Some(munis) = ar_munis.get_mut(&muni.ar) {
                    munis.push(muni);
                } else {
                    ar_munis.insert(muni.ar.to_string(), vec![muni]);
                }
            }
            for (ar, aojs) in &mut ar_munis {
                aojs.sort_by(|a, b| a.ar.cmp(&b.ar));
                let fout = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_muni.bin");
                println!("{ar} write {} to {fout}", aojs.len());
                if let Ok(ser) = bincode::serialize(&aojs) {
                    std::fs::write(fout, ser)?;
                }
            }
        }
    }
    Ok(())
}

/*
pub fn p12_chk_popu() -> Result<(), Box<dyn Error>> {
    //let fpop = format!("/mnt/e/CHMBACK/pea-data/data1/amp_pop.bin");
    let fpop = format!("/mnt/e/CHMBACK/pea-data/data1/mun_pop.bin");
    println!("fpop: {fpop}");
    let re1 = Regex::new(r"เทศบาลตำบลเมือง(.+)").unwrap();
    let re2 = Regex::new(r"เทศบาลตำบลตำบล(.+)").unwrap();
    if let Ok(fpop) = File::open(&fpop) {
        let fpop = BufReader::new(fpop);
        if let Ok(fpop) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, AmpPop>>(fpop)
        {
            println!("muni pop: {}", fpop.len());
            for (k, po) in &fpop {
                println!("pop {k} {}", po.pop);
            }
            for ar in ar_list() {
                let fgon = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_muni.bin");
                if let Ok(fgon) = File::open(&fgon) {
                    let fgon = BufReader::new(fgon);
                    if let Ok(fgon) =
                        bincode::deserialize_from::<BufReader<File>, Vec<GisMuni>>(fgon)
                    {
                        println!("{ar} = {}", fgon.len());
                        for mu in &fgon {
                            let mut muni = mu.muni.clone().unwrap_or("-".to_string());
                            if let Some(cap) = re1.captures_iter(&muni).next() {
                                muni = format!("เทศบาลเมือง{}", &cap[1]);
                                println!("===== 1: {muni}");
                            }
                            if let Some(cap) = re2.captures_iter(&muni).next() {
                                muni = format!("เทศบาลตำบล{}", &cap[1]);
                                println!("===== 1: {muni}");
                            }
                            if let Some(mu) = TESABAN_MAP1.get(&muni) {
                                muni = mu.to_string();
                                println!("===== 2: {muni}");
                            }
                            if let Some(po) = fpop.get(&muni) {
                            } else {
                                let prov = mu.prov.clone().unwrap_or("_".to_string());
                                println!("{muni} in {prov}");
                            }
                        }
                    }
                }
                //break;
            }
        }
    }
    Ok(())
}
*/

pub fn p12_map_muni_pop() -> Result<(), Box<dyn Error>> {
    //let fpop = format!("/mnt/e/CHMBACK/pea-data/data1/amp_pop.bin");
    let fpop = format!("/mnt/e/CHMBACK/pea-data/data1/mun_pop.bin");
    println!("fpop: {fpop}");
    let re1 = Regex::new(r"เทศบาลตำบลเมือง(.+)").unwrap();
    let re2 = Regex::new(r"เทศบาลตำบลตำบล(.+)").unwrap();
    let re3 = Regex::new(r"เทศบาลเมือง(.+)").unwrap();
    let re4 = Regex::new(r"เทศบาลนคร(.+)").unwrap();
    let re5 = Regex::new(r"เทศบาลตำบล(.+)").unwrap();
    let re6 = Regex::new(r"เทศบาล(.+)").unwrap();
    if let Ok(fpop) = File::open(&fpop) {
        let fpop = BufReader::new(fpop);
        if let Ok(fpop) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, AmpPop>>(fpop)
        {
            println!("muni pop: {}", fpop.len());
            let mut popm = HashMap::<String, AmpPop>::new();
            for (k, po) in &fpop {
                let mut muni = k.to_string();
                muni = muni.trim().to_string();
                if let Some(cap) = re1.captures_iter(&muni).next() {
                    muni = format!("{}", &cap[1]);
                }
                if let Some(cap) = re2.captures_iter(&muni).next() {
                    muni = format!("{}", &cap[1]);
                }
                if let Some(cap) = re3.captures_iter(&muni).next() {
                    muni = format!("{}", &cap[1]);
                }
                if let Some(cap) = re4.captures_iter(&muni).next() {
                    muni = format!("{}", &cap[1]);
                }
                if let Some(cap) = re5.captures_iter(&muni).next() {
                    muni = format!("{}", &cap[1]);
                }
                if let Some(cap) = re6.captures_iter(&muni).next() {
                    muni = format!("{}", &cap[1]);
                }
                let key = format!("{muni}-{}", po.prv);
                if popm.contains_key(&key) {
                    //println!("=========== duplicate '{key}' {}", po.pop);
                } else {
                    //println!("pop {key} {}", po.pop);
                    popm.insert(key, po.clone());
                }
            }
            for ar in ar_list() {
                let fgon = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_muni.bin");
                if let Ok(fgon) = File::open(&fgon) {
                    let fgon = BufReader::new(fgon);
                    if let Ok(fgon) =
                        bincode::deserialize_from::<BufReader<File>, Vec<GisMuni>>(fgon)
                    {
                        println!("{ar} = {}", fgon.len());
                        let mut podens = Vec::<PopuDense>::new();
                        let mut mu_de_po = Vec::<PopuDenseSave>::new();
                        for mu in &fgon {
                            let mut muni = mu.muni.clone().unwrap_or("-".to_string());
                            muni = muni.trim().to_string();
                            if let Some(cap) = re1.captures_iter(&muni).next() {
                                muni = format!("{}", &cap[1]);
                            }
                            if let Some(cap) = re2.captures_iter(&muni).next() {
                                muni = format!("{}", &cap[1]);
                            }
                            if let Some(cap) = re3.captures_iter(&muni).next() {
                                muni = format!("{}", &cap[1]);
                            }
                            if let Some(cap) = re4.captures_iter(&muni).next() {
                                muni = format!("{}", &cap[1]);
                            }
                            if let Some(cap) = re5.captures_iter(&muni).next() {
                                muni = format!("{}", &cap[1]);
                            }
                            if let Some(cap) = re6.captures_iter(&muni).next() {
                                muni = format!("{}", &cap[1]);
                            }
                            let prov = mu.prov.clone().unwrap_or("".to_string());
                            let key = format!("{muni}-{prov}");

                            if let (Some(po), Some(area)) = (popm.get(&key), mu.area) {
                                let popu = po.pop as f32;
                                let dens = popu / area;

                                let mut gons = vec![];
                                for go in &mu.gons {
                                    let mut lines = vec![];
                                    for (x, y) in go {
                                        lines.push(coord! { x: *x as f32, y: *y as f32, });
                                    }
                                    let line_string = LineString::new(lines);
                                    let polygon = Polygon::new(line_string.clone(), vec![]);
                                    gons.push(polygon);
                                }
                                let pode = PopuDense {
                                    key: key.to_string(),
                                    popu,
                                    area,
                                    dens,
                                    gons,
                                };
                                podens.push(pode);

                                let mut gons = vec![];
                                for go in &mu.gons {
                                    let mut lines = vec![];
                                    for (x, y) in go {
                                        lines.push((*x, *y));
                                    }
                                    gons.push(lines);
                                }
                                let pode = PopuDenseSave {
                                    key,
                                    popu,
                                    area,
                                    dens,
                                    gons,
                                };
                                mu_de_po.push(pode);
                            } else {
                                //let prov = mu.prov.clone().unwrap_or("_".to_string());
                                //println!("'{key}' in {prov}");
                            }
                        }
                        let f_mu_po_de =
                            format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_mu_po_de.bin");
                        if let Ok(ser) = bincode::serialize(&mu_de_po) {
                            println!("  map finish {} write to {f_mu_po_de}", podens.len());
                            std::fs::write(f_mu_po_de, ser)?;
                        }

                        //=== transformer
                        println!("===== load transformer");
                        let fctr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
                        if let Ok(fctr) = File::open(&fctr) {
                            let fctr = BufReader::new(fctr);
                            if let Ok(ctrs) =
                                bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(fctr)
                            {
                                println!("{} loaded", ctrs.len());
                                let mut tr_in_mun = vec![Vec::<usize>::new(); ctrs.len()];
                                let mut cn = 0;
                                for (ti, ctr) in ctrs.iter().enumerate() {
                                    let (x, y) = n1d_2_utm(ctr.n1d_f);
                                    let pn = point!(x: x, y: y);
                                    for (gi, po) in podens.iter().enumerate() {
                                        for pp in po.gons.iter() {
                                            if pp.contains(&pn) {
                                                cn += 1;
                                                tr_in_mun[ti].push(gi);
                                            }
                                        }
                                    }
                                }
                                let ftr_in_mun =
                                    format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_tr_in_mun.bin");
                                if let Ok(ser) = bincode::serialize(&tr_in_mun) {
                                    println!("  map finish {cn} write to {ftr_in_mun}");
                                    std::fs::write(ftr_in_mun, ser)?;
                                }
                            }
                        }
                    }
                }
                //break;
            }
        }
    }
    Ok(())
}

pub static TESABAN_MAP1: phf::Map<&'static str, &'static str> = phf_map! {
    "เมือง-ปทุมธานี" => "เมืองปทุมธานี-ปทุมธานี",
    "บึงกาฬ-หนองคาย" => "เมืองบึงกาฬ-บึงกาฬ",
    "บุ่งคล้า-หนองคาย" => "บุ่งคล้า-บึงกาฬ",
    "โซ่พิสัย-หนองคาย" => "โซ่พิสัย-บึงกาฬ",
    "ศรีวิไล-หนองคาย" => "ศรีวิไล-บึงกาฬ",
    "เซกา-หนองคาย" => "เซกา-บึงกาฬ",
    "พรเจริญ-หนองคาย" => "พรเจริญ-บึงกาฬ",
    "บึงโขงหลง-หนองคาย" => "บึงโขงหลง-บึงกาฬ",
    "วัวซอ-อุดรธานี" => "หนองวัวซอ-อุดรธานี",
    "ประจักษ์-อุดรธานี" => "ประจักษ์ศิลปาคม-อุดรธานี",
    " วังยาง-นครพนม" => "วังยาง-นครพนม",
    "หนองนา-ขอนแก่น" => "หนองนาคำ-ขอนแก่น",
    "บ้านแฮ-ขอนแก่น" => "บ้านแฮด-ขอนแก่น",
    "โคกโพธิ์-ขอนแก่น" => "โคกโพธิ์ไชย-ขอนแก่น",
    "เมืองศรีเกษ-ศรีสะเกษ" => "เมืองศรีสะเกษ-ศรีสะเกษ",
    "เมืองกาฬสินธ์-กาฬสินธุ์" => "เมืองกาฬสินธุ์-กาฬสินธุ์",
    "ต.สมเด็จ-กาฬสินธุ์" => "สมเด็จ-กาฬสินธุ์",
    "สุไหงโก-ลก-นราธิวาส" => "สุไหงโกลก-นราธิวาส",
    "ต.เบตง-ยะลา" => "เบตง-ยะลา",
    "หาดให่ญ-สงขลา" => "หาดใหญ่-สงขลา",
    "กันตรัง-ตรัง" => "กันตัง-ตรัง",
    "เมือง-นครราชสีมา" => "เมืองนครราชสีมา-นครราชสีมา",
};

use geo::Contains;
use geo::{point, Polygon};
use geo_types::{coord, LineString};

#[derive(Debug, Clone, Default)]
pub struct PopuDense {
    pub key: String,
    pub popu: f32,
    pub area: f32,
    pub dens: f32,
    pub gons: Vec<Polygon<f32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PopuDenseSave {
    pub key: String,
    pub popu: f32,
    pub area: f32,
    pub dens: f32,
    pub gons: Vec<Vec<(f32, f32)>>,
}

pub fn p12_map_amp_pop() -> Result<(), Box<dyn Error>> {
    let fpop = format!("/mnt/e/CHMBACK/pea-data/data1/amp_pop.bin");
    //let fpop = format!("/mnt/e/CHMBACK/pea-data/data1/mun_pop.bin");
    println!("fpop: {fpop}");
    let re1 = Regex::new(r"อำเภอ(.+)").unwrap();
    let re2 = Regex::new(r"กิ่งอำเภอ(.+)").unwrap();
    let re3 = Regex::new(r"อ.(.+)").unwrap();
    let re4 = Regex::new(r"กิ่งอ.(.+)").unwrap();
    if let Ok(fpop) = File::open(&fpop) {
        let fpop = BufReader::new(fpop);
        if let Ok(fpop) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, AmpPop>>(fpop)
        {
            println!("amp pop: {}", fpop.len());
            let mut popm = HashMap::<String, AmpPop>::new();
            for (k, po) in &fpop {
                let mut amp = k.to_string();
                amp = amp.trim().to_string();
                if let Some(cap) = re1.captures_iter(&amp).next() {
                    amp = format!("{}", &cap[1]);
                }
                if let Some(cap) = re2.captures_iter(&amp).next() {
                    amp = format!("{}", &cap[1]);
                }
                let key = format!("{amp}-{}", po.prv);
                if popm.contains_key(&key) {
                    //println!("=========== duplicate '{key}' {}", po.pop);
                } else {
                    //println!("pop {key} {}", po.pop);
                    popm.insert(key, po.clone());
                }
            }
            for ar in ar_list() {
                let fgon = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_amp.bin");
                if let Ok(fgon) = File::open(&fgon) {
                    let fgon = BufReader::new(fgon);
                    if let Ok(fgon) =
                        bincode::deserialize_from::<BufReader<File>, Vec<GisAmp>>(fgon)
                    {
                        println!("{ar} = {}", fgon.len());
                        // pre pare polygon
                        let mut podens = Vec::<PopuDense>::new();
                        let mut am_de_po = Vec::<PopuDenseSave>::new();
                        for am in &fgon {
                            let mut amp = am.amp.clone().unwrap_or("-".to_string());
                            amp = amp.trim().to_string();
                            if let Some(cap) = re3.captures_iter(&amp).next() {
                                amp = format!("{}", &cap[1]);
                            }
                            if let Some(cap) = re4.captures_iter(&amp).next() {
                                amp = format!("{}", &cap[1]);
                            }
                            let prov = am.prov.clone().unwrap_or("".to_string());
                            let mut key = format!("{amp}-{prov}");
                            if let Some(k) = TESABAN_MAP1.get(&key) {
                                key = k.to_string();
                            }
                            if let (Some(po), Some(area)) = (popm.get(&key), am.area) {
                                let popu = po.pop as f32;
                                let dens = popu / area;
                                //println!("{key} {} {:?}", po.pop, am.area);
                                let mut gons = vec![];
                                for go in &am.gons {
                                    let mut lines = vec![];
                                    for (x, y) in go {
                                        lines.push(coord! { x: *x as f32, y: *y as f32, });
                                    }
                                    let line_string = LineString::new(lines);
                                    let polygon = Polygon::new(line_string.clone(), vec![]);
                                    gons.push(polygon);
                                }
                                let pode = PopuDense {
                                    key: key.to_string(),
                                    popu,
                                    area,
                                    dens,
                                    gons,
                                };
                                podens.push(pode);

                                let mut gons = vec![];
                                for go in &am.gons {
                                    let mut lines = vec![];
                                    for (x, y) in go {
                                        lines.push((*x, *y));
                                    }
                                    gons.push(lines);
                                }
                                let pode = PopuDenseSave {
                                    key,
                                    popu,
                                    area,
                                    dens,
                                    gons,
                                };
                                am_de_po.push(pode);
                            } else {
                                //let prov = am.prov.clone().unwrap_or("_".to_string());
                                //println!("'{key}' in {prov}");
                            }
                        } // end prepare polygon

                        let f_am_po_de =
                            format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_am_po_de.bin");
                        if let Ok(ser) = bincode::serialize(&am_de_po) {
                            println!("  map finish {} write to {f_am_po_de}", podens.len());
                            std::fs::write(f_am_po_de, ser)?;
                        }

                        //=== transformer
                        println!("===== load transformer");
                        let fctr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
                        if let Ok(fctr) = File::open(&fctr) {
                            let fctr = BufReader::new(fctr);
                            if let Ok(ctrs) =
                                bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(fctr)
                            {
                                println!("{} loaded", ctrs.len());
                                let mut tr_in_amp = vec![Vec::<usize>::new(); ctrs.len()];
                                let mut cn = 0;
                                for (ti, ctr) in ctrs.iter().enumerate() {
                                    let (x, y) = n1d_2_utm(ctr.n1d_f);
                                    let pn = point!(x: x, y: y);
                                    for (gi, po) in podens.iter().enumerate() {
                                        for pp in po.gons.iter() {
                                            if pp.contains(&pn) {
                                                cn += 1;
                                                tr_in_amp[ti].push(gi);
                                            }
                                        }
                                    }
                                }
                                let ftr_in_amp =
                                    format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_tr_in_amp.bin");
                                if let Ok(ser) = bincode::serialize(&tr_in_amp) {
                                    println!("  map finish {cn} write to {ftr_in_amp}");
                                    std::fs::write(ftr_in_amp, ser)?;
                                }
                            }
                        }
                    }
                }
                //break;
            }
        }
    }
    Ok(())
}

pub fn p12_map_tr_aoj() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        let faoj = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_aoj.bin");
        let ftr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
        if let (Ok(faoj), Ok(ftr)) = (File::open(&faoj), File::open(&ftr)) {
            let faoj = BufReader::new(faoj);
            let ftr = BufReader::new(ftr);
            if let (Ok(faoj), Ok(ftr)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<GisAoj>>(faoj),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(ftr),
            ) {
                println!("tr:{} aoj:{}", ftr.len(), faoj.len());
                let mut aojgon = vec![];
                for aoj in &faoj {
                    let mut gons = vec![];
                    for go in &aoj.gons {
                        let mut lines = vec![];
                        for (x, y) in go {
                            lines.push(coord! { x: *x, y: *y, });
                        }
                        let line_string = LineString::new(lines);
                        let polygon = Polygon::new(line_string.clone(), vec![]);
                        gons.push(polygon);
                    }
                    aojgon.push(gons);
                }
                let mut tr_in_aoj = vec![Vec::<usize>::new(); ftr.len()];
                let mut cn = 0;
                for (ti, ctr) in ftr.iter().enumerate() {
                    let (x, y) = n1d_2_utm(ctr.n1d_f);
                    let pn = point!(x: x, y: y);
                    for (gi, po) in aojgon.iter().enumerate() {
                        for pp in po.iter() {
                            if pp.contains(&pn) {
                                cn += 1;
                                tr_in_aoj[ti].push(gi);
                            }
                        }
                    }
                }
                let ftr_in_aoj = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_tr_in_aoj.bin");
                if let Ok(ser) = bincode::serialize(&tr_in_aoj) {
                    println!("  map finish {cn} write to {ftr_in_aoj}");
                    std::fs::write(ftr_in_aoj, ser)?;
                }
            }
        }
    }
    Ok(())
}

pub fn p12_map_tr_zone() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        let fzn = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_zone.bin");
        let ftr = format!("/mnt/e/CHMBACK/pea-data/data1/p10_{ar}_cnl_trs.bin");
        if let (Ok(fzn), Ok(ftr)) = (File::open(&fzn), File::open(&ftr)) {
            let fzn = BufReader::new(fzn);
            let ftr = BufReader::new(ftr);
            if let (Ok(fzn), Ok(ftr)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<GisZone>>(fzn),
                bincode::deserialize_from::<BufReader<File>, Vec<CnlTrans>>(ftr),
            ) {
                println!("tr:{} zn:{}", ftr.len(), fzn.len());
                let mut zngon = vec![];
                for zn in &fzn {
                    let mut gons = vec![];
                    for go in &zn.gons {
                        let mut lines = vec![];
                        for (x, y) in go {
                            lines.push(coord! { x: *x, y: *y, });
                        }
                        let line_string = LineString::new(lines);
                        let polygon = Polygon::new(line_string.clone(), vec![]);
                        gons.push(polygon);
                    }
                    zngon.push(gons);
                }
                let mut tr_in_zn = vec![Vec::<usize>::new(); ftr.len()];
                let mut cn = 0;
                for (ti, ctr) in ftr.iter().enumerate() {
                    let (x, y) = n1d_2_utm(ctr.n1d_f);
                    let pn = point!(x: x, y: y);
                    for (gi, po) in zngon.iter().enumerate() {
                        for pp in po.iter() {
                            if pp.contains(&pn) {
                                cn += 1;
                                tr_in_zn[ti].push(gi);
                            }
                        }
                    }
                }
                let ftr_in_zn = format!("/mnt/e/CHMBACK/pea-data/data1/p13_{ar}_tr_in_zn.bin");
                if let Ok(ser) = bincode::serialize(&tr_in_zn) {
                    println!("  map finish {cn} write to {ftr_in_zn}");
                    std::fs::write(ftr_in_zn, ser)?;
                }
            }
        }
    }
    Ok(())
}

//use crate::aoj::DB2_DIR;
use crate::geo1::DB2_DIR;

pub fn p12_read_zone() -> Result<(), Box<dyn Error>> {
    for ar in ar_list() {
        println!("Area: {ar}");
        let frg = format!("{DB2_DIR}/{ar}_Zone_Use.rg");
        let fat = format!("{DB2_DIR}/{ar}_Zone_use.at");
        let mut zones = Vec::<GisZone>::new();
        println!("read {frg} {fat}");
        if let (Ok(frg), Ok(fat)) = (File::open(&frg), File::open(&fat)) {
            let frg = BufReader::new(frg);
            let fat = BufReader::new(fat);
            if let (Ok(frg), Ok(fat)) = (
                bincode::deserialize_from::<BufReader<File>, Vec<Vec<Vec<(f64, f64)>>>>(frg),
                bincode::deserialize_from::<BufReader<File>, Vec<HashMap<String, DbfData>>>(fat),
            ) {
                println!("read {}", frg.len());
                for (_i, (rg, at)) in frg.iter().zip(fat.iter()).enumerate() {
                    //zncd	ZONE_CODE - Text41
                    let zncd = if let Some(DbfData::Text(v)) = at.get("ZONE_CODE") {
                        Some(v.to_string())
                    } else {
                        None
                    };
                    //zone	ZONE_NAME - Text
                    let zone = if let Some(DbfData::Text(v)) = at.get("ZONE_NAME") {
                        Some(v.to_string())
                    } else {
                        None
                    };
                    //leng	SHAPE_Leng - Real(24101.47265625)
                    let leng = if let Some(DbfData::Real(v)) = at.get("SHAPE_Leng") {
                        Some(*v as f32)
                    } else {
                        None
                    };
                    //area	SHAPE_Area - Real(17712380.0)
                    let area = if let Some(DbfData::Real(v)) = at.get("SHAPE_Area") {
                        Some(*v as f32)
                    } else {
                        None
                    };
                    let mut gons = Vec::<Vec<(f32, f32)>>::new();
                    for rg1 in rg {
                        let mut gon = Vec::<(f32, f32)>::new();
                        for rg2 in rg1 {
                            gon.push((rg2.0 as f32, rg2.1 as f32));
                        }
                        gons.push(gon);
                    }
                    let ar = ar.to_string();
                    let zn = GisZone {
                        ar,
                        zncd,
                        zone,
                        area,
                        leng,
                        gons,
                    };
                    zones.push(zn);
                } // end loop rg
                let fout = format!("/mnt/e/CHMBACK/pea-data/data1/p12_{ar}_zone.bin");
                println!("{ar} write {} to {fout}", zones.len());
                if let Ok(ser) = bincode::serialize(&zones) {
                    std::fs::write(fout, ser)?;
                }
            } // end deserial
        } // end file
    }
    Ok(())
}

use crate::geo1::AmpPop;
use regex::Regex;
use sglab02_lib::sg::imp::xlsx_data;

pub async fn amp_popu_read() -> Result<(), Box<dyn Error>> {
    let flst = vec![format!("/mnt/e/CHMBACK/pea-data/inp1/stat_a67_ampho.xlsx")];
    println!("{:?}", flst);
    let re = Regex::new(r"ท้องถิ่น(.+)").unwrap();
    let repv = Regex::new(r"จังหวัด(.+)").unwrap();
    if let Ok(xlsv) = xlsx_data(&flst).await {
        let mut ampos = vec![];
        for x in &xlsv {
            println!("'{}' '{}' {} {}", x.path, x.shnm, x.rcnt, x.ccnt);
            for (i, r) in x.data.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                let mut prv = r[2].to_string();
                if let Some(cap) = repv.captures_iter(&prv).next() {
                    prv = cap[1].to_string();
                }
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
                println!("AM: {c3}.{} in {}", ap.amp, ap.prv);
                ampm.insert(ap.amp.to_string(), ap.clone());
                c3 += 1;
            }
        }
        println!("c1:{c1} c2:{c2} c3:{c3}");
        if let Ok(ser) = bincode::serialize(&ampm) {
            std::fs::write("/mnt/e/CHMBACK/pea-data/data1/amp_pop.bin", ser).unwrap();
        }
        if let Ok(ser) = bincode::serialize(&muni) {
            std::fs::write("/mnt/e/CHMBACK/pea-data/data1/mun_pop.bin", ser).unwrap();
        }
    }
    Ok(())
}

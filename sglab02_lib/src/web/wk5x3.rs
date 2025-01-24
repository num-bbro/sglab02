use crate::sg::{dcl, dcl::DaVa, /*ldp*/ ldp::base, uty::NumForm, wk5};
use askama::Template;
//use askama_axum;
//use axum::extract::{Path, Query};
//use regex::Regex;
use serde::{Deserialize, Serialize};
//use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
//use std::collections::{HashMap, HashSet};
use std::sync::Arc;
//use thousands::Separable;
use tokio::sync::RwLock;
use tokio::sync::{OwnedRwLockReadGuard, /*RwLockReadGuard*/};

#[derive(Template, Debug)]
#[template(path = "pg2/wk5x.html", escape = "none")]
pub struct ReportTemp {
    pub title: String,
    pub wk: OwnedRwLockReadGuard<wk5::Wk5Proc>,
}

fn rp(wk5prc: &wk5::Wk5Proc) -> &Report {
    &wk5prc.wk5x3
}
fn sp(wk5prc: &mut wk5::Wk5Proc, rp: Report) {
    wk5prc.wk5x3 = rp;
}

impl ReportTemp {
    pub fn repo(&self) -> &Report {
        &self.wk.wk5x3
    }
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>) -> Self {
        let wk = wk5prc.read_owned().await;
        let title = "LIST SUBSTATIONS".to_string();
        ReportTemp { wk, title }
    }
    pub fn cell(&self, r: &usize, c: &usize) -> String {
        let mut ce = rp(&self.wk).dava(&self.wk.ssv, *r, *c);
        if *c == 1 {
            if let DaVa::Text(v) = ce {
                let prov = &rp(&self.wk).rows[*r].prov;
                ce = DaVa::Text(format!(
                    "<a href='wk5x4/{}'>{}</a>",
                    prov,
                    v
                ));
			}
		} else if *c == 2 {
            if let DaVa::Text(v) = ce {
                let ssid = &rp(&self.wk).rows[*r].ssid;
                ce = DaVa::Text(format!(
                    "<a href='wk5x2/{}'>{}</a>",
                    ssid,
                    v
                ));
			}
		}
        match ce {
            DaVa::Text(s) => s,
            DaVa::F32(f) => f.form(),
            DaVa::F64(f) => f.form(),
            DaVa::USZ(u) => format!("{}", u),
            d => format!("{:?}", d),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Report {
    pub rows: Vec<RepoRow1>,
    pub cols: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoRow1 {
    pub s: usize, // substation
    pub f: usize, // feeder
	pub prov: String,
	pub ssid: String,
}

const TT: [&str; 3] = ["NO", "NAME", "SSID",];

pub async fn make_repo(wk5prc: &mut wk5::Wk5Proc, _acfg: Arc<RwLock<dcl::Config>>) {
    let mut repo = rp(wk5prc).clone();

    //let cfg = acfg.read().await;
    for t in TT {
        repo.cols.push(t.to_string());
    }
    for s in 0..wk5prc.ssv.len() {
        let mut rw = RepoRow1::default();
		rw.prov = wk5prc.ssv[s].prov.to_string();
		rw.ssid = wk5prc.ssv[s].ssid.to_string();
		repo.rows.push(rw);
	}
    repo.rows.sort_by(|a, b| {
		if a.prov == b.prov {
			a.ssid.partial_cmp(&b.ssid).unwrap()
		} else {
			a.prov.partial_cmp(&b.prov).unwrap()
		}
	});

	//repo.rows.push(rw_cash_flow);
	
    sp(wk5prc, repo);
}

impl Report {
    pub fn dava(&self, ssv: &Vec<wk5::Substation>, r: usize, c: usize) -> dcl::DaVa {
        let s = self.rows[r].s;
        let f = self.rows[r].f;
		let rw = &self.rows[r];
        let _ss = &ssv[s];
        let _fd = &ssv[s].feeders[f];
        match c {
            0 => DaVa::USZ(r + 1),
            1 => DaVa::Text(rw.prov.to_string()),
            2 => DaVa::Text(rw.ssid.to_string()),
            n => DaVa::F32(n as f32),
        }
    }
}

pub async fn handler() -> ReportTemp {
    ReportTemp::new(base().wk5prc.clone()).await
}


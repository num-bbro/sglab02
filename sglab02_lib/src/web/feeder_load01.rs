//use crate::sg::dcl;
use crate::sg::ldp::base;
use crate::sg::wk4;
use askama::Template;
//use askama_axum;
use axum::extract::{Path, /*Query*/};
//use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/feeder_load01.html")]
pub struct FeederLoad {
    pub ssid: String,
    pub fdid: String,
    pub year_load: wk4::YearLoad,
}
impl FeederLoad {
    #[allow(dead_code)]
    async fn new(wk4prc: Arc<RwLock<wk4::Wk4Proc>>, ssid: String, fdid: String) -> Self {
        let wk4prc = wk4prc.read().await;
        let mut year_load = wk4::YearLoad::default();
        for ss in &wk4prc.ssv {
            if ss.sbst == ssid {
                for fd in &ss.feeders {
                    if fd.feed == fdid {
                        year_load = fd.year_load.clone();
                        print!("year load {}\n", year_load.loads.len());
                    }
                }
            }
        }
        FeederLoad {
            ssid,
            fdid,
            year_load,
        }
    }
}

#[allow(dead_code)]
pub async fn handler(Path((ssid, fdid)): Path<(String, String)>) -> FeederLoad {
    let bs = base();
    FeederLoad::new(bs.wk4_ssv.clone(), ssid, fdid).await
}

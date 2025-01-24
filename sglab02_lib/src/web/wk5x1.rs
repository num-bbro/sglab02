use crate::sg::{dcl, dcl::DaVa, /*ldp*/ ldp::base, uty::NumForm, wk5};
use askama::Template;
//use askama_axum;
//use axum::extract::{Path, Query};
use regex::Regex;
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
    &wk5prc.wk5x1
}
fn sp(wk5prc: &mut wk5::Wk5Proc, rp: Report) {
    wk5prc.wk5x1 = rp;
}

impl ReportTemp {
    pub fn repo(&self) -> &Report {
        &self.wk.wk5x1
    }
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>) -> Self {
        let wk = wk5prc.read_owned().await;
        let title = "EV CAR PROJECTION : WK5X1";
        let title = title.to_string();

        ReportTemp { wk, title }
    }
    pub fn cell(&self, r: &usize, c: &usize) -> String {
        let ce = rp(&self.wk).dava(&self.wk.ssv, *r, *c);
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
	pub item: String,
	pub sum1: f32,
	pub sum2: f32,
	pub amts: Vec<f32>,
}

const TT: [&str; 4] = ["NO", "ITEM NAME ---------", "SUM1", "SUM2"];

pub async fn make_repo(wk5prc: &mut wk5::Wk5Proc, _acfg: Arc<RwLock<dcl::Config>>) {
    let mut repo = rp(wk5prc).clone();

    //let cfg = acfg.read().await;
    for t in TT {
        repo.cols.push(t.to_string());
    }
    let cfg = base().config.read().await;
    let syf = cfg.criteria.start_year_from_2022;
    let imy = cfg.criteria.implement_year;
    let opy = cfg.criteria.operate_year;
    let yrl = syf + imy + opy;
    let yrl = yrl as usize;
    for i in 0..yrl {
        let yr = 2022 + i + 1;
        repo.cols.push(format!("{}", yr));
    }
	let mut s_list = Vec::new();
    for s in 0..wk5prc.ssv.len() {
		if wk5prc.ssv[s].ssid!="BLA" { continue; }
		s_list.push(s);
	}
	
	let mut rw_solar_power = RepoRow1::default();
	rw_solar_power.item = "Solar power : MW".to_string();
	let mut rw_solar_day_energy = RepoRow1::default();
	rw_solar_day_energy.item = "Solar energy : MWH".to_string();
	let mut rw_solar_storage = RepoRow1::default();
	rw_solar_storage.item = "Solar storage : MWH".to_string();
	let mut rw_solar_revenue = RepoRow1::default();
	rw_solar_revenue.item = "Solar revenue : THB".to_string();
	let mut rw_storage_cost = RepoRow1::default();
	rw_storage_cost.item = "Storage cost : THB".to_string();
	let mut rw_trx_cost = RepoRow1::default();
	rw_trx_cost.item = "Tranx cost : THB".to_string();
	let mut rw_m1p_cost = RepoRow1::default();
	rw_m1p_cost.item = "Meter 1 cost : THB".to_string();
	let mut rw_m3p_cost = RepoRow1::default();
	rw_m3p_cost.item = "Meter 3 cost : THB".to_string();
	let mut rw_comm_cost = RepoRow1::default();
	rw_comm_cost.item = "Commu cost : THB".to_string();
	let mut rw_plat_cost = RepoRow1::default();
	rw_plat_cost.item = "Platform cost : THB".to_string();
	let mut rw_impl_cost = RepoRow1::default();
	rw_impl_cost.item = "Implement cost : THB".to_string();
	let mut rw_oper_cost = RepoRow1::default();
	rw_oper_cost.item = "Operation cost : THB".to_string();
	let mut rw_read_cost = RepoRow1::default();
	rw_read_cost.item = "Reading cost : THB".to_string();
	let mut rw_ev_car = RepoRow1::default();
	rw_ev_car.item = "EV car number".to_string();
	let mut rw_ev_power = RepoRow1::default();
	rw_ev_power.item = "EV Power : MW".to_string();
	let mut rw_ev_energy = RepoRow1::default();
	rw_ev_energy.item = "EV Energy : MWH".to_string();
	let mut rw_ev_revenue = RepoRow1::default();
	rw_ev_revenue.item = "EV Revenue : THB".to_string();

	let mut rw_oper_cost_trx = RepoRow1::default();
	rw_oper_cost_trx.item = "Operation trx : THB".to_string();
	let mut rw_oper_cost_m1p = RepoRow1::default();
	rw_oper_cost_m1p.item = "Operation m1p : THB".to_string();
	let mut rw_oper_cost_m3p = RepoRow1::default();
	rw_oper_cost_m3p.item = "Operation m3p : THB".to_string();
	let mut rw_oper_cost_ess = RepoRow1::default();
	rw_oper_cost_ess.item = "Operation ess : THB".to_string();

	let mut rw_fin_return = RepoRow1::default();
	rw_fin_return.item = "Fin return : THB".to_string();
	let mut rw_all_cost = RepoRow1::default();
	rw_all_cost.item = "All cost : THB".to_string();
	let mut rw_cash_flow = RepoRow1::default();
	rw_cash_flow.item = "Cash flow: THB".to_string();

	let mut rw_fin_45 = RepoRow1::default();
	rw_fin_45.item = "Fin 4.5% : THB".to_string();
	let mut rw_cost_45 = RepoRow1::default();
	rw_cost_45.item = "Cost 4.5% : THB".to_string();
	
	let mut rw_fin_10 = RepoRow1::default();
	rw_fin_10.item = "Fin 10% : THB".to_string();
	let mut rw_cost_10 = RepoRow1::default();
	rw_cost_10.item = "Cost 10% : THB".to_string();
	
	let mut rw_fin_15 = RepoRow1::default();
	rw_fin_15.item = "Fin 14.5% : THB".to_string();
	let mut rw_cost_15 = RepoRow1::default();
	rw_cost_15.item = "Cost 14.5% : THB".to_string();
	
	let mut rw_inve_ret = RepoRow1::default();
	rw_inve_ret.item = "Investor : THB".to_string();
	
	for i in 0..17 {
		rw_solar_power.amts.push(0f32);
		rw_solar_day_energy.amts.push(0f32);
		rw_solar_storage.amts.push(0f32);
		rw_solar_revenue.amts.push(0f32);
		rw_storage_cost.amts.push(0f32);
		rw_trx_cost.amts.push(0f32);
		rw_m1p_cost.amts.push(0f32);
		rw_m3p_cost.amts.push(0f32);
		rw_comm_cost.amts.push(0f32);
		rw_plat_cost.amts.push(0f32);
		rw_impl_cost.amts.push(0f32);
		rw_oper_cost.amts.push(0f32);
		rw_read_cost.amts.push(0f32);
		rw_ev_car.amts.push(0f32);
		rw_ev_power.amts.push(0f32);
		rw_ev_energy.amts.push(0f32);
		rw_ev_revenue.amts.push(0f32);
		rw_fin_return.amts.push(0f32);
		rw_all_cost.amts.push(0f32);
		rw_cash_flow.amts.push(0f32);
		rw_fin_45.amts.push(0f32);
		rw_cost_45.amts.push(0f32);
		rw_fin_10.amts.push(0f32);
		rw_cost_10.amts.push(0f32);
		rw_fin_15.amts.push(0f32);
		rw_cost_15.amts.push(0f32);

		rw_oper_cost_trx.amts.push(0f32);
		rw_oper_cost_m1p.amts.push(0f32);
		rw_oper_cost_m3p.amts.push(0f32);
		rw_oper_cost_ess.amts.push(0f32);

		let mut invr = 360000000f32;
		if i<5 { invr = 0f32; }
		rw_inve_ret.amts.push(invr);
		rw_all_cost.amts[i] += invr;
		rw_all_cost.sum1 += invr;
	}
    let re = Regex::new(r"..._[0-9][0-9].+").unwrap();
    for s in s_list {
        for f in 0..wk5prc.ssv[s].feeders.len() {
            let fd = &wk5prc.ssv[s].feeders[f];
            if re.is_match(fd.fdid.as_str()) {
                if fd.ev.ev_ds > 0.0 && fd.tx.tx_no > 0 {
					rw_trx_cost.sum2 += fd.tx.tx_no as f32;
					rw_m1p_cost.sum2 += fd.tx.mt1_no as f32;
					rw_m3p_cost.sum2 += fd.tx.mt3_no as f32;
					for i in 0..17 {
						rw_solar_power.amts[i] += fd.solar_power_series[i];
						rw_solar_day_energy.amts[i] += fd.solar_day_energy_series[i];
						rw_solar_storage.amts[i] += fd.solar_storage_series[i];
						rw_solar_revenue.amts[i] += fd.solar_revenue_series[i];
						rw_solar_revenue.sum1 += fd.solar_revenue_series[i];
						rw_storage_cost.amts[i] += fd.solar_storage_cost_series[i];
						rw_storage_cost.sum1 += fd.solar_storage_cost_series[i];
						rw_trx_cost.amts[i] += fd.smart_trx_cost_series[i];
						rw_trx_cost.sum1 += fd.smart_trx_cost_series[i];
						
						rw_m1p_cost.amts[i] += fd.smart_m1p_cost_series[i];
						rw_m1p_cost.sum1 += fd.smart_m1p_cost_series[i];

						rw_m3p_cost.amts[i] += fd.smart_m3p_cost_series[i];
						rw_m3p_cost.sum1 += fd.smart_m3p_cost_series[i];

						rw_comm_cost.amts[i] += fd.comm_cost_year_series[i];
						rw_comm_cost.sum1 += fd.comm_cost_year_series[i];

						rw_plat_cost.amts[i] += fd.platform_cost_series[i];
						rw_plat_cost.sum1 += fd.platform_cost_series[i];

						rw_impl_cost.amts[i] += fd.implement_cost_series[i];
						rw_impl_cost.sum1 += fd.implement_cost_series[i];

						rw_oper_cost.amts[i] += fd.operation_cost_series[i];
						rw_oper_cost.sum1 += fd.operation_cost_series[i];

						rw_read_cost.amts[i] += fd.meter_reading_cost_series[i];
						rw_read_cost.sum1 += fd.meter_reading_cost_series[i];
						
						rw_ev_car.amts[i] += fd.ev_car_series[i];
						rw_ev_car.sum1 += fd.ev_car_series[i];
						
						rw_ev_power.amts[i] += fd.ev_power_series[i];
						rw_ev_power.sum1 += fd.ev_power_series[i];
						
						rw_ev_energy.amts[i] += fd.ev_energy_series[i];
						rw_ev_energy.sum1 += fd.ev_energy_series[i];
						
						rw_ev_revenue.amts[i] += fd.ev_revenue_series[i];
						rw_ev_revenue.sum1 += fd.ev_revenue_series[i];
						
						rw_fin_return.amts[i] += fd.financial_benefit_series[i];
						rw_fin_return.sum1 += fd.financial_benefit_series[i];

						rw_all_cost.amts[i] += fd.total_cost_series[i];
						rw_all_cost.sum1 += fd.total_cost_series[i];

						rw_oper_cost_trx.amts[i] += fd.operation_cost_trx_series[i];
						rw_oper_cost_trx.sum1 += fd.operation_cost_trx_series[i];

						rw_oper_cost_m1p.amts[i] += fd.operation_cost_m1p_series[i];
						rw_oper_cost_m1p.sum1 += fd.operation_cost_m1p_series[i];

						rw_oper_cost_m3p.amts[i] += fd.operation_cost_m3p_series[i];
						rw_oper_cost_m3p.sum1 += fd.operation_cost_m3p_series[i];

						rw_oper_cost_ess.amts[i] += fd.operation_cost_ess_series[i];
						rw_oper_cost_ess.sum1 += fd.operation_cost_ess_series[i];

					}
				}
			}
		}
	}
	let rt45 = 0.045f32;
	let rt10 = 0.1f32;
	let rt15 = 0.145f32;
	for i in 0..15 {
		let t = i as f32 + 1f32;
		
		let deno : f32 = 1.0 + rt45;
		let deno : f32 = deno.powf(t);
		rw_fin_45.amts[i+2] = rw_fin_return.amts[i+2] / deno;
		rw_cost_45.amts[i+2] = rw_all_cost.amts[i+2] / deno;
		
		let deno : f32 = 1.0 + rt10;
		let deno : f32 = deno.powf(t);
		rw_fin_10.amts[i+2] = rw_fin_return.amts[i+2] / deno;
		rw_cost_10.amts[i+2] = rw_all_cost.amts[i+2] / deno;
		
		let deno : f32 = 1.0 + rt15;
		let deno : f32 = deno.powf(t);
		rw_fin_15.amts[i+2] = rw_fin_return.amts[i+2] / deno;
		rw_cost_15.amts[i+2] = rw_all_cost.amts[i+2] / deno;
	}
	for i in 0..17 {
		rw_cash_flow.amts[i] = rw_fin_return.amts[i] - rw_all_cost.amts[i];
		rw_cash_flow.sum1 += rw_cash_flow.amts[i];
		rw_fin_45.sum1 += rw_fin_45.amts[i];
		rw_cost_45.sum1 += rw_cost_45.amts[i];
		rw_fin_10.sum1 += rw_fin_10.amts[i];
		rw_cost_10.sum1 += rw_cost_10.amts[i];
		rw_fin_15.sum1 += rw_fin_15.amts[i];
		rw_cost_15.sum1 += rw_cost_15.amts[i];
	}
	repo.rows.push(rw_solar_power);
	repo.rows.push(rw_solar_day_energy);
	repo.rows.push(rw_solar_storage);
	repo.rows.push(rw_solar_revenue);
	
	repo.rows.push(rw_ev_car);
	repo.rows.push(rw_ev_power);
	repo.rows.push(rw_ev_energy);
	repo.rows.push(rw_ev_revenue);
	
	repo.rows.push(rw_storage_cost);
	repo.rows.push(rw_trx_cost);
	repo.rows.push(rw_m1p_cost);
	repo.rows.push(rw_m3p_cost);
	
	repo.rows.push(rw_oper_cost_trx);
	repo.rows.push(rw_oper_cost_m1p);
	repo.rows.push(rw_oper_cost_m3p);
	repo.rows.push(rw_oper_cost_ess);
	
	repo.rows.push(rw_comm_cost);
	repo.rows.push(rw_plat_cost);
	repo.rows.push(rw_impl_cost);
	repo.rows.push(rw_oper_cost);
	repo.rows.push(rw_inve_ret);
	repo.rows.push(rw_read_cost);
	
	repo.rows.push(rw_fin_return);
	repo.rows.push(rw_all_cost);
	
	repo.rows.push(rw_fin_45);
	repo.rows.push(rw_cost_45);
	repo.rows.push(rw_fin_10);
	repo.rows.push(rw_cost_10);
	repo.rows.push(rw_fin_15);
	repo.rows.push(rw_cost_15);

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
            1 => DaVa::Text(rw.item.to_string()),
            2 => DaVa::F32(rw.sum1),
            3 => DaVa::F32(rw.sum2),
            n => DaVa::F32(rw.amts[n - 4]),
        }
    }
}

pub async fn handler() -> ReportTemp {
    ReportTemp::new(base().wk5prc.clone()).await
}


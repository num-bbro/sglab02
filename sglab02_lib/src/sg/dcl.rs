use crate::sg::{ldp, wk1, wk2, wk3, wk4, wk5};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
//use std::sync::OnceLock;
//use tokio::sync::mpsc;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct BaseData {
    pub config: Arc<RwLock<Config>>,
    pub sbvc_2022: Arc<RwLock<Vec<String>>>,
    pub sbmp_2022: Arc<RwLock<HashMap<String, Vec<Box<FeederLoad>>>>>,
    pub sbvc_2021: Arc<RwLock<Vec<String>>>,
    pub sbmp_2021: Arc<RwLock<HashMap<String, Vec<Box<FeederLoad>>>>>,
    pub sbvc_2023: Arc<RwLock<Vec<String>>>,
    pub sbmp_2023: Arc<RwLock<HashMap<String, Vec<Box<FeederLoad>>>>>,
    pub wk1_load_prof_list: Arc<RwLock<wk1::LoadProfList>>,
    pub wk2_load_prof_list: Arc<RwLock<wk2::LoadProfList>>,
    pub ss_pv_mp: Arc<RwLock<HashMap<String, String>>>,
    pub wk3_subst: Arc<RwLock<Vec<wk3::Substation>>>,
    pub wk4_ssv: Arc<RwLock<wk4::Wk4Proc>>,
    pub wk5prc: Arc<RwLock<wk5::Wk5Proc>>,
    pub fd_tx_info: Arc<RwLock<ldp::FeederTranxInfo>>,
    pub ss_fd_ot: Arc<RwLock<HashMap<String, HashMap<String, Vec<(String, String, String)>>>>>,
    #[allow(dead_code)]
    pub pv_ca_mp: Arc<RwLock<HashMap<String, f64>>>,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub criteria: ConfigCriteria,
    pub residence: ConfigResidence,
    #[allow(dead_code)]
    pub industry: ConfigIndustry,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct ConfigCriteria {
    //pub solar_peak_ratio: f64,  //0.15
    #[allow(dead_code)]
    pub solar_factor: f64,      //6.0
    pub solar_time_window: f32, // 4.0
    pub operate_year: f32,
    pub bess_sell_per_mwh: f32,
	pub bess_sell_per_mw: f32, 
    pub energy_sale_price: f32, // 4000 Bath per MWh
    pub car_reg_bkk_province: String,
    pub car_reg_bkk_percent: String,
    pub car_reg_to_province: String,
    pub car_reg_to_percent: String,
    pub ev_car_reg_cnt: f32,
    pub start_year_from_2022: f32,
    pub implement_year: f32,
    pub ev_energy_unit_price: f32,
    pub vspp_energy_ratio_start: f32, // = 0.05
    pub vspp_energy_ratio_end: f32,   // = 0.10
    #[allow(dead_code)]
    pub energy_growth_rate: f32,      // = 0.05
    pub ev_growth_rate_start: f32,    //= 0.05
    pub ev_growth_rate_end: f32,      //= 0.08
    pub ev_car_all_reg: f32,          // = 100219.0
                                      //
    #[allow(dead_code)]
	pub ev_batt_power_ratio: f32, // 0.3
    #[allow(dead_code)]
	pub infra_invest_last_six_year: f32, //
    #[allow(dead_code)]
	pub infra_invest_per_year: f32, //
	pub ev_real_charge: f32, // 3.0
	pub expect_min_firr: f32,
	
	pub smart_trx_unit_cost: f32, // 100000
	pub smart_m1p_unit_cost: f32, // 8000
	pub smart_m3p_unit_cost: f32, // 12000
    #[allow(dead_code)]
	pub comm_per_devic_per_month: f32, // 12
	pub platform_cost_per_device: f32, // 1200
	pub implement_cost_per_device: f32, // 1000
    #[allow(dead_code)]
	pub operation_cost_per_year_device: f32, // 100
    #[allow(dead_code)]
	pub operation_cost_per_year: f32, // 100
	pub meter_reading_cost_cut: f32,
	pub economi_discount_rate: f32, // 0.08
	pub outage_operation_cost_per_hour: f32,
	pub loss_in_power_line_rate: f32,
	pub loss_in_phase_balance_rate: f32,
	
	pub operate_per_year_m1p: f32,
	pub operate_per_year_m3p: f32,
	pub operate_per_year_dtms: f32,
	pub operate_per_year_bess: f32,

    //
    #[allow(dead_code)]
    pub power_growth_rate: f32,         //1.217
    #[allow(dead_code)]
    pub predicted_ev_car: f32,          //155400.0
    pub evcharger_type1_pw: f32,        //0.022kW
    #[allow(dead_code)]
    pub industry_min_tranx_number: f32, //40
    #[allow(dead_code)]
    pub industry_min_meter_number: f32, //40
    #[allow(dead_code)]
    pub home_min_power: f32,            //800kW
    #[allow(dead_code)]
    pub ev_charger_min_power: f32,      //600kW
    #[allow(dead_code)]
    pub home_min_meter_number: f32,     //10
    #[allow(dead_code)]
    pub pea_min_firr: f32,              // = 4.26
    #[allow(dead_code)]
    pub pea_min_eirr: f32,              // = 10.0
    pub solar_energy_ratio: f32,
    pub bess_energy_max: f32,
	pub bess_power_max: f32, // 0.80 MW
    //pub solar_energy_ratio: f64,
	
	pub solar_bess_capacity_ratio: f32,
	pub meter_plc_per_sim_ratio: f32,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct ConfigResidence {
    #[allow(dead_code)]
    pub cost_soft_per_device: f64, // = 3000
    #[allow(dead_code)]
    pub cost_comm_per_device: f64, // = 1800
    #[allow(dead_code)]
    pub cost_oper_per_device: f64, // = 2000
    #[allow(dead_code)]
    pub influ_factor: f64,         // = 0.5
    pub sat_energy: f32,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct ConfigIndustry {
    #[allow(dead_code)]
    pub cost_soft_per_device: f64, // = 3000
    #[allow(dead_code)]
    pub cost_comm_per_device: f64, // = 1800
    #[allow(dead_code)]
    pub cost_oper_per_device: f64, // = 2000
    //pub influ_factor: f64,         // = 0.5
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum LoadProfVal {
    #[default]
    None,
    Null,
    Value(f32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubstLoad {
    feeders: Vec<Box<FeederLoad>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeederLoad {
    pub sbst: String,
    pub name: String,
    pub feed: String,
    pub time_r: Vec<LoadProfVal>,
    pub time_v: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum DaVa {
    #[default]
    None,
    F64(f64),
    F32(f32),
    I32(i32),
    I64(i64),
    USZ(usize),
    U32(u32),
    U64(u64),
    Text(String),
}

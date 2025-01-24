use tokio::join;
use tokio::spawn;

pub async fn run() {
    //let base = crate::sg::ldp::base();
    // read data
    let _ = join!(
        spawn(async { crate::sg::ldp::load_lpyd().await }),
        spawn(async { crate::sg::ldp::load_sspvmp().await }),
        spawn(async { crate::sg::ldp::load_txmtmp().await }),
    );

    // process data
    let _ = join!(
        //        spawn(async { crate::sg::wk1::run().await }),
        //        spawn(async { crate::sg::wk2::run().await }),
        //        spawn(async { crate::sg::wk3::run().await }),
        spawn(async { crate::sg::wk4::run().await }),
        //spawn(async { crate::sg::wk4::run3().await }),
    );
    print!("===========================\n");

    // serv web
    let _ = join!(spawn(async { crate::ws::srv::http_serve().await }));
}

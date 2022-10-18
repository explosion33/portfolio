use rocket::{
    self,
    Config,
};

#[rocket::get("/")]
fn test() -> &'static str {
    return "test!";
}


pub fn start_api() {
    rocket::tokio::runtime::Builder::new_multi_thread()
        .worker_threads(Config::from(Config::figment()).workers)
        // NOTE: graceful shutdown depends on the "rocket-worker" prefix.
        .thread_name("rocket-worker-thread")
        .enable_all()
        .build()
        .expect("create tokio runtime")
        .block_on(async move {
            let _ = rocket::build()
            .mount("/", rocket::routes![test])
            //.manage()
            .launch()
            .await;
        });
}
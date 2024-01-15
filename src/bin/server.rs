use rust_live_server::api::get_server_future;

#[tokio::main]
async fn main() {
    println!("server_start");
    #[cfg(debug_assertions)]
    {
        println!("Debug on");
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    get_server_future().await.unwrap();
}

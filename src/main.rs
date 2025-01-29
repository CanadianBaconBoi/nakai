use std::env;
use hostess_api::{banned, download, index};
use actix_files::Files as Fs;
use listenfd::ListenFd;
use actix_web::{
    middleware, web, App, HttpServer,
};

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_location = env::var("DATABASE_URL").unwrap_or("./nakai.db".into()); //.expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").unwrap_or("127.0.0.1".into());//.expect("HOST is not set in .env file");
    let port = env::var("PORT").unwrap_or("8080".into());//.expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .service(index)
            .service(banned)
            .service(download)
            .service(Fs::new("/static", "./api/static"))
            .wrap(middleware::Logger::default()) // enable logger
            .default_service(web::route().to(hostess_api::not_found))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
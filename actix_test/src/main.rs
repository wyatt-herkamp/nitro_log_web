use actix_web::{App, HttpServer, web};
use log::info;
use nitro_log::{LoggerBuilders, NitroLogger};
use nitro_log_web::WebLogger;

#[actix::main]
async fn main() -> std::io::Result<()> {
    let value = include_str!("test.json");
    let logger = NitroLogger::create(serde_json::from_str(value).unwrap(), LoggerBuilders::default()).unwrap();
    let web_log = WebLogger::new(logger).unwrap();
    info!("Hello, world!");
    HttpServer::new(move || {
        App::new()
            .service(web_log.clone())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

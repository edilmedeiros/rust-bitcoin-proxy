use crate::actions::test;
use crate::cli::Args;
use crate::routes::proxy;
use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use roxy::error::*;

pub async fn run(cli: &Args) -> Result<(), Error> {
    if cli.debug {
        env_logger::init();
    }
    // TODO: get cookie path via cli
    // TODO: share resources (DB, username, password) with services
    // let args: Vec<String> = env::args().collect();

    let client = test::run(cli).await?;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("config/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("config/cert.pem")
        .unwrap();

    let bind = String::from_iter([&cli.roxy_bind.clone(), ":", &cli.roxy_port.to_string()]);

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(client.clone()))
            .service(proxy::route)
    })
    .bind_openssl(bind, builder)?
    .run()
    .await;

    Ok(())
}

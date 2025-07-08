use crate::actions::test;
use crate::cli::Args;
use crate::routes::proxy;
use actix_web::{web, App, HttpServer};
use roxy::error::*;

pub async fn run(cli: &Args) -> Result<(), Error> {
    if cli.debug {
        env_logger::init();
    }
    // TODO: share resources (DB, username, password) with services
    // let args: Vec<String> = env::args().collect();

    // TODO: get cookie path via cli
    let client = test::run(cli).await?;

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(client.clone()))
            .service(proxy::route)
    })
    .bind((cli.roxy_bind.clone(), cli.roxy_port))?
    .run()
    .await;

    Ok(())
}

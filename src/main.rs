use actix_router::IntoPattern;
use actix_service::ServiceFactory;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    web,
    web::resource,
    App, HttpResponse, HttpServer, Route,
};
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const PATH: &str = "/foo";

    let server_loop = HttpServer::new(move || {
        App::new()
            .my_route(
                PATH,
                web::get().to(|| HttpResponse::Ok().finish()),
            )
            .my_route(
                PATH,
                web::put().to(|| HttpResponse::Ok().finish()),
            )
    })
        .bind("0.0.0.0:8800")?
        .run();
    server_loop.await?;
    Ok(())
}

trait AppExt<T, B> {
    fn my_route<P: IntoPattern>(self, path: P, route: Route) -> Self;
}

impl<T, B> AppExt<T, B> for App<T, B>
    where
        B: MessageBody,
        T: ServiceFactory<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse<B>,
            Error = actix_web::Error,
            InitError = (),
        >,
{
    fn my_route<P: IntoPattern>(self, path: P, route: Route) -> Self {
        self.service(resource(path).route(route))
    }
}

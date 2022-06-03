use std::future::Future;
use std::pin::Pin;
use std::process::Output;
use std::rc::Rc;
use std::task::{Context, Poll};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web::dev::{AppService, HttpServiceFactory, ResourceDef, ServiceRequest, ServiceResponse};
use actix_web_actors::ws;
use crate::WebLogger;
use actix::StreamHandler;
use actix::Actor;
use actix_service::{Service, ServiceFactory};
use actix_web::web::Data;
use futures_core::future::LocalBoxFuture;

pub struct WebLoggerService(Rc<WebLogger>);
impl Service<ServiceRequest> for WebLoggerService {
    type Response = ServiceResponse;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        todo!()
    }
}

impl ServiceFactory<ServiceRequest> for WebLogger {
    type Response = ServiceResponse;
    type Error = Error;
    type Config = ();
    type Service = WebLoggerService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        let logger = self.clone();

        Box::pin(async move {
            Ok(WebLoggerService(Rc::new(logger))) })
    }
}

impl HttpServiceFactory for WebLogger {
    fn register(self, config: &mut AppService) {
        config.register_service(ResourceDef::new("logger"), None, self, None);
    }
}

struct LoggerWebsocket {
    logger: web::Data<WebLogger>,
}

impl Actor for LoggerWebsocket {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LoggerWebsocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload, logger: web::Data<WebLogger>) -> Result<HttpResponse, Error> {
    let resp = ws::start(LoggerWebsocket { logger }, &req, stream);
    println!("{:?}", resp);
    resp
}
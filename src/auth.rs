use std::pin::Pin;
use std::task::{Context, Poll};

use sqlx::{Pool, Sqlite};

use actix_service::{Service, Transform};
use actix_web::dev::{MessageBody, Response};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web, Error, http, error};
use futures::future::{ok, Ready};
use futures::{Future, FutureExt};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct TokenAuthInit {
    database: Pool<Sqlite>,
}

impl TokenAuthInit {
    pub fn new(db: Pool<Sqlite>) -> Self {
        TokenAuthInit { database: db }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S: 'static + Clone, B> Transform<S, ServiceRequest> for TokenAuthInit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TokenAuth<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TokenAuth {
            service: service,
            database: self.database.clone(),
        })
    }
}

pub struct TokenAuth<S: Clone> {
    service: S,
    database: Pool<Sqlite>,
}

impl<S: 'static + Clone, B> Service<ServiceRequest> for TokenAuth<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> ,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        // return Box::pin(async {
        //     Err(actix_web::Error::from(error::InternalError::new("", http::StatusCode::UNAUTHORIZED)))
        // });

        let srv = self.service.clone();

        async move {
            let fut = srv.call(req);
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        }.boxed_local()
    }
}

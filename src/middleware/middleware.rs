use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::{env, rc::Rc};

use crate::utils::jwt::Claims;

pub struct Jwt;

impl<S, B> Transform<S, ServiceRequest> for Jwt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct JwtMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

        // 🧩 Try Authorization header first, then fallback to cookie
        let token_opt = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                if v.starts_with("Bearer ") {
                    Some(v.trim_start_matches("Bearer ").trim().to_string())
                } else {
                    None
                }
            })
            .or_else(|| req.cookie("jwt").map(|c| c.value().to_string()));

        let fut = if let Some(token) = token_opt {
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(data) => {
                    let user_id = data.claims.id.clone();
                    req.extensions_mut().insert(data.claims);
                    req.extensions_mut().insert(user_id);
                    Some(service.call(req))
                }
                Err(_) => None,
            }
        } else {
            None
        };

        Box::pin(async move {
            match fut {
                Some(f) => f.await,
                None => Err(actix_web::error::ErrorUnauthorized("Invalid or missing token")),
            }
        })
    }
}

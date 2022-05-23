use super::AppResponse;
use crate::{
    config::crypto::{Auth, CryptoService},

    errors::{AppErrorCode,AppError},
};

use actix_web::{web::Data,web::Form, FromRequest, HttpResponse,http::Cookie, dev::HttpResponseBuilder,web::Json};
use actix_web_httpauth::extractors::{bearer::BearerAuth};
use futures::future::{ready, BoxFuture};
use tracing::{debug, instrument};
use uuid::Uuid;
use serde::{Deserialize,Serialize};
use futures::future::{Ready};


#[derive(Debug)]
pub struct AuthenticatedUser(pub String);

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();
    #[instrument(skip(req,payload))]
    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        //println!("suis dans autenticated users, avant me");
        let bearer_result = BearerAuth::from_request(req, payload).into_inner();
    
        let crypto_service_result = Data::<CryptoService>::from_request(req, payload).into_inner();
        let cookie_result = CookieJWT::from_request(req , payload).into_inner();
        // let bearer_result2 = BearerAuth::from_request(req, payload).into_inner();
        // println!("bearer result : {:?}", bearer_result2.unwrap().token());
        let cookie_result2 = CookieJWT::from_request(req , payload).into_inner();
       // println!("{:?}" , &cookie_result2.unwrap().cookie_value);


                
        match (cookie_result, crypto_service_result) {
                (Ok(cookie), Ok(crypto_service)) => {
                    let future = async move {
                        let role: String = crypto_service

                            .verify_jwt(cookie.cookie_value)
                            .await
                            .map(|data| data.claims.sub)
                            .map_err(|err| { debug!("cannot verify jwt {:?}", err);
                                             AppError::NOT_AUTHORIZED})?;
                            Ok(AuthenticatedUser(role))
                    };
               
                    Box::pin(future)
                }
                
                _ => {
                        let error = ready(Err(AppError::NOT_AUTHORIZED.into()));
                        Box::pin(error)
                }
        }
    }
}

#[derive(Debug,Clone)]
pub struct CookieJWT {
    pub cookie_value: String,
}

impl FromRequest for CookieJWT {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    #[instrument(skip(req, payload))]
    fn from_request(    
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let cookie_result : std::result::Result<&actix_web::http::HeaderValue, u32> = req
                .headers()
                .get("cookie")
                .ok_or_else(||0);

        let cookie_value1 : Vec<&str> ;
        let cookie_to_send : CookieJWT;
        match req.headers().get("cookie") {
            Some(v) => {cookie_value1 = req
                                .headers()
                                .get("cookie")
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .split("=")
                                .collect();
                    cookie_to_send = CookieJWT {
                    cookie_value: cookie_value1[1].to_string()
                    };
                },
            None => return ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
        

        match cookie_result {
            Ok(cookie) => ready(Ok(
               cookie_to_send

            )),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
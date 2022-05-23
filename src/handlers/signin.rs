use super::AppResponse;
use super::{authentication::AuthenticatedUser, authentication::CookieJWT};
use crate::{
    config::crypto::{Auth, CryptoService},

    errors::{AppErrorCode,AppError},
};
use actix_web::cookie::Cookie;
use actix_web::{web::Payload,web::Data,web::Form, FromRequest, HttpResponse, HttpRequest,dev::HttpResponseBuilder,Responder,get,HttpMessage};
use futures::future::{ready, BoxFuture};
use tracing::{debug, instrument};
use uuid::Uuid;
use serde::{Deserialize,Serialize};


#[derive(Debug,Deserialize,Serialize)]
pub struct Inputlogin {
    pub username:String,
    pub password:String,
}
pub fn verify_auth_basic(username : String, password : String) -> u8{

    if (username == "patient" && password == "patient") {
        return 0
    }
    else if (username == "infirmiere" && password == "infirmiere"){
        return 1
    }
    else if (username == "docteur" && password == "docteur"){
        return 2
    }
    else {
        return 4
    }
}




pub async fn signin(
    basic : Form<Inputlogin>,

    hashing: Data<CryptoService>,
    req: HttpRequest) -> Result<actix_web::HttpResponse,actix_web::HttpResponse> {

        let username = &basic.username;
        //println!("{}", username);
        let password = &basic.password;
        
        println!("from signin : {}", password);
       let valid = verify_auth_basic(username.clone(), password.clone());
        
        match valid { 
            0 => {
                let token = hashing.generate_jwt(String::from("patient")).await.expect("err");
                            println!("{:?}",token);
                            let cookie = Cookie::new("JWT", &token);
                            println!("{}",&cookie);
                                                              
                            Ok(HttpResponse::Found()
                                                    .header("Location", "http://127.0.0.1:5500/thales_challenge/index.html")
                                                    .cookie(Cookie::build("JWT", &token)        
                                                    .finish())
                                                    .finish()
                                                
                                                )
            },
            1 => {
                let token = hashing.generate_jwt(String::from("infirmiere")).await.expect("err");
                println!("{:?}",token);
                let cookie = Cookie::new("JWT", &token);
                println!("{}",&cookie);
                                                  
                Ok(HttpResponse::Found()
                                        .header("Location", "http://127.0.0.1:5500/thales_challenge/index.html")
                                        .cookie(Cookie::build("JWT", &token)        
                                        .finish())
                                        .finish()
                                    
                                    )
            },
            2 => {
                let token = hashing.generate_jwt(String::from("docteur")).await.expect("err");
                println!("{:?}",token);
                let cookie = Cookie::new("JWT", &token);
                println!("{}",&cookie);
                                                  
                Ok(HttpResponse::Found()
                                        .header("Location", "http://127.0.0.1:5500/thales_challenge/index.html")
                                        .cookie(Cookie::build("JWT", &token)        
                                        .finish())
                                        .finish()
                                    
                                    )
            },
            _ =>  Err(HttpResponse::Found().header("Location", "http://127.0.0.1:5500/thales_challenge/login.html").header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","*").finish()),
                           
                          
    }

}

pub async fn verify_file_1(user : AuthenticatedUser) -> impl Responder{
    print!("dans file 1");
    print!("{:?}", user.0);
   if user.0 == String::from("patient") {
        HttpResponse::Ok().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("authorized")
   }
   else if user.0 == String::from("infirmiere") {
        HttpResponse::Created().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("authorized")
   }
   else if user.0 ==String::from("docteur") {
        HttpResponse::Accepted().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("authorized")
   }
   else {
        HttpResponse::Unauthorized().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("unauthorized")
   }
}

pub async fn verify_file_2(user : AuthenticatedUser) -> impl Responder{
    println!("dans file 2");
    println!("{:?}", user.0);
    if user.0 == String::from("patient") {
        HttpResponse::Unauthorized().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("unauthorized")
   }
   else if user.0 == String::from("infirmiere") {
        HttpResponse::Created().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("authorized")
   }
   else if user.0 ==String::from("docteur") {
        HttpResponse::Accepted().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("authorized")
   }
   else {
        HttpResponse::Unauthorized().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("unauthorized")
   }
}

pub async fn verify_file_3(user : AuthenticatedUser) -> impl Responder{
    println!("dans file 3");
    println!("{:?}", user.0);
    if user.0 == String::from("patient") {
        HttpResponse::Unauthorized().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("unauthorized")
   }
   else if user.0 == String::from("infirmiere") {
        HttpResponse::Unauthorized().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("unauthorized")
   }
   else if user.0 ==String::from("docteur") {
        HttpResponse::Accepted().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("authorized")
   }
   else {
        HttpResponse::Unauthorized().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","http://127.0.0.1:5500").header("Access-Control-Allow-Credentials","true").body("unauthorized")
   }
}
/*
  let token = hashing.generate_jwt(user.id).await.expect("err");
                            println!("{:?}",token);
                            let cookie = Cookie::new("JWT", &token);
                            println!("{}",&cookie);
                                                              
                            Ok(HttpResponse::Found()
                                                    .header("Location", "https://yoloooo.com")
                                                    .cookie(Cookie::build("JWT", &token) 
                                                    .secure(true)
                                                    .http_only(true)       
                                                    .finish())
                                                    .finish()
                                                
                                                )
                        }
                        else {
                            Err(HttpResponse::Found().header("Location", "https://yoloooo.com/front").header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","*").finish())
                            //Err(AppError::INVALID_CREDENTIALS.into())
                        }              
*/
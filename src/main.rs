#[macro_use]
extern crate clap;
use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

extern crate dotenv;

mod db;
use db::*;
mod myws;
pub mod pmodels;

async fn index() -> impl Responder {
     // let mut medi_index: Vec<CreateMedicine> = Vec::new();
     // medi_index.push(CreateMedicine {
     //      title: String::from("abc"),
     //      price: 1,
     //      id: Some(String::from("iiiiid")),
     //      descr: None,
     //      tags: None,
     // });
     let medi_index = get_medicines();

     HttpResponse::Ok().json(medi_index)
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateMedicine {
     id: Option<String>,
     title: String,
     descr: Option<String>,
     price: i32,
     tags: Option<Vec<String>>,
}

async fn create_medicine(medicine: web::Json<CreateMedicine>) -> impl Responder {
     println!("{:?}", medicine);
     let result = create_med(medicine.0.title.as_ref(), medicine.0.price);
     HttpResponse::Ok().body(result)
}

async fn publish_medicine(path: web::Path<String>) -> impl Responder {
     let result = publish_med(path.to_string());
     HttpResponse::Ok().json(result)
}

async fn medicine_ws(
     id: Identity,
     r: HttpRequest,
     stream: web::Payload,
) -> Result<HttpResponse, Error> {
     if let None = id.identity() {
          return Ok(HttpResponse::Unauthorized().finish());
     }
     println!("{:?}", r);
     let res = ws::start(myws::MyWebSocket::new(), &r, stream);
     println!("{:?}", res);
     res
}

#[derive(Serialize, Deserialize)]
pub struct LoginParams {
     name: String,
     passwd: String,
}

impl LoginParams {
     pub fn vali(&self) -> bool {
          self.name == self.passwd
     }
}

// async fn login(params: web::Form<LoginParams>, id: Identity) -> HttpResponse {
async fn login(params: web::Form<LoginParams>, id: Identity) -> HttpResponse {
     if params.deref().vali() {
          id.remember(params.name.to_owned());
          return HttpResponse::Found().header("location", "/").finish();
     }
     HttpResponse::Found().header("location", "/login").finish()
}

async fn logout(id: Identity) -> HttpResponse {
     id.forget();
     HttpResponse::Found().header("location", "/").finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
     std::env::set_var("RUST_LOG", "actix_web=info");
     env_logger::init();
     let matches = clap_app!(pharmacy =>
          (version: "0.1")
          (author: "m0ssc0de <hi.paul.q@gmail.com>")
          (about: "")
          (@subcommand server =>
               (about: "Run the server")
               (version: "0.1")
               (author: "m0ssc0de <hi.paul.q@gmail.com>")
               (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
               (@arg PORT: -p --port +takes_value "Sets a custom listen port")
          )
     )
     .get_matches();

     if let Some(matches) = matches.subcommand_matches("server") {
          let config_file = matches.value_of("CONFIG").unwrap_or("default.conf");
          println!("Printing specify config file {}", config_file);

          let port = matches.value_of("PORT").unwrap_or(":8080");
          println!("Printing specify port to listen {}", port);
     }

     let private_key = rand::thread_rng().gen::<[u8; 32]>();
     HttpServer::new(move || {
          App::new()
               .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&private_key)
                         .name("auth-example")
                         .secure(false),
               ))
               .wrap(middleware::Logger::default())
               .route("/log1n", web::post().to(login))
               .route("/log0ut", web::post().to(logout))
               .route("/medicine", web::get().to(index))
               .route("/medicine", web::post().to(create_medicine))
               .route("/medicine/{id}", web::put().to(publish_medicine))
               .route("/medicine/ws/", web::get().to(medicine_ws))
     })
     .bind("127.0.0.1:8088")?
     .run()
     .await
}

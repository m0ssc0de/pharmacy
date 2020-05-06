#[macro_use]
extern crate clap;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[macro_use]
extern crate serde_derive;

async fn index() -> impl Responder {
     let mut medi_index: Vec<CreateMedicine> = Vec::new();
     medi_index.push(CreateMedicine {
          name: String::from("abc"),
          price: 1,
          id: Some(String::from("iiiiid")),
          desc: None,
          tags: None,
     });
     HttpResponse::Ok().json(medi_index)
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateMedicine {
     id: Option<String>,
     name: String,
     desc: Option<String>,
     price: u64,
     tags: Option<Vec<String>>,
}

async fn create_medicine(medicine: web::Json<CreateMedicine>) -> impl Responder {
     println!("{:?}", medicine);
     HttpResponse::Ok().body("ook")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
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

     HttpServer::new(|| {
          App::new()
               .route("/medicine", web::get().to(index))
               .route("/medicine", web::post().to(create_medicine))
     })
     .bind("127.0.0.1:8088")?
     .run()
     .await
}

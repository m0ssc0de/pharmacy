#[macro_use]
extern crate clap;

fn main() {
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
}

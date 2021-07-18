use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use tera::{Tera, Context};

#[get("/")]
async fn greet() -> impl Responder {

    let tera = Tera::new("templates/*").unwrap();
    // tera.add_raw_template("templates/main.html.tera", "the body").unwrap();
    let template = tera.render("main.html.tera", &Context::new()).unwrap();
    // let template = tera.render_str(input, context)
    // println!("{}", hh);
    // let name = req.match_info().get("name").unwrap_or("World");
    // format!("{}", tera)
    //
    HttpResponse::Ok().body(template)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {


     // let mut templates = match Tera::new("templates/**/*") {
     //        Ok(t) => t,
     //        Err(e) => {
     //            println!("Parsing error(s): {}", e);
     //            ::std::process::exit(1);
     //        }
     //    };
     //    templates.autoescape_on(vec!["tera"]);
    

    HttpServer::new(|| {
        App::new()
            .service(greet)
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}



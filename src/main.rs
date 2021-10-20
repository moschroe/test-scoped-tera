#![deny(warnings)]

use actix_web::web::{resource, scope, Data, Path};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::collections::HashMap;
use tera::{Context, Tera, Value};

async fn greet(req: HttpRequest, tera: Data<Tera>) -> impl Responder {
    // this will not compile with
    // > error[E0277]: `Rc<actix_web::request::HttpRequestInner>` cannot be shared between threads safely
    // so a relaxed context has to be created, which is not Send, though!
    // let mut con = Context::new();
    let mut con = Context::new_relaxed();

    // this must be an owned value since a Context only has a 'static scope
    let name = req
        .match_info()
        .get("name")
        .map(String::from)
        .unwrap_or_else(|| String::from("World"));

    let req_move = req.clone();

    con.register_function("test_fn", move |args: &HashMap<_, _>| {
        let str_tpl = args.get("template").map(Value::as_str).unwrap().unwrap();
        let url = req_move.url_for("abcd", vec![&name]).unwrap();
        return Ok(Value::from(format!(
            "FN(tpl={:?}, name={:?}, url={})",
            str_tpl, name, url
        )));
    });

    let rendered = tera.render("test", &con).unwrap();

    // format!("Hello {}!", &name)
    rendered
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(create_tera())
            .route("/", web::get().to(greet))
            .service(resource("/{name}").to(greet))
            .service(
                scope("/magicscope").service(
                    resource("/other_url_with_parameter/{name}")
                        .name("abcd")
                        .to(|param: Path<String>| async move {
                            format!("Thank you for providing the following data: {:?}", param.0)
                        }),
                ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(8)
    .run()
    .await
}

fn create_tera() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "test",
        r##"TEXT START
{{ test_fn(template="STATIC TPL") }}
TEXT END"##,
    )
    .unwrap();
    tera
}

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("/assets/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}


pub fn render_template(
    template: &str,
    context: &mut tera::Context,
) -> HttpResponse {
    match TEMPLATES.render(template, context) {
        Ok(body) => HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::html())
            .body(body),
        Err(e) => HttpResponse::build(StatusCode::BAD_REQUEST)
            .content_type(ContentType::plaintext())
            .body(e.to_string()),
    }
}
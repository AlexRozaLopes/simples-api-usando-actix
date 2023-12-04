use actix_web::{get, post, web, Responder, Result};

use crate::{model::livro::Livro, service::livro_service};

#[get("/")]
pub async fn hello() -> Result<impl Responder> {
    Ok(web::Json(Livro::new("OI".to_string(), "OI".to_string())))
}

#[post("/")]
pub async fn post_livro(book_request: web::Json<Livro>) -> Result<impl Responder> {
    println!("Livro salvo");
    livro_service::create_livro(book_request.title.as_str(), book_request.author.as_str());
    Ok(web::Json(book_request))
}

#[get("/all")]
pub async fn get_all_livros() -> Result<impl Responder> {
    Ok(web::Json(livro_service::get_all_livros()))
}

pub mod model;
pub mod controller;
pub mod service;
pub mod schema;

use actix_web::{ App, HttpServer };
use controller::controller_livro::{hello, post_livro, get_all_livros};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
                    .service(hello)
                    .service(post_livro)
                    .service(get_all_livros)
                    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}



use std::env;

use diesel::{PgConnection, Connection, SelectableHelper, RunQueryDsl, prelude::Insertable};
use dotenvy::dotenv;

use crate::{model::livro::Livro, schema::livros};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("url do bando de dados nao encontrado");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error Connection to {}", database_url))

}

#[derive(Insertable)]
#[diesel(table_name=livros)]
pub struct NewLivro<'a> {
    pub title: &'a str,
    pub author: &'a str,
}

pub fn create_livro(title: &str, author: &str) -> Livro {


    let new_livro = NewLivro {title,author};

    diesel::insert_into(livros::table)
        .values(&new_livro)
        .returning(Livro::as_returning())
        .get_result(&mut establish_connection())
        .expect("erro ao salvar o livro")

}

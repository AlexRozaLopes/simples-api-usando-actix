use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = crate::schema::livros)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Livro {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Livro {
    pub fn new(title: String, author: String) -> Livro {
        Livro {id: 1, title, author, published:false}
    }
}

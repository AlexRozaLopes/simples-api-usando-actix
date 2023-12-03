use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = crate::schema::livros)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Livro {
    pub title: String,
    pub author: String,
}

impl Livro {
    pub fn new(title: String, author: String) -> Livro {
        Livro { title, author }
    }
}

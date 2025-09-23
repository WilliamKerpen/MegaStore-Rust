use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Produto {
    pub id: i32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

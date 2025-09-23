pub mod produto;
pub mod grafo;
pub mod recomendador;

pub use produto::Produto;
pub use grafo::{GrafoSimilaridade, construir_grafo, recomendar_por_grafo};
pub use recomendador::{recomendar_por_nome, recomendar_por_categoria};

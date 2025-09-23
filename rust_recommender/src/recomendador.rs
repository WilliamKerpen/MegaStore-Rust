use std::collections::{HashMap, HashSet};
use crate::Produto;

pub type GrafoSimilaridade = HashMap<i32, Vec<i32>>;

/// Normaliza um nome para uso em comparações
fn normalizar_nome(nome: &str) -> HashSet<String> {
    nome.trim()
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

/// Recomendação por nome (busca textual)
pub fn recomendar_por_nome(mapa: &HashMap<String, Produto>, termo: &str) -> Vec<Produto> {
    let termo_normalizado = termo.trim().to_lowercase();

    mapa.iter()
        .filter(|(nome, _)| nome.to_lowercase().contains(&termo_normalizado))
        .map(|(_, produto)| produto.clone())
        .take(4)
        .collect()
}

/// Recomendação por categoria (direta ou por produto relacionado)
pub fn recomendar_por_categoria(mapa: &HashMap<String, Produto>, termo: &str) -> Vec<Produto> {
    let termo_normalizado = termo.trim().to_lowercase();

    let categorias: HashSet<String> = mapa
        .values()
        .map(|p| p.categoria.to_lowercase())
        .collect();

    if categorias.contains(&termo_normalizado) {
        return mapa
            .values()
            .filter(|p| p.categoria.to_lowercase() == termo_normalizado)
            .cloned()
            .take(4)
            .collect();
    }

    if let Some(produto) = mapa.get(termo) {
        return mapa
            .values()
            .filter(|p| p.categoria == produto.categoria && p.nome != produto.nome)
            .cloned()
            .take(4)
            .collect();
    }

    vec![]
}

/// Constrói grafo de similaridade por nome e categoria
pub fn construir_grafo(mapa: &HashMap<String, Produto>) -> GrafoSimilaridade {
    let mut grafo = HashMap::new();

    for produto_a in mapa.values() {
        let palavras_a = normalizar_nome(&produto_a.nome);

        let similares: Vec<i32> = mapa
            .values()
            .filter(|produto_b| {
                produto_a.id != produto_b.id &&
                (
                    produto_a.categoria == produto_b.categoria ||
                    !palavras_a.is_disjoint(&normalizar_nome(&produto_b.nome))
                )
            })
            .map(|produto_b| produto_b.id)
            .take(5)
            .collect();

        grafo.insert(produto_a.id, similares);
    }

    grafo
}

/// Recomendação baseada no grafo
pub fn recomendar_por_grafo(
    grafo: &GrafoSimilaridade,
    mapa: &HashMap<String, Produto>,
    termo: &str,
) -> Vec<Produto> {
    let termo_normalizado = termo.trim().to_lowercase();

    if let Some(produto) = mapa.values().find(|p| p.nome.to_lowercase() == termo_normalizado) {
        if let Some(similares) = grafo.get(&produto.id) {
            return similares
                .iter()
                .filter_map(|id| mapa.values().find(|p| p.id == *id))
                .cloned()
                .collect();
        }
    }

    vec![]
}

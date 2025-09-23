use std::collections::{HashMap, HashSet};
use crate::Produto;

pub type GrafoSimilaridade = HashMap<i32, Vec<i32>>; // chave por ID

fn normalizar_nome(nome: &str) -> HashSet<String> {
    nome.trim()
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}


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

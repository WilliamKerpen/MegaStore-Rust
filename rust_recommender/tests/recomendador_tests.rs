use rust_recommender::{
    Produto,
    construir_grafo,
    recomendar_por_grafo,
    recomendar_por_nome,
    recomendar_por_categoria,
};

use std::collections::HashMap;

fn mock_produtos() -> HashMap<String, Produto> {
    let mut mapa = HashMap::new();

    mapa.insert("Geladeira Frost".to_string(), Produto {
        id: 1,
        nome: "Geladeira Frost".to_string(),
        categoria: "Eletrodoméstico".to_string(),
        preco: 1200.0,
    });

    mapa.insert("Geladeira Compacta".to_string(), Produto {
        id: 2,
        nome: "Geladeira Compacta".to_string(),
        categoria: "Eletrodoméstico".to_string(),
        preco: 1100.0,
    });

    mapa.insert("Fogão 4 bocas".to_string(), Produto {
        id: 3,
        nome: "Fogão 4 bocas".to_string(),
        categoria: "Eletrodoméstico".to_string(),
        preco: 800.0,
    });

    mapa.insert("Notebook Gamer".to_string(), Produto {
        id: 4,
        nome: "Notebook Gamer".to_string(),
        categoria: "Informática".to_string(),
        preco: 4500.0,
    });

    mapa
}

#[test]
fn test_recomendacao_por_nome() {
    let mapa = mock_produtos();
    let resultados = recomendar_por_nome(&mapa, "geladeira");

    assert_eq!(resultados.len(), 2);
    assert!(resultados.iter().any(|p| p.nome.contains("Frost")));
    assert!(resultados.iter().any(|p| p.nome.contains("Compacta")));
}

#[test]
fn test_recomendacao_por_categoria() {
    let mapa = mock_produtos();
    let resultados = recomendar_por_categoria(&mapa, "Eletrodoméstico");

    assert_eq!(resultados.len(), 3);
    assert!(resultados.iter().all(|p| p.categoria == "Eletrodoméstico"));
}

#[test]
fn test_recomendacao_por_grafo() {
    let mapa = mock_produtos();
    let grafo = construir_grafo(&mapa);
    let resultados = recomendar_por_grafo(&grafo, &mapa, "Geladeira Frost");

    assert!(resultados.len() >= 1);
    assert!(resultados.iter().any(|p| p.nome == "Geladeira Compacta"));
}

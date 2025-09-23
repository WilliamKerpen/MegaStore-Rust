use rusqlite::{Connection, params};
use rust_recommender::{
    Produto, construir_grafo, recomendar_por_grafo,
    recomendar_por_nome, recomendar_por_categoria,
};
use std::collections::HashMap;

fn setup_banco_em_memoria() -> HashMap<String, Produto> {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE produtos (
            id INTEGER PRIMARY KEY,
            nome TEXT NOT NULL,
            categoria TEXT,
            preco REAL
        )",
        [],
    ).unwrap();

    let produtos = vec![
        ("Geladeira Frost", "Eletrodoméstico", 1200.0),
        ("Geladeira Compacta", "Eletrodoméstico", 1100.0),
        ("Fogão 4 bocas", "Eletrodoméstico", 800.0),
        ("Notebook Gamer", "Informática", 4500.0),
    ];

    for (i, (nome, categoria, preco)) in produtos.into_iter().enumerate() {
        conn.execute(
            "INSERT INTO produtos (id, nome, categoria, preco) VALUES (?1, ?2, ?3, ?4)",
            params![i as i32 + 1, nome, categoria, preco],
        ).unwrap();
    }

    // Simula a função carregar_produtos
    let mut stmt = conn.prepare("SELECT id, nome, categoria, preco FROM produtos").unwrap();
    let produtos_iter = stmt.query_map([], |row| {
        Ok(Produto {
            id: row.get(0)?,
            nome: row.get(1)?,
            categoria: row.get(2)?,
            preco: row.get(3)?,
        })
    }).unwrap();

    let mut mapa = HashMap::new();
    for produto in produtos_iter {
        let p = produto.unwrap();
        mapa.insert(p.nome.clone(), p);
    }

    mapa
}

#[test]
fn test_carregamento_e_recomendacao_com_sqlite() {
    let mapa = setup_banco_em_memoria();
    assert_eq!(mapa.len(), 4);

    let grafo = construir_grafo(&mapa);
    let recomendados = recomendar_por_grafo(&grafo, &mapa, "Geladeira Frost");

    assert!(recomendados.iter().any(|p| p.nome == "Geladeira Compacta"));
}

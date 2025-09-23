use axum::{
    extract::Path,
    response::Json,
    routing::get,
    Router,
};
use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use axum::serve;

use rust_recommender::{
    Produto, GrafoSimilaridade,
    construir_grafo, recomendar_por_grafo,
    recomendar_por_nome, recomendar_por_categoria,
};

fn carregar_produtos(conn: &Connection) -> Result<HashMap<String, Produto>> {
    let mut stmt = conn.prepare("SELECT id, nome, categoria, preco FROM produtos")?;
    let produtos_iter = stmt.query_map([], |row| {
        Ok(Produto {
            id: row.get(0)?,
            nome: row.get(1)?,
            categoria: row.get(2)?,
            preco: row.get(3)?,
        })
    })?;

    let mut mapa = HashMap::new();
    for produto in produtos_iter {
        let p = produto?;
        mapa.insert(p.nome.clone(), p);
    }

    Ok(mapa)
}

async fn recomendar(
    Path(termo): Path<String>,
    mapa: Arc<HashMap<String, Produto>>,
    grafo: Arc<GrafoSimilaridade>,
) -> Json<Vec<Produto>> {
    let mut recomendados = recomendar_por_nome(&mapa, &termo);

    if recomendados.is_empty() {
        recomendados = recomendar_por_categoria(&mapa, &termo);
    }

    if recomendados.is_empty() {
        recomendados = recomendar_por_grafo(&grafo, &mapa, &termo);
    }

    Json(recomendados)
}

#[tokio::main]
async fn main() {
    let conn = Connection::open("../instance/database.db").expect("Erro ao abrir o banco");
    let mapa_raw = carregar_produtos(&conn).expect("Erro ao carregar produtos");
    let grafo_raw = construir_grafo(&mapa_raw);

    let mapa = Arc::new(mapa_raw);
    let grafo = Arc::new(grafo_raw);

    let app = Router::new()
        .route("/recomendar/:termo", get({
            let mapa = mapa.clone();
            let grafo = grafo.clone();
            move |path| recomendar(path, mapa, grafo)
        }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor Rust rodando em http://localhost:3000");

    serve(listener, app).await.unwrap();
}

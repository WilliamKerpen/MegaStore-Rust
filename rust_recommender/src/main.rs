use axum::{
    extract::Path,
    response::Json,
    routing::get,
    Router,
};
use rusqlite::{Connection, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use axum::serve;

#[derive(Debug, Clone, Serialize)]
struct Produto {
    id: i32,
    nome: String,
    categoria: String,
    preco: f64,
}

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

fn recomendar_por_nome(mapa: &HashMap<String, Produto>, termo: &str) -> Vec<Produto> {
    mapa.iter()
        .filter(|(nome, _)| nome.to_lowercase().contains(&termo.to_lowercase()))
        .map(|(_, produto)| produto.clone())
        .take(4)
        .collect()
}

fn recomendar_por_categoria(mapa: &HashMap<String, Produto>, categoria: &str) -> Vec<Produto> {
    mapa.values()
        .filter(|p| p.categoria == categoria)
        .cloned()
        .take(4)
        .collect()
}

async fn recomendar(Path(termo): Path<String>, mapa: Arc<HashMap<String, Produto>>) -> Json<Vec<Produto>> {
    let mut recomendados = recomendar_por_nome(&mapa, &termo);

    if recomendados.is_empty() {
        if let Some(produto) = mapa.get(&termo) {
            recomendados = recomendar_por_categoria(&mapa, &produto.categoria);
        }
    }

    Json(recomendados)
}

#[tokio::main]
async fn main() {
    let conn = Connection::open("../instance/database.db").expect("Erro ao abrir o banco");
    let mapa = Arc::new(carregar_produtos(&conn).expect("Erro ao carregar produtos"));

    let app = Router::new()
        .route("/recomendar/:termo", get({
            let mapa = mapa.clone();
            move |path| recomendar(path, mapa)
        }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor Rust rodando em http://localhost:3000");

    serve(listener, app).await.unwrap();
}
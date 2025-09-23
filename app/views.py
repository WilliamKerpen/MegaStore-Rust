import requests
from urllib.parse import quote
from unidecode import unidecode
from flask import render_template, request
from app import app, db
from app.models import Produtos


def obter_recomendacoes(produto_nome):
    termo = quote(produto_nome)
    url = f"http://localhost:3000/recomendar/{termo}"

    try:
        response = requests.get(url)
        if response.status_code == 200:
            dados = response.json()
            ids = [item.get("id") for item in dados if item.get("id")]
            recomendados = Produtos.query.filter(Produtos.id.in_(ids)).all()
            return recomendados
    except Exception as e:
        print("Erro ao conectar com o serviço Rust:", e)

    return []


@app.route('/', methods=['GET', 'POST'])
def index():
    termo = request.form.get('termo', '').strip()
    termo_normalizado = unidecode(termo.lower())

    if termo:
        todos = Produtos.query.all()
        produtos = [
            p for p in todos
            if termo_normalizado in unidecode(p.nome.lower()) or
               termo_normalizado in unidecode(p.categoria.lower())
        ]
    else:
        produtos = Produtos.query.order_by(Produtos.created_at.desc()).limit(8).all()

    return render_template(
        'index.html',
        produtos_recomendados=produtos,
        ultimas_pesquisas=[termo] if termo else []
    )


@app.route('/produto_details/<int:id>')
def produto_details(id):
    produto = Produtos.query.get_or_404(id)
    imagem = produto.imagens.filter_by(is_primary=True).first()

    recomendados = obter_recomendacoes(produto.nome)

    if not recomendados:
        recomendados = Produtos.query.filter(
            Produtos.categoria == produto.categoria,
            Produtos.id != produto.id
        ).order_by(Produtos.created_at.desc()).limit(4).all()

    return render_template(
        'produto.html',
        produto=produto,
        imagem=imagem,
        produtos_recomendados=recomendados
    )

import requests
from app import app, db
from flask import render_template, request
from sqlalchemy import or_
from app.models import Produtos

def obter_recomendacoes(produto_nome):
    url = f"http://localhost:3000/recomendar/{produto_nome}"
    try:
        response = requests.get(url)
        if response.status_code == 200:
            return response.json()
    except Exception as e:
        print("Erro ao conectar com o serviço Rust:", e)
    return []

@app.route('/', methods=['GET', 'POST'])
def index():
    termo = request.form.get('termo', '')
    
    if termo:
        produtos = Produtos.query.filter(
            or_(
                Produtos.nome.ilike(f'%{termo}%'),
                Produtos.categoria.ilike(f'%{termo}%')
            )
        ).all()
    else:
        produtos = Produtos.query.order_by(Produtos.created_at.desc()).limit(8).all()

    return render_template('index.html', produtos_recomendados=produtos, ultimas_pesquisas=[termo] if termo else [])

@app.route('/produto_details/<int:id>')
def produto_details(id):
    produto = Produtos.query.get_or_404(id)
    imagem = produto.imagens.filter_by(is_primary=True).first()

    # Tenta obter recomendação via Rust
    recomendados = obter_recomendacoes(produto.nome)

    # Se não houver recomendação, busca por categoria no banco Flask
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

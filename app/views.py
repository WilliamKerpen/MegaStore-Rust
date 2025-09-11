from app import app, db 
from flask import render_template, url_for, request, redirect, abort
from sqlalchemy import or_


from app.models import Produtos


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

@app.after_request
def aplicar_csp(response):
    response.headers['Content-Security-Policy'] = "script-src 'self';"
    return response
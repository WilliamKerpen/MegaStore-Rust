from app import db

'''
flask db init
flask db migrate -m ""
flask db upgrade
'''

from . import db
import datetime

class Produtos(db.Model):
    __tablename__ = "produtos"
    id = db.Column(db.Integer, primary_key=True)
    nome = db.Column(db.String(200), nullable=False)
    categoria = db.Column(db.String(100), nullable=True)
    quantidade = db.Column(db.Integer, nullable=True)
    descricao = db.Column(db.Text, nullable=True)
    preco = db.Column(db.Float, nullable=True)
    created_at = db.Column(db.DateTime, default=datetime.datetime.utcnow)

class ProdutoImagem(db.Model):
    __tablename__ = "produto_imagens"
    id = db.Column(db.Integer, primary_key=True)
    produto_id = db.Column(db.Integer, db.ForeignKey('produtos.id', ondelete='CASCADE'), nullable=False)
    filename = db.Column(db.String(300), nullable=False)   # arquivo salvo em app/static/uploads/
    mimetype = db.Column(db.String(50), nullable=True)
    is_primary = db.Column(db.Boolean, default=False)
    produto = db.relationship('Produtos', backref=db.backref('imagens', lazy='dynamic', cascade="all, delete-orphan"))
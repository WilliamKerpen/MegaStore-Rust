from app import app, db 
from flask import render_template, url_for, request, redirect, abort


from app.models import Produtos


@app.route('/', methods=['GET', 'POST'])
def homepage():
    return render_template('index.html')
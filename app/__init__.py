from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_migrate import Migrate
from dotenv import load_dotenv
import os

load_dotenv('.env')

app = Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = os.getenv('DATABASE_URI')
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
app.config['SECRET_KEY'] = os.getenv('SECRET_KEY')

db = SQLAlchemy(app)
migrate = Migrate(app, db)

# ✅ Política de segurança para permitir scripts locais
@app.after_request
def aplicar_csp(response):
    response.headers['Content-Security-Policy'] = "script-src 'self';"
    return response

from app import views
from app.models import Produtos
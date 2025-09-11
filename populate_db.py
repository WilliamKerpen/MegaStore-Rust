from app.models import db, Produtos, ProdutoImagem
from main import app  # importa o app principal
from faker import Faker
import random

CATEGORIES = [
    'Eletrônicos', 'Roupas', 'Livros', 'Móveis', 'Brinquedos',
    'Esporte', 'Beleza', 'Automotivo', 'Ferramentas', 'Casa'
]

def populate(n=2000, batch=200):
    fake = Faker('pt_BR')
    with app.app_context():
        db.create_all()  # garante que as tabelas existam
        to_add = []
        for i in range(n):
            p = Produtos(
                nome=fake.sentence(nb_words=3).replace('.', ''),
                categoria=random.choice(CATEGORIES),
                quantidade=random.randint(0, 200),
                descricao=fake.text(max_nb_chars=200),
                preco=round(random.uniform(5.0, 1000.0), 2)
            )
            db.session.add(p)
            db.session.flush()  # força criar o id sem commit ainda

            # adiciona imagem default
            img = ProdutoImagem(
                produto_id=p.id,
                filename="default.png",
                mimetype="image/png",
                is_primary=True
            )
            db.session.add(img)

            if (i + 1) % batch == 0:
                db.session.commit()
                print(f"Inseridos {i+1} produtos...")

        db.session.commit()
        print("População concluída!")

if __name__ == "__main__":
    populate(n=2000)  # ajusta a quantidade que quiser

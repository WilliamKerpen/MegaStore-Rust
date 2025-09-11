from app import create_app, db
from app.models import Produtos
from faker import Faker
import random

CATEGORIES = ['Eletrônicos','Roupas','Livros','Móveis','Brinquedos','Esporte','Beleza','Alimentos','Ferramentas','Casa']

def populate(n=10000, batch=500):
    fake = Faker('pt_BR')
    app = create_app()
    with app.app_context():
        db.create_all()
        to_add = []
        for i in range(n):
            p = Produtos(
                nome=fake.sentence(nb_words=3).replace('.', ''),
                categoria=random.choice(CATEGORIES),
                quantidade=random.randint(0, 200),
                descricao=fake.text(max_nb_chars=200),
                preco=round(random.uniform(5.0, 1000.0), 2)
            )
            to_add.append(p)
            if len(to_add) >= batch:
                db.session.add_all(to_add)
                db.session.commit()
                to_add = []
                print(f'Inseridos {i+1} produtos...')
        if to_add:
            db.session.add_all(to_add)
            db.session.commit()
        print('População concluída.')

if __name__ == '__main__':
    populate(n=10000)
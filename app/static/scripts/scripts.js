document.addEventListener('DOMContentLoaded', () => {
  const form = document.querySelector('form');
  const input = document.querySelector('input[name="termo"]');
  const container = document.getElementById('ultimas-pesquisas');

  // Salvar termo no localStorage
  function salvarPesquisa(termo) {
    let pesquisas = JSON.parse(localStorage.getItem('pesquisas')) || [];
    termo = termo.trim();
    if (termo && !pesquisas.includes(termo)) {
      pesquisas.unshift(termo);
      if (pesquisas.length > 5) pesquisas.pop();
      localStorage.setItem('pesquisas', JSON.stringify(pesquisas));
    }
  }

  // Exibir pesquisas na tela
  function mostrarPesquisas() {
    let pesquisas = JSON.parse(localStorage.getItem('pesquisas')) || [];
    container.innerHTML = '';
    pesquisas.forEach(p => {
      const col = document.createElement('div');
      col.className = 'col-md-3';
      col.innerHTML = `
        <div class="search-card text-center">
          <p>${p}</p>
        </div>
      `;
      container.appendChild(col);
    });
  }

  // Evento de envio do formulário
  if (form && input) {
    form.addEventListener('submit', () => {
      salvarPesquisa(input.value);
    });
  }

  // Inicializar exibição
  mostrarPesquisas();

  console.log("JS carregado com sucesso!");
});


function showProductDetail(productId) {
  // Simulação de dados
  const productData = {
    sofa: {
      image: "sofa-large.jpg",
      title: "Domusensen Sofá 2 lugares",
      description: "Sofá compacto com tecido teddy fleece, ideal para sala, quarto ou escritório.",
      price: "€151,14"
    }
  };

  const product = productData[productId];
  document.getElementById("detail-image").src = product.image;
  document.getElementById("detail-title").textContent = product.title;
  document.getElementById("detail-description").textContent = product.description;
  document.getElementById("detail-price").textContent = product.price;

  document.getElementById("product-detail").classList.remove("hidden");
}

function closeDetail() {
  document.getElementById("product-detail").classList.add("hidden");
}

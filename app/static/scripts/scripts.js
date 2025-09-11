// Salvar termo
function salvarPesquisa(termo) {
  let pesquisas = JSON.parse(localStorage.getItem('pesquisas')) || [];
  if (!pesquisas.includes(termo)) {
    pesquisas.unshift(termo);
    if (pesquisas.length > 5) pesquisas.pop();
    localStorage.setItem('pesquisas', JSON.stringify(pesquisas));
  }
}

// Exibir pesquisas
function mostrarPesquisas() {
  let pesquisas = JSON.parse(localStorage.getItem('pesquisas')) || [];
  const container = document.getElementById('ultimas-pesquisas');
  container.innerHTML = '';
  pesquisas.forEach(p => {
    container.innerHTML += `<div class="col-md-3"><div class="search-card text-center"><p>${p}</p></div></div>`;
  });
}

console.log("JS carregado com sucesso!");
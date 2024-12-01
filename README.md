# Gerenciador de Livros

Este é um sistema simples para gerenciar livros diretamente do terminal. Com ele, você pode adicionar, listar, remover e atualizar o status de livros. O projeto utiliza um banco de dados SQLite para armazenar as informações.

## Instalação

1. **Clone o repositório**:
   ```bash
   git clone https://github.com/vkskayo/2024-2-trabalho-topicos-especiais-rust.git
2. **Compile o projeto com o seguinte comando:**:
   ```bash
   cargo build

## Comandos

1. **Adicionar um Livro**:
   
   ```bash
   cargo run add-livro "Título do Livro" "2023-12-01" 300 "lido"
 Parâmetros:
* Título do Livro: O título do livro.
* 2023-12-01: A data de publicação do livro no formato YYYY-MM-DD.
* 300: O número de páginas do livro.
* lido: O status do livro. Pode ser um dos seguintes:
  * lido
  * naolido
  * emprogresso

2. **Listar Livros**:
   
   ```bash
   cargo run list-livro
4. **Remover um Livro**:
   
   ```bash
   cargo run remove-livro 1
 Parâmetros:
 * 1: O ID do livro que você deseja remover.
   
4. **Atualizar o Status de um Livro**:
   
   ```bash
   cargo run update-status-livro 1 "emprogresso"
Parâmetros:
* 1: O ID do livro cujo status será atualizado.
* emprogresso: O novo status do livro. Pode ser um dos seguintes:
  * lido
  * naolido
  * emprogresso

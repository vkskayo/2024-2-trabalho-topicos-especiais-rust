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
   cargo run add-livro "Título do Livro" "2023-12-01" 300 
 Parâmetros:
* Título do Livro: O título do livro.
* 2023-12-01: A data de publicação do livro no formato YYYY-MM-DD.
* 300: O número de páginas do livro.

2. **Listar Livros**:
   
   ```bash
   cargo run list-livro

3. **Remover um Livro**:
   
   ```bash
   cargo run remove-livro 1
 Parâmetros:
 * 1: O ID do livro que você deseja remover.
   
4. **Adicionar Usuário**:

 ```bash
   cargo run add-user "Usuário" 11122233344 
 Parâmetros:
* Usuário: O nome do usuário.
* 11122233344 : O cpf do usuário contendo 11 dígitos.

5. **Listar Usuários**:

 ```bash
   cargo run list-user

7. **Remover Usuário**:

 ```bash
   cargo run remove-user "Usuário" 11122233344 
 Parâmetros:
* Usuário: O nome do usuário que deseja remover.
* 11122233344 : O cpf do usuário contendo 11 dígitos que deseja remover.

8. **Relacionar Usuário com Livro**:
 
 ```bash
   cargo run relate-user-book 1 11122233344 "emprogresso"
 Parâmetros:
* 1: O ID do livro a ser relacionado com o usuário.
* 11122233344 : O cpf do usuário contendo 11 dígitos.
* emprogresso: O status do livro. Pode ser um dos seguintes:
  * lido
  * naolido
  * emprogresso


6. **Atualizar o Status de um Livro**:
   
   ```bash
   cargo run update-status-livro 1 11122233344 "lido"
Parâmetros:
* 1: O ID do livro cujo status será atualizado.
* 11122233344 : O cpf do usuário contendo 11 dígitos para acessar o livro que será atualizado.
* lido: O novo status do livro. Pode ser um dos seguintes:
  * lido
  * naolido
  * emprogresso

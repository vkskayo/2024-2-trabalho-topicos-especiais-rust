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
  
  ---
<p>&nbsp;</p>

2. **Listar Livros**:
   
   ```bash
   cargo run -- list-livro
   
  --- 
<p>&nbsp;</p>

3. **Remover um Livro**:
   
   ```bash
   cargo run -- remove-livro 1
 Parâmetros:
 * 1: O ID do livro que você deseja remover.
   
---
   <p>&nbsp;</p>
   
4. **Adicionar Usuário**:

   ```bash
   cargo run -- add-user "Usuário" 11122233344 
 Parâmetros:
* Usuário: O nome do usuário.
* 11122233344 : O cpf do usuário contendo 11 dígitos.
  
  ---
<p>&nbsp;</p>

5. **Listar Usuários**:

   ```bash
   cargo run -- list-user
   
---
<p>&nbsp;</p>

6. **Remover Usuário**:

   ```bash
   cargo run -- remove-user 1
 Parâmetros:
* 1 : O id do usuário que deseja remover.
  
---
  <p>&nbsp;</p>

7. **Relacionar Usuário com Livro**:
 
   ```bash
   cargo run -- user-add-livro 1 2 "emprogresso"
 Parâmetros:
* 1 : O id do usuário.
* 2: O ID do livro a ser relacionado com o usuário.
* emprogresso: O status do livro. Pode ser um dos seguintes:
  * lido
  * naolido
  * emprogresso
    
---
<p>&nbsp;</p>

8. **Atualizar o Status de um Livro**:
   
   ```bash
   cargo run -- update-status-livro 1 2 "lido"
Parâmetros:
* 1 : O id do usuário para acessar o livro que será atualizado.
* 2: O ID do livro cujo status será atualizado.
* lido: O novo status do livro. Pode ser um dos seguintes:
  * lido
  * naolido
  * emprogresso
    
---
<p>&nbsp;</p>

9. **Listar todas as relações**:

      ```bash
      cargo run -- list-relationship
      
 ---     
<p>&nbsp;</p>    

10. **Listar todos os livros de um usuário**:
   
      ```bash
      cargo run -- list-user-livro 1
   Parâmetros:
   * 1: O ID do usuário.


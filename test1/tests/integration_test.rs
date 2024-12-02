use rusqlite::Connection;
use test1::commands::{livro_command, user_command, user_livro_command};
//use test1::entities::status::Status;

#[test]
fn teste_fluxo_completo() {
    // Cria uma conexão em memória para os testes
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();

    // Criação das tabelas no banco de dados
    conn.execute_batch(
        "
        CREATE TABLE usuarios (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            cpf TEXT NOT NULL UNIQUE
        );

        CREATE TABLE livros (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL,
            data_publicacao TEXT NOT NULL,
            numero_paginas INTEGER NOT NULL
        );

        CREATE TABLE user_livro (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            livro_id INTEGER NOT NULL,
            status TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES usuarios(id),
            FOREIGN KEY(livro_id) REFERENCES livros(id)
        );
        ",
    )
    .unwrap();

    // Adiciona um usuário
    user_command::add_user(&conn, "João Silva", "12345678901");

    // Lista usuários para obter o ID
    println!("Usuários cadastrados:");
    user_command::list_users(&conn);

    // Adiciona um livro
    livro_command::add_livro(&conn, "Livro Exemplo", "2024-12-01", 300);

    // Lista livros para obter o ID
    println!("Livros cadastrados:");
    livro_command::list_livros(&conn);

    // Adiciona um relacionamento entre o usuário e o livro
    println!("Criando relacionamento entre usuário e livro...");
    user_livro_command::add_user_livro(&conn, 1, 1, "naolido");

    // Lista relacionamentos para validação
    println!("Relacionamentos cadastrados:");
    user_livro_command::list_relationship(&conn);

    // Atualiza o status do relacionamento
    println!("Atualizando status do relacionamento...");
    user_livro_command::update_status_user_livro(&conn, 1, 1, "lido");

    // Lista novamente para verificar a atualização
    println!("Relacionamentos após atualização:");
    user_livro_command::list_relationship(&conn);

    // Remove o livro
    println!("Removendo o livro...");
    livro_command::remove_livro(&conn, 1);

    // Lista livros novamente para verificar a remoção
    println!("Livros restantes após remoção:");
    livro_command::list_livros(&conn);

    // Tenta listar os relacionamentos novamente (deve falhar devido à remoção do livro)
    println!("Relacionamentos restantes após remoção do livro:");
    user_livro_command::list_relationship(&conn);

    println!("Teste de fluxo completo finalizado!");
}

use rusqlite::{Connection, Result};

/// Inicializa a conexão com o banco de dados SQLite.
/// 
/// Retorna uma conexão pronta para ser usada ou um erro caso a conexão falhe.
pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("database.db")?;

    // Criação de tabelas, se necessário
    conn.execute(
        "CREATE TABLE IF NOT EXISTS livros (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL,
            data_publicacao TEXT NOT NULL,
            numero_paginas INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS usuarios (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            cpf TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_livro (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            livro_id INTEGER NOT NULL,
            status TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES usuarios (id) ON DELETE CASCADE,
            FOREIGN KEY (livro_id) REFERENCES livros (id) ON DELETE CASCADE,
            UNIQUE(user_id, livro_id)
        )",
        [],
    )?;

    Ok(conn)
}

use rusqlite::{Connection, Result};

// Inicializa o banco de dados e cria a tabela se necessário
pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("livros.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS livros (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL,
            data_publicacao TEXT NOT NULL,
            numero_paginas INTEGER NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

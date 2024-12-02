use rusqlite::{Connection, Result};
use chrono::NaiveDate;
use crate::entities::livro::{Livro, Status};
use crate::entities::livro::listar_livros;

fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap(); // Cria banco em memória para testes
    conn.execute(
        "CREATE TABLE livros (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL,
            data_publicacao TEXT NOT NULL,
            numero_paginas INTEGER NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
    conn
}

#[test]
fn test_add_livro() {
    let conn = setup();
    let livro = Livro::new(
        "Livro de Teste".to_string(),
        NaiveDate::from_ymd(2024, 1, 1),
        100,
        Status::Lido,
    )
    .expect("Erro ao criar livro");

    livro.save(&conn).expect("Erro ao salvar livro");

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM livros WHERE titulo = ?1").unwrap();
    let count: i32 = stmt.query_row(&["Livro de Teste"], |row| row.get(0)).unwrap();
    assert_eq!(count, 1); // Verifica se há exatamente 1 livro com o título "Livro de Teste"
}

#[test]
fn test_list_livros() {
    let conn = setup();
    let livro = Livro::new(
        "Livro para Listagem".to_string(),
        NaiveDate::from_ymd(2024, 2, 1),
        200,
        Status::NaoLido,
    )
    .expect("Erro ao criar livro");

    livro.save(&conn).expect("Erro ao salvar livro");

    let livros = listar_livros(&conn).expect("Erro ao listar livros");

    assert_eq!(livros.len(), 1); // Verifica que um livro foi listado
    assert_eq!(livros[0].titulo, "Livro para Listagem"); // Verifica se o título do livro é o esperado
}


use crate::entities::{livro::Livro};
use rusqlite::{Connection};
use chrono::NaiveDate;
use crate::entities::livro::{listar_livros, remover_livro_por_id};

pub fn add_livro(conn: &Connection, titulo: &str, data_publicacao: &str, numero_paginas: i32) {

    // Converte a data para NaiveDate
    let data = match NaiveDate::parse_from_str(data_publicacao, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            eprintln!("Formato de data inválido. Use o formato: YYYY-MM-DD.");
            return;
        }
    };

    // Cria a entidade Livro
    let mut livro = match Livro::new(titulo.to_string(), data, numero_paginas) {
        Ok(livro) => livro,
        Err(e) => {
            eprintln!("Erro ao criar o livro: {}", e);
            return;
        }
    };

    // Salva o livro no banco de dados
    if let Err(e) = livro.save(conn) {
        eprintln!("Erro ao salvar o livro: {}", e);
    }
}

pub fn list_livros(conn: &Connection) {
    match listar_livros(conn) {
        Ok(livros) => {
            if livros.is_empty() {
                println!("Nenhum livro encontrado.");
            } else {
                for livro in livros {
                    println!("{:?}", livro.fmt_display());
                }
            }
        }
        Err(err) => {
            eprintln!("Erro ao listar livros: {}", err);
        }
    }
}

pub fn remove_livro(conn: &Connection, livro_id: i32) {
    match remover_livro_por_id(conn, livro_id) {
        Ok(_) => println!("Livro com ID {} removido com sucesso.", livro_id),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            eprintln!("Nenhum livro encontrado com o ID {}.", livro_id); // Mensagem de erro personalizada
        }
        Err(err) => eprintln!("Erro ao remover livro: {}", err),
    }
}








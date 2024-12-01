use chrono::NaiveDate;
use rusqlite::{Connection, Result};
use std::fmt;
use crate::entities::status::Status;

#[derive(Debug)]
pub struct Livro {
    pub id: Option<i32>,
    pub titulo: String,
    pub data_publicacao: NaiveDate,
    pub numero_paginas: i32,
    pub status: Status,
}

impl Livro {
    pub fn new(
        titulo: String,
        data_publicacao: NaiveDate,
        numero_paginas: i32,
        status: Status,
    ) -> Result<Self, String> {
        if titulo.is_empty() {
            return Err("Título não pode ser vazio.".to_string());
        }
        if numero_paginas <= 0 {
            return Err("Número de páginas deve ser maior que zero.".to_string());
        }

        Ok(Self {
            id: None,
            titulo,
            data_publicacao,
            numero_paginas,
            status,
        })
    }

    pub fn save(&mut self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO livros (titulo, data_publicacao, numero_paginas, status) 
             VALUES (?1, ?2, ?3, ?4)",
            (
                &self.titulo,
                &self.data_publicacao.to_string(),
                &self.numero_paginas,
                &self.status,
            ),
        )?;
        self.id = Some(conn.last_insert_rowid() as i32);
        Ok(())
    }

    // Implementando fmt::Display para customizar a impressão
    pub fn fmt_display(&self) -> String {
        format!(
            "ID: {}, Título: {}, Data de publicação: {}, Número de páginas: {}, Status: {:?}",
            self.id.unwrap_or_default(), // Exibe o ID, ou 0 caso seja None
            self.titulo,
            self.data_publicacao,
            self.numero_paginas,
            self.status
        )
    }
}

pub fn listar_livros(conn: &Connection) -> Result<Vec<Livro>> {
    let mut stmt = conn.prepare("SELECT id, titulo, data_publicacao, numero_paginas, status FROM livros")?;
    let livros = stmt
        .query_map([], |row| {
            Ok(Livro {
                id: row.get(0)?,
                titulo: row.get(1)?,
                data_publicacao: NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d")
    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                numero_paginas: row.get(3)?,
                status: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(livros)
}

 // Função para remover um livro por ID
 pub fn remover_livro_por_id(conn: &Connection, livro_id: i32) -> Result<()> {
    let rows_affected = conn.execute("DELETE FROM livros WHERE id = ?1", [livro_id])?;

    // Verifica se alguma linha foi afetada
    if rows_affected == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows); // Retorna erro se nenhuma linha for afetada
    }

    Ok(())
}

pub fn atualizar_status_por_id(
    conn: &Connection,
    livro_id: i32,
    novo_status: Status,
) -> Result<()> {
    let rows_affected = conn.execute(
        "UPDATE livros 
         SET status = ?1 
         WHERE id = ?2",
        (novo_status, livro_id),
    )?;

    // Verifica se alguma linha foi afetada
    if rows_affected == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows); // Retorna erro se nenhuma linha for afetada
    }

    Ok(())
}






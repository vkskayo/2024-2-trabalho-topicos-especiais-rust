use chrono::NaiveDate;
use rusqlite::{Connection, Result};
use crate::status::Status;

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

    pub fn atualizar(&self, conn: &Connection) -> Result<()> {
        if let Some(id) = self.id {
            conn.execute(
                "UPDATE livros 
                 SET titulo = ?1, data_publicacao = ?2, numero_paginas = ?3, status = ?4 
                 WHERE id = ?5",
                (
                    &self.titulo,
                    &self.data_publicacao.to_string(),
                    &self.numero_paginas,
                    &self.status,
                    id,
                ),
            )?;
            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    pub fn remover(self, conn: &Connection) -> Result<()> {
        if let Some(id) = self.id {
            conn.execute("DELETE FROM livros WHERE id = ?1", [id])?;
            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
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

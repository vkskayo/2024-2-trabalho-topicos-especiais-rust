use chrono::NaiveDate;
use rusqlite::{Connection, Result};
use crate::entities::livro::Livro;
use crate::entities::status::Status;

#[derive(Debug)]
pub struct User {
    pub id: Option<i32>,
    pub nome: String,
    pub cpf: String,
    pub livro_id: i32, // Relacionamento com o ID do Livro
    pub status: Status,
}

impl User {
    // Criação de um novo usuário
    pub fn new(nome: String, cpf: String, livro_id: i32, status: Status) -> Result<Self, String> {
        if nome.is_empty() {
            return Err("Nome não pode ser vazio.".to_string());
        }
        if cpf.is_empty() || cpf.len() != 11 || cpf.parse::<u64>().is_err() {
            return Err("CPF deve ter exatamente 11 dígitos numéricos.".to_string());
        }

        Ok(Self {
            id: None,
            nome,
            cpf,
            livro_id,
            status,
        })
    }

    // Salvar usuário no banco de dados
    pub fn save(&mut self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO usuarios (nome, cpf, livro_id, status) 
             VALUES (?1, ?2, ?3, ?4)",
            (&self.nome, &self.cpf, &self.livro_id, &self.status),
        )?;
        self.id = Some(conn.last_insert_rowid() as i32);
        Ok(())
    }

    // Listar usuários e os livros associados
    pub fn listar_usuarios(conn: &Connection) -> Result<Vec<(User, Livro)>> {
        let mut stmt = conn.prepare(
            "SELECT u.id, u.nome, u.cpf, u.livro_id, u.status, 
                    l.titulo, l.data_publicacao, l.numero_paginas, l.status 
             FROM usuarios u
             JOIN livros l ON u.livro_id = l.id",
        )?;

        let usuarios = stmt
            .query_map([], |row| {
                let livro = Livro {
                    id: row.get(3)?,
                    titulo: row.get(5)?,
                    data_publicacao: NaiveDate::parse_from_str(&row.get::<_, String>(6)?, "%Y-%m-%d")
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    numero_paginas: row.get(7)?,
                    status: row.get(8)?,
                };

                let user = User {
                    id: row.get(0)?,
                    nome: row.get(1)?,
                    cpf: row.get(2)?,
                    livro_id: row.get(3)?,
                    status: row.get(4)?,
                };

                Ok((user, livro))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(usuarios)
    }
}

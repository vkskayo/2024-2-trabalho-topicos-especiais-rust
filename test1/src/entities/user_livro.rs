use rusqlite::{Connection, Result};
use crate::entities::status::Status;

#[derive(Debug)]
pub struct UserLivro {
    pub id: Option<i32>,
    pub user_id: i32,   // Chave estrangeira para usuários
    pub livro_id: i32,  // Chave estrangeira para livros
    pub status: Status, // Status de leitura como enum
}

impl UserLivro {
    /// Cria um novo relacionamento UserLivro
    pub fn new(user_id: i32, livro_id: i32, status: Status) -> Result<Self, String> {
        Ok(Self {
            id: None,
            user_id,
            livro_id,
            status,
        })
    }

    /// Salva o relacionamento no banco de dados
    pub fn save(&mut self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO user_livro (user_id, livro_id, status) VALUES (?1, ?2, ?3)",
            (
                &self.user_id,
                &self.livro_id,
                &self.status,
            ),
        )?;
        self.id = Some(conn.last_insert_rowid() as i32);
        Ok(())
    }

    pub fn update_status(&mut self, conn: &Connection, novo_status: Status) -> Result<()> {
        // Atualiza o status no banco de dados
        conn.execute(
            "UPDATE user_livro SET status = ?1 WHERE user_id = ?2 AND livro_id = ?3",
            (novo_status.clone(), self.user_id, self.livro_id),
        )?;
        
        // Atualiza o status na instância local
        self.status = novo_status;
        Ok(())
    }

    /// Formata os dados para exibição amigável
    pub fn fmt_display(&self) -> String {
        format!(
            "ID: {}, User ID: {}, Livro ID: {}, Status: {:?}",
            self.id.unwrap_or_default(),
            self.user_id,
            self.livro_id,
            self.status
        )
    }
}

pub fn listar_relacionamento(conn: &Connection) -> Result<Vec<UserLivro>> {
    let mut stmt = conn.prepare("SELECT id, user_id, livro_id, status FROM user_livro")?;
    let user_livros_iter = stmt.query_map([], |row| {
        Ok(UserLivro {
            id: row.get(0)?,
            user_id: row.get(1)?,
            livro_id: row.get(2)?,
            status: row.get(3)?,  // Supondo que status seja um tipo enum ou string
        })
    })?;

    let mut user_livros = Vec::new();
    for user_livro in user_livros_iter {
        match user_livro {
            Ok(ul) => user_livros.push(ul),
            Err(e) => eprintln!("Erro ao recuperar UserLivro: {}", e),
        }
    }

    Ok(user_livros)
}

pub fn atualizar_status_user_livro(conn: &Connection, user_id: i32, livro_id: i32, novo_status: Status) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, livro_id, status FROM user_livro WHERE user_id = ?1 AND livro_id = ?2",
    )?;

    let user_livro_iter = stmt.query_map([user_id, livro_id], |row| {
        Ok(UserLivro {
            id: row.get(0)?,
            user_id: row.get(1)?,
            livro_id: row.get(2)?,
            status: row.get(3)?,
        })
    })?;

    // Verifica se o relacionamento foi encontrado
    let x = if let Some(user_livro) = user_livro_iter.into_iter().next() {
        let mut user_livro = user_livro?;
        // Atualiza o status
        user_livro.update_status(conn, novo_status)?;
        Ok(())
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    };x
}

pub fn listar_livros_por_usuario(conn: &Connection, user_id: i32) -> Result<(String, String, Vec<(String, String, String)>)> {
    let mut stmt = conn.prepare(
        "SELECT u.nome, u.cpf, l.titulo, l.data_publicacao, ul.status
         FROM usuarios u
         JOIN user_livro ul ON u.id = ul.user_id
         JOIN livros l ON l.id = ul.livro_id
         WHERE u.id = ?1",
    )?;

    // Aqui, garantimos que estamos coletando corretamente uma tupla de 3 Strings
    let livros: Vec<(String, String, String)> = stmt
        .query_map([user_id], |row| {
            let titulo: String = row.get(2)?;  // Pega a terceira coluna (título)
            let data_publicacao: String = row.get(3)?;  // Pega a quarta coluna (data_publicacao)
            let status: String = row.get(4)?;  // Pega a quinta coluna (status)
            Ok((titulo, data_publicacao, status))  // Retorna uma tupla com título, data_publicacao e status
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // Se não houver livros, retornamos um erro
    if livros.is_empty() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    // Obtendo o nome e CPF do usuário
    let nome: String = stmt.query_row([user_id], |row| row.get(0))?;
    let cpf: String = stmt.query_row([user_id], |row| row.get(1))?;

    // Retornando o nome, cpf e a lista de livros
    Ok((nome, cpf, livros))
}
use rusqlite::{Connection, Result, OptionalExtension};

#[derive(Debug)]
pub struct User {
    pub id: Option<i32>,
    pub nome: String,
    pub cpf: String,
}

impl User {
    // Criação de um novo usuário
    pub fn new(nome: String, cpf: String) -> Result<Self, String> {
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
        })
    }

    // Salvar usuário no banco de dados
    pub fn save(&mut self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO usuarios (nome, cpf) 
             VALUES (?1, ?2)",
            (&self.nome, &self.cpf),
        )?;
        self.id = Some(conn.last_insert_rowid() as i32);
        Ok(())
    }

    // Implementando fmt::Display para customizar a impressão
    pub fn fmt_display(&self) -> String {
        format!(
            "ID: {}, Nome: {}, CPF: {}",
            self.id.unwrap_or_default(), // Exibe o ID, ou 0 caso seja None
            self.nome,
            self.cpf
        )
    }

}

pub fn listar_usuarios(conn: &Connection) -> Result<Vec<User>> {
    let mut stmt = conn.prepare("SELECT id, nome, cpf FROM usuarios")?;
    let usuarios = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                nome: row.get(1)?,
                cpf: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(usuarios)
}

pub fn remover_usuario(conn: &Connection, user_id: i32) -> Result<()> {
    // Verifica se o usuário existe antes de tentar deletar
    let mut stmt = conn.prepare("SELECT id FROM usuarios WHERE id = ?1")?;
    let user_exists: Option<i32> = stmt.query_row([user_id], |row| row.get(0)).optional()?;

    if user_exists.is_none() {
        // Se o usuário não existir, retorna um erro personalizado
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    // Exclui o usuário da tabela 'usuarios' com base no ID
    conn.execute(
        "DELETE FROM usuarios WHERE id = ?1",
        [user_id],
    )?;
    Ok(())
}


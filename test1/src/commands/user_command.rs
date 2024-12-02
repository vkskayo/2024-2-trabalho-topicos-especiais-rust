
use rusqlite::{Connection, Result};
use crate::entities::user::User; 

// Adiciona um novo usuário ao banco de dados

pub fn add_user(conn: &Connection, nome: &str, cpf: &str, livro_id: i32, status: &str) {
    match User::new(nome.to_string(), cpf.to_string(), livro_id, status.to_string()) {
        Ok(mut user) => {
            if let Err(err) = user.save(conn) {
                eprintln!("Erro ao salvar usuário: {}", err);
            } else {
                println!("Usuário adicionado com sucesso!");
            }
        }
        Err(err) => eprintln!("Erro na criação do usuário: {}", err),
    }
}

// Lista todos os usuários e os livros associados
pub fn list_users(conn: &Connection) {
    match User::listar_usuarios(conn) {
        Ok(usuarios) => {
            for user in usuarios {
                println!(
                    "Usuário: {} (CPF: {}), Livro: {}, Status: {}",
                    user.nome, user.cpf, user.livro_titulo, user.status
                );
            }
        }
        Err(err) => eprintln!("Erro ao listar usuários: {}", err),
    }
}

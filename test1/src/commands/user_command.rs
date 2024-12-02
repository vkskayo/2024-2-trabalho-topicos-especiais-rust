use crate::entities::user::User;
use rusqlite::Connection;
use crate::entities::user::{listar_usuarios, remover_usuario};

pub fn add_user(conn: &Connection, nome: &str, cpf: &str) {
    // Criação do novo usuário
    match User::new(nome.to_string(), cpf.to_string()) {
        Ok(mut user) => {
            // Tenta salvar o usuário no banco de dados
            match user.save(conn) {
                Ok(_) => println!("Usuário adicionado com sucesso! ID: {:?}", user.id),
                Err(err) => eprintln!("Erro ao salvar o usuário no banco de dados: {}", err),
            }
        }
        Err(err) => {
            // Caso os dados do usuário sejam inválidos
            eprintln!("Erro ao criar o usuário: {}", err);
        }
    }
}

pub fn list_users(conn: &Connection) {
    match listar_usuarios(conn) {
        Ok(users) => {
            if users.is_empty() {
                println!("Nenhum usuário encontrado.");
            } else {
                for user in users {
                    println!("{:?}", user.fmt_display());
                }
            }
        }
        Err(err) => {
            eprintln!("Erro ao listar usuários: {}", err);
        }
    }
}

pub fn remove_user(conn: &Connection, user_id: i32) {
    // Tenta remover o usuário do banco de dados
    match remover_usuario(conn, user_id) {
        Ok(_) => println!("Usuário com ID {} removido com sucesso.", user_id),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            eprintln!("Nenhum usuário encontrado com o ID {}.", user_id);
        }
        Err(err) => eprintln!("Erro ao remover o usuário: {}", err),
    }
}


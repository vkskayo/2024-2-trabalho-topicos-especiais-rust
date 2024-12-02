use rusqlite::Connection;
use crate::entities::status::Status;
use crate::entities::user_livro::UserLivro;
use crate::entities::user_livro::{listar_relacionamento, atualizar_status_user_livro, listar_livros_por_usuario};

pub fn add_user_livro(conn: &Connection, user_id: i32, livro_id: i32, status: &str) {
    // Converte o status para o enum Status
    let status_enum = match status.to_lowercase().as_str() {
        "lido" => Status::Lido,
        "naolido" | "não lido" => Status::NaoLido,
        "emprogresso" | "em progresso" => Status::EmProgresso,
        _ => {
            eprintln!("Status inválido. Use: lido, naolido ou emprogresso.");
            return;
        }
    };

    // Cria a entidade UserLivro
    let mut user_livro = match UserLivro::new(user_id, livro_id, status_enum) {
        Ok(ul) => ul,
        Err(e) => {
            eprintln!("Erro ao criar o relacionamento UserLivro: {}", e);
            return;
        }
    };

   // Tenta salvar o relacionamento no banco de dados
   if let Err(e) = user_livro.save(conn) {
    match e {
        rusqlite::Error::SqliteFailure(ref sqlite_error, _) => {
            if sqlite_error.code == rusqlite::ffi::ErrorCode::ConstraintViolation {
                eprintln!("Erro: O relacionamento entre este usuário e este livro já existe.");
            } else {
                eprintln!("Erro ao salvar o relacionamento UserLivro: {}", e);
            }
        }
        _ => eprintln!("Erro ao salvar o relacionamento UserLivro: {}", e),
    }
    } else {
        println!("Relacionamento UserLivro salvo com sucesso:");
        println!("{}", user_livro.fmt_display());
    }
}

pub fn list_relationship(conn: &Connection) {
    match listar_relacionamento(conn) {
        Ok(user_livros) => {
            if user_livros.is_empty() {
                println!("Nenhum relacionamento encontrado.");
            } else {
                for user_livro in user_livros {
                    println!("{}", user_livro.fmt_display());  // Utilize o método fmt_display para exibir as informações de forma legível
                }
            }
        },
        Err(e) => eprintln!("Erro ao listar relacionamentos: {}", e),
    }
}

pub fn update_status_user_livro(conn: &Connection, user_id: i32, livro_id: i32, novo_status: &str) {
    // Converte o status para o enum Status
    let novo_status_enum = match novo_status.to_lowercase().as_str() {
        "lido" => Status::Lido,
        "naolido" => Status::NaoLido,
        "emprogresso" => Status::EmProgresso,
        _ => {
            eprintln!("Status inválido. Use: lido, naolido ou emprogresso.");
            return;
        }
    };

    // Tenta atualizar o status do relacionamento
    match atualizar_status_user_livro(conn, user_id, livro_id, novo_status_enum) {
        Ok(_) => println!("Status do relacionamento entre o usuário com ID {} e o livro com ID {} atualizado com sucesso.", user_id, livro_id),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            eprintln!("Nenhum relacionamento encontrado entre o usuário com ID {} e o livro com ID {}.", user_id, livro_id);
        }
        Err(err) => eprintln!("Erro ao atualizar o status do relacionamento: {}", err),
    }
}

pub fn list_livros_by_user(conn: &Connection, user_id: i32) {
    match listar_livros_por_usuario(conn, user_id) {
        Ok((nome, cpf, livros)) => {
            // Imprime as informações do usuário
            println!("Nome: {}", nome);
            println!("CPF: {}", cpf);
            println!("Livros do Usuário:");

            // Imprime os detalhes dos livros
            for (titulo, data_publicacao, status) in livros {
                println!("- Título: {}", titulo);
                println!("  Data de Publicação: {}", data_publicacao);
                println!("  Status: {}", status);
                println!("----------------------------");
            }
        }
        Err(e) => {
            eprintln!("Erro ao listar livros para o usuário com ID {}: {}", user_id, e);
        }
    }
}
/*
pub fn list_livros_by_user(conn: &Connection, user_id: i32) {
    match listar_livros_por_usuario(conn, user_id) {
        Ok((nome, cpf, livros)) => {
            println!("Usuário: {}", nome);
            println!("CPF: {}", cpf);
            if livros.is_empty() {
                println!("Nenhum livro relacionado a este usuário.");
            } else {
                println!("Livros relacionados:");
                for livro in livros {
                    println!("- {}", livro);
                }
            }
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            eprintln!("Nenhum usuário encontrado com o ID {}.", user_id);
        }
        Err(err) => eprintln!("Erro ao listar livros do usuário: {}", err),
    }
}
*/
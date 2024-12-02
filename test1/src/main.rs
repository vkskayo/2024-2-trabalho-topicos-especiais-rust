// src/main.rs
mod db;
mod commands;
mod entities;

use clap::{Parser, Subcommand};
//use entities::{livro::Livro, status::Status};
use commands::{
    livro_command::{add_livro, list_livros, remove_livro},
    user_command::{add_user, list_users, remove_user},
    user_livro_command::{add_user_livro, list_relationship, update_status_user_livro, list_livros_by_user}
};

use db::init_db;

#[derive(Parser)]
#[command(name = "Gerenciador de Livros")]
#[command(about = "Sistema para gerenciar livros diretamente do terminal", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AddLivro {
        titulo: String,
        data_publicacao: String,
        numero_paginas: i32,
    },
    ListLivro,
    RemoveLivro {
        id: i32,
    },
    AddUser {
        nome: String,
        cpf: String,
    },
    ListUser,
    RemoveUser {
        user_id: i32,
    },
    UserAddLivro {
        user_id: i32,
        livro_id: i32,
        status: String,
    },
    ListRelationship,
    UpdateStatusLivro {
        user_id: i32,
        livro_id: i32,
        status: String,
    },
    ListUserLivro {
        user_id: i32,
    },
}

fn main() {
    let cli = Cli::parse();

    // Inicializa o banco de dados
    let conn = init_db().expect("Erro ao conectar com o banco de dados.");

    match &cli.command {
        Commands::AddLivro { titulo, data_publicacao, numero_paginas} => {
            // Chama o comando para adicionar um livro, passando a conexão e os parâmetros brutos
            add_livro(&conn, titulo, data_publicacao, *numero_paginas);
        }
        Commands::ListLivro => {
            // Chama o comando para listar livros
            list_livros(&conn);
        }
        Commands::RemoveLivro { id } => {
            // Chama o comando para remover um livro
            remove_livro(&conn, *id);
        }

        //Chama comandos para adicionar usuários e listar livros associados
        Commands::AddUser { nome, cpf
        } => {
            add_user(&conn, nome, cpf);
        }
        
        Commands::ListUser => {
            list_users(&conn);
        }
        Commands::RemoveUser { user_id } => {
           
            remove_user(&conn, *user_id);
        }

        Commands::UserAddLivro { user_id, livro_id, status
        } => {
            add_user_livro(&conn, *user_id, *livro_id, status);
        }

        Commands::ListRelationship => {
            list_relationship(&conn);
        }
        Commands::UpdateStatusLivro { user_id, livro_id, status
        } => {
            update_status_user_livro(&conn, *user_id, *livro_id, status);
        }
        Commands::ListUserLivro { user_id } => {
           
            list_livros_by_user(&conn, *user_id);
        }
        
    }
}

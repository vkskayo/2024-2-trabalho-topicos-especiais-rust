// src/main.rs
mod db;
mod commands;
mod entities;

use clap::{Parser, Subcommand};
//use entities::{livro::Livro, status::Status};
use commands::{
    livro_command::{add_livro, list_livros, remove_livro, update_status_livro},
    user_command::{add_user, list_users}
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
        status: String,
    },
    ListLivro,
    RemoveLivro {
        id: i32,
    },
    UpdateStatusLivro {
        id: i32,
        status: String,
    },
    AddUser {
        nome: String,
        cpf: String,
        livro_id: i32,
        status: String,
    },
    ListUser,
}

fn main() {
    let cli = Cli::parse();

    // Inicializa o banco de dados
    let conn = init_db().expect("Erro ao conectar com o banco de dados.");

    match &cli.command {
        Commands::AddLivro { titulo, data_publicacao, numero_paginas, status } => {
            // Chama o comando para adicionar um livro, passando a conexão e os parâmetros brutos
            add_livro(&conn, titulo, data_publicacao, *numero_paginas, status);
        }
        Commands::ListLivro => {
            // Chama o comando para listar livros
            list_livros(&conn);
        }
        Commands::RemoveLivro { id } => {
            // Chama o comando para remover um livro
            remove_livro(&conn, *id);
        }
        Commands::UpdateStatusLivro { id, status } => {
            // Chama o comando para atualizar o status de um livro
            update_status_livro(&conn, *id, status);
        }
        
        //Chama comandos para adicionar usuários e listar livros associados
        Commands::AddUser { nome, cpf, livro_id, status 
        } => {
            add_user(&conn, nome, cpf, *livro_id, status);
        }
        Commands::ListUser => {
            list_users(&conn);
        }
            
    }
}

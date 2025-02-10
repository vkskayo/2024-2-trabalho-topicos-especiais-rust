#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod biblioteca {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::prelude::string::ToString;

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct User {
        pub id: u32,
        pub nome: String,
        pub cpf: String,
        pub idade: u32,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Livro {
        pub id: u32,
        pub titulo: String,
        pub data_publicacao: String,
        pub numero_paginas: u32,
    }

    #[ink(storage)]
    pub struct Biblioteca {
        users: Mapping<u32, User>,
        livros: Mapping<u32, Livro>,
        next_user_id: u32,
        next_livro_id: u32,
    }

    impl Default for Biblioteca {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Biblioteca {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                users: Mapping::default(),
                livros: Mapping::default(),
                next_user_id: 0,
                next_livro_id: 0,
            }
        }

        // ----- CRUD para Users -----

        #[ink(message)]
        pub fn criar_user(&mut self, nome: String, cpf: String, idade: u32) -> Result<u32, String> {
            if nome.is_empty() || cpf.is_empty() {
                return Err("Nome e CPF não podem estar vazios".to_string());
            }

            let id = self.next_user_id;
            let user = User { id, nome, cpf, idade };
            self.users.insert(id, &user);
            self.next_user_id = self.next_user_id.checked_add(1).expect("Overflow");
            Ok(id)
        }

        #[ink(message)]
        pub fn ler_user(&self, id: u32) -> Option<User> {
            self.users.get(id)
        }

        #[ink(message)]
        pub fn atualizar_user(&mut self, id: u32, nome: String, cpf: String, idade: u32) -> Result<bool, String> {
            if let Some(mut user) = self.users.get(id) {
                if nome.is_empty() || cpf.is_empty() {
                    return Err("Nome e CPF não podem estar vazios".to_string());
                }

                user.nome = nome;
                user.cpf = cpf;
                user.idade = idade;
                self.users.insert(id, &user);
                Ok(true)
            } else {
                Err("User não encontrado".to_string())
            }
        }

        #[ink(message)]
        pub fn deletar_user(&mut self, id: u32) -> bool {
            if self.users.contains(id) {
                self.users.remove(id);
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn listar_users(&self) -> Vec<User> {
            let mut lista = Vec::new();
            for id in 0..self.next_user_id {
                if let Some(user) = self.users.get(id) {
                    lista.push(user);
                }
            }
            lista
        }

        // ----- CRUD para Livros -----

        #[ink(message)]
        pub fn criar_livro(&mut self, titulo: String, data_publicacao: String, numero_paginas: u32) -> Result<u32, String> {
            if titulo.is_empty() || data_publicacao.is_empty() {
                return Err("Título e data de publicação não podem estar vazios".to_string());
            }

            let id = self.next_livro_id;
            let livro = Livro { id, titulo, data_publicacao, numero_paginas };
            self.livros.insert(id, &livro);
            self.next_livro_id = self.next_livro_id.checked_add(1).expect("Overflow");
            Ok(id)
        }

        #[ink(message)]
        pub fn ler_livro(&self, id: u32) -> Option<Livro> {
            self.livros.get(id)
        }

        #[ink(message)]
        pub fn atualizar_livro(&mut self, id: u32, titulo: String, data_publicacao: String, numero_paginas: u32) -> Result<bool, String> {
            if let Some(mut livro) = self.livros.get(id) {
                if titulo.is_empty() || data_publicacao.is_empty() {
                    return Err("Título e data de publicação não podem estar vazios".to_string());
                }

                livro.titulo = titulo;
                livro.data_publicacao = data_publicacao;
                livro.numero_paginas = numero_paginas;
                self.livros.insert(id, &livro);
                Ok(true)
            } else {
                Err("Livro não encontrado".to_string())
            }
        }

        #[ink(message)]
        pub fn deletar_livro(&mut self, id: u32) -> bool {
            if self.livros.contains(id) {
                self.livros.remove(id);
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn listar_livros(&self) -> Vec<Livro> {
            let mut lista = Vec::new();
            for id in 0..self.next_livro_id {
                if let Some(livro) = self.livros.get(id) {
                    lista.push(livro);
                }
            }
            lista
        }
    }
}
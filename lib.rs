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
        pub livros_emprestados: Vec<u32>, // IDs dos livros emprestados
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
        pub emprestado_para: Option<u32>, // ID do User que possui o livro
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
            if nome.is_empty() {
                return Err("Nome não pode estar vazio".to_string());
            }

            if !Self::cpf_valido(&cpf) {
                return Err("CPF inválido. Deve conter exatamente 11 dígitos numéricos".to_string());
            }

            let id = self.next_user_id;
            let user = User { id, nome, cpf, idade, livros_emprestados: Vec::new() };
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
                if nome.is_empty() {
                    return Err("Nome não pode estar vazio".to_string());
                }

                if !Self::cpf_valido(&cpf) {
                    return Err("CPF inválido. Deve conter exatamente 11 dígitos numéricos".to_string());
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
        pub fn deletar_user(&mut self, id: u32) -> Result<bool, String> {
            if self.users.contains(id) {
                self.users.remove(id);
                Ok(true)
            } else {
                Err("Usuário não encontrado".to_string())
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
            if titulo.is_empty() {
                return Err("Título não pode estar vazio".to_string());
            }

            if !Self::data_valida(&data_publicacao) {
                return Err("Data de publicação inválida. O formato deve ser dd-mm-yyyy".to_string());
            }

            let id = self.next_livro_id;
            let livro = Livro { id, titulo, data_publicacao, numero_paginas, emprestado_para: None };
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
                if titulo.is_empty() {
                    return Err("Título não pode estar vazio".to_string());
                }

                if !Self::data_valida(&data_publicacao) {
                    return Err("Data de publicação inválida. O formato deve ser dd-mm-yyyy".to_string());
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
        pub fn deletar_livro(&mut self, id: u32) -> Result<bool, String> {
            if self.livros.contains(id) {
                self.livros.remove(id);
                Ok(true)
            } else {
                Err("Livro não encontrado".to_string())
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

        // ----- Relacionamento User-Livro -----

        #[ink(message)]
        pub fn emprestar_livro(&mut self, user_id: u32, livro_id: u32) -> Result<bool, String> {
            let mut user = self.users.get(user_id).ok_or("Usuário não encontrado")?;
            let mut livro = self.livros.get(livro_id).ok_or("Livro não encontrado")?;

            if livro.emprestado_para.is_some() {
                return Err("Livro já está emprestado".to_string());
            }

            livro.emprestado_para = Some(user_id);
            user.livros_emprestados.push(livro_id);

            self.livros.insert(livro_id, &livro);
            self.users.insert(user_id, &user);
            Ok(true)
        }

        #[ink(message)]
        pub fn devolver_livro(&mut self, user_id: u32, livro_id: u32) -> Result<bool, String> {
            let mut user = self.users.get(user_id).ok_or("Usuário não encontrado")?;
            let mut livro = self.livros.get(livro_id).ok_or("Livro não encontrado")?;

            if livro.emprestado_para != Some(user_id) {
                return Err("Livro não está emprestado para este usuário".to_string());
            }

            livro.emprestado_para = None;
            user.livros_emprestados.retain(|&id| id != livro_id);

            self.livros.insert(livro_id, &livro);
            self.users.insert(user_id, &user);
            Ok(true)
        }

        // ----- Validações -----

        fn cpf_valido(cpf: &str) -> bool {
            cpf.len() == 11 && cpf.chars().all(|c| c.is_ascii_digit())
        }

        fn data_valida(data: &str) -> bool {
            let partes: Vec<&str> = data.split('-').collect();
    
            if partes.len() != 3 {
                return false;
            }
    
            if !partes.iter().all(|parte| parte.chars().all(|c| c.is_ascii_digit())) {
                return false;
            }
    
            let dia: usize = partes[0].parse().unwrap_or(0);
            let mes: usize = partes[1].parse().unwrap_or(0);
            let ano: usize = partes[2].parse().unwrap_or(0);
    
            if dia == 0 || mes == 0 || mes > 12 || ano == 0 {
                return false;
            }
    
            let dias_por_mes = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
            let dias_mes = if mes == 2 && Self::ano_bissexto(ano) {
                29
            } else {
                dias_por_mes[mes.saturating_sub(1)]
            };
    
            dia <= dias_mes
        }

        // Verifica se um ano é bissexto
        fn ano_bissexto(ano: usize) -> bool {
            (ano % 4 == 0 && ano % 100 != 0) || (ano % 400 == 0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_criar_user_valido() {
            let mut biblioteca = Biblioteca::new();
            let nome = "Vinicius Kayo de Souza".to_string();
            let cpf = "19345738028".to_string();
            let idade = 25;

            let result = biblioteca.criar_user(nome.clone(), cpf.clone(), idade);
            assert!(result.is_ok(), "Falha ao criar usuário");

            let id = result.unwrap();
            let user = biblioteca.ler_user(id).expect("O usuário deve existir");
            assert_eq!(user.nome, nome);
            assert_eq!(user.cpf, cpf);
            assert_eq!(user.idade, idade);
        }

        #[ink::test]
        fn test_criar_user_cpf_invalido() {
            let mut biblioteca = Biblioteca::new();
            let nome = "Vinicius Kayo de Souza".to_string();
            let cpf = "1423".to_string(); // CPF inválido
            let idade = 25;

            let result = biblioteca.criar_user(nome, cpf, idade);
            assert!(result.is_err(), "Usuário não deve ser criado com CPF inválido");
        }

        #[ink::test]
        fn test_deletar_user_existente() {
            let mut biblioteca = Biblioteca::new();
            let nome = "Vinicius Kayo de Souza".to_string();
            let cpf = "12345678901".to_string();
            let idade = 25;

            let id = biblioteca.criar_user(nome, cpf, idade).unwrap();

            let result = biblioteca.deletar_user(id);
            assert!(result.is_ok(), "Falha ao deletar usuário existente");
            assert!(biblioteca.ler_user(id).is_none(), "Usuário ainda existe após a exclusão");
        }

        #[ink::test]
        fn test_deletar_user_inexistente() {
            let mut biblioteca = Biblioteca::new();

            let result = biblioteca.deletar_user(999);
            assert!(result.is_err(), "Deleção de usuário inexistente não deve ser bem-sucedida");
            assert_eq!(result.unwrap_err(), "Usuário não encontrado");
        }

        #[ink::test]
        fn test_emprestar_livro_valido() {
            let mut biblioteca = Biblioteca::new();

            // Cria usuário e livro
            let user_id = biblioteca.criar_user("João Silva".to_string(), "12345678901".to_string(), 25).unwrap();
            let livro_id = biblioteca.criar_livro("Rust Programming".to_string(), "01-01-2023".to_string(), 500).unwrap();

            let result = biblioteca.emprestar_livro(user_id, livro_id);
            assert!(result.is_ok(), "Falha ao emprestar livro válido");

            let livro = biblioteca.ler_livro(livro_id).expect("O livro deve existir");
            assert_eq!(livro.emprestado_para, Some(user_id));
        }

        #[ink::test]
        fn test_devolver_livro_valido() {
            let mut biblioteca = Biblioteca::new();

            // Cria usuário e livro, realiza empréstimo
            let user_id = biblioteca.criar_user("João Silva".to_string(), "12345678901".to_string(), 25).unwrap();
            let livro_id = biblioteca.criar_livro("Rust Programming".to_string(), "01-01-2023".to_string(), 500).unwrap();
            biblioteca.emprestar_livro(user_id, livro_id).unwrap();

            let result = biblioteca.devolver_livro(user_id, livro_id);
            assert!(result.is_ok(), "Falha ao devolver livro válido");

            let livro = biblioteca.ler_livro(livro_id).expect("O livro deve existir");
            assert_eq!(livro.emprestado_para, None);
        }

        #[ink::test]
        fn test_atualizar_user_valido() {
            let mut biblioteca = Biblioteca::new();

            // Cria usuário
            let id = biblioteca.criar_user("João Silva".to_string(), "12345678901".to_string(), 25).unwrap();

            let result = biblioteca.atualizar_user(id, "Maria Silva".to_string(), "09876543210".to_string(), 30);
            assert!(result.is_ok(), "Falha ao atualizar usuário válido");

            let user = biblioteca.ler_user(id).expect("O usuário deve existir");
            assert_eq!(user.nome, "Maria Silva");
            assert_eq!(user.cpf, "09876543210");
            assert_eq!(user.idade, 30);
        }

        #[ink::test]
        fn test_atualizar_user_inexistente() {
            let mut biblioteca = Biblioteca::new();

            let result = biblioteca.atualizar_user(999, "Maria Silva".to_string(), "09876543210".to_string(), 30);
            assert!(result.is_err(), "Atualização de usuário inexistente não deve ser bem-sucedida");
            assert_eq!(result.unwrap_err(), "User não encontrado");
        }
    }
}
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct Contato {
    pub nome: String,
    pub telefone: String,
    pub email: Option<String>,
    pub endereco: Option<String>,
}

pub struct Agenda {
    contatos_por_nome: HashMap<String, Contato>,
    contatos_por_telefone: HashMap<String, Contato>,
    lista_contatos: Vec<Contato>,
}

impl Agenda {
    pub fn novo() -> Self {
        let mut agenda = Agenda {
            contatos_por_nome: HashMap::new(),
            contatos_por_telefone: HashMap::new(),
            lista_contatos: Vec::new(),
        };

        agenda.carregar_de_arquivo();
        agenda
    }

    pub fn adicionar_contato(&mut self, contato: Contato) {
        self.contatos_por_nome.insert(contato.nome.clone(), contato.clone());
        self.contatos_por_telefone.insert(contato.telefone.clone(), contato.clone());
        self.lista_contatos.push(contato);
        self.salvar_em_arquivo();
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Contato> {
        self.contatos_por_nome.get(nome)
    }

    pub fn mostrar_todos_nomes(&self) -> Vec<Contato> {
        self.lista_contatos.clone()
    }

    pub fn buscar_por_telefone(&self, telefone: &str) -> Option<&Contato> {
        self.contatos_por_telefone.get(telefone)
    }

    pub fn remover_por_nome(&mut self, nome: &str) -> Option<Contato> {
        if let Some(contato) = self.contatos_por_nome.remove(nome) {
            self.contatos_por_telefone.remove(&contato.telefone);
            self.lista_contatos.retain(|c| c.nome != nome);
            self.salvar_em_arquivo();
            Some(contato)
        } else {
            None
        }
    }

    pub fn remover_por_telefone(&mut self, telefone: &str) -> Option<Contato> {
        if let Some(contato) = self.contatos_por_telefone.remove(telefone) {
            self.contatos_por_nome.remove(&contato.nome);
            self.lista_contatos.retain(|c| c.telefone != telefone);
            self.salvar_em_arquivo();
            Some(contato)
        } else {
            None
        }
    }

    fn salvar_em_arquivo(&self) {
        let mut arquivo = File::create("agenda.txt").expect("Não foi possível criar o arquivo");

        for contato in &self.lista_contatos {
            let linha = format!(
                "{},{},{},{}\n",
                contato.nome,
                contato.telefone,
                contato.email.clone().unwrap_or_default(),
                contato.endereco.clone().unwrap_or_default()
            );
            arquivo.write_all(linha.as_bytes()).expect("Erro ao escrever no arquivo");
        }
    }

    fn carregar_de_arquivo(&mut self) {
        let arquivo = File::open("agenda.txt");

        if let Ok(arquivo) = arquivo {
            let reader = BufReader::new(arquivo);

            for linha in reader.lines() {
                if let Ok(linha) = linha {
                    let partes: Vec<&str> = linha.split(',').collect();
                    if partes.len() == 4 {
                        let contato = Contato {
                            nome: partes[0].to_string(),
                            telefone: partes[1].to_string(),
                            email: if partes[2].is_empty() { None } else { Some(partes[2].to_string()) },
                            endereco: if partes[3].is_empty() { None } else { Some(partes[3].to_string()) },
                        };
                        self.adicionar_contato(contato);
                    }
                }
            }
        }
    }
}

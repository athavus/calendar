use serde::{Serialize, Deserialize};
use std::fs::{File};
use std::io::{BufReader, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contato {
    pub nome: String,
    pub telefone: String,
    pub email: Option<String>,
    pub endereco: Option<String>,
}

pub struct Agenda {
    contatos: Vec<Contato>,
}

impl Agenda {
    pub fn novo() -> Self {
        let contatos = Self::carregar_de_arquivo("contatos.json").unwrap_or_else(|_| Vec::new());
        Self { contatos }
    }

    pub fn adicionar_contato(&mut self, contato: Contato) {
        self.contatos.push(contato);
        self.salvar_em_arquivo("contatos.json").expect("Erro ao salvar contatos");
    }

    pub fn mostrar_todos_nomes(&self) -> Vec<Contato> {
        self.contatos.clone()
    }

    pub fn remover_por_nome(&mut self, nome: &str) {
        self.contatos.retain(|contato| contato.nome != nome);
        self.salvar_em_arquivo("contatos.json").expect("Erro ao salvar contatos");
    }

    fn salvar_em_arquivo(&self, caminho: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.contatos)?;
        let mut file = File::create(caminho)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn carregar_de_arquivo(caminho: &str) -> std::io::Result<Vec<Contato>> {
        if Path::new(caminho).exists() {
            let file = File::open(caminho)?;
            let reader = BufReader::new(file);
            let contatos = serde_json::from_reader(reader)?;
            Ok(contatos)
        } else {
            Ok(Vec::new())
        }
    }
}

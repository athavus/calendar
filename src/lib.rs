use std::collections::HashMap;

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
        Agenda {
            contatos_por_nome: HashMap::new(),
            contatos_por_telefone: HashMap::new(),
            lista_contatos: Vec::new(),
        }
    }

    pub fn adicionar_contato(&mut self, contato: Contato) {
        self.contatos_por_nome.insert(contato.nome.clone(), contato.clone());
        self.contatos_por_telefone.insert(contato.telefone.clone(), contato.clone());
        self.lista_contatos.push(contato);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Contato> {
        self.contatos_por_nome.get(nome)
    }

    pub fn mostrar_todos_nomes(&self) -> Vec<&Contato> {
        self.lista_contatos.values().collect()
    }
    

    pub fn buscar_por_telefone(&self, telefone: &str) -> Option<&Contato> {
        self.contatos_por_telefone.get(telefone)
    }

    pub fn remover_por_nome(&mut self, nome: &str) -> Option<Contato> {
        if let Some(contato) = self.contatos_por_nome.remove(nome) {
            self.contatos_por_telefone.remove(&contato.telefone);
            self.lista_contatos.retain(|c| c.nome != nome);
            Some(contato)
        } else {
            None
        }
    }

    pub fn remover_por_telefone(&mut self, telefone: &str) -> Option<Contato> {
        if let Some(contato) = self.contatos_por_telefone.remove(telefone) {
            self.contatos_por_nome.remove(&contato.nome);
            self.lista_contatos.retain(|c| c.telefone != telefone);
            Some(contato)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adicionar_contato() {
        let mut agenda = Agenda::novo();
        let contato = Contato {
            nome: "João".to_string(),
            telefone: "123456789".to_string(),
            email: Some("joao@example.com".to_string()),
            endereco: None,
        };
        agenda.adicionar_contato(contato.clone());
        assert_eq!(agenda.buscar_por_nome("João"), Some(&contato));
    }

    #[test]
    fn test_remover_contato_por_nome() {
        let mut agenda = Agenda::novo();
        let contato = Contato {
            nome: "Maria".to_string(),
            telefone: "987654321".to_string(),
            email: Some("maria@example.com".to_string()),
            endereco: None,
        };
        agenda.adicionar_contato(contato.clone());
        let removido = agenda.remover_por_nome("Maria");
        assert_eq!(removido, Some(contato));
        assert!(agenda.buscar_por_nome("Maria").is_none());
    }

    #[test]
    fn test_remover_contato_por_telefone() {
        let mut agenda = Agenda::novo();
        let contato = Contato {
            nome: "Pedro".to_string(),
            telefone: "555555555".to_string(),
            email: Some("pedro@example.com".to_string()),
            endereco: Some("Rua A, 123".to_string()),
        };
        agenda.adicionar_contato(contato.clone());
        let removido = agenda.remover_por_telefone("555555555");
        assert_eq!(removido, Some(contato));
        assert!(agenda.buscar_por_telefone("555555555").is_none());
    }
}

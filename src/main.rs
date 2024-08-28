use std::io::{self, Write};
use agenda::{Agenda, Contato};

fn main() {
    let mut agenda = Agenda::novo();

    loop {
        println!("Escolha uma opção:");
        println!("1. Adicionar contato");
        println!("2. Mostrar todos os contatos");
        println!("3. Buscar contato por nome");
        println!("4. Buscar contato por telefone");
        println!("5. Remover contato por nome");
        println!("6. Remover contato por telefone");
        println!("7. Sair");        


        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();
        let escolha: u32 = escolha.trim().parse().unwrap();
        
        match escolha {
            1 => {
                let nome = ler_entrada("Nome:");
                let telefone = ler_entrada("Telefone:");
                let email = ler_entrada("Email (deixe em branco se não quiser):");
                let endereco = ler_entrada("Endereço (deixe em branco se não quiser):");

                let contato = Contato {
                    nome: nome.clone(),
                    telefone: telefone.clone(),
                    email: if email.is_empty() { None } else { Some(email) },
                    endereco: if endereco.is_empty() { None } else { Some(endereco) },
                };

                agenda.adicionar_contato(contato);
                println!("Contato adicionado com sucesso!");
            }
            2 => {
                agenda.mostrar_todos_nomes();
            }
            3 => {
                let nome = ler_entrada("Digite o nome do contato:");
                match agenda.buscar_por_nome(&nome) {
                    Some(contato) => println!("{:?}", contato),
                    None => println!("Contato não encontrado."),
                }
            }
            4 => {
                let telefone = ler_entrada("Digite o telefone do contato:");
                match agenda.buscar_por_telefone(&telefone) {
                    Some(contato) => println!("{:?}", contato),
                    None => println!("Contato não encontrado."),
                }
            }
            5 => {
                let nome = ler_entrada("Digite o nome do contato a ser removido:");
                match agenda.remover_por_nome(&nome) {
                    Some(contato) => println!("Contato removido: {:?}", contato),
                    None => println!("Contato não encontrado."),
                }
            }
            6 => {
                let telefone = ler_entrada("Digite o telefone do contato a ser removido:");
                match agenda.remover_por_telefone(&telefone) {
                    Some(contato) => println!("Contato removido: {:?}", contato),
                    None => println!("Contato não encontrado."),
                }
            },
            7 => break,
            _ => println!("Escolha inválida."),
        }
    }
}

fn ler_entrada(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

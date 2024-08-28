use agenda::{Agenda, Contato};

#[test]
fn test_integration() {
    let mut agenda = Agenda::novo();
    let contato = Contato {
        nome: "Ana".to_string(),
        telefone: "666666666".to_string(),
        email: Some("ana@example.com".to_string()),
        endereco: Some("Rua B, 456".to_string()),
    };

    agenda.adicionar_contato(contato.clone());

    assert_eq!(agenda.buscar_por_nome("Ana"), Some(&contato));
    assert_eq!(agenda.buscar_por_telefone("666666666"), Some(&contato));

    agenda.remover_por_nome("Ana");
    assert!(agenda.buscar_por_nome("Ana").is_none());
    assert!(agenda.buscar_por_telefone("666666666").is_none());
}

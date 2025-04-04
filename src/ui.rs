use crate::contacts::{Agenda, Contato};
use eframe::egui::{self, Vec2, ScrollArea};
use eframe::{App as EframeApp, CreationContext};
use regex::Regex;

pub(crate) enum Visualizacao {
    Json,
    Formatada,
}

pub struct App {
    pub agenda: Agenda,
    pub nome: String,
    pub telefone: String,
    pub email: String,
    pub endereco: String,
    pub visualizacao: Visualizacao,
    pub pesquisa: String,
}

impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            agenda: Agenda::novo(),
            nome: String::new(),
            telefone: String::new(),
            email: String::new(),
            endereco: String::new(),
            visualizacao: Visualizacao::Json,
            pesquisa: String::new(),
        }
    }

    pub fn adicionar_contato(&mut self) {
        let contato = Contato {
            nome: self.nome.clone(),
            telefone: self.telefone.clone(),
            email: if self.email.is_empty() {
                None
            } else {
                Some(self.email.clone())
            },
            endereco: if self.endereco.is_empty() {
                None
            } else {
                Some(self.endereco.clone())
            },
        };

        self.agenda.adicionar_contato(contato);
        self.nome.clear();
        self.telefone.clear();
        self.email.clear();
        self.endereco.clear();
    }

    pub fn validar_entrada(&self) -> bool {
        let nome_valido = !self.nome.trim().is_empty();
        
        let telefone_regex = Regex::new(r"^\(?\d{2}\)?\s?\d{4,5}-?\d{4}$").unwrap();
        let telefone_valido = telefone_regex.is_match(self.telefone.trim());

        nome_valido && telefone_valido
    }

    pub fn pesquisar_contatos(&self) -> Vec<Contato> {
        let pesquisa = self.pesquisa.to_lowercase();
        self.agenda
            .mostrar_todos_nomes()
            .into_iter()
            .filter(|contato| contato.nome.to_lowercase().contains(&pesquisa))
            .collect()
    }
}

impl EframeApp for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal_centered(|ui| {
                    ui.group(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Agenda");
                            
                            ui.add_space(20.0);

                            let padding_x = 12.5;
                            let padding_y = 0.0;

                            egui::Grid::new("contact_grid")
                                .num_columns(2)
                                .spacing([20.0, 10.0])
                                .show(ui, |ui| {
                                    ui.label("Nome:");
                                    ui.text_edit_singleline(&mut self.nome);
                                    ui.end_row();

                                    ui.label("Telefone:");
                                    ui.text_edit_singleline(&mut self.telefone);
                                    ui.end_row();

                                    ui.label("Email:");
                                    ui.text_edit_singleline(&mut self.email);
                                    ui.end_row();

                                    ui.label("Endereço:");
                                    ui.text_edit_singleline(&mut self.endereco);
                                    ui.end_row();
                                });

                            ui.add_space(20.0);

                            let entrada_valida = self.validar_entrada();

                            if ui.add_enabled(entrada_valida, egui::Button::new("Adicionar Contato")
                                .min_size(Vec2::new(120.0 + padding_x, 35.0 + padding_y))).clicked() {
                                self.adicionar_contato();
                            }

                            ui.add_space(20.0);

                            ui.separator();

                            ui.label("Pesquisar Contato por Nome:");
                            ui.add_space(5.0);
                            ui.text_edit_singleline(&mut self.pesquisa);

                            ui.add_space(10.0);

                            if ui.add(egui::Button::new("Alternar Visualização")
                                .min_size(Vec2::new(120.0 + padding_x, 35.0 + padding_y))).clicked() {
                                self.visualizacao = match self.visualizacao {
                                    Visualizacao::Json => Visualizacao::Formatada,
                                    Visualizacao::Formatada => Visualizacao::Json,
                                };
                            }

                            ui.add_space(10.0);

                            ui.heading("Contatos");

                            ScrollArea::vertical().show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    let contatos = self.pesquisar_contatos();

                                    for contato in contatos {
                                        ui.add_space(10.0);
                                        match self.visualizacao {
                                            Visualizacao::Json => {
                                                ui.label(format!(
                                                    r#"{{"nome": "{}",
"telefone": "{}",
"email": "{}",
"endereco": "{}"}}"#,
                                                    contato.nome,
                                                    contato.telefone,
                                                    contato.email.clone().unwrap_or_default(),
                                                    contato.endereco.clone().unwrap_or_default()
                                                ));
                                            }
                                            Visualizacao::Formatada => {
                                                ui.label(format!(
                                                    "Nome: {}\nTelefone: {}\nEmail: {}\nEndereço: {}",
                                                    contato.nome,
                                                    contato.telefone,
                                                    contato.email.clone().unwrap_or_default(),
                                                    contato.endereco.clone().unwrap_or_default()
                                                ));
                                            }
                                        }

                                        ui.add_space(10.0);
                                        
                                        if ui.add(
                                            egui::Button::new("🗑️")
                                                .min_size(Vec2::new(22.0, 22.0))
                                                .sense(egui::Sense::click())
                                                .fill(egui::Color32::from_rgb(255, 0, 0)) 
                                                .stroke(egui::Stroke::new(0.0, egui::Color32::from_rgb(0,0,0)))
                                        ).clicked() {
                                            self.agenda.remover_por_nome(&contato.nome);
                                        }

                                        ui.separator();
                                    }
                                });
                            });
                        });
                    });
                });
            });
        });
    }
}

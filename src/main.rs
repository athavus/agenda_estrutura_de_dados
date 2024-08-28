use agenda::{Agenda, Contato};
use eframe::{egui::{self, Vec2, ViewportBuilder}, App as EframeApp, CreationContext};
use regex::Regex;

struct App {
    agenda: Agenda,
    nome: String,
    telefone: String,
    email: String,
    endereco: String,
}

impl App {
    fn new(_cc: &CreationContext) -> Self {
        Self {
            agenda: Agenda::novo(),
            nome: String::new(),
            telefone: String::new(),
            email: String::new(),
            endereco: String::new(),
        }
    }

    fn adicionar_contato(&mut self) {
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

    fn validar_entrada(&self) -> bool {
        let nome_valido = !self.nome.trim().is_empty();
        
        let telefone_regex = Regex::new(r"^\(?\d{2}\)?\s?\d{4,5}-?\d{4}$").unwrap();
        let telefone_valido = telefone_regex.is_match(self.telefone.trim());

        nome_valido && telefone_valido
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

                            if ui.add_enabled(entrada_valida, egui::Button::new("Adicionar Contato")).clicked() {
                                self.adicionar_contato();
                            }

                            ui.add_space(20.0);

                            ui.separator();

                            ui.heading("Contatos");

                            for contato in self.agenda.mostrar_todos_nomes() {
                                ui.horizontal(|ui| {
                                    // Exibe o contato no formato JSON com quebras de linha
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
                                    
                                    // Botão de lixeira estilizado
                                    if ui.add(
                                        egui::Button::new("🗑️")
                                            .min_size(Vec2::new(24.0, 24.0))
                                            .sense(egui::Sense::click())
                                            .fill(egui::Color32::from_rgb(255, 0, 0)) // Fundo vermelho
                                            .stroke(egui::Stroke::new(0.0, egui::Color32::from_rgb(0,0,0))) // Remove a borda
                                    ).clicked() {
                                        self.agenda.remover_por_nome(&contato.nome);
                                    }
                                });
                                ui.add_space(5.0); // Adiciona um espaço entre os contatos
                            }
                        });
                    });
                });
            });
        });
    }
}



fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(Vec2::new(350.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Agenda",
        options, 
        Box::new(|cc| {
            let app = App::new(cc);
            Box::new(app)
        }),
    )
}

use eframe::egui;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
struct AppState {
    server_url: String,
    status: String,
    client: Arc<Mutex<Option<Client>>>,
}

#[derive(Deserialize)]
struct HealthResponse {
    status: String,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Clunadministratif");
            ui.separator();

            ui.label("Adresse du serveur ClunyDirect :");
            ui.text_edit_singleline(&mut self.server_url);
            if ui.button("Tester la connexion").clicked() {
                let url = self.server_url.clone();
                let client = Client::new();
                let rt = tokio::runtime::Runtime::new().unwrap();
                let client_arc = Arc::clone(&self.client);
                rt.spawn(async move {
                    let client = client_arc.lock().await;
                    if let Some(c) = client.as_ref() {
                        match c.get(format!("{}/health", url)).send().await {
                            Ok(resp) => {
                                if let Ok(health) = resp.json::<HealthResponse>().await {
                                    // statut mis à jour
                                }
                            }
                            Err(_) => {}
                        }
                    }
                });
            }

            ui.separator();
            ui.label(&self.status);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Clunadministratif",
        options,
        Box::new(|_cc| Box::<AppState>::default()),
    );
}

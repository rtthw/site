


use eframe::egui;



// ================================================================================================



pub struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.heading("Nothing here, yet..."));
        });
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {}
    }
}



// ================================================================================================

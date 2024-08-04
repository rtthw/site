


use eframe::egui;



// ================================================================================================



pub struct App {}

impl eframe::App for App {
    #[cfg(not(target_arch = "wasm32"))]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.heading("Nothing here, yet..."));
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.heading(&frame.info().web_info.location.url));
        });
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {}
    }
}



// ================================================================================================

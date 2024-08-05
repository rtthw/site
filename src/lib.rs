
use eframe::egui;



// ================================================================================================



pub struct App {
    anchor: Anchor,
}

impl eframe::App for App {
    #[allow(unused)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        {
            if &frame.info().web_info.location.hash != self.anchor.as_ref() {
                self.anchor = Anchor::from(&frame.info().web_info.location.hash);
            }
        }

        match self.anchor {
            Anchor::Home => {
                egui::TopBottomPanel::top("top").show(ctx, |ui| {
                    ui.visuals_mut().button_frame = false;
                    ui.horizontal_centered(|ui| {
                        if ui.button("About").clicked() {
                            self.anchor = Anchor::About;
                            #[cfg(target_arch = "wasm32")]
                            {
                                frame.info().web_info.location.hash = self.anchor.as_ref().to_string();
                            }
                        }
                    });
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.centered_and_justified(|ui| ui.heading("Home Page"));
                });
            }
            Anchor::About => {
                egui::TopBottomPanel::top("top").show(ctx, |ui| {
                    ui.visuals_mut().button_frame = false;
                    ui.horizontal_centered(|ui| {
                        if ui.button("Home").clicked() {
                            self.anchor = Anchor::Home;
                            #[cfg(target_arch = "wasm32")]
                            {
                                frame.info().web_info.location.hash = self.anchor.as_ref().to_string();
                            }
                        }
                    });
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.centered_and_justified(|ui| ui.heading("Home Page"));
                });
            }
            Anchor::NotFound => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.centered_and_justified(|ui| ui.heading("Nothing here, yet..."));
                });
            }
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            anchor: Anchor::Home,
        }
    }
}



// ================================================================================================



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Anchor {
    Home,
    About,
    NotFound,
}

impl From<&str> for Anchor {
    fn from(value: &str) -> Self {
        match value {
            "" => Self::Home,
            "about" => Self::About,
            _ => Self::NotFound,
        }
    }
}

impl AsRef<str> for Anchor {
    fn as_ref(&self) -> &str {
        match self {
            Self::Home => "",
            Self::About => "about",
            Self::NotFound => "404",
        }
    }
}


// ================================================================================================

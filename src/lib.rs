
use eframe::egui;



// ================================================================================================



pub struct App {
    anchor: Anchor,
}

impl eframe::App for App {
    #[allow(unused)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        #[cfg(target_arch = "wasm32")]
        if let Some(anchor) = frame.info().web_info.location.hash.strip_prefix('#') {
            if anchor != self.anchor.as_ref() {
                self.anchor = Anchor::from(&frame.info().web_info.location.hash);
            }
        }

        match self.anchor {
            Anchor::Home => {
                egui::TopBottomPanel::top("home-top").show(ctx, |ui| {
                    ui.visuals_mut().button_frame = false;
                    ui.horizontal_centered(|ui| {
                        if ui.button("About").clicked() {
                            self.anchor = Anchor::About;
                            if frame.is_web() {
                                ui.ctx().open_url(egui::OpenUrl::same_tab("#about"));
                            }
                        }
                    });
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.centered_and_justified(|ui| ui.heading("Home Page"));
                });
            }
            Anchor::About => {
                egui::TopBottomPanel::top("about-top").show(ctx, |ui| {
                    ui.visuals_mut().button_frame = false;
                    ui.horizontal_centered(|ui| {
                        if ui.button("Home").clicked() {
                            self.anchor = Anchor::Home;
                            if frame.is_web() {
                                ui.ctx().open_url(egui::OpenUrl::same_tab(""));
                            }
                        }
                    });
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.centered_and_justified(|ui| ui.heading("About Page"));
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

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Home => "home",
            Self::About => "about",
            Self::NotFound => "404",
        })
    }
}

impl From<&String> for Anchor {
    fn from(value: &String) -> Self {
        match value.as_str() {
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

mod file_reader;

use eframe::egui::Context;
use eframe::{egui, Frame};

#[derive(Debug, Default)]
struct EguiApp {
    current_tab: Tabs,
}

impl EguiApp {
    fn setup(self, ctx: &eframe::CreationContext<'_>) -> Self {
        ctx.egui_ctx.set_visuals(egui::Visuals::dark());

        self
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
enum Tabs {
    #[default]
    File,
    Help,
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::TopBottomPanel::top("menu_bar")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.current_tab, Tabs::File, "File");
                    ui.selectable_value(&mut self.current_tab, Tabs::Help, "Help");
                });
            });
        egui::SidePanel::left("file_explorer")
            .resizable(true)
            .width_range(80.0..=500.0)
            .show(ctx, |ui| {
                egui::scroll_area::ScrollArea::horizontal().show(ui, |ui| {
                    ui.vertical(|ui| {
                        egui::CollapsingHeader::new("Games").show(ui, |ui| {});
                        egui::CollapsingHeader::new("Sounds").show(ui, |ui| {});
                        egui::CollapsingHeader::new("Images").show(ui, |ui| {});
                        egui::CollapsingHeader::new("Strings").show(ui, |ui| {});
                    });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Windows");
            });
            egui::scroll_area::ScrollArea::horizontal().show(ui, |ui| {});
        });
    }
}

fn main() {
    let app = EguiApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "gluxrs",
        native_options,
        Box::new(|cc| Box::new(app.setup(cc))),
    )
    .expect("Unable to load eframe");
}

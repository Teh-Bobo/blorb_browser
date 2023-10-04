mod file_reader;

use crate::file_reader::blorb_reader::ChunkData;
use crate::file_reader::GameType;
use eframe::egui::{ColorImage, Context, TextureHandle};
use eframe::{egui, Frame};
use std::collections::HashMap;

struct EguiApp {
    current_tab: Tabs,
    loaded_game: Option<GameType<'static>>,
    main_draw: Box<dyn Fn(&mut egui::Ui)>,
    loaded_images: HashMap<i32, TextureHandle>,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            current_tab: Default::default(),
            loaded_game: None,
            main_draw: Box::new(|ui: &mut egui::Ui| {
                ui.heading("Windows");
            }),
            loaded_images: Default::default(),
        }
    }
}

impl EguiApp {
    fn setup(self, ctx: &eframe::CreationContext<'_>) -> Self {
        ctx.egui_ctx.set_visuals(egui::Visuals::dark());
        egui_extras::install_image_loaders(&ctx.egui_ctx);

        let game_path = "test_games/imagetest.gblorb";
        let bytes: &'static [u8] = std::fs::read(game_path)
            .expect("Unable to open specified path")
            .leak();
        let game = bytes
            .try_into()
            .expect("Unable to parse game file. Not actually a game?");

        EguiApp {
            loaded_game: Some(game),
            ..self
        }
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
                egui::scroll_area::ScrollArea::both().show(ui, |ui| {
                    ui.vertical(|ui| {
                        egui::CollapsingHeader::new("Games").show(ui, |ui| {
                            match &self.loaded_game {
                                Some(GameType::Ulx(_)) => {
                                    ui.label("0");
                                }
                                Some(GameType::Blorb(b)) => {
                                    b.exec_ids().iter().for_each(|id| {
                                        ui.label(format!("{id}"));
                                    });
                                }
                                _ => {}
                            }
                        });
                        egui::CollapsingHeader::new("Sounds").show(ui, |_ui| {});
                        egui::CollapsingHeader::new("Images").show(ui, |ui| {
                            match &self.loaded_game {
                                Some(GameType::Blorb(b)) => {
                                    let mut ids = b.image_ids();
                                    ids.sort();
                                    ids.iter().for_each(|&id| {
                                        if ui.button(format!("{id}")).clicked() {
                                            eprintln!("{id} double clicked!");

                                            let handle = self.loaded_images.entry(id).or_insert_with(|| {
                                                let ChunkData::Picture(picture_bytes) = b.get_image(id).unwrap().data else {
                                                    panic!("Failed to pre-load image id: {id}");
                                                };
                                                let image = image::load_from_memory(picture_bytes).expect("Chunk provided invalid image");
                                                let size = [image.width() as _, image.height() as _];
                                                let image_buffer = image.to_rgba8();
                                                let pixels = image_buffer.as_flat_samples();
                                                let picture = ColorImage::from_rgba_unmultiplied(
                                                    size,
                                                    pixels.as_slice(),
                                                );
                                                ctx.load_texture("", picture, Default::default())
                                            }).clone();

                                            self.main_draw = Box::new(move |ui| {
                                                let sa = egui::scroll_area::ScrollArea::both();
                                                sa.show(ui, |ui| {
                                                    ui.image((handle.id(), handle.size_vec2()));
                                                });
                                            });
                                        }
                                    });
                                }
                                _ => {}
                            }
                        });
                        egui::CollapsingHeader::new("Strings").show(ui, |_ui| {});
                    });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            (self.main_draw)(ui);
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

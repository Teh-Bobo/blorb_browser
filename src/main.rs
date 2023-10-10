mod file_reader;

use crate::file_reader::blorb_chunk_types::BlorbChunkType;
use crate::file_reader::GameType;
use eframe::egui::{ColorImage, Context, TextureHandle, Ui, WidgetText};
use eframe::{egui, Frame};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use strum::IntoEnumIterator;

struct EguiApp {
    current_menu: Menus,
    current_tab: Tabs,
    loaded_game: Option<GameType<'static>>,
    main_draw: Box<dyn Fn(&mut Ui)>,
    loaded_images: HashMap<i32, TextureHandle>,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            current_menu: Default::default(),
            current_tab: Default::default(),
            loaded_game: None,
            main_draw: Box::new(|ui: &mut Ui| {
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

        let game_path = "test_games/sensory.blb";
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

    fn draw_sound_sub_header(
        &mut self,
        ui: &mut Ui,
        chunk_type: BlorbChunkType,
        heading: impl Into<WidgetText>,
    ) {
        if let Some(GameType::Blorb(b)) = &self.loaded_game {
            let mut ids = b.get_ids(chunk_type);
            ids.sort();

            if !ids.is_empty() {
                egui::CollapsingHeader::new(heading).show(ui, |ui| {
                    ids.iter().for_each(|id| {
                        ui.label(format!("{id}"));
                    });
                });
            }
        }
    }

    fn draw_menu_from_enum<I, D>(ui: &mut Ui, current_option: &mut D, options: I)
    where
        I: Iterator<Item = D>,
        D: Display + Eq + Copy,
    {
        ui.horizontal(|ui| {
            options.for_each(|o| {
                ui.selectable_value(current_option, o, o.to_string());
            });
        });
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, strum_macros::EnumIter)]
enum Menus {
    #[default]
    File,
    Help,
}

impl Display for Menus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, strum_macros::EnumIter)]
enum Tabs {
    #[default]
    Games,
    Images,
    Sounds,
    Strings,
}

impl Display for Tabs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::TopBottomPanel::top("menu_bar")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    EguiApp::draw_menu_from_enum(ui, &mut self.current_menu, Menus::iter());
                    EguiApp::draw_menu_from_enum(ui, &mut self.current_tab, Tabs::iter());
                })
            });
        egui::SidePanel::left("file_explorer")
            .resizable(true)
            .width_range(80.0..=500.0)
            .show(ctx, |ui| {
                egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {
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
                        egui::CollapsingHeader::new("Sounds").show(ui, |ui| {
                            self.draw_sound_sub_header(ui, BlorbChunkType::SOUND, "Sound");
                            self.draw_sound_sub_header(ui, BlorbChunkType::SOUND_MOD, "MOD Sounds");
                            self.draw_sound_sub_header(ui, BlorbChunkType::SOUND_SONG, "Songs");
                        });
                        egui::CollapsingHeader::new("Images").show(ui, |ui| {
                            match &self.loaded_game {
                                Some(GameType::Blorb(b)) => {
                                    let mut ids = b.image_ids();
                                    ids.sort();
                                    ids.iter().for_each(|&id| {
                                        if ui.button(format!("{id}")).clicked() {
                                            let handle = self
                                                .loaded_images
                                                .entry(id)
                                                .or_insert_with(|| {
                                                    let picture_bytes =
                                                        b.get_image(id).unwrap().data;
                                                    let image =
                                                        image::load_from_memory(picture_bytes)
                                                            .expect("Chunk provided invalid image");
                                                    let size =
                                                        [image.width() as _, image.height() as _];
                                                    let image_buffer = image.to_rgba8();
                                                    let pixels = image_buffer.as_flat_samples();
                                                    let picture =
                                                        ColorImage::from_rgba_unmultiplied(
                                                            size,
                                                            pixels.as_slice(),
                                                        );
                                                    ctx.load_texture(
                                                        "",
                                                        picture,
                                                        Default::default(),
                                                    )
                                                })
                                                .clone();

                                            self.main_draw = Box::new(move |ui| {
                                                let sa = egui::scroll_area::ScrollArea::both();
                                                sa.show(ui, |ui| {
                                                    ui.image((
                                                        handle.id(),
                                                        ui.available_size_before_wrap(),
                                                    ));
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
        "Blorb Browser",
        native_options,
        Box::new(|cc| Box::new(app.setup(cc))),
    )
    .expect("Unable to load eframe");
}

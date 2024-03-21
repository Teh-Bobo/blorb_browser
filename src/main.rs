use crate::egui::Ui;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use eframe::egui::{ColorImage, Context, TextureHandle, WidgetText};
use eframe::{egui, Frame};
use egui_extras::Column;
use strum::IntoEnumIterator;

use crate::file_reader::blorb_chunk_types::BlorbChunkType;
use crate::file_reader::blorb_reader::BlorbReader;
use crate::file_reader::ulx_reader::ParsedString;
use crate::file_reader::GameType;

mod file_reader;
mod strings;

#[derive(Default)]
struct EguiApp {
    current_menu: Menus,
    current_tab: Tabs,
    loaded_game: Option<GameType<'static>>,
    loaded_images: HashMap<i32, TextureHandle>,
    image_tab_data: ImageTabData,
    parsed_strings: Option<Vec<ParsedString>>,
}

impl EguiApp {
    fn setup(self, ctx: &eframe::CreationContext<'_>) -> Self {
        ctx.egui_ctx.set_visuals(egui::Visuals::dark());
        egui_extras::install_image_loaders(&ctx.egui_ctx);

        let game_path = "test_games/glulxercise.ulx";
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

    fn draw_current_tab(&mut self, ui: &mut Ui) {
        match self.current_tab {
            Tabs::Games => self.draw_games_tab(ui),
            Tabs::Images => self.draw_images_tab(ui),
            Tabs::Sounds => self.draw_sound_tab(ui),
            Tabs::Strings => self.draw_strings_tab(ui),
        }
    }

    fn draw_games_tab(&mut self, ui: &mut Ui) {
        let game = match &self.loaded_game {
            Some(GameType::Blorb(b)) => b.get_exec(0).expect("Blorb had no games"),
            Some(GameType::Ulx(u)) => u.clone(),
            None => panic!("Tried to draw the game tab without loaded game"),
        };
        egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.heading("Game Header");
                ui.label(game.header.to_string());
                ui.heading("Debugging Header");
                ui.label(game.debugging_header.to_string());
            });
        });
    }

    fn draw_images_tab(&mut self, ui: &mut Ui) {
        let count = egui::SidePanel::left("image_options")
            .show_inside(ui, |ui| {
                if let Some(GameType::Blorb(b)) = &self.loaded_game {
                    let mut ids = b.image_ids();
                    ids.sort();
                    ids.iter()
                        .find(|&&id| ui.button(format!("{id}")).clicked())
                        .map(|&id| {
                            self.image_tab_data.selected_image = Some(
                                self.loaded_images
                                    .entry(id)
                                    .or_insert_with(|| {
                                        let picture_bytes = b.get_image(id).unwrap().data;
                                        let image = image::load_from_memory(picture_bytes)
                                            .expect("Chunk provided invalid image");
                                        let size = [image.width() as _, image.height() as _];
                                        let image_buffer = image.to_rgba8();
                                        let pixels = image_buffer.as_flat_samples();
                                        let picture = ColorImage::from_rgba_unmultiplied(
                                            size,
                                            pixels.as_slice(),
                                        );
                                        ui.ctx().load_texture("", picture, Default::default())
                                    })
                                    .clone(),
                            )
                        });
                    ids.len()
                } else {
                    0
                }
            })
            .inner;
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if count == 0 {
                ui.heading("No images found in this game file");
                return;
            }
            let sa = egui::scroll_area::ScrollArea::both();
            sa.show(ui, |ui| {
                if let Some(handle) = &self.image_tab_data.selected_image {
                    ui.image((handle.id(), ui.available_size_before_wrap()));
                }
            });
        });
    }

    fn draw_sound_tab(&mut self, ui: &mut Ui) {
        fn draw_sub_header(
            b: &BlorbReader,
            ui: &mut Ui,
            chunk_type: BlorbChunkType,
            heading: impl Into<WidgetText>,
        ) -> usize {
            let mut ids = b.get_ids(chunk_type);
            ids.sort();

            if !ids.is_empty() {
                egui::CollapsingHeader::new(heading).show(ui, |ui| {
                    ids.iter().for_each(|id| {
                        ui.label(format!("{id}"));
                    });
                });
            }

            ids.len()
        }

        let count = egui::SidePanel::left("sound_options")
            .show_inside(ui, |ui| {
                if let Some(GameType::Blorb(b)) = &self.loaded_game {
                    draw_sub_header(b, ui, BlorbChunkType::SOUND, "Sound")
                        + draw_sub_header(b, ui, BlorbChunkType::SOUND_MOD, "MOD Sounds")
                        + draw_sub_header(b, ui, BlorbChunkType::SOUND_SONG, "Songs")
                } else {
                    0
                }
            })
            .inner;
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if count == 0 {
                ui.heading("No sounds found in this game file");
                return;
            }
            //TODO
        });
    }

    fn draw_strings_tab(&mut self, ui: &mut Ui) {
        let strings =
            self.parsed_strings
                .get_or_insert_with(|| match self.loaded_game.as_ref().unwrap() {
                    GameType::Ulx(game) => game.parse_strings(),
                    GameType::Blorb(game) => game.get_exec(0).unwrap().parse_strings(),
                });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui_extras::TableBuilder::new(ui)
                .columns(Column::auto(), 2)
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Type");
                    });
                    header.col(|ui| {
                        ui.heading("Address");
                    });
                    header.col(|ui| {
                        ui.heading("String");
                    });
                })
                .body(|body| {
                    body.rows(18.0, strings.len(), |mut row| {
                        let row_index = row.index();
                        row.col(|ui| {
                            ui.label(format!("{:?}", strings[row_index].string_type));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", strings[row_index].start_address));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:?}", strings[row_index].data));
                        });
                    });
                });
        });
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

#[derive(Clone, Default, Eq, PartialEq, Hash)]
struct ImageTabData {
    selected_image: Option<TextureHandle>,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_current_tab(ui);
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

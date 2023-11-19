#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io;
use std::io::BufReader;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct KaemojiData {
    emoticon: String,
    tags: Option<Vec<String>>,
}

impl KaemojiData {
    fn new() -> Self {
        KaemojiData {
            emoticon: String::new(),
            tags: None,
        }
    }
}

impl Default for KaemojiData {
    fn default() -> Self {
        KaemojiData {
            emoticon: "( ͡° ͜ʖ ͡°)".to_string(),
            tags: Some(["example".to_string()].to_vec()),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct KaemojiConfig {
    kaemojis: HashMap<String, Vec<KaemojiData>>,
}

impl KaemojiConfig {
    fn new() -> Self {
        KaemojiConfig {
            kaemojis: HashMap::from([(String::new(), vec![KaemojiData::new()])]),
        }
    }
}

impl Default for KaemojiConfig {
    fn default() -> Self {
        KaemojiConfig {
            kaemojis: HashMap::from([(
                "Example kaemoji".to_string(),
                vec![KaemojiData::default()],
            )]),
        }
    }
}

fn main() {
    if let Err(error) = egui_render_app(read_config()) {
        panic!("SHIT, problem rendering app: {:?}", error);
    }
    read_config();
}

//fn read_file<P: AsRef<Path>>(path: P) -> Result<KaemojiData, io::Error> {  TODO: error handling
fn read_config() -> KaemojiConfig {
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    let config_path: PathBuf = dirs::config_dir().unwrap();
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    let config_path = dirs::home_dir().unwrap().join(".config");
    let config_path = config_path.join("kaemoji_board");
    if !config_path.exists() {
        create_dir(&config_path);
    }
    let config_file = config_path.join("KaemojiConfig.json");

    match File::open(&config_file) {
        Ok(file) => serde_json::from_reader(BufReader::new(file)).unwrap(),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            File::create(config_file)
                .unwrap()
                .write_all(
                    serde_json::to_string_pretty(&KaemojiConfig::default())
                        .unwrap()
                        .as_ref(),
                )
                .unwrap();
            KaemojiConfig::default()
        }
        Err(error) => panic!("SHIT, problem reading file: {:?}", error),
    }
}

fn egui_render_app(uidata: KaemojiConfig) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(240.0, 120.0)),
        ..Default::default()
    };

    eframe::run_simple_native("Kaemoji board", options, move |ctx, frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Kaemoji board");

            for (heading, kaemojis) in uidata.kaemojis.iter() {
                ui.collapsing(heading, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout::with_main_wrap(
                                // adds the button wrapping
                                egui::Layout::left_to_right(egui::Align::TOP), // sets the button alignment
                                // to be left to right
                                true,
                            ),
                            |ui| {
                                for kaemoji in kaemojis {
                                    if ui.button(&kaemoji.emoticon).clicked() {
                                        ui.output_mut(|o| {
                                            o.copied_text = kaemoji.emoticon.clone()
                                            // copies the text of the current kaemoji to clipboard
                                        });
                                        frame.close() // closes app
                                    }
                                }
                            },
                        );
                    });
                });
            }
        });
    })
}

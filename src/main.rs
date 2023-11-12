#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct KaemojiData {
    emoticon: String,
    kind: String,
    tags: Option<Vec<String>>,
}

impl KaemojiData {
    fn new() -> Self {
        KaemojiData {
            emoticon: "( ͡° ͜ʖ ͡°)".to_string(),
            kind: "Example kaemoji".to_string(),
            tags: Some(["example".to_string()].to_vec()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct KaemojiConfig {
    kaemojis: Vec<KaemojiData>,
}

impl KaemojiConfig {
    fn new() -> Self {
        KaemojiConfig {
            kaemojis: vec![KaemojiData::new()],
        }
    }
}

fn main() {
    let _ = render_app();
    let test = KaemojiConfig::new();
    let ser_test = serde_json::to_string(&test).unwrap();
    println!("{}", ser_test);
    //let des_test: KaemojiData = serde_json::from_str(&ser_test).unwrap();
    let des_test: KaemojiData = read_file("test.json");
    //    let des_test: Kaemojis = serde_json::from_str(&ser_test).unwrap();
    println!("{:?}", des_test)
}

//fn read_file<P: AsRef<Path>>(path: P) -> Result<KaemojiData, io::Error> {
fn read_file<P: AsRef<Path> + std::marker::Copy>(path: P) -> KaemojiData {
    match File::open(path) {
        Ok(file) => serde_json::from_reader(BufReader::new(file)).unwrap(),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            File::create(path)
                .unwrap()
                .write_all(
                    serde_json::to_string_pretty(&KaemojiData::new())
                        .unwrap()
                        .as_ref(),
                )
                .unwrap();
            KaemojiData::new()
        }
        Err(error) => panic!("SHIT, Problem reading file: {:?}", error),
    }
}

fn render_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(240.0, 120.0)),
        ..Default::default()
    };

    eframe::run_simple_native("Kaemoji board", options, move |ctx, frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Kaemoji board");

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(
                    egui::Layout::with_main_wrap(
                        // adds the button wrapping
                        egui::Layout::left_to_right(egui::Align::TOP), // sets the button alignment
                        // to be left to right
                        true,
                    ),
                    |ui| {
                        let var = [0, 1, 2];
                        for i in var {
                            if ui.button(var[i].to_string()).clicked() {
                                ui.output_mut(|o| o.copied_text = var[i].to_string()); // copies
                                                                                       // the text
                                                                                       // of the
                                                                                       // current
                                                                                       // kaemoji
                                                                                       // to
                                                                                       // clipboard
                                frame.close()
                            }
                        }
                    },
                );
            });
        });
    })
}

use crate::KaemojiConfig;

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::window::Window;
use cacao::appkit::{App, AppDelegate};
use cacao::button::{self, Button};
use cacao::color::Color;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::pasteboard::Pasteboard;
use cacao::view::View;
use eframe::egui;

const APP_TITLE: &str = "Kaemoji board";

#[derive(Default)]
struct NativeMacApp {
    window: Window,
    content: View,
    content_view: View,
    uidata: KaemojiConfig,
}

impl AppDelegate for NativeMacApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(240., 240.);
        self.window.set_title(APP_TITLE);
        self.window
            .set_title_visibility(cacao::appkit::window::TitleVisibility::Hidden);
        App::set_menu(vec![
            Menu::new(
                "",
                vec![
                    MenuItem::new("Preferences").key(",").action(|| todo!()),
                    MenuItem::Separator,
                    MenuItem::Quit,
                    MenuItem::Minimize,
                ],
            ),
            //Menu::new("Test", vec![MenuItem::new("Hello world!".to_string())]),
        ]);
        App::activate();
        self.window.set_background_color(Color::Separator);

        self.content.add_subview(&self.content_view);
        self.window.set_content_view(&self.content);

        let mut button = Button::new("My button title");
        let paste: Pasteboard = Pasteboard::default();

        button.set_action(move || {
            //BUG: Doesn't actually do shit when pressed. Big no no for release
            paste.clear_contents();
            paste.copy_text("test");
            App::terminate();
        });

        self.content_view.add_subview(&button);

        LayoutConstraint::activate(&[
            self.content_view
                .top
                .constraint_equal_to(&self.content.top)
                .offset(16.),
            self.content_view
                .leading
                .constraint_equal_to(&self.content.leading)
                .offset(16.),
            self.content_view
                .trailing
                .constraint_equal_to(&self.content.trailing)
                .offset(-16.),
            self.content_view
                .bottom
                .constraint_equal_to(&self.content.bottom)
                .offset(-16.),
        ]);

        self.window.show();
    }
}

pub fn render_app(uidata: KaemojiConfig) {
    //App::new("garfunkle.kaemoji_board.app", NativeMacApp::default()).run();

    if let Err(error) = egui_render_app(uidata) {
        panic!("SHIT, problem rendering app: {:?}", error);
    }
}

fn egui_render_app(uidata: KaemojiConfig) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(240.0, 120.0)),
        ..Default::default()
    };

    eframe::run_simple_native(APP_TITLE, options, move |ctx, frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(APP_TITLE);

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

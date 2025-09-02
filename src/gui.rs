use crate::compress;
use eframe::egui::text::LayoutJob;
use eframe::egui::{self, Button, RichText, UiBuilder, ViewportCommand};
use eframe::{App, NativeOptions, run_native};
use egui::Context;
use rfd::FileDialog;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_gui() -> eframe::Result<()> {
    let state = Arc::new(Mutex::new(State::new()));

    eframe::run_native(
        "File Compressor",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(CompressApp { state }))),
    )
}

#[derive(Debug)]
struct State {
    status: Status,
    input_path: Option<String>,
    output_name: Option<String>,
    quality: u8,
}

impl State {
    fn new() -> Self {
        State {
            status: Status::Default,
            input_path: None,
            output_name: None,
            quality: 70,
        }
    }
}
#[derive(Debug)]
enum Status {
    Default,
    Ready,
    Compressing,
    Error,
    Finished,
}

#[derive(Debug)]
struct CompressApp {
    state: Arc<Mutex<State>>,
}

impl App for CompressApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.state.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            state.status = Status::Ready;

            // will cause a deadlock, state hasn't been dropped
            //let mut test = self.state.lock().unwrap();
            //test.input_path = Some(String::from("string"));

            use egui::{Align, Color32, FontFamily, FontId, TextFormat};

            let mut job = LayoutJob::default();
            job.append(
                "Select File to Compress",
                0.0,
                TextFormat {
                    font_id: FontId::new(14.0, FontFamily::Proportional),
                    color: Color32::BLACK,
                    valign: Align::Center,
                    ..Default::default()
                },
            );

            if ui.button(job).clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    state.input_path = Some(path.display().to_string());
                }
            }

            if let Some(ref path) = state.input_path {
                ui.label(format!("Input file path: {}", path));
                ui.end_row();
            }

            // output
            if state.output_name.is_none() {
                if let Some(ref input_path) = state.input_path {
                    let file_stem = PathBuf::from(input_path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    if !file_stem.is_empty() {
                        state.output_name = Some(format!("{}_compressed", file_stem));
                    }
                }
            }

            let ui_builder = egui::UiBuilder::new();
            ui.scope_builder(ui_builder, |ui| {
                egui::Grid::new("output grid")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("Output File Name: ");
                        ui.text_edit_singleline(
                            state.output_name.get_or_insert_with(|| "".to_string()),
                        );
                    });
            });

            // Quality slider
            ui.add(egui::Slider::new(&mut state.quality, 1..=100).text("Quality"));

            // Status
            ui.label(format!("Status: {:?}", state.status));

            if ui.button("Compress").clicked() {
                eprintln!("button clicked");
                let state_clone = self.state.clone();
                // Note - values are moved into a thread (ie state_clone is moved)
                eprintln!("About to spawn a thread");

                std::thread::spawn(move || {
                    eprintln!("Inside the thread");
                    // lock the cloned mutex, take the input path as a reference, clone that too (E0502: avoid both mutable and immutable borrow)
                    let mut s = state_clone.lock().unwrap();
                    if let Some(ref input) = s.input_path {
                        let input_clone = input.clone();
                        let output_name = s.output_name.clone();
                        let quality = s.quality.clone();
                        eprintln!("input clone {:?}", input_clone);
                        s.status = Status::Compressing.into();
                        eprintln!("state clone {:?}", s);
                        drop(s); // drop s 

                        // Compression
                        eprintln!("going to compress");
                        let result = crate::compress::compress_file(
                            PathBuf::from(input_clone),
                            output_name,
                            Some(quality),
                        );
                        eprintln!("result {:?}", result);

                        let mut t = state_clone.lock().unwrap();
                        eprintln!("cloned lock again {:?}", t);
                        match result {
                            Ok(_) => t.status = Status::Finished.into(),
                            Err(_) => t.status = Status::Error.into(),
                        }
                    } else {
                        s.status = Status::Ready;
                    }
                    eprintln!("bottom of thread");
                });
            }
        });

        ctx.request_repaint();
    }
}

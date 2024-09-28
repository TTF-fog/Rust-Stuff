
#![allow(rustdoc::missing_crate_level_docs)]
use sha256::{digest, Sha256Digest};
use std::fs::{write};
use eframe::egui;
use rfd::FileDialog;
fn save_file(text:&String) -> bool{
    let files = FileDialog::new()
        .add_filter("text", &["txt", "rs"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .save_file();
    if files == None{
        println!("No File Provided");
        return false
    }

    let path = files.unwrap();
    println!("{:?}",path);
    write(path, text);
    true
}
//use eval::{eval, to_value};
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 720.0]),
        ..Default::default()
    };


    let mut hashes =String::new();
    let mut text = String::new();
    let mut updatemessage:&str = "";
    let mut file_name = "File1";
    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Shortcuts");

            if ui.button("Save").clicked{
                updatemessage = "Saved!";
                file_name = "File1";

                save_file(&text);
                hashes = digest(&text);
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                // ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::BottomUp), |ui|{
                //     ui.label("test");
                // }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.label(format!("'{updatemessage}'"));



                });
            });

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(file_name);

            let response = ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut text));
            if response.changed(){
                if hashes == digest(&text) {
                    println!("exists");
                    file_name = "File1";

                }else{
                    file_name = "File1*";
                }
            }


        });
    })
}

#![allow(rustdoc::missing_crate_level_docs)]
use sha256::{digest, Sha256Digest};
use eframe::egui;
use egui::util::hash;

//use eval::{eval, to_value};
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 720.0]),
        ..Default::default()
    };


    let mut hashes:Vec<String> =vec![];
    let mut Text = String::new();
    let mut updatemessage:&str = "";
    let mut fileName = "File1";
    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Shortcuts");

            if ui.button("Save").clicked{
                updatemessage = "Saved!";

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
            ui.heading(fileName);

            let response = ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut Text));
            if response.changed(){
                if hashes.iter().any(|e| digest(&Text).contains(e)){
                    println!("exists");
                }else{
                    hashes.push(digest(&Text));
                    println!("The hashes are as follows \n {:?}",hashes);
                }
            }


        });
    })
}
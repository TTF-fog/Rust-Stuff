#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use std::{thread, time};
use eframe::egui;
//use eval::{eval, to_value};
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 720.0]),
        ..Default::default()
    };

    // Our application state:
    let mut age = 42;
    let mut Text = String::new();
    let mut updatemessage:&str = "";
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
            ui.heading("Simple Text Editor");

            let response = ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut Text));


        });
    })
}
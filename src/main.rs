
#![allow(rustdoc::missing_crate_level_docs)]
use sha256::{digest, Sha256Digest};
use std::fs::{write};
use eframe::egui;
use rfd::FileDialog;


//use eval::{eval, to_value};
fn main() -> eframe::Result {



    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };


    let mut hashes =String::new();
    let mut text = String::new();
    let mut updatemessage:String = "".to_string();
    let mut file_name = "File1";
    eframe::run_simple_native("Rust Notes", options, move |ctx, _frame| {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Shortcuts");

            if ui.button("Save As").clicked{
                let v = save_file(&text);
                let b = v.to_string();

                updatemessage = b;
                file_name = "File1";


                hashes = digest(&text);
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                // ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::BottomUp), |ui|{
                //     ui.label("test");
                // }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.label(format!("{updatemessage}"));



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
fn save_file(text:&String) -> String{
    let files = FileDialog::new()
        .add_filter("text", &["txt", "rs"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .save_file();
    if files == None{

            return "No File Specified!".to_string()
    }

    let path = files.unwrap();

    write(path.clone(), text);
    return format!("saved to {:?}",path)

}
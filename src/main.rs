
#![allow(rustdoc::missing_crate_level_docs)]
use sha256::{digest, Sha256Digest};
use std::fs::{File, write};
use std::io::{Read};
use std::path::PathBuf;
use eframe::egui;
use rfd::FileDialog;


//use eval::{eval, to_value};
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    let mut glob_save_location = String::new();
    let mut hashes =String::new();
    let mut text = String::new();
    let mut updatemessage: String = "".to_string();
    let mut file_name:String = "File1".to_string();
    let mut indicator:bool = true;
    let mut u:Vec<String> = vec![];
    eframe::run_simple_native("Rust Notes", options.clone(), move |ctx, _frame| {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Shortcuts");

            if ui.button("Save As").clicked{
                let v = save_file(&text);
                glob_save_location = v;
                updatemessage = format!("saved to {}",glob_save_location);
                file_name = "File1".parse().unwrap();
                if updatemessage != "No File Specified!"{

                    indicator=false;
                }


            }
            if ui.button("Load").clicked{
                 u = load_file();

                if u[2] != "fudge" {
                    println!("indicator true");
                    indicator = false;
                    glob_save_location = u[2].clone();
                }

                updatemessage = String::from(&u[0]);

                text = String::from(&u[2]);
                hashes = digest(&text);

            }
            if ui.button("Save").clicked{
                if indicator == true {
                    updatemessage = String::from("No save location, try Save As");
                }
                else{
                    write(PathBuf::from(&glob_save_location), &text).expect("TODO: panic message");
                    hashes = digest(&text);
                    println!("{}", file_name.replace("*", ""));
                    file_name = file_name.replace("*","");
                }

            }

            ui.label("Recent Files");
            for v in 1..10{
                ui.add(egui::Label::new(v.to_string()));
            }


            ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                // ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::BottomUp), |ui|{
                //     ui.label("test");
                // }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {

                    ui.label(format!("{updatemessage}"));
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {

                    ui.label("Time Spent Editing");
                });
            });

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(file_name.as_str());

            let response = ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut text));
            if response.changed(){
                if hashes == digest(&text) {
                    println!("exists");
                    file_name = "File1".parse().unwrap();
                }else{
                    file_name = "File1*".parse().unwrap();
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
    println!("{:?}", path);

    write(path.clone(), text);

    return path.to_string_lossy().to_string()
}


fn load_file() -> Vec<String> {
    let files = FileDialog::new()
        .add_filter("text", &["txt", "rs"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file();
    if files == None{

        return vec!["No File Specified!".to_string() ,  "".to_string(),"fudge".to_string()]
    }

    let path = files.unwrap();


    let mut f = File::open(&path);
    let mut data =String::new();
    f.unwrap().read_to_string(&mut data);

    return vec![path.to_string_lossy().parse().unwrap(), "".to_string(), data]

}



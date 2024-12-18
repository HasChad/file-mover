#![windows_subsystem = "windows"]

use egui::{Color32, RichText, Slider};
use rand::prelude::*;
use std::fs::{self};
use walkdir::WalkDir;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 250.0])
            .with_position([300.0, 300.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    let mut files: Vec<_> = vec![];
    let mut file_count = 0;
    let mut random_file_count = 0;
    let mut old_folder: Option<String> = None;
    let mut new_folder: Option<String> = None;
    let mut main_program_enabled = false;

    eframe::run_simple_native("Random File Mover", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Random File Mover");

            ui.add_space(15.0);

            // ! first file picker
            match old_folder.to_owned() {
                Some(folder_path) => ui.label(
                    RichText::new(format!("Move from: {}", folder_path))
                        .color(Color32::from_rgb(255, 255, 255)),
                ),
                None => ui.label("Move from: No folder selected!"),
            };

            if ui.button("Select folder").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    old_folder = Some(path.display().to_string());

                    files = WalkDir::new(old_folder.to_owned().unwrap())
                        .min_depth(1)
                        .max_depth(1)
                        .into_iter()
                        .filter_map(|f| f.ok()) // filter out errors (silently!)
                        .collect();
                }
            }

            file_count = files.len();

            ui.add_space(10.0);

            // ! second file picker
            match new_folder.to_owned() {
                Some(folder_path) => ui.label(
                    RichText::new(format!("Move to: {}", folder_path))
                        .color(Color32::from_rgb(255, 255, 255)),
                ),
                None => ui.label("Move to: No folder selected!"),
            };

            if ui.button("Select folder").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    new_folder = Some(path.display().to_string());
                }
            }

            ui.add_space(10.0);

            // ! file count slider
            ui.horizontal(|ui| {
                ui.label("File count: ");

                ui.add(Slider::new(&mut random_file_count, 0..=file_count).trailing_fill(true));
                if ui.button(" - ").clicked() && random_file_count > 0 {
                    random_file_count -= 1;
                }
                if ui.button(" + ").clicked() {
                    random_file_count += 1;
                }
            });

            ui.add_space(10.0);

            // ! program runner
            ui.add_enabled_ui(main_program_enabled, |ui| {
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
                    Color32::from_hex("#ff8383").unwrap();
                ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                    Color32::from_hex("#f3eed9").unwrap();
                ui.style_mut().visuals.override_text_color = Some(Color32::BLACK);

                if ui.button("Run Program").clicked() {
                    let _ = move_files(
                        &mut files,
                        &old_folder.to_owned().unwrap(),
                        &new_folder.to_owned().unwrap(),
                        random_file_count,
                    );
                }
            });

            if old_folder.is_some() && new_folder.is_some() {
                if old_folder.to_owned().unwrap() == new_folder.to_owned().unwrap() {
                    ui.label(
                        RichText::new("You can't choose the same folder for moving files!")
                            .color(Color32::RED),
                    );
                } else {
                    main_program_enabled = true;
                }
            };
        });
    })
}

fn move_files(
    files: &mut Vec<walkdir::DirEntry>,
    old_folder: &str,
    new_folder: &str,
    random_file_count: usize,
) -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    for _ in 0..random_file_count {
        let index = rng.gen_range(0..files.len());
        let old_file = &files[index];
        let new_file = format!("{}", old_file.path().display());
        let file_name = new_file.replace(old_folder, "");

        fs::rename(old_file.path(), format!("{}{}", new_folder, file_name))?;
        files.remove(index);
    }

    Ok(())
}

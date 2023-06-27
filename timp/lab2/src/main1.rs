mod trial;
use trial::LaunchCounter;
use eframe::egui;
use std::fs;
use std::io::prelude::*;

fn main() {
    let mut counter = LaunchCounter::from_threshold(5).unwrap();
    if counter.check() {
        println!("Unlocked, free launches left: {}",counter.count_left());
    } else {
        println!("Free Trial ended buy our fucking license you cheap fuck");
    }
    counter.log().unwrap();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lab2",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}

struct App {
    name: String,
    surname: String,
    o: String,
}


impl Default for App {
    fn default() -> Self {
        App {
            name: "".to_owned(),
            surname: "".to_owned(),
            o: "".to_owned(),
        }
    }
}

fn save(app: &App) -> Result<(),std::io::Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("name.txt")?;
    file.write(format!("{} {} {}\n",app.surname, app.name, app.o).as_bytes())?;
    Ok(())
}

impl eframe::App for App {
     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
         egui::CentralPanel::default().show(ctx, |ui| {
             ui.heading("Lab2 Nikolaev Gleb");
             ui.horizontal(|ui| {
                 ui.label("Enter your name: ");
                 ui.text_edit_singleline(&mut self.name);
             });
             ui.horizontal(|ui| {
                 ui.label("Enter your surname: ");
                 ui.text_edit_singleline(&mut self.surname);
             });
             ui.horizontal(|ui| {
                 ui.label("Enter your o: ");
                 ui.text_edit_singleline(&mut self.o);
             });
             if ui.button("Save").clicked() {
                 println!("{} {} {}",self.surname, self.name, self.o);
                 match save(&self){
                     Ok(_) => {
                         println!("OK");
                         ui.label("OK");
                     }
                     Err(e) => {
                         eprintln!("Error: {}",e);
                     }
                 };
             }
//             ui.label(format!("Hello {} {} {}",self.surname, self.name, self.o));
         });
     }
 }

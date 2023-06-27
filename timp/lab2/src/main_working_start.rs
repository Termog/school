mod trial;
use trial::LaunchCounter;
use eframe::egui;
use std::fs;
use std::io::prelude::*;

fn main() {
    let mut counter = LaunchCounter::from_threshold(5).unwrap();
    let access = if counter.check() {
        //println!("Unlocked, free launches left: {}",counter.count_left());
        true
    } else {
        //println!("Free Trial ended buy our fucking license you cheap fuck");
        false
    };
    counter.log().unwrap();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lab2",
        options,
        Box::new(move |_cc| Box::new(App::default_counter(counter,access))),
    );
}

struct App {
    name: String,
    surname: String,
    o: String,
    show_confirmation: bool,
    counter: LaunchCounter,
    access: bool,
}

impl App {
    fn default_counter(counter: LaunchCounter,access: bool) -> Self {
        App {
            name: "".to_owned(),
            surname: "".to_owned(),
            o: "".to_owned(),
            show_confirmation: false,
            counter: counter,
            access: access,
        }
    }
    fn save_name(&self) -> Result<bool,std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open("name.txt")?;
        if self.name.is_empty() | self.surname.is_empty() | self.o.is_empty()  {
            return Ok(false)
        }
        let name = format!("{} {} {}\n",self.surname, self.name, self.o);
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let contains = contents.contains(&name);
        if contains {
            //println!("this name is already saved");
        } else {
            contents.push_str(&name);
            file.seek(std::io::SeekFrom::Start(0))?;
            file.write_all(contents.as_bytes())?;
        }
        Ok(contains)
    }
}


impl eframe::App for App {

     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
         egui::CentralPanel::default().show(ctx, |ui| {
             ui.heading("Лабораторная работа №2\n\t\tНиколаев Глеб");
             if self.access {
                 ui.horizontal(|ui| {
                     ui.label("Введите Имя: ");
                     ui.text_edit_singleline(&mut self.name);
                 });
                 ui.horizontal(|ui| {
                     ui.label("Введите Фамилию: ");
                     ui.text_edit_singleline(&mut self.surname);
                 });
                 ui.horizontal(|ui| {
                     ui.label("Введите Отчетсво: ");
                     ui.text_edit_singleline(&mut self.o);
                 });
                 if ui.button("Сохранить").clicked() {
                     //println!("{} {} {}",self.surname, self.name, self.o);
                     self.show_confirmation = match self.save_name(){
                         Ok(a) => a,
                         Err(e) => {
                             eprintln!("Error: {}",e);
                             false
                         }
                     };
                     if !self.show_confirmation {
                        self.name = "".to_owned();
                        self.surname = "".to_owned();
                        self.o = "".to_owned();
                     }
                 }
                 ui.label(format!("Бесплатных запусков осталось: {}",self.counter.count_left()));
    //             ui.label(format!("Hello {} {} {}",self.surname, self.name, self.o));
             } else {
                 ui.label("Пробная версия закончилась\nКупите лицензию");
             }

         });
         if self.show_confirmation {
             egui::Window::new("File already contains this name")
                 .collapsible(false)
                 .resizable(false)
                 .show(ctx, |ui| {
                     if ui.button("Ok").clicked() {
                         self.show_confirmation = false;
                     }
             });
         }
     }
 }


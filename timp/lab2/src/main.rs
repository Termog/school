#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod trial;
use trial::{LaunchCounter,TimeCounter};
use eframe::egui;
use std::fs;
use std::io::prelude::*;
use std::thread;
use std::time::{Duration,SystemTime};
use std::process;
use std::panic;
use std::sync::{Arc, Mutex,mpsc,mpsc::channel};
use dirs_next::document_dir;

fn main() {
    let (tx, rx) = channel();
    let time_mutex = Arc::new(Mutex::new(0 as u32));
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        process::exit(1);
    }));
    let time = Arc::clone(&time_mutex);
    let mut timer = TimeCounter::from_threshold_sec(180).unwrap();
    let now = SystemTime::now();
    thread::spawn(move || {
        loop {
            match now.elapsed() {
                Ok(elapsed) => {
                    timer.log_time(elapsed.as_secs() as u32).unwrap();
                    let mut time = time.lock().unwrap();
                    *time = timer.time_left_sec();
                }
                Err(e) => {
                    println!("Error {:?}",e);
                }
            };
            if !timer.check() {
                tx.send(()).unwrap();
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    let mut counter = LaunchCounter::from_threshold(5).unwrap();
    let access = if counter.check() {
        //println!("Unlocked, free launches left: {}",counter.count_left());
        true
    } else {
        //println!("Free Trial ended buy our fucking license you cheap fuck");
        false
    };
    counter.log().unwrap();
    let mut options = eframe::NativeOptions::default();
    options.vsync = false;
    options.renderer = eframe::Renderer::Wgpu;
    eframe::run_native(
        "Lab2",
        options,
        Box::new(move |_cc| Box::new(App::default_counter(counter,access,time_mutex,rx))),
    );
}

struct App {
    name: String,
    surname: String,
    o: String,
    show_confirmation: bool,
    counter: LaunchCounter,
    access: bool,
    time: Arc<Mutex<u32>>,
    rx: mpsc::Receiver<()>,
}

impl App {
    fn default_counter(counter: LaunchCounter,access: bool,time: Arc<Mutex<u32>>,rx: mpsc::Receiver<()>) -> Self {
        App {
            name: "".to_owned(),
            surname: "".to_owned(),
            o: "".to_owned(),
            show_confirmation: false,
            counter: counter,
            access: access,
            time: time,
            rx: rx,
        }
    }
    fn save_name(&self) -> Result<bool,std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(document_dir().unwrap().as_path().join("Names.txt"))?;
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
             if Ok(()) == self.rx.try_recv() {
                 self.access = false;
             }
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
                 let time = self.time.lock().unwrap();
                 ui.label(format!("Бесплатных запусков осталось: {}",self.counter.count_left()));
                 ui.label(format!("Бесплатного времени осталось: {}:{}",*time/60,*time%60));
                 ctx.request_repaint();
             } else {
                 ui.label("Пробная версия закончилась\nКупите лицензию");
             }

         });
         if self.show_confirmation {
             egui::Window::new("Данное имя уже содержится в файле")
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


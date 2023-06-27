//library with wrapper functions for ioctl libc set and get atribute functions
mod ioctl;

//std modules to work with files and io
use std::fs;
use std::env::args;
use std::io::prelude::*;
use std::io::stdin;
use std::io::Write;

//crate for hashing passwords with argon2 algorithm
use argon2::{self, Config};

use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::OpenOptionsExt;

//crate for finding files with regex
use file_matcher::FilesNamed;

//crate to escalte to root permissions
use sudo;
//create to get password from terminal
use rpassword;

fn main() -> std::io::Result<()> {
    //put comand line arguments inside a vector
    let args: Vec<_> = args().collect();
    //match arguments to valid comands and print help otherwise
    if args.len() > 1 {
        match args[1].as_str() {
            "lock" => lock_files()?,
            "unlock" => unlock_files()?,
            "init" => create_file()?,
            _ => print_help(),
        }
    } else {
        print_help();
    }
    Ok(())
}

//function that prints help information
fn print_help() {
    println!(
        "usage: part_one <command>\n\ncommands:\n\tinit\t\tcreate config file in current directory\
        \n\tlock\t\tlock files listed in template.tbl from being modifyed or deleted\
        \n\tunlock\t\tunlock locked files")
}

//function that locks files
fn lock_files() -> std::io::Result<()> {
    //relaunch program with sudo if not onought rights
    sudo::escalate_if_needed().unwrap();
    //open config file
    let mut config = match fs::OpenOptions::new().read(true).write(true).open("template.tbl") {
        Ok(f) => f,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound  => {
            println!("No template.tbl file, try part_one init");
            return Ok(());
        },
        Err(e) => {
            return Err(e);
        },
    };
    //read content of configfile inside a string
    let mut content = String::new();
    config.read_to_string(&mut content)?;
    //split each line of content and put inside vector
    let content: Vec<&str> = content.split('\n').collect();
    //check if password exists
    let password_exists = content[0] != "";
    let mut content = content.iter();
    let _ = content.next_back().unwrap();
    //create a new config vector
    let mut confile: Vec<String> = Vec::new();
    //if password is set ask user for password and validate
    if password_exists {
        //extract hash from content vector
            let hash = content.next().unwrap().trim_end();
            //read password from terminal
            let password = rpassword::prompt_password("Input password: ").unwrap();
           //verify password
            let correct_password = argon2::verify_encoded(&hash, password.trim_end().as_bytes()).unwrap();
            //if password is incorrect exit program
            if correct_password == false {
                eprintln!("Incorrect password");
                return Ok(())
            }
            //push password to the new config vector
     //       confile.push(format!("{}\n",hash.to_string()));
    } else {
        //if the password is not set put empty line in new config vector
        let _ = content.next().unwrap();
      //  confile.push("\n".to_string());
    }
    //iterate over files in config vector
    for file in content {
        //try to open file
        let filenames = FilesNamed::wildmatch(*file)
                 .within("./")
                 .find().unwrap();
        for filename in filenames.iter() {
            let fil = match fs::File::open(&filename) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: {}",e);
                    continue;
                }
            };
            //try to get metadata
            let metadata = match fil.metadata() {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error: {}",e);
                    continue;
                }
            };
            //push file name and old permissions to new config vector
            confile.push(format!("{}:{:o}\n",filename.clone().into_os_string().into_string().unwrap(),metadata.permissions().mode()));
        //set permissions to 000
            fil.set_permissions(std::fs::Permissions::from_mode(0o000))?;
            //make the file immutable
            match ioctl::set_immut_atribute(&fil) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error: {}",e);
                    continue;
                }
            };
        }
    }
    //set permissions of config file
    fs::set_permissions("template.tbl",fs::Permissions::from_mode(0o000))?;

    println!("Files locked");
    //clear old config file and put new config information
    /*
    config.seek(std::io::SeekFrom::Start(0))?;
    */
    let mut fileperms =  fs::OpenOptions::new().create(true).read(true).write(true).open(".fileperms").unwrap();

    for line in confile {
        fileperms.write_all(line.as_bytes())?;
    }
    fs::set_permissions(".fileperms",fs::Permissions::from_mode(0o000))?;
    match ioctl::set_immut_atribute(&config) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}",e);
        }
    };
    match ioctl::set_immut_atribute(&fileperms) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}",e);
        }
    };
    Ok(())
}

fn unlock_files() -> std::io::Result<()> {
    sudo::escalate_if_needed().unwrap();
    let tempfile = std::fs::File::open("template.tbl")?;
    match ioctl::unset_immut_atribute(&tempfile) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}",e);
        }
    };
    fs::set_permissions("template.tbl",fs::Permissions::from_mode(0o600))?;
    let mut config = match fs::OpenOptions::new().read(true).write(true).open("template.tbl") {
        Ok(f) => f,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound  => {
            println!("No template.tbl file, try filelock init");
            return Ok(());
        },
        Err(e) => {
            return Err(e);
        },
    };
    let tempfile = std::fs::File::open(".fileperms")?;
    match ioctl::unset_immut_atribute(&tempfile) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}",e);
        }
    };
    fs::set_permissions(".fileperms",fs::Permissions::from_mode(0o600))?;
    let mut fileperms = match fs::OpenOptions::new().read(true).write(true).open(".fileperms") {
        Ok(f) => f,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound  => {
            println!("No template.tbl file, try filelock init");
            return Ok(());
        },
        Err(e) => {
            return Err(e);
        },
    };
    let mut content = String::new();

    config.read_to_string(&mut content)?;

    let content: Vec<&str> = content.split('\n').collect();


    let password_exists = content[0] != "";

    let mut content = content.iter();
    let _ = content.next_back().unwrap();
    //let mut confile: Vec<String> = Vec::new();
    
    if password_exists {
            let hash = content.next().unwrap().trim_end();
            let password = rpassword::prompt_password("Input password: ").unwrap();
            print!("{}",password);
            let correct_password = argon2::verify_encoded(&hash, password.as_bytes()).unwrap();
            if correct_password == false {
                eprintln!("Incorrect password");
                return Ok(())
            }
            //confile.push(format!("{}\n",hash.to_string()));
    } else {
        let _ = content.next().unwrap();
        //confile.push("\n".to_string());
    }
    let mut perms = String::new();
    fileperms.read_to_string(&mut perms)?;
    let perms: Vec<&str> = perms.split('\n').collect();
    let mut perms = perms.iter();
    let _ = perms.next_back().unwrap();
    for line in perms {
        let mut line = line.split_terminator(':');
        let mode = line.next_back().unwrap();
        let mode = u64::from_str_radix(mode, 8).unwrap();
        let file: String = line.collect();
        let fil = match std::fs::File::open(&file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: {}",e);
                continue;
            }
        };
        match ioctl::unset_immut_atribute(&fil) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error: {}",e);
                continue;
            }
        };
        fs::set_permissions(&file,fs::Permissions::from_mode(mode as u32))?;
     //   confile.push(format!("{}\n",file));
    }
    println!("Files unlocked");
    fileperms.set_len(0)?;
    fileperms.seek(std::io::SeekFrom::Start(0))?;
    /*for line in confile {
        config.write_all(line.as_bytes())?;
    }
    */
    Ok(())
}

fn create_file() -> std::io::Result<()> {
    let stdin = stdin();
    println!("Creating configuration file...");
    let mut file = fs::OpenOptions::new().create(true).mode(0o600).write(true).open("template.tbl")?;
    print!("Password protect program (y/n): ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    let ch = input.chars().next();
    match ch {
        Some(a) => {
            if !(a == 'n' || a == 'N') {
                let password = rpassword::prompt_password("Input password: ").unwrap();
                let config = Config::default();
                let salt = b"randomsalt";
                //hashing the passwrod
                let mut hash = argon2::hash_encoded(password.as_bytes(),salt, &config).unwrap();
                hash.push('\n');
                //writing password to file
                file.write_all(hash.as_bytes())?;
            } else {
                file.write(b"\n")?;
            }
        }
        None => {}
    }
    let mut filenames = String::new();
    println!("Input coma separated names of files that you want to secure");
    std::io::stdout().flush()?;
    //reading file names to a String
    stdin.read_line(&mut filenames)?;
    let mut files: Vec<String> = Vec::new();
    // spliting file names by , trimming all whitespace and adding to array
    for file in filenames.split(',') {
        files.push(file.trim().to_string());
    }
    for fil in files {
        file.write(fil.as_bytes())?;
        file.write(b"\n")?;
    }
    Ok(())
}

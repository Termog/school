use registry::{Hive,Security};
use std::{str,process::Command};

fn main() {
    #[allow(non_snake_case)]
    let MachineGuid = Hive::LocalMachine
        .open(r"SOFTWARE\Microsoft\Cryptography" ,Security::Read)
        .unwrap()
        .value("MachineGuid")
        .unwrap();

    println!("{}",MachineGuid);
    let uuid = Command::new("cmd")
        .args(["/C", "wmic csproduct get UUID"])
        .output()
        .unwrap()
        .stdout;
    let uuid = str::from_utf8(&uuid).unwrap();
    let uuid = uuid.lines().collect::<Vec<&str>>().get(1).unwrap().trim();
    println!("{}",uuid);
    let mb_uuid = Command::new("cmd")
        .args(["/C", "wmic baseboard get serialnumber"])
        .output()
        .unwrap()
        .stdout;
    let mb_uuid = str::from_utf8(&mb_uuid).unwrap();
    let mb_uuid = mb_uuid.lines().collect::<Vec<&str>>().get(1).unwrap().trim();
    println!("{}",mb_uuid);
}

use registry::{Hive,Security,Data,key,value};

pub struct LaunchCounter {
    threshold: u32,
    launch_count: u32,
}

pub struct TimeCounter {
    threshold_sec: u32,
    base_time_sec: u32,
    time_spent_sec: u32,
}

#[derive(Debug)]
pub enum Error {
    PermissionDenied,
    UnknownVal(registry::value::Error),
    UnknownKey(registry::key::Error),
    RegistryCorrupted,
}

impl TimeCounter {
    pub fn from_threshold_sec(time_threshold: u32) -> Result<Self,Error> {
        Ok(Self {
            threshold_sec: time_threshold,
            base_time_sec: get_registry_value_count(r"timecounter")?,
            time_spent_sec: 0
        })
    }
    pub fn time_left_sec(& self) -> u32 {
        self.threshold_sec - (self.time_spent_sec + self.base_time_sec)
    }
    pub fn log_time(&mut self,time: u32) -> Result<(), Error> {
        write_registry_value_count(r"timecounter",self.base_time_sec + time)?;
        self.time_spent_sec = time;
        Ok(())
    }
    pub fn check(&self) -> bool {
        self.time_spent_sec + self.base_time_sec < self.threshold_sec
    }
}

impl LaunchCounter {
    pub fn from_threshold(launch_threshold: u32) -> Result<Self,Error> {
        Ok(Self {
            threshold: launch_threshold,
            launch_count: get_registry_value_count(r"launchcounter")?,
        })
    }

    pub fn count_left(& self) -> u32 {
        self.threshold - self.launch_count
    }


    pub fn log(&mut self) -> Result<(), Error> {
        write_registry_value_count(r"launchcounter",self.launch_count + 1)?;
        self.launch_count += 1;
        Ok(())
    }

    pub fn check(&self) -> bool {
        self.launch_count < self.threshold
    }
}

fn get_registry_value_count(value_name: &str) -> Result<u32, Error> {
    let registry_key = match Hive::CurrentUser
        .open(r"SOFTWARE\Lab2",Security::Read) {
            Ok(key) => key,
            Err(_e @ key::Error::NotFound(_,_)) => create_registry_key(value_name)?,
            Err(_e @ key::Error::PermissionDenied(_,_)) => {
                return Err(Error::PermissionDenied);
  
            }
            Err(e) => {
                return Err(Error::UnknownKey(e));
            }
        };
    let value = match registry_key.value(value_name)  {
        Ok(ref _d @ Data::U32(val)) => val,
        Ok(_) => {
            return Err(Error::RegistryCorrupted);
        }
        Err(_e @ value::Error::NotFound(_,_)) => create_registry_value(value_name)?,
        Err(e) => {
            return Err(Error::UnknownVal(e));
        }
    };
    Ok(value)
}

fn create_registry_key(value_name: &str) -> Result<registry::RegKey, Error> {
    let registry_key = match Hive::CurrentUser
        .create(r"SOFTWARE\Lab2",Security::Write|Security::Read) {
            Ok(key) => key,
            Err(_e @ key::Error::PermissionDenied(_,_)) => {
                return Err(Error::PermissionDenied);
            }
            Err(e) => {
                return Err(Error::UnknownKey(e));
            }
    };
    match registry_key.set_value(value_name, &Data::U32(0)) {
        Ok(_) => (),
        Err(e) => {
            return Err(Error::UnknownVal(e));
        }
    }
    Ok(registry_key)
}
fn create_registry_value(value_name: &str) -> Result<u32, Error> {
    let registry_key = match Hive::CurrentUser
        .open(r"SOFTWARE\Lab2",Security::Write) {
            Ok(key) => key,
            Err(_e @ key::Error::PermissionDenied(_,_)) => {
                return Err(Error::PermissionDenied);
  
            }
            Err(e) => {
                return Err(Error::UnknownKey(e));
            }
        };
    match registry_key.set_value(value_name, &Data::U32(0)) {
        Ok(_) => (),
        Err(e) => {
            return Err(Error::UnknownVal(e));
        }
    }
    Ok(0)
}
//written this with 1% brain capacity
//TODO check Error handeling later
fn write_registry_value_count(value_name: &str,count: u32) -> Result<(), Error> {
    let registry_key = match Hive::CurrentUser
        .open(r"SOFTWARE\Lab2",Security::Write) {
            Ok(key) => key,
            Err(_e @ key::Error::NotFound(_,_)) => create_registry_key(value_name)?,
            Err(_e @ key::Error::PermissionDenied(_,_)) => {
                return Err(Error::PermissionDenied);
            }
            Err(e) => {
                return Err(Error::UnknownKey(e));
            }
        };
    match registry_key.set_value(value_name,&Data::U32(count)) {
        Ok(_) => (),
        Err(e) => {
            return Err(Error::UnknownVal(e));
        }
    }
    Ok(())
}

#[test]
fn test_create_registry_value() {
    let error = create_registry_value();
    eprintln!("{:?}",error.err());
}


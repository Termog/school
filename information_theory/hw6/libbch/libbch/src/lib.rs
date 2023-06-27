use libbch_sys::{bch_control, decode_bch, encode_bch, init_bch};
use std::alloc::alloc;
use std::{alloc::Layout, ptr};

pub struct Bch {
    control: *mut bch_control,
}

#[derive(Debug)]
pub enum Error {
    BadMessage,
    EinVal,
    Unknown,
}

impl Bch {
    pub fn new(m: i32, t: i32, prim_poly: Option<u32>) -> Self {
        let prim_poly = match prim_poly {
            Some(p) => p,
            None => 0,
        };
        let control = unsafe { init_bch(m, t, prim_poly) };
        assert!(!control.is_null());
        Self { control }
    }

    pub fn get_ecc(&self, data: &[u8]) -> Vec<u8> {
        unsafe {
            let ecc = alloc(Layout::from_size_align_unchecked(
                (*self.control).ecc_bytes as usize,
                1,
            ));
            encode_bch(self.control, data.as_ptr(), data.len() as u32, ecc);
            Vec::from_raw_parts(
                ecc,
                (*self.control).ecc_bytes as usize,
                (*self.control).ecc_bytes as usize,
            )
        }
    }

    pub fn get_errors(&self, data: &[u8], ecc: &[u8]) -> Result<Vec<u32>, Error> {
        let errloc_size = unsafe { (*self.control).t as usize };
        let errloc: *mut u32 =
            unsafe { alloc(Layout::from_size_align_unchecked(errloc_size * 4, 4)) as *mut u32 };
        let res = unsafe {
            decode_bch(
                self.control,
                data.as_ptr(),
                data.len() as u32,
                ecc.as_ptr(),
                ptr::null(),
                ptr::null(),
                errloc,
            )
        };
        match res {
            -74 => return Err(Error::BadMessage),
            -22 => return Err(Error::EinVal),
            n if n < 0 => {
                println!("FUUCKED n: {}", n);
                panic!();
            }
            n => unsafe {
                Ok(Vec::from_raw_parts(
                    errloc,
                    n as usize,
                    errloc_size as usize,
                ))
            },
        }
    }
    /* if (errloc[n] < 8*len), then n-th error is located in data and can be */
    /* corrected with statement data[errloc[n]/8] ^= 1 << (errloc[n] % 8); */
    pub fn decode_from_errors(&self, data: &mut [u8], errloc: &[u32]) {
        let border: usize = data.len() * 8;
        errloc
            .iter()
            .filter(|x| **x < border as u32)
            .for_each(|x| data[(x / 8) as usize] ^= 1 << (x % 8))
    }

    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        let mut ecc = self.get_ecc(data);
        let mut result = Vec::with_capacity(data.len() + ecc.len());
        result.append(&mut data.to_vec());
        result.append(&mut ecc);
        result
    }

    pub fn decode(&self, mut message: Vec<u8>) -> Result<Vec<u8>,Vec<u8>> {
        let split = message.len() - unsafe { (*self.control).ecc_bytes } as usize;
        let (err_message, ecc) = message.split_at_mut(split);
        match self.get_errors(err_message, ecc) {
            Ok(errors) => {
                self.decode_from_errors(err_message, &errors);
                Ok(err_message.to_vec())
            }
            Err(_) => {
                Err(err_message.to_vec())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn build_bch() {
    //     let bch = Bch::new(6, 1, Some(0b1100111));
    // }
    // #[test]
    // fn bu_bch() {
    //     println!("finished");
    //     let bch = Bch::new(6, 1, None);
    //     let mut message = vec![0, 0, 0, 0];
    //     let ecc = bch.get_ecc(&message);
    //     println!("{:?}", ecc);
    //     message[2] = 8;
    //     let errors = bch.get_errors(&message, &ecc).unwrap();
    //     println!("{:?}", errors);
    //     bch.decode_from_errors(&mut message, &errors);
    //     println!("{:?}", message);
    // }
    #[test]
    fn test_bulshit() {
        println!("finished");
        let bch = Bch::new(8,15, None);
        let message = vec![0, 0, 0, 0];
        println!("message: {:?}", message);
        let mut enc = bch.encode(&message);
        println!("enc: {:?}", enc);
        enc[1] = 127;
        enc[2] = 1;
        enc[0] = 1;
        enc[3] = 1;
        enc[6] ^= 8;
        let dec = bch.decode(enc);
        println!("dec: {:?}", dec);
    }
}

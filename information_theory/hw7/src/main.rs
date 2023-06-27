use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    ops::{BitAnd, BitXor, ShrAssign},
};

const REG_SIZE: usize = 3;
const BIT_SIZE: usize = 2;

lazy_static! {
    static ref LOOKUP_MAP: HashMap<u8, u8> = {
        let comb_num = 1_u8 << REG_SIZE;
        let mut map = HashMap::with_capacity(comb_num as usize);
        for i in 0_u8..comb_num {
            let bits = ((((i >> 2) & 1) ^ (i & 1)) << 1) | ((i >> 2 & 1) ^ (i >> 1 & 1) ^ (i & 1));
            map.insert(i, bits);
        }
        map
    };
}

struct Conv;

impl Conv {
    fn encode(m: u32, size: usize) -> (u64, usize) {
        let mut reg: u8 = 0;
        let mut out: u64 = 0;
        for i in 0..(size) {
            reg = (reg << 1) & 7;
            reg |= ((m >> i) & 1) as u8;
            out |= (*LOOKUP_MAP.get(&reg).unwrap() as u64) << (BIT_SIZE * i);
        }
        (out, size * 2)
    }
    fn decode(m: u64, size: usize) -> (u32, u64) {
        let m_size = size / BIT_SIZE;
        // dp: time steps Vec<(most likely previous state, hamming_error_count)>
        let mut dp: Vec<Vec<(i16, i16)>> = vec![vec![(-1, -1); 1 << REG_SIZE as usize]; m_size];
        let bits = *LOOKUP_MAP.get(&0).unwrap();
        dp[0][0] = (0, count_bit_err((m & 3) as i32, bits as i32) as i16);
        let bits = *LOOKUP_MAP.get(&1).unwrap();
        dp[0][1] = (0, count_bit_err((m & 3) as i32, bits as i32) as i16);
        for i in 1..(m_size) {
            for j in 0..(1 << REG_SIZE) {
                let (state, _) = dp[i - 1][j];
                if state != -1 {
                    let bits = (m >> (i * BIT_SIZE)) & 3;
                    put_path(i, bits as u8, j as u8, &mut dp);
                }
            }
        }
        let mut min = (0, i16::MAX);
        for i in 0..(1 << REG_SIZE) {
            let (_, err_count) = dp[m_size - 1][i];
            if err_count < min.1 && err_count >= 0 {
                min = (i, err_count);
            }
        }
        let mut state = min.0 as i16;
        let mut orig_m: u64 =
            (*LOOKUP_MAP.get(&(state as u8)).unwrap() as u64) << ((m_size-1) * BIT_SIZE);
        for j in (1..(m_size)).rev() {
            (state, _) = dp[j][state as usize];
            orig_m |= (*LOOKUP_MAP.get(&(state as u8)).unwrap() as u64) << ((j - 1) * BIT_SIZE);
        }
        let mut out = 0;
        let mut state = 0;
        for i in 0..m_size {
            let r_bits = (orig_m >> (i * BIT_SIZE)) & 3;
            let bits_0 = (state << 1) & 7;
            let bits_1 = (state << 1) & 7 | 1;
            if r_bits == *LOOKUP_MAP.get(&(bits_0)).unwrap() as u64 {
                state = bits_0;
            } else if r_bits == *LOOKUP_MAP.get(&(bits_1)).unwrap() as u64 {
                out |= 1 << i;
                state = bits_1;
            } else {
                panic!()
            }
        }
        (out, orig_m)
    }
}

fn put_path(step: usize, r_bits: u8, state: u8, dp: &mut Vec<Vec<(i16, i16)>>) {
    let inp_0 = (state << 1) & 7;
    let inp_1 = ((state << 1) & 7) | 1;
    let (_, err_count) = dp[step - 1][state as usize];
    let bits_0 = *LOOKUP_MAP.get(&inp_0).unwrap();
    let bits_1 = *LOOKUP_MAP.get(&inp_1).unwrap();
    let calc_err_0 = count_bit_err(r_bits as i32, bits_0 as i32) as i16;
    let (state_0, err_0) = dp[step][inp_0 as usize];
    if state_0 == -1 {
        dp[step][inp_0 as usize] = (state as i16, err_count + calc_err_0);
    } else {
        if err_count + calc_err_0 < err_0 {
            dp[step][inp_0 as usize] = (state as i16, err_count + calc_err_0);
        }
    }
    let calc_err_1 = count_bit_err(r_bits as i32, bits_1 as i32) as i16;
    let (state_1, err_1) = dp[step][inp_1 as usize];
    if state_1 == -1 {
        dp[step][inp_1 as usize] = (state as i16, err_count + calc_err_1);
    } else {
        if err_count + calc_err_1 < err_1 {
            dp[step][inp_1 as usize] = (state as i16, err_count + calc_err_1);
        }
    }
}

fn count_bit_err<T>(a: T, b: T) -> usize
where
    T: BitXor,
    <T as BitXor>::Output: BitAnd<i32>,
    <T as BitXor>::Output: ShrAssign<i32>,
    <T as BitXor>::Output: PartialEq<i32>,
    <<T as BitXor>::Output as BitAnd<i32>>::Output: PartialEq<i32>,
    <T as BitXor>::Output: Copy,
{
    let mut xored = a ^ b;
    let mut count = 0;
    while xored != 0 {
        if xored & 1 == 1 {
            count += 1;
        }
        xored >>= 1;
    }
    count
}

fn get_errors(a: u64, b: u64, size: usize) -> Vec<usize> {
    let mut errors = Vec::new();
    let mut xored = a ^ b;
    for i in 0..size {
        if xored & 1 == 1 {
            errors.push(i);
        }
        xored >>= 1;
        if xored == 0 {
            break;
        }
    }
    errors
}

fn main() {
    println!("Part One:");
    let message = 0b10101101100110;
    println!("Initial Message:\t{:#0width$b}", message, width = 16);
    let encoded = Conv::encode(message, 14);
    println!(
        "Encoded Message:\t{:#0width$b}",
        encoded.0,
        width = encoded.1+2
    );
    let (decoded, _) = Conv::decode(encoded.0, encoded.1);
    println!("Decoded Message:\t{:#0width$b}", decoded, width = 16);
    assert_eq!(message, decoded);
    println!("\nPart Two:");
    let message: u64 = 0b1101111001001010111110001011110111001101;
    println!("Encoded Message:\t{:#0width$b}", message, width = 40);
    let (decoded, m_orig) = Conv::decode(message, 40);
    let errors = get_errors(message, m_orig, 40);
    println!("Error Indices:\t\t{:?}", errors);
    println!("Corrected Message:\t{:#0width$b}", message, width = 40);
    println!("Decoded Message:\t{:#0width$b}", decoded, width = 20);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_err_count() {
        assert_eq!(2, count_bit_err(0b11_i32, 0b00_i32));
        assert_eq!(1, count_bit_err(0b10, 0b00));
        assert_eq!(1, count_bit_err(0b01, 0b00));
        assert_eq!(1, count_bit_err(0b11, 0b10));
        assert_eq!(0, count_bit_err(0b11, 0b11));
    }
}

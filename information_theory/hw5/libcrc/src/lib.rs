pub fn crc(mut m: u32, gx: u32) -> u32 {
    let gx_len = byte_len(gx);
    m <<= gx_len-1;
    let mut m_len = byte_len(m);
    let mut rem = if m_len < gx_len {
        m
    } else {
        let rem = m>>(m_len-gx_len);
        m_len -= gx_len;
        rem
    };
    loop {
        let rem_len = byte_len(rem);
        if rem_len == gx_len {
            rem ^= gx;
        }         
        let offset = gx_len - byte_len(rem);
        if m_len == 0 {
            break;
        } else if m_len < offset {
            let mask = byte_mask_right(m_len);
            rem <<= m_len;
            rem |= mask & m;
            m_len = 0;
        } else {
            rem <<= offset;
            let mask = byte_mask_right(offset);
            m_len -= offset;
            rem |= mask & (m>>(m_len));
        }
    }
    rem
}


fn byte_mask_right(x: u32) -> u32 {
    let mut mask = 0;
    for i in 0..x {
        mask |= 1<<i;
    }
    mask
}

fn byte_len(x: u32) -> u32 {
    for i in (0..32).rev() {
        if (1 << i) & x != 0 {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn div_rem() {
        let gx = 0b1101;
        let m = 0b100100;
        assert_eq!(0b001, crc(m, gx));
        let gx = 0b111;
        let m = 0b110001;
        assert_eq!(0b0, crc(m, gx));
        let gx = 0b1101;
        let m = 0b1001010;
        assert_eq!(0b101, crc(m, gx));
        let gx = 0b1101;
        let m = 4;
        assert_eq!(0b11, crc(m,gx));
        let gx = 0b1101;
        let m = 6;
        assert_eq!(0b100, crc(m,gx));
    }
    #[test]
    fn byte_mask() {
        assert_eq!(0b1111, byte_mask_right(4));
    }
    #[test]
    fn len() {
        assert_eq!(3, byte_len(0b101));
        assert_eq!(4, byte_len(0b1000));
        assert_eq!(8, byte_len(0b11111111));
        assert_eq!(0, byte_len(0));
    }
}

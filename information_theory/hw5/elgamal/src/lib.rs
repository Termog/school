use libcrc::crc;
use rand::prelude::*;

const P: i64 = 139;
const G: i64 = 22;
const X: i64 = 3;

#[derive(Debug)]
pub struct SignedMessage {
    message: u32,
    r: i64,
    s: i64,
}
impl SignedMessage {
    pub fn sign(m: u32, gx: u32) -> Option<SignedMessage> {
        let h = crc(m, gx);
        assert!(h > 1 && (h as i64) < P);
        let k = random_comprime(P - 1);
        assert!(k > 1 && k < P - 1);
        let r = powmod(G, k, P);
        let u = (h as i64 - X * r).rem_euclid(P - 1);
        let s = (mod_inverse(k, P - 1)? * u).rem_euclid(P - 1);
        Some(SignedMessage { message: m, r, s })
    }
    pub fn check(&self, gx:u32) -> bool {
        let h = crc(self.message, gx);
        let y = powmod(G, X, P);
        let left = (powmod(y, self.r, P) * powmod(self.r, self.s, P)).rem_euclid(P);
        let right = powmod(G, h as i64, P);
        left == right
    }
    pub fn get_message(&self) -> u32 {
        self.message
    }
}

fn powmod(mut x: i64, mut y: i64, p: i64) -> i64 {
    let mut r = 1;
    x %= p;
    if x == 0 {
        return 0;
    }
    while y > 0 {
        if y & 1 == 1 {
            r = (r * x) % p;
        }
        y >>= 1;
        x = (x * x) % p;
    }
    return r;
}

fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let r: i64 = a % b;
    if r == 0 {
        (0, 1, b)
    } else {
        let q: i64 = a / b;
        let (s, t, g) = gcd(b, r);
        (t, s - q * t, g)
    }
}

fn random_comprime(x: i64) -> i64 {
    let mut rng = thread_rng();
    loop {
        let f = rng.gen_range(2..x);
        let (_, _, gcd) = gcd(x.into(), f.into());
        if gcd == 1 {
            return f;
        }
    }
}

fn mod_inverse(a: i64, b: i64) -> Option<i64> {
    let (s, _, gcd) = gcd(a, b);
    if gcd != 1 {
        None
    } else {
        Some((s % b + b) % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comprimes() {
        let f = random_comprime(5);
        let (_, _, g) = gcd(f.into(), 5);
        assert_eq!(1, g);
    }
    #[test]
    fn power() {
        assert_eq!(1, powmod(157, 10, 12));
        assert_eq!(154, powmod(15745456, 1045, 186));
    }
    #[test]
    fn signature() {
        let gx = 0b10111;
        let m = SignedMessage::sign(215,gx).unwrap();
        assert!(m.check(gx));
    }
}

#[derive(Clone, Copy)]
pub struct Key {
    pub prime: i32,
    pub mod_inverse: i32,
    pub random: i32,
}

#[derive(Clone, Copy)]
pub struct Keys {
    pub contacts: Key,
    pub orders: Key,
    pub services: Key,
    pub workers: Key,
}

impl Key {
    pub fn new(prime: i32, random: i32) -> Self {
        Self {
            prime,
            mod_inverse: Self::mod_inverse(prime),
            random,
        }
    }

    fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = Self::egcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }

    fn mod_inverse(a: i32) -> i32 {
        const MAX: i64 = i32::MAX as i64 + 1;
        let (g, x, _) = Self::egcd(a as i64, MAX);
        if g != 1 {
            panic!("Invalid prime number")
        } else {
            ((x % MAX + MAX) % MAX) as i32
        }
    }
}

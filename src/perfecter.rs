use rug::{Float, Integer, Rational};
use std::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Perfecter {
    expected_duration: Duration,
    actual_duration: Duration,
}

impl Perfecter {
    pub fn expected_duration(&self) -> &Duration {
        &self.expected_duration
    }

    pub fn actual_duration(&self) -> &Duration {
        &self.actual_duration
    }

    pub fn rational(&self) -> Rational {
        Rational::from((
            Integer::from(self.expected_duration().as_nanos()),
            Integer::from(self.actual_duration().as_nanos()),
        ))
    }

    pub fn perfect<T>(&self, value: &T) -> T
    where
        T: Perfect,
    {
        Perfect::perfect(value, self)
    }
}

pub trait Perfect {
    fn perfect(&self, perfecter: &Perfecter) -> Self;
}

macro_rules! def_int_conv {
    ( $t:ty, $i:ident ) => {
        impl Perfect for $t {
            fn perfect(&self, perfecter: &Perfecter) -> Self {
                let (n, d) = (perfecter.rational() * Rational::from((*self, 1))).into_numer_denom();
                let n = Float::with_val(52, n.$i().unwrap());
                let d = Float::with_val(52, d.$i().unwrap());
                (n / d).to_integer().unwrap().$i().unwrap()
            }
        }
    };
}

def_int_conv!(isize, to_isize);
def_int_conv!(usize, to_usize);
def_int_conv!(i8, to_i8);
def_int_conv!(u8, to_u8);
def_int_conv!(i16, to_i16);
def_int_conv!(u16, to_u16);
def_int_conv!(i32, to_i32);
def_int_conv!(u32, to_u32);
def_int_conv!(i64, to_i64);
def_int_conv!(u64, to_u64);
def_int_conv!(i128, to_i128);
def_int_conv!(u128, to_u128);

impl Perfect for f32 {
    fn perfect(&self, perfecter: &Perfecter) -> Self {
        (Float::with_val(52, self) * perfecter.rational()).to_f32()
    }
}

impl Perfect for f64 {
    fn perfect(&self, perfecter: &Perfecter) -> Self {
        (Float::with_val(52, self) * perfecter.rational()).to_f64()
    }
}

pub struct PerfecterProvider {
    expected_duration: Duration,
    prev: RwLock<Option<Instant>>,
}

impl PerfecterProvider {
    pub fn new(duration: Duration) -> PerfecterProvider {
        PerfecterProvider {
            expected_duration: duration,
            prev: RwLock::new(None),
        }
    }

    fn get_prev(&self) -> Option<Instant> {
        *(self.prev.read().unwrap())
    }

    fn set_prev(&self, prev: Option<Instant>) {
        let mut p = self.prev.write().unwrap();
        *p = prev;
    }

    pub fn get(&self) -> Option<Perfecter> {
        match self.get_prev() {
            Some(prev) => {
                let now = Instant::now();
                let dur = now - prev;
                let res = Perfecter {
                    expected_duration: self.expected_duration,
                    actual_duration: dur,
                };
                self.set_prev(Some(now));
                Some(res)
            }
            None => {
                self.set_prev(Some(Instant::now()));
                None
            }
        }
    }
}

unsafe impl Send for PerfecterProvider {}
unsafe impl Sync for PerfecterProvider {}

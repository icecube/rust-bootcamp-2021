use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{PartialEq, PartialOrd};

pub trait Distance {
    fn factor() -> f32;
    fn val(&self) -> f32;
    fn to_base(&self) -> Meters {
        Meters(self.val()*Self::factor())
    }
}

macro_rules! impl_ops {
    ($ty:ty) => {
        impl<T: Distance> Add<T> for $ty {
            type Output = Self;
            fn add(self, other: T) -> Self {
                Self((self.to_base().val() + other.to_base().val())/Self::factor())
            }
        }

        impl<T: Distance> Sub<T> for $ty {
            type Output = Self;
            fn sub(self, other: T) -> Self {
                Self((self.to_base().val() - other.to_base().val())/Self::factor())
            }
        }

        impl Mul<f32> for $ty {
            type Output = Self;
            fn mul(self, other: f32) -> Self {
                Self(self.val() * other)
            }
        }

        impl Div<f32> for $ty {
            type Output = Self;
            fn div(self, other: f32) -> Self {
                Self(self.val() / other)
            }
        }

        impl<T: Distance> PartialOrd<T> for $ty {
            fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
                self.to_base().val().partial_cmp(&other.to_base().val())
            }
        }

        impl<T: Distance> PartialEq<T> for $ty {
            fn eq(&self, other: &T) -> bool {
                self.to_base().val() == other.to_base().val()
            }
        }
    };
}


#[derive(Debug)]
pub struct Meters(f32);
impl Distance for Meters {
    fn val(&self) -> f32 { self.0 }
    fn factor() -> f32 { 1. }
}
impl_ops!(Meters);

#[derive(Debug)]
pub struct Kilometers(f32);
impl Distance for Kilometers {
    fn val(&self) -> f32 { self.0 }
    fn factor() -> f32 { 1000. }
}
impl_ops!(Kilometers);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Meters(12.);
        let b = Kilometers(1.);
        assert_eq!(a + b, Meters(1012.));
    }

    #[test]
    fn sub() {
        let a = Meters(12.);
        let b = Kilometers(1.);
        assert_eq!(a - b, Meters(-988.));
    }

    #[test]
    fn mul() {
        let a = Meters(12.);
        assert_eq!(a * 10., Meters(120.));
    }

    #[test]
    fn mul2() {
        let a = Kilometers(12.);
        assert_eq!(a * 10., Kilometers(120.));
    }

    #[test]
    fn div() {
        let a = Kilometers(1.);
        assert_eq!(a / 10., Meters(100.));
    }

    #[test]
    fn eq() {
        let a = Meters(1000.);
        let b = Kilometers(1.);
        assert!(a == b);
    }

    #[test]
    fn eq2() {
        let a = Meters(1000.);
        let b = Kilometers(1.);
        assert!(b == a);
    }

    #[test]
    fn cmp() {
        let a = Meters(1000.);
        let b = Kilometers(2.);
        assert!(a < b);
    }

    #[test]
    fn cmp2() {
        let a = Meters(10000.);
        let b = Kilometers(2.);
        assert!(a > b);
    }
}

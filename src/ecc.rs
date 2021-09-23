use crate::errors::ValueError;
use std::cmp::PartialEq;
use std::ops::{Add, Neg, Sub};

#[derive(Clone, Debug, Copy)]
pub struct FieldElement {
    pub num: u128,
    pub prime: u128,
}

pub trait FieldElementOps {
    fn new(num: u128, prime: u128) -> Result<FieldElement, ValueError>;
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: Self) -> Self {
        FieldElement {
            num: (self.num + rhs.num) % self.prime,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: Self) -> Self {
        let new_num = if self.num < rhs.num {
            self.prime - ((rhs.num - self.num) % self.prime)
        } else {
            self.num - rhs.num
        };
        FieldElement {
            num: new_num,
            prime: self.prime,
        }
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> Self {
        FieldElement {
            num: self.prime - (self.num % self.prime),
            prime: self.prime,
        }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl FieldElementOps for FieldElement {
    fn new(num: u128, prime: u128) -> Result<FieldElement, ValueError> {
        match num >= prime {
            true => Err(ValueError {
                message: format!("num {} not in field range 0 to {}", num, prime - 1),
            }),
            false => Ok(FieldElement { num, prime }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(13, 13);
        let c = FieldElement::new(14, 13);
        let d = FieldElement::new(0, 13);
        assert!(a.is_ok());
        assert!(b.is_err());
        assert!(c.is_err());
        assert!(d.is_ok());
    }

    #[test]
    fn test_ne() {
        let a = FieldElement::new(2, 13).unwrap();
        let b = FieldElement::new(10, 13).unwrap();
        let c = FieldElement::new(2, 13).unwrap();
        assert_eq!(a, c);
        assert!(a != b);
        assert!(b != c);
    }

    #[test]
    fn test_add() {
        let mut a = FieldElement::new(2, 13).unwrap();
        let mut b = FieldElement::new(10, 13).unwrap();
        assert_eq!(a + b, FieldElement::new(12, 13).unwrap());
        a = FieldElement::new(5, 13).unwrap();
        b = FieldElement::new(12, 13).unwrap();
        assert_eq!(a + b, FieldElement::new(4, 13).unwrap());
    }

    #[test]
    fn test_sub() {
        let mut a = FieldElement::new(10, 13).unwrap();
        let mut b = FieldElement::new(2, 13).unwrap();
        assert_eq!(a - b, FieldElement::new(8, 13).unwrap());
        a = FieldElement::new(5, 13).unwrap();
        b = FieldElement::new(12, 13).unwrap();
        assert_eq!(a - b, FieldElement::new(6, 13).unwrap());
    }
}

use std::fmt::{Debug, Display};
use crate::{conversion_error, take_from_vec};
use super::errors::Error;
//use crate::{make_error, core::errors::Error};


#[derive(Clone, PartialEq, Default)]
pub struct BinaryUnit {
    data: Vec<u8>
}
impl Display for BinaryUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "len={}: {:?}", self.data.len(), &self.data)
    }
}
impl Debug for BinaryUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} byte(s) of data", self.data.len())
    }
}
impl From<Vec<u8>> for BinaryUnit {
        fn from(value: Vec<u8>) -> Self {
            Self {
                data: value
            }
        }
    }
impl BinaryUnit {
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn expose(&self) -> &[u8] {
        &self.data
    }
}
impl TryInto<i8> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i8, Self::Error> {
        if self.len() != 1 {
            return Err(conversion_error!("expected 1 byte, got {}", self.len()));
        }

        Ok( i8::from_ne_bytes([ self.data[0] ]) )
    }
}
impl TryInto<i16> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i16, Self::Error> {
        if self.len() != 2 {
            return Err(conversion_error!("expected 2 bytes, got {}", self.len()));
        }

        Ok( i16::from_ne_bytes(take_from_vec!(2, self.data, u8)) )
    }
}
impl TryInto<i32> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i32, Self::Error> {
        if self.len() != 4 {
            return Err(conversion_error!("expected 4 bytes, got {}", self.len()));
        }

        Ok( i32::from_ne_bytes(take_from_vec!(4, self.data, u8)) )
    }
}
impl TryInto<i64> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i64, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( i64::from_ne_bytes(take_from_vec!(8, self.data, u8)) )
    }
}
impl TryInto<i128> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i128, Self::Error> {
        if self.len() != 16 {
            return Err(conversion_error!("expected 16 bytes, got {}", self.len()));
        }

        let extracted = take_from_vec!(16, self.data, u8);
        Ok( i128::from_ne_bytes(extracted) )
    }
}
impl TryInto<u8> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u8, Self::Error> {
        if self.len() != 1 {
            return Err(conversion_error!("expected 1 byte, got {}", self.len()));
        }

        Ok( u8::from_ne_bytes([ self.data[0] ]) )
    }
}
impl TryInto<u16> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u16, Self::Error> {
        if self.len() != 2 {
            return Err(conversion_error!("expected 2 bytes, got {}", self.len()));
        }

        Ok( u16::from_ne_bytes(take_from_vec!(2, self.data, u8)) )
    }
}
impl TryInto<u32> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u32, Self::Error> {
        if self.len() != 4 {
            return Err(conversion_error!("expected 4 bytes, got {}", self.len()));
        }

        Ok( u32::from_ne_bytes(take_from_vec!(4, self.data, u8)) )
    }
}
impl TryInto<u64> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u64, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( u64::from_ne_bytes(take_from_vec!(8, self.data, u8)) )
    }
}
impl TryInto<u128> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u128, Self::Error> {
        if self.len() != 16 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( u128::from_ne_bytes(take_from_vec!(16, self.data, u8)) )
    }
}
impl TryInto<f32> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<f32, Self::Error> {
        if self.len() != 4 {
            return Err(conversion_error!("expected 4 bytes, got {}", self.len()));
        }

        Ok( f32::from_ne_bytes(take_from_vec!(4, self.data, u8)) )
    }
}
impl TryInto<f64> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<f64, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( f64::from_ne_bytes(take_from_vec!(8, self.data, u8)) )
    }
}
impl TryInto<usize> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<usize, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( usize::from_ne_bytes(take_from_vec!(8, self.data, u8)) )
    }
}
impl TryInto<isize> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<isize, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( isize::from_ne_bytes(take_from_vec!(8, self.data, u8)) )
    }
}

impl From<i8> for BinaryUnit {
    fn from(value: i8) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<i16> for BinaryUnit {
    fn from(value: i16) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<i32> for BinaryUnit {
    fn from(value: i32) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<i64> for BinaryUnit {
    fn from(value: i64) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<i128> for BinaryUnit {
    fn from(value: i128) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<u8> for BinaryUnit {
    fn from(value: u8) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<u16> for BinaryUnit {
    fn from(value: u16) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<u32> for BinaryUnit {
    fn from(value: u32) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<u64> for BinaryUnit {
    fn from(value: u64) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<u128> for BinaryUnit {
    fn from(value: u128) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<f32> for BinaryUnit {
    fn from(value: f32) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<f64> for BinaryUnit {
    fn from(value: f64) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<usize> for BinaryUnit {
    fn from(value: usize) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}
impl From<isize> for BinaryUnit {
    fn from(value: isize) -> Self {
        BinaryUnit::from(value.to_ne_bytes().to_vec())
    }
}

#[macro_export]
macro_rules! binary_unit {
    ($x:expr) => {
        {
            let tmp: BinaryUnit = BinaryUnit::from($x);
            tmp
        }
    }
}

#[test]
fn test_into_binary_unit() {
    let x = 43;
    let x_binary = binary_unit!(x);
    let dx: Result<i32, _> = x_binary.try_into();
    assert_eq!(dx.unwrap(), x);
}
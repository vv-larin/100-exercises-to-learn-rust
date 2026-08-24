// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    inner: u16,
}

impl From<u8> for SaturatingU16 {
    fn from(inner: u8) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(inner: &u8) -> Self {
        Self {
            inner: (*inner).into(),
        }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(inner: u16) -> Self {
        Self { inner }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(inner: &u16) -> Self {
        Self { inner: *inner }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        self.inner.saturating_add(rhs.inner).into()
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        self.inner.saturating_add(rhs).into()
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        self.inner.saturating_add(*rhs).into()
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        self.inner.saturating_add(rhs.inner).into()
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, rhs: &u16) -> bool {
        self.inner == *rhs
    }
}

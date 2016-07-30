use core::ops::*;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct u2size {
    pub msw: usize,
    pub lsw: usize,
}

impl u2size {
    #[inline]
    pub fn trailing_zeros(self) -> usize {
        (self.lsw.trailing_zeros() +
         if self.lsw == 0 { self.msw.trailing_zeros() } else { 0 }) as usize
    }

    #[inline]
    pub fn count_ones(self) -> usize {
        (self.lsw.count_ones() + self.msw.count_ones()) as usize
    }
}

impl From<usize> for u2size {
    #[inline]
    fn from(n: usize) -> Self { u2size { msw: 0, lsw: n, } }
}

impl Shr<usize> for u2size {
    type Output = Self;
    #[inline]
    fn shr(self, k: usize) -> Self {
        if k < word_bits() { u2size {
            msw: self.msw >> k,
            lsw: shrd(self.lsw, self.msw, k),
        } } else { Self::from(self.msw >> (k - word_bits())) }
    }
}

impl Shl<usize> for u2size {
    type Output = Self;
    #[inline]
    fn shl(self, k: usize) -> Self {
        if k < word_bits() { u2size {
            msw: shld(self.msw, self.lsw, k),
            lsw: self.lsw << k,
        } } else { u2size {
            msw: self.lsw << (k - word_bits()),
            lsw: 0,
        } }
    }
}

impl BitXor for u2size {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        u2size { msw: self.msw ^ other.msw, lsw: self.lsw ^ other.lsw }
    }
}

impl BitXorAssign for u2size {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) { *self = *self ^ other }
}

#[inline(always)]
fn shld(x: usize, y: usize, k: usize) -> usize {
    if k == 0 { x } else { x << k | y >> (k.wrapping_neg() & (word_bits() - 1)) }
}

#[inline(always)]
fn shrd(x: usize, y: usize, k: usize) -> usize {
    if k == 0 { x } else { x >> k | y << (k.wrapping_neg() & (word_bits() - 1)) }
}

#[inline(always)]
fn word_bits() -> usize { 0usize.count_zeros() as usize }

#![allow(dead_code)]
use anchor_lang::prelude::*;

use crate::errors::BlsError;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, InitSpace)]
pub struct BitVec {
    // Must store 86 bits for rule and the execute result.
    // the size is 86/8 + 1 = 11 bytes.
    #[max_len(11)]
    bits: Vec<u8>,

    len: u32,
}

impl BitVec {
    pub fn new(bits: usize) -> Self {
        Self::new_with_default(bits, 0)
    }
    pub fn new_with_default(bits: usize, default: u8) -> Self {
        let rem = bits % 8 == 0;
        let cap = bits / 8;
        let size = if rem { cap } else { cap + 1 };
        Self {
            bits: vec![default; size],
            len: bits as u32,
        }
    }

    /// set the bit offset to 1 or 0, true is 1 and false is 0
    /// the bit start from 0.
    pub fn set(&mut self, bit: usize, b: bool) -> Result<()> {
        if bit as u32 >= self.len {
            return Err(BlsError::OutofBound.into());
        }
        let idx = bit / 8;
        let off = bit % 8;

        let mut val = self.bits[idx];
        if b {
            val = val | (1 << off);
        } else {
            val = val & (!(1 << off));
        }
        self.bits[idx] = val;
        Ok(())
    }

    /// get the bit offset value, the result is true or false, true is 1 and false is 0
    pub fn get(&mut self, bit: usize) -> Result<bool> {
        if bit as u32 >= self.len {
            return Err(BlsError::OutofBound.into());
        }
        let idx = bit / 8;
        let off = bit % 8;
        let val = self.bits[idx];
        let bit = 1 << off;
        let rs: bool = val & bit == bit;
        Ok(rs)
    }
}

#[cfg(test)]
mod tests {
    use crate::states::bitvec::BitVec;

    #[test]
    fn test_get_set() {
        let bits = 86;
        let mut bit_vec = BitVec::new(bits);
        for i in 0..bits {
            bit_vec.set(i, true).unwrap();
            assert!(bit_vec.get(i).unwrap() == true);
        }
        for i in 0..bits {
            bit_vec.set(i, false).unwrap();
            assert!(bit_vec.get(i).unwrap() == false);
        }
    }

    #[test]
    fn test_bitmap_rule() {
        let bits = 86;
        let mut bit_vec = BitVec::new(bits);
        let mut idx = vec![];
        for i in (12..=33).step_by(3) {
            bit_vec.set(i, true).unwrap();
            idx.push(i);
        }

        for i in 0..bits {
            let v = bit_vec.get(i).unwrap();
            if idx.contains(&i) {
                assert!(v)
            } else {
                assert!(!v)
            }
        }
    }
}

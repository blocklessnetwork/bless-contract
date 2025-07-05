use anchor_lang::prelude::*;

use crate::errors::BlsError;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, InitSpace)]
pub struct BitVec {
    // Must store 86 bits for rule and the execute result.
    // the size is 86/8 + 1 = 11 bytes.
    #[max_len(11)]
    bits: Vec<u8>,
}

impl BitVec {
    pub fn new(bits: usize) -> Self {
        let rem = bits % 8 == 0;
        let cap = bits / 8;
        let size = if rem { cap } else { cap + 1 };
        Self {
            bits: vec![0; size],
        }
    }

    /// set the bit offset to 1 or 0, true is 1 and false is 0
    /// the bit start from 0.
    pub fn set(&mut self, bit: usize, b: bool) -> Result<()> {
        let idx = bit / 8;
        let off = bit % 8;
        if idx >= self.bits.len() {
            return Err(BlsError::OutofBound.into());
        }
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
        let idx = bit / 8;
        let off = bit % 8;
        if idx >= self.bits.len() {
            return Err(BlsError::OutofBound.into());
        }
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
}

#[inline(always)]
pub fn count_bits(data: u64) -> u8 {
    count_bits_manual(data)
}

#[inline(always)]
pub fn get_lsb(data: u64) -> u8 {
    get_lsb_rust(data)
}

#[inline(always)]
fn count_bits_manual(mut data: u64) -> u8 {
    let mut count = 0;
    
    while data != 0 {
        data &= data - 1;
        count += 1;
    }
    
    count
}

#[inline(always)]
fn count_bits_rust(data: u64) -> u8 {
    data.count_ones() as u8
}

#[inline(always)]
fn get_lsb_manual(data: u64) -> u8 {
    count_bits_manual((data & (!data + 1)) - 1)
}

#[inline(always)]
fn get_lsb_rust(data: u64) -> u8 {
    data.trailing_zeros() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_bits_manual_counts_correctly() {
        assert_eq!(count_bits_manual(0b0011010101111001), 9)
    }
}

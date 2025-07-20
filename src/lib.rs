pub fn round_up_fill(d: &mut [u8]) -> Option<u8> {
    match d.iter().rposition(|&c| c != b'9') {
        Some(i) => {
            // d[i+1..n] is all nines
            d[i] += 1;
            d[i + 1..].fill(b'0');
            None
        }
        None if d.is_empty() => {
            // an empty buffer rounds up (a bit strange but reasonable)
            Some(b'1')
        }
        None => {
            // 999..999 rounds to 1000..000 with an increased exponent
            d[0] = b'1';
            d[1..].fill(b'0');
            Some(b'0')
        }
    }
}

pub fn round_up_iter(d: &mut [u8]) -> Option<u8> {
    match d.iter().rposition(|&c| c != b'9') {
        Some(i) => {
            // d[i+1..n] is all nines
            d[i] += 1;
            d.iter_mut().skip(i + 1).for_each(|c| *c = b'0');
            None
        }
        None if d.is_empty() => {
            // an empty buffer rounds up (a bit strange but reasonable)
            Some(b'1')
        }
        None => {
            // 999..999 rounds to 1000..000 with an increased exponent
            d[0] = b'1';
            d.iter_mut().skip(1).for_each(|c| *c = b'0');
            Some(b'0')
        }
    }
}

pub fn round_up_for(d: &mut [u8]) -> Option<u8> {
    match d.iter().rposition(|&c| c != b'9') {
        Some(i) => {
            // d[i+1..n] is all nines
            d[i] += 1;
            for j in i + 1..d.len() {
                d[j] = b'0';
            }
            None
        }
        None if d.len() > 0 => {
            // 999..999 rounds to 1000..000 with an increased exponent
            d[0] = b'1';
            for j in 1..d.len() {
                d[j] = b'0';
            }
            Some(b'0')
        }
        None => {
            // an empty buffer rounds up (a bit strange but reasonable)
            Some(b'1')
        }
    }
}

pub fn hamming_distance(mut x: i32, mut y: i32) -> u8 {
    let mut dist = 0;

    while x != 0 || y != 0 {
        if is_first_bit_differs(x, y) {
            dist += 1;
        }

        x >>= 1;
        y >>= 1;
    }

    dist
}

fn first_bit(x: i32) -> u8 {
    return (x & 1) as u8
}

fn is_first_bit_differs(x: i32, y: i32) -> bool {
    first_bit(x) != first_bit(y)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hamming_distance_should_return_zero_for_same_numbers() {
        assert_eq! (0, hamming_distance(2, 2))
    }

    #[test]
    fn hamming_distance_for_different_numbers() {
        assert_eq! (2, hamming_distance(1, 4))
    }
}

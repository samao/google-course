pub fn luhn(cc_number: &str) -> bool {
    let mut digits = 0;
    let mut sum: u32 = 0;
    let mut odd = false;
    let mut chars = cc_number.chars().rev();
    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            continue;
        }
        if let Some(digit) = c.to_digit(10) {
            if odd {
                let double = 2 * digit;
                sum += if double > 9 { double - 9 } else { double };
            } else {
                sum += digit;
            }
            digits += 1;
            odd = !odd;
        } else {
            return false;
        }
    }
    digits >= 2 && sum % 10 == 0
}

#[cfg(test)]
mod luhn_tests {
    use super::luhn;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_non_digit_cc_number() {
        assert!(!luhn("foo"));
        assert!(!luhn("4223 9826 4026 929x"));
    }

    #[test]
    fn test_empty_cc_number() {
        assert!(!luhn(""));
        assert!(!luhn(" "));
        assert!(!luhn("  "));
        assert!(!luhn("  1  "));
    }

    #[test]
    fn test_single_digit_cc_number() {
        assert!(!luhn("0"));
    }

    #[test]
    fn test_two_digit_cc_number() {
        assert!(luhn(" 0 0 "));
    }
}

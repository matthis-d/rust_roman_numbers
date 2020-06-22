struct RomanNumber {
    base: i32,
    letter: String,
}

impl RomanNumber {
    fn new(base: i32, letter: String) -> Self {
        RomanNumber { base, letter }
    }
}

pub fn roman_numbers(input: i32) -> String {
    let mut result = String::new();
    let mut base = input;

    let roman_numbers_list = vec![
        RomanNumber::new(100, String::from("C")),
        RomanNumber::new(90, String::from("XC")),
        RomanNumber::new(50, String::from("L")),
        RomanNumber::new(40, String::from("XL")),
        RomanNumber::new(10, String::from("X")),
        RomanNumber::new(9, String::from("IX")),
        RomanNumber::new(5, String::from("V")),
        RomanNumber::new(4, String::from("IV")),
        RomanNumber::new(1, String::from("I")),
    ];

    for roman_number in roman_numbers_list {
        while base / roman_number.base > 0 {
            result += &roman_number.letter;
            base -= roman_number.base;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_1() {
        assert_eq!(roman_numbers(1), "I");
    }

    #[test]
    fn convert_2() {
        assert_eq!(roman_numbers(2), "II");
    }

    #[test]
    fn convert_3() {
        assert_eq!(roman_numbers(3), "III");
    }

    #[test]
    fn convert_4() {
        assert_eq!(roman_numbers(4), "IV");
    }

    #[test]
    fn convert_5() {
        assert_eq!(roman_numbers(5), "V");
    }

    #[test]
    fn convert_6() {
        assert_eq!(roman_numbers(6), "VI");
    }

    #[test]
    fn convert_7() {
        assert_eq!(roman_numbers(7), "VII");
    }

    #[test]
    fn convert_10() {
        assert_eq!(roman_numbers(10), "X");
    }

    #[test]
    fn convert_9() {
        assert_eq!(roman_numbers(9), "IX");
    }

    #[test]
    fn convert_15() {
        assert_eq!(roman_numbers(15), "XV");
    }

    #[test]
    fn convert_14() {
        assert_eq!(roman_numbers(14), "XIV");
    }

    #[test]
    fn convert_19() {
        assert_eq!(roman_numbers(19), "XIX");
    }

    #[test]
    fn convert_20() {
        assert_eq!(roman_numbers(20), "XX");
    }

    #[test]
    fn convert_21() {
        assert_eq!(roman_numbers(21), "XXI");
    }

    #[test]
    fn convert_39() {
        assert_eq!(roman_numbers(39), "XXXIX");
    }

    #[test]
    fn convert_50() {
        assert_eq!(roman_numbers(50), "L");
    }

    #[test]
    fn convert_40() {
        assert_eq!(roman_numbers(40), "XL");
    }

    #[test]
    fn convert_41() {
        assert_eq!(roman_numbers(41), "XLI");
    }

    #[test]
    fn convert_48() {
        assert_eq!(roman_numbers(49), "XLIX");
    }

    #[test]
    fn convert_100() {
        assert_eq!(roman_numbers(100), "C");
    }

    #[test]
    fn convert_90() {
        assert_eq!(roman_numbers(90), "XC");
    }
}

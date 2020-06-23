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
        RomanNumber::new(1000, String::from("M")),
        RomanNumber::new(900, String::from("CM")),
        RomanNumber::new(500, String::from("D")),
        RomanNumber::new(400, String::from("CD")),
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

    // This is a macro to be able to parameterize tests
    // Source: https://stackoverflow.com/a/34666891/1479908
    macro_rules! roman_numbers_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(roman_numbers(input), expected);
            }
        )*
        }
    }
    roman_numbers_tests! {
        convert_1: (1, "I"),
        convert_2: (2, "II"),
        convert_3: (3, "III"),
        convert_4: (4, "IV"),
        convert_5: (5, "V"),
        convert_6: (6, "VI"),
        convert_7: (7, "VII"),
        convert_9: (9, "IX"),
        convert_10: (10, "X"),
        convert_11: (11, "XI"),
        convert_15: (15, "XV"),
        convert_14: (14, "XIV"),
        convert_19: (19, "XIX"),
        convert_20: (20, "XX"),
        convert_21: (21, "XXI"),
        convert_39: (39, "XXXIX"),
        convert_50: (50, "L"),
        convert_40: (40, "XL"),
        convert_41: (41, "XLI"),
        convert_49: (49, "XLIX"),
        convert_100: (100, "C"),
        convert_90: (90, "XC"),
        convert_400: (400, "CD"),
        convert_500: (500, "D"),
        convert_900: (900, "CM"),
        convert_1000: (1000, "M"),
        convert_3999: (3999, "MMMCMXCIX"),
    }
}

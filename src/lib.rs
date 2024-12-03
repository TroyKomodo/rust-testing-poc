#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}


pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}


pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}


pub fn power(a: i32, b: u32) -> i32 {
    a.pow(b)
}


pub fn sqrt(a: f64) -> f64 {
    a.sqrt()
}


pub fn log(a: f64, base: f64) -> f64 {
    a.log(base)
}

pub fn test_branches(a: f64, b: f64) -> f64 {
    if a > b || a == b {
        sqrt(a) * sqrt(b) * 0.5
    } else {
        log(a, b)
    }
}

#[derive(Debug, PartialEq)]
pub enum TestEnum {
    A,
    B,
    B2,
    C,
}

pub fn test_branches_enum(a: TestEnum) -> TestEnum {
    if a == TestEnum::A {
        TestEnum::A
    } else if a == TestEnum::B || a == TestEnum::B2 {
        TestEnum::B
    } else {
        TestEnum::C
    }
}

#[cfg_attr(all(coverage_nightly, test), coverage(off))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(1, 2), -1);
        assert_eq!(subtract(2, 1), 1);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(1, 2), 2);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(1, 2), 0);
        assert_eq!(divide(2, 1), 2);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2, 3), 8);
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(4.0), 2.0);
    }

    #[test]
    fn test_log() {
        assert_eq!(log(10.0, 10.0), 1.0);
    }

    #[test]
    fn test_test_branches() {
        assert_eq!(test_branches(1.0, 2.0), 0.0);
        assert_eq!(test_branches(1.0, 1.0), 0.5);
        assert_eq!(test_branches(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_snapshot() {
        insta::assert_compact_debug_snapshot!(test_branches(2.0, 1.0));
        insta::assert_compact_debug_snapshot!(test_branches(5.0, 1.2));
        insta::assert_compact_debug_snapshot!(test_branches(10.0, 1.0));
        insta::assert_compact_debug_snapshot!(test_branches(100.0, 1.0));
    }

    #[test]
    fn test_test_branches_enum() {
        assert_eq!(test_branches_enum(TestEnum::A), TestEnum::A);
        assert_eq!(test_branches_enum(TestEnum::B), TestEnum::B);
        assert_eq!(test_branches_enum(TestEnum::B2), TestEnum::B);
        assert_eq!(test_branches_enum(TestEnum::C), TestEnum::C);
    }
}

#[derive(Default)]
pub struct StringCalculator;

impl StringCalculator {
    pub fn new() -> Self {
        StringCalculator
    }

    //足し算の実装
    pub fn add(&self, numbers: &str) -> i32 {
        if numbers.is_empty() {
            return 0; // 空文字列の場合は0を返す
        }

        let nums = numbers.split(',');

        let mut sum = 0;
        for num in nums {
            let parsed: i32 = num.parse().unwrap();
            sum += parsed;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_string_calculator() -> StringCalculator {
        StringCalculator::new()
    }

    // =========================
    // 基本的な文字列解析テスト
    // =========================
    #[test]
    fn 空文字列を足し算すると0になること() {
        // Arrange
        let calc = setup_string_calculator();

        // Act
        let result = calc.add("");

        // Assert
        assert_eq!(result, 0);
    }

    #[test]
    fn 単一の数字を解析できること() {
        // Arrange
        let calc = setup_string_calculator();

        // Act
        let result = calc.add("5");

        // Assert
        assert_eq!(result, 5);
    }

    #[test]
    fn 複数の数字をカンマで区切って足し算できること() {
        // Arrange
        let calc = setup_string_calculator();

        // Act
        let result = calc.add("1,2,3");

        // Assert
        assert_eq!(result, 6);
    }
}

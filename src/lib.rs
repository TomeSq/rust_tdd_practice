#[derive(Default)]
pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    //    use super::*;

    use crate::Calculator;

    #[test]
    fn 計算機が作成できること() {
        // Arrange

        // Act
        let _calc = Calculator::new();

        // Assert
        // Rustではインスタンス作成が成功すればOK
    }

    #[test]
    fn 正の数同士を足し算できること() {
        // Arrange
        let calc = Calculator::new();

        // Act
        let result = calc.add(2, 3);

        // Asesrt
        assert_eq!(result, 5);
    }

    #[test]
    fn 負の数同士を足し算できること() {
        // Arrange
        let calc = Calculator::new();

        // Act
        let result = calc.add(-2, -3);

        // Asesrt
        assert_eq!(result, -5);
    }

    #[test]
    fn 正負混合の数を足し算できること() {
        // Arrange
        let calc = Calculator::new();

        // Act
        let result = calc.add(-5, 3);

        // Asesrt
        assert_eq!(result, -2);
    }
}

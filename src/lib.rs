#[derive(Default)]
pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    // 足し算
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    // 引き算
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn divide(&self, a: i32, b: i32) -> Result<f64, String> {
        if b == 0 {
            // ゼロ除算のエラー
            return Err("ゼロで割ることはできません".to_string());
        }
        //        a / b
        Ok(a as f64 / b as f64)
    }
}

#[cfg(test)]
mod tests {
    //    use super::*;

    use crate::Calculator;

    fn setup_calculator() -> Calculator {
        Calculator::new()
    }

    #[test]
    fn 計算機が作成できること() {
        // Arrange

        // Act
        let _calc = Calculator::new();

        // Assert
        // Rustではインスタンス作成が成功すればOK
    }

    // =========================
    // 足し算のテスト
    // =========================
    #[test]
    fn 正の数同士を足し算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.add(2, 3);

        // Asesrt
        assert_eq!(result, 5);
    }

    #[test]
    fn 負の数同士を足し算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.add(-2, -3);

        // Asesrt
        assert_eq!(result, -5);
    }

    #[test]
    fn 正負混合の数を足し算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.add(-5, 3);

        // Asesrt
        assert_eq!(result, -2);
    }

    // =========================
    // 引き算のテスト
    // =========================
    #[test]
    fn 正の数同士を引き算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.subtract(5, 3);

        // Asesrt
        assert_eq!(result, 2);
    }

    #[test]
    fn 負の数同士を引き算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.subtract(5, -3);

        // Asesrt
        assert_eq!(result, 8);
    }

    // =========================
    // 掛け算のテスト
    // =========================
    #[test]
    fn 正の数同士を掛け算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.multiply(4, 3);

        // Asesrt
        assert_eq!(result, 12);
    }

    #[test]
    fn ゼロを掛け算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.multiply(5, 0);

        // Asesrt
        assert_eq!(result, 0);
    }

    #[test]
    fn 負の数を掛け算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.multiply(-3, 4);

        // Asesrt
        assert_eq!(result, -12);
    }

    // =========================
    // 割り算のテスト（Rustのエラーハンドリング）
    // =========================
    #[test]
    fn 正の数同士を割り算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.divide(10, 2);

        // Asesrt
        assert_eq!(result, Ok(5.0));
    }

    #[test]
    fn 割り切れない数を割り算できること() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.divide(7, 3);

        // Asesrt
        // f64の比較では誤差を考慮
        match result {
            Ok(v) => assert!((v - 2.3333).abs() < 0.0001),
            Err(_) => panic!("割り算が失敗しました"),
        }
    }

    #[test]
    fn ゼロ除算でエラーを返すこと() {
        // Arrange
        let calc = setup_calculator();

        // Act
        let result = calc.divide(5, 0);

        // Asesrt
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "ゼロで割ることはできません");
    }
}

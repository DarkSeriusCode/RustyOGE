#[macro_export]
macro_rules! Test {
    (
        Name = $name:tt,
        Input = ($($input:expr),+),
        Spec = $spec:expr,
        Output = $output:literal
    ) => {
        #[test]
        fn $name() {
            use rusty_oge::utils::Validated;
            let input_data = $crate::InputData::new($($input),+, $spec);
            assert!(input_data.valid().is_ok());
            let answer = $crate::solve(input_data).expect("");
            assert_eq!(answer, $output);
        }
    };

    (
        Name = $name:tt,
        Input = ($($input:expr),+),
        Output = $output:literal
    ) => {
        #[test]
        fn $name() {
            use rusty_oge::utils::Validated;
            let input_data = $crate::InputData::new($($input),+);
            assert!(input_data.valid().is_ok());
            let answer = $crate::solve(input_data).expect("");
            assert_eq!(answer, $output);
        }
    };
}

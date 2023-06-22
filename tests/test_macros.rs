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
            let input_data = $crate::InputData::new($($input),+, $spec);
            assert!(input_data.is_valid());
            let answer = $crate::solve(input_data).expect("");
            assert_eq!(answer, $output);
        }
    };
}

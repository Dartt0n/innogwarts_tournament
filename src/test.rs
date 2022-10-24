#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::solution;

    fn as_string(file: &str) -> String {
        let mut buffer = String::new();

        File::open(file)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        buffer
    }

    #[test]
    fn example_1() {
        solution("tests/example1_input.txt", "tests/example1_output.txt");
        assert_eq!(
            as_string("tests/example1_output.txt"),
            as_string("tests/example1_solution.txt")
        )
    }

    #[test]
    fn example_2() {
        solution("tests/example2_input.txt", "tests/example2_output.txt");
        assert_eq!(
            as_string("tests/example2_output.txt"),
            as_string("tests/example2_solution.txt")
        )
    }

    #[test]
    fn example_3() {
        solution("tests/example3_input.txt", "tests/example3_output.txt");
        assert_eq!(
            as_string("tests/example3_output.txt"),
            as_string("tests/example3_solution.txt")
        )
    }

    #[test]
    fn example_4() {
        solution("tests/example4_input.txt", "tests/example4_output.txt");
        assert_eq!(
            as_string("tests/example4_output.txt"),
            as_string("tests/example4_solution.txt")
        )
    }

    #[test]
    fn example_5() {
        solution("tests/example5_input.txt", "tests/example5_output.txt");
        assert_eq!(
            as_string("tests/example5_output.txt"),
            as_string("tests/example5_solution.txt")
        )
    }

    #[test]
    fn example_6() {
        solution("tests/example6_input.txt", "tests/example6_output.txt");
        assert_eq!(
            as_string("tests/example6_output.txt"),
            as_string("tests/example6_solution.txt")
        )
    }

    #[test]
    fn example_7() {
        solution("tests/example7_input.txt", "tests/example7_output.txt");
        assert_eq!(
            as_string("tests/example7_output.txt"),
            as_string("tests/example7_solution.txt")
        )
    }
}

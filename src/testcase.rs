#[cfg(not(test))]
use crate::CLI_ARGS;
#[cfg(not(test))]
use crate::{COLORS_FAIL, COLORS_PASS};
use colored::{Color, Colorize};
#[cfg(test)]
use lazy_static::lazy_static;

#[cfg(test)]
// Can be removed if the #[cfg(not(test))] is removed in main.rs
lazy_static! {
    static ref COLORS_PASS: (Color, Color) = match CLI_ARGS.colorblind {
        true => (Color::BrightBlue, Color::Blue),
        false => (Color::BrightGreen, Color::Green),
    };
    static ref COLORS_FAIL: (Color, Color) = match CLI_ARGS.colorblind {
        true => (Color::BrightYellow, Color::Yellow),
        false => (Color::BrightRed, Color::Red),
    };
    static ref CLI_ARGS: crate::Cli = crate::Cli {
        mode: crate::Mode::Server,
        port: 8192,
        verbose: false,
        waves: false,
        colorblind: std::env::var("COLORBLIND").is_ok(),
    };
}

#[derive(Default)]
pub struct Tests {
    tests: Vec<Test>,
}

impl Tests {
    /// Add a [`Test`] to the list of tests to-be-executed.
    pub fn add(
        &mut self,
        module: &str,
        name: &str,
        method: Box<dyn Fn(u16) -> Result<(), String>>,
    ) {
        self.tests.push(Test::new(module, name, method))
    }

    /// Executes all tests. Updates the result field for each [`Test`] in the internal vector of tests.
    pub fn execute(&mut self) {
        for test in self.tests.iter_mut() {
            test.execute()
        }
    }

    /// Executes all tests and displays the test results.
    ///
    /// Updates the result field for each [`Test`] in the internal vector of tests.
    pub fn execute_display(&mut self) {
        println!(
            "Executing {} {}.",
            self.tests.len().to_string().bold(),
            match self.tests.len() {
                1 => "test",
                _ => "tests",
            }
        );
        self.execute();
        let mut passed = true;
        for test in self.tests.iter() {
            // Displaying also includes a summary of if all tests passed or not. To check if all tests
            // passed, we iterate over them, check their result and then irrevocably set "passed" to
            // false if any of them failed.
            if let Some(result) = &test.result {
                if passed {
                    passed = result.is_ok()
                }
            }
            println!("{}", test)
        }

        match passed {
            true => println!(
                "{}",
                "\nAll tests passed.\n"
                    .color(COLORS_PASS.1)
                    .underline()
                    .bold()
            ),
            false => println!(
                "{}",
                "\nNot all tests passed.\n"
                    .color(COLORS_FAIL.1)
                    .underline()
                    .bold()
            ),
        }
    }

    /// Shorthand for `Self::default()`; creates an empty list of [`Tests`]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Represents a `stimmgabel` test acceptance case.
pub struct Test {
    pub module: String,
    pub name: String,
    pub method: Box<dyn Fn(u16) -> Result<(), String>>,
    result: Option<Result<(), String>>,
}

impl Test {
    /// Create a new test case.
    pub fn new(module: &str, name: &str, method: Box<dyn Fn(u16) -> Result<(), String>>) -> Self {
        Test {
            name: name.to_string(),
            module: module.to_string(),
            method,
            result: None,
        }
    }

    /// Executes the method supplied in `self` and saves the [`Result`] inside of `self`.
    ///
    /// Consider using the [`Tests`] struct to execute and print a lot of tests at once.
    pub fn execute(&mut self) {
        match (self.method)(CLI_ARGS.port) {
            Ok(_) => self.result = Some(Ok(())),
            Err(e) => self.result = Some(Err(e)),
        }
    }
}

impl std::fmt::Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatter =
            |indication: &str, module: &str, test_name: &str, light: Color, dark: Color| {
                format!(
                    "{} {}{}{}",
                    indication.white().on_color(dark).bold(),
                    module.to_uppercase().color(light),
                    "/".bold().color(dark),
                    test_name.to_uppercase().color(dark)
                )
            };
        if let Some(result) = &self.result {
            match result {
                // Successful test
                Ok(_) => write!(
                    f,
                    "{}",
                    formatter(
                        " PASS ",
                        &self.module,
                        &self.name,
                        COLORS_PASS.0,
                        COLORS_PASS.1
                    )
                ),
                // Failed test
                Err(e) => write!(
                    f,
                    "{}: {}",
                    formatter(
                        " FAIL ",
                        &self.module,
                        &self.name,
                        COLORS_FAIL.0,
                        COLORS_FAIL.1
                    ),
                    e
                ),
            }
        } else {
            // Skipped test
            write!(
                f,
                "{}",
                formatter(
                    " SKIP ",
                    &self.module,
                    &self.name,
                    Color::White,
                    Color::BrightBlack
                )
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn port_is_power_of_two(port: u16) -> Result<(), String> {
        if port.is_power_of_two() {
            Ok(())
        } else {
            Err("The supplied port is not a power of two.".to_string())
        }
    }

    fn port_is_not_power_of_two(port: u16) -> Result<(), String> {
        if port_is_power_of_two(port).is_ok() {
            Err("The supplied port was a power of two.".to_string())
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_struct() {
        let mut test = Test::new("module", "test_struct_test", Box::new(port_is_power_of_two));

        test.execute();
        assert!(test.result.is_some());
        println!("{}", test);
    }

    #[test]
    fn tests_struct() {
        let mut tests = Tests::new();
        tests.add(
            "module",
            "tests_struct_test",
            Box::new(port_is_power_of_two),
        );
        tests.execute_display()
    }

    #[test]
    fn tests_struct_failure() {
        let mut tests = Tests::new();
        tests.add(
            "module",
            "tests_struct_test",
            Box::new(port_is_not_power_of_two),
        );
        tests.execute_display()
    }

    #[test]
    fn skipped_test() {
        let test = Test::new("module", "test_struct_test", Box::new(port_is_power_of_two));
        assert!(test.result.is_none());
        println!("{}", test);
    }

    #[test]
    fn all_fail_if_one_fails() {
        let mut tests = Tests::new();
        tests.add(
            "module",
            "this_one_fails",
            Box::new(port_is_not_power_of_two),
        );
        tests.add(
            "module",
            "this_one_succeeds",
            Box::new(port_is_power_of_two),
        );
        tests.execute_display()
    }

    #[ignore]
    #[test]
    fn many_tests() {
        let mut tests = Tests::new();
        tests.add(
            "module",
            "this_one_fails",
            Box::new(port_is_not_power_of_two),
        );
        tests.add(
            "module",
            "this_one_succeeds",
            Box::new(port_is_power_of_two),
        );
        tests.add(
            "module",
            "this_one_succeeds",
            Box::new(port_is_power_of_two),
        );
        tests.add(
            "module",
            "this_one_succeeds",
            Box::new(port_is_power_of_two),
        );
        tests.add(
            "module",
            "this_one_fails",
            Box::new(port_is_not_power_of_two),
        );
        tests.add(
            "module",
            "this_one_succeeds",
            Box::new(port_is_power_of_two),
        );
        tests.add(
            "module",
            "this_one_fails",
            Box::new(port_is_not_power_of_two),
        );
        tests.add(
            "module",
            "this_one_fails",
            Box::new(port_is_not_power_of_two),
        );
        let test = Test::new("module", "test_struct_test", Box::new(port_is_power_of_two));
        assert!(test.result.is_none());
        println!("{}", test);
        tests.execute_display();
        let test = Test::new("module", "test_struct_test", Box::new(port_is_power_of_two));
        assert!(test.result.is_none());
        println!("{}", test);
    }
}

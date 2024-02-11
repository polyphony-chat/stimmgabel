#[cfg(not(test))]
use crate::CLI_ARGS;

#[cfg(test)]
static CLI_ARGS: crate::Cli = crate::Cli {
    mode: crate::Mode::Server,
    port: 8192,
    verbose: false,
    waves: false,
};

/// Represents a `stimmgabel` test acceptance case.
pub struct Test {
    pub name: String,
    pub description: String,
    pub method: Box<dyn Fn(u16) -> Result<(), String>>,
    result: Option<Result<(), String>>,
}

impl Test {
    pub fn new(
        name: &str,
        description: &str,
        method: Box<dyn Fn(u16) -> Result<(), String>>,
    ) -> Self {
        Test {
            name: name.to_string(),
            description: description.to_string(),
            method,
            result: None,
        }
    }

    pub fn execute(&mut self) {
        match (self.method)(CLI_ARGS.port) {
            Ok(_) => self.result = Some(Ok(())),
            Err(e) => self.result = Some(Err(e)),
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

    #[test]
    fn test_test_struct() {
        let mut test = Test::new(
            "My synchronous Test",
            "a synchronous test",
            Box::new(port_is_power_of_two),
        );

        test.execute();
        assert!(test.result.is_some());
    }
}

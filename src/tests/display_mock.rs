use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

#[derive(Copy, Clone, PartialEq, Eq, Debug, defmt::Format)]
pub enum ExpectedAction<'a> {
    Command(&'a [u8]),
    Data(&'a [u8]),
}
use ExpectedAction::*;

pub struct DisplayMock<'a, 'b> {
    expected_actions: &'a [ExpectedAction<'b>],
}

impl DisplayMock<'_, '_> {
    pub fn new() -> Self {
        Self {
            expected_actions: &[],
        }
    }
    pub fn expect<'a, 'b>(mut self, expected: &'a [ExpectedAction<'b>]) -> DisplayMock<'a, 'b> {
        assert!(
            self.expected_actions.is_empty(),
            "Previous expected actions did not happen yet!"
        );
        DisplayMock {
            expected_actions: expected,
        }
    }
}

impl WriteOnlyDataCommand for DisplayMock<'_, '_> {
    fn send_commands(
        &mut self,
        data: display_interface::DataFormat<'_>,
    ) -> Result<(), DisplayError> {
        let actual = if let U8(actual) = data {
            Command(actual)
        } else {
            panic!("Only U8 data supported for now!")
        };

        let (expected, leftover) = self
            .expected_actions
            .split_first()
            .expect("No action was expected!");

        self.expected_actions = leftover;

        assert_eq!(&actual, expected, "Unexpected action received!");

        Ok(())
    }
    fn send_data(&mut self, data: display_interface::DataFormat<'_>) -> Result<(), DisplayError> {
        let actual = if let U8(actual) = data {
            Data(actual)
        } else {
            panic!("Only U8 data supported for now!")
        };

        let (expected, leftover) = self
            .expected_actions
            .split_first()
            .expect("No action was expected!");

        self.expected_actions = leftover;

        assert_eq!(&actual, expected, "Unexpected data received!");

        Ok(())
    }
}

impl Drop for DisplayMock<'_, '_> {
    fn drop(&mut self) {
        if !self.expected_actions.is_empty() {
            panic!(
                "Actions were expected, but did not happen: {:?}",
                self.expected_actions
            );
        }
    }
}

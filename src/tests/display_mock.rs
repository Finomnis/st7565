use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub enum ExpectedAction<'a> {
    Command(&'a [u8]),
    Data(&'a [u8]),
}
use ExpectedAction::*;

pub struct DisplayMock<'a, 'b> {
    expected_actions: &'a [ExpectedAction<'b>],
}

impl DisplayMock<'_, '_> {
    pub fn with_expect<'a, 'b, T>(
        expected: &'a [ExpectedAction<'b>],
        f: impl FnOnce(&mut DisplayMock<'a, 'b>) -> T,
    ) -> T {
        let mut mock = DisplayMock {
            expected_actions: expected,
        };

        let result = f(&mut mock);

        assert!(
            mock.expected_actions.is_empty(),
            "Actions were expected, but did not happen: {:?}",
            mock.expected_actions
        );

        result
    }
}

impl WriteOnlyDataCommand for &mut DisplayMock<'_, '_> {
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

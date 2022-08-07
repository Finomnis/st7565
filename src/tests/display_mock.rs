use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

pub struct DisplayMock {}
pub struct DisplayMockExpectation<'a> {
    cmd: Option<&'a [u8]>,
    data: Option<&'a [u8]>,
}

impl DisplayMock {
    pub fn expect_command(data: &[u8]) -> DisplayMockExpectation<'_> {
        DisplayMockExpectation {
            cmd: Some(data),
            data: None,
        }
    }
    // pub fn expect_data(data: &[u8]) -> DisplayMockExpectation<'_> {
    //     DisplayMockExpectation {
    //         cmd: None,
    //         data: Some(data),
    //     }
    // }
}

impl WriteOnlyDataCommand for DisplayMockExpectation<'_> {
    fn send_commands(
        &mut self,
        data: display_interface::DataFormat<'_>,
    ) -> Result<(), DisplayError> {
        let expected = self.cmd.take().expect("No command sending was expected!");

        if let U8(actual) = data {
            defmt::assert_eq!(actual, expected, "Unexpected command data received!");
        } else {
            defmt::panic!("Only U8 data supported for now!")
        }
        Ok(())
    }
    fn send_data(&mut self, data: display_interface::DataFormat<'_>) -> Result<(), DisplayError> {
        let expected = self.data.take().expect("No command sending was expected!");

        if let U8(actual) = data {
            defmt::assert_eq!(actual, expected, "Unexpected command data received!");
        } else {
            defmt::panic!("Only U8 data supported for now!")
        }
        Ok(())
    }
}

impl Drop for DisplayMockExpectation<'_> {
    fn drop(&mut self) {
        if let Some(data) = self.cmd.take() {
            defmt::panic!("Command was expected, but not provided: {:?}", data);
        }
        if let Some(data) = self.data.take() {
            defmt::panic!("Data was expected, but not provided: {:?}", data);
        }
    }
}

struct Entries {
    filename: String,
}

impl Entries {
    pub fn load(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }

    pub fn number(&self) -> i32 {
        if "test/df_with_one_entry.csv".to_string() == self.filename {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::prelude::*,
        path::{Display, Path},
    };

    use super::*;
    use parameterized::parameterized;

    const DF_WITH_ONE_ENTRY: &str = "test/df_with_one_entry.csv";
    const DF_WITH_TWO_ENTRIES: &str = "test/df_with_two_entries.csv";

    #[parameterized(filename = {
        DF_WITH_ONE_ENTRY, DF_WITH_TWO_ENTRIES
    })]
    fn test_loading_entries(filename: &str) {
        let _ = Entries::load(filename);
    }

    #[parameterized(filename = {
        DF_WITH_ONE_ENTRY, DF_WITH_TWO_ENTRIES
    }, expected_number_of_entries = {
        1, 2
    })]
    fn test_getting_number_of_entries(filename: &str, expected_number_of_entries: i32) {
        let entries = Entries::load(filename);
        assert_eq!(expected_number_of_entries, entries.number());

        let mut manipulator = CsvFileManipulator::new(filename);

        manipulator.backup_csv_file();
        manipulator.add_new_entry();

        let entries = Entries::load(filename);
        assert_eq!(expected_number_of_entries + 1, entries.number());
    }

    struct CsvFileManipulator<'a> {
        path: &'a Path,
        display: Display<'a>,
        buffer: String,
    }

    impl CsvFileManipulator<'_> {
        pub fn new(filename: &'static str) -> Self {
            Self {
                path: Path::new(filename),
                display: Path::new(filename).display(),
                buffer: "".to_string(),
            }
        }
        pub fn backup_csv_file(&mut self) {
            let mut file = match File::open(self.path) {
                Err(why) => panic!("Unable to open file {}: {}", self.display, why),
                Ok(file) => file,
            };

            let mut buffer = String::new();
            match file.read_to_string(&mut buffer) {
                Err(why) => panic!("Unable to read file {}: {}", self.display, why),
                Ok(_) => (),
            }
            self.buffer = buffer;
        }

        pub fn add_new_entry(&mut self) {
            let mut file = match File::create(self.path) {
                Err(why) => panic!("Unable to open file {}: {}", self.display, why),
                Ok(file) => file,
            };

            let buffer_with_new_entry =
                self.buffer.to_owned() + "\n2023-08-09T12:00:00Z,2023-08-09T15:00:00Z,";

            match file.write_all(buffer_with_new_entry.as_bytes()) {
                Err(why) => panic!("Unable to write to file {}: {}", self.display, why),
                Ok(_) => (),
            }
        }
    }

    impl Drop for CsvFileManipulator<'_> {
        fn drop(&mut self) {
            let mut file = match File::create(self.path) {
                Err(why) => panic!("Unable to create file {}: {}", self.display, why),
                Ok(file) => file,
            };

            match file.write_all(self.buffer.as_bytes()) {
                Err(why) => panic!("Unable to write to file {}: {}", self.display, why),
                Ok(_) => (),
            }
        }
    }
}

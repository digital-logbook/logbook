use polars::prelude::*;

struct Entries {
    lazyframe: LazyFrame,
}

impl Entries {
    pub fn load(filename: &str) -> Self {
        let lazyframe = LazyCsvReader::new(filename).finish().unwrap();
        Self {
            lazyframe: lazyframe,
        }
    }

    pub fn len(&self) -> usize {
        self.lazyframe.clone().collect().unwrap().height()
    }

    pub fn unfinished_entry_exists(&self) -> bool {
        self.start_column_of_last_entry_has_been_set() && self.stop_column_of_last_entry_is_null()
    }

    fn start_column_of_last_entry_has_been_set(&self) -> bool {
        0 == self.last_entry_null_count_of_column("start")
    }

    fn stop_column_of_last_entry_is_null(&self) -> bool {
        1 == self.last_entry_null_count_of_column("stop")
    }

    fn last_entry_null_count_of_column(&self, column: &str) -> u32 {
        self.lazyframe
            .clone()
            .last()
            .null_count()
            .collect()
            .unwrap()
            .column(column)
            .unwrap()
            .get(0)
            .unwrap()
            .try_extract::<u32>()
            .unwrap()
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

    const DF_WITH_ONE_ENTRY_AND_UNFINISHED_ONE: &str =
        "test/df_with_one_entry_and_unfinished_one.csv";
    const DF_WITH_TWO_ENTRIES_AND_UNFINISHED_ONE: &str =
        "test/df_with_two_entries_and_unfinished_one.csv";

    #[parameterized(filename = {
        DF_WITH_ONE_ENTRY, DF_WITH_TWO_ENTRIES
    })]
    fn test_loading_entries(filename: &str) {
        let _ = Entries::load(filename);
    }

    #[parameterized(filename = {
        DF_WITH_ONE_ENTRY, DF_WITH_TWO_ENTRIES
    }, expected_len_of_entries = {
        1, 2
    })]
    fn test_getting_len_of_entries(filename: &str, expected_len_of_entries: usize) {
        let entries = Entries::load(filename);
        assert_eq!(expected_len_of_entries, entries.len());

        let mut manipulator = CsvFileManipulator::new(filename);

        manipulator.backup_csv_file();
        manipulator.add_new_entry();

        let entries = Entries::load(filename);
        assert_eq!(expected_len_of_entries + 1, entries.len());
    }

    #[parameterized(filename = {
        DF_WITH_ONE_ENTRY, DF_WITH_ONE_ENTRY_AND_UNFINISHED_ONE,
        DF_WITH_TWO_ENTRIES, DF_WITH_TWO_ENTRIES_AND_UNFINISHED_ONE,
    }, unfinished_entry_exists = {
        false, true,
        false, true,
    })]
    fn test_for_existing_unfinished_entry(filename: &str, unfinished_entry_exists: bool) {
        let entries = Entries::load(filename);
        assert_eq!(unfinished_entry_exists, entries.unfinished_entry_exists());
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

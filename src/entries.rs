struct Entries {}

impl Entries {
    pub fn load(_filename: &str) -> Self {
        Self {}
    }

    pub fn number(&self) -> i32 {
        1
    }
}

#[cfg(test)]
mod test {
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
    }
}

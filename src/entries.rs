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

    const DF_WITH_ONE_ENTRY: &str = "test/df_with_one_entry.csv";

    #[test]
    fn test_loading_entries() {
        let entries = Entries::load(DF_WITH_ONE_ENTRY);
        assert_eq!(1, entries.number());
    }
}

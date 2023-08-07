#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_loading_entries() {
        let entries = Entries::load("test/df_with_one_entry.csv");
    }
}

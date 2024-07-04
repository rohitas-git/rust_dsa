use crate::data_structures::hash_table::open_addressing::hash_table::OpenAddresserHashTable as HashTable;
use crate::data_structures::hash_functions::basic::common::*;
#[test]
fn test_as_user() {
    let mut hash_table_u32 = OpenAddresserHashTable::new(
        16,
        LinearProbingHandler::default(),
        GenericHasherStrategy::<std::collections::hash_map::DefaultHasher>::new(),
    );

    let mut hash_table_string =
        OpenAddresserHashTable::new(16, LinearProbingHandler::default(), StringHash);
}

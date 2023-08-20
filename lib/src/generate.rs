use std::collections::HashMap;

use hmac_sha256::HMAC;

use crate::types::{Field, Table, TableExtra};

pub trait GenerateDummy {
    fn generate_dummy(number: usize) -> Self;
}

pub(crate) fn hash_number(input: usize) -> [u8; 32] {
    let bytes = input.to_le_bytes();
    let h = HMAC::new(bytes);

    h.finalize()
}

pub(crate) fn u8s_to_string(input: &[u8]) -> String {
    input.iter().map(|item| (item % 24 + 65) as char).collect()
}

pub(crate) fn hash_number_and_stringify(input: usize) -> String {
    u8s_to_string(&hash_number(input))
}

pub fn generate_table(counter: usize, fields_per_table: usize) -> Table {
    let name = hash_number_and_stringify(counter);

    let mut fields = HashMap::new();

    for i in 0..fields_per_table {
        let field = Field::generate_dummy(i.wrapping_add(counter.wrapping_mul(100)));

        fields.insert(field.name.clone(), field);
    }

    let first_field_for_pk = fields.keys().next().unwrap().clone();

    Table::new(
        name,
        fields,
        TableExtra::new_with_pk(vec![first_field_for_pk]),
    )
}

#[cfg(test)]
mod tests {
    use crate::generate::{hash_number, hash_number_and_stringify, u8s_to_string};

    #[test]
    fn hash_number_just_works() {
        assert_eq!(hash_number(1), hash_number(1));
        assert_ne!(hash_number(1), hash_number(2));
    }

    #[test]
    fn u8s_to_string_just_works() {
        assert_eq!(u8s_to_string(&[0, 1, 2, 3, 4]), "ABCDE".to_string());
        assert_eq!(u8s_to_string(&[24, 25, 26, 27, 28]), "ABCDE".to_string());
    }

    #[test]
    fn hash_number_and_stringify_just_works() {
        assert_eq!(hash_number_and_stringify(1), hash_number_and_stringify(1));
        assert_ne!(hash_number_and_stringify(1), hash_number_and_stringify(2));
    }
}

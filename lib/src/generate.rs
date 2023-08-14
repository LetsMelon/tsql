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

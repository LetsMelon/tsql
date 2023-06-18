import hashlib
from typing import IO

def number_to_string(number: int) -> str:
    byte_representation = number.to_bytes((number.bit_length() + 7) // 8, 'big')

    hash_object = hashlib.sha256(byte_representation)
    hash_value = hash_object.digest()

    letters = [chr((byte % 26) + ord('A')) for byte in hash_value]
    hash_string = ''.join(letters)

    return hash_string

def write_table(file: IO[bytes], fields: int, number: int) -> None:
    datatypes = ["int", "double", "varchar(100)", "char(6)", "uuid"]

    fields = [f"\t{datatypes[i % len(datatypes)]} {number_to_string(i + 100)},\n" for i in range(fields)]

    lines = ["@primary_key(id)\n", f"table {number_to_string(number)} " + "{\n", "\tint id,\n" ]
    lines.extend(fields)
    lines.extend(["};\n"])

    file.writelines(lines)

if __name__ == "__main__":
    file_name = "big.tsql"

    with open(file_name, "w") as file:
        tables = 1_000
        fields = 500

        for i in range(1, tables):
            write_table(file, fields, i);

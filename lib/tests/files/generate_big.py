import hashlib
import sys
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
    args = sys.argv[1:]

    if len(args) != 6:
        HELP = """
        python3 generate_big.py

        USAGE:
            python3 generate_big.py --name FILE_NAME --tables TABLE_COUNT --fields FIELDS_COUNT
        """
        print(HELP)
        exit(1)

    args = {args[i].replace("--", ""): args[i + 1] for i in range(0, len(args), 2)}

    file_name = args["name"]

    with open(file_name, "w") as file:
        tables = int(args["tables"])
        fields = int(args["fields"])

        for i in range(tables):
            write_table(file, fields, i);

    print(f"Created file: '{file_name}'")

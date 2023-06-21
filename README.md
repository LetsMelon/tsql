# tsql

SQL with a little bit extra sugar on top and blazingly fast ✨

(no specs, no finished cli <bold>BUT</bold> under active development)

---- 

## How to use the cli

1.  Build with `cargo build --release`
2.  Create `test.tsql` and insert content

```
@primary_key(id)
table Human {
    int id,
    varchar(32) name,
    date birth,
};

@primary_key(start, end)
table Termin {
    datetime start,
    datetime end,
    varchar(16) description,
};

@primary_key(human_id, termin_start, termin_end)
table has_appointment {
    @foreign_key()
    Human human,
    @foreign_key()
    Termin termin,
};
```

3. Move executable `mv ./target/release/tsql ./tsql`
4. Run executable `./tsql ./test.tsql out.sql`
5. Inspect your generated sql file

## Examples

Look into `/examples` or `/tests/files` but be aware that because of active development, the parsing status can change a any moment in time.

use diesel::table;

table! {
    transmissions (id) {
        id -> Varchar,
        href -> Varchar,
        initially_played -> Int8,
        last_played -> Int8,
    }
}
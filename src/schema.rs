use diesel::table;

table! {
    guilds (id) {
        id -> Varchar,
        name -> Varchar,
        subscribed -> Bool,
        expiration -> Int8,
    }
}

table! {
    queues (id) {
        id -> Varchar,

    }
}

table! {
    sources (id) {
        id -> Varchar,
        
    }
}

table! {
    transmissions (id) {
        id -> Varchar,
        href -> Varchar,
        initially_played -> Int8,
        last_played -> Int8,
    }
}
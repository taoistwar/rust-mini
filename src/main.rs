use rusqlite::Connection;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./tests/sql_migrations");
}

fn main() {
    let mut conn = Connection::open("./test.sqlite").unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
}

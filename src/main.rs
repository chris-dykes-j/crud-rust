// Testing CRUD with Rust
use postgres::{Client, NoTls, Error};

struct BlogEntry {
    id: i32,
    title: String,
    // date: String,
    // description: String,
    content: String,
}

fn main() -> Result<(), postgres::Error>{
    let mut client = Client::connect("host=localhost port=5432 dbname=blogentries user=chris password=''", NoTls)?;

    for row in client.query("SELECT * FROM blogentry", &[])? {
        let (id, title, content): (Option<i32>, Option<String>, Option<String>) = (row.get(0), row.get(1), row.get(4));

        if id.is_some() && title.is_some() && content.is_some() {
            let entry = BlogEntry {
                id: id.unwrap(),
                title: title.unwrap(),
                content: content.unwrap(),
            };
            println!("{} {} {}", entry.id, entry.title, entry.content);
        }
    }

    Ok(())
}

// Testing CRUD with Rust
use postgres::{Client, NoTls, Error};
use chrono::NaiveDate;

struct BlogEntry {
    id: i32,
    title: String,
    // date: String,
    description: String,
    content: String,
}

fn main() -> Result<(), Error> {
    let mut client: Client = Client::connect("host=localhost port=5432 dbname=blogentries user=chris password=''", NoTls)?;

    create_entry(&mut client);
    read_entries(&mut client).expect("TODO: panic message");

    Ok(())
}

fn create_entry(client: &mut Client) -> Result<(), Error> {
    let create_query: &str = "INSERT INTO blogentry (title, description, date, content) VALUES ($1, $2, $3, $4)";

    let date = NaiveDate::from_ymd_opt(2022, 12, 30);

    client.execute(create_query, &[&"title".to_owned(), &"description".to_owned(), &date.is_some(), &"quality content".to_owned()])?;

    Ok(())
}

fn read_entries(client: &mut Client) -> Result<(), Error> {
    let select_query: &str = "SELECT * FROM blogentry";
    for row in client.query(select_query, &[])? {

        let (id, title, description, content): (Option<i32>, Option<String>, Option<String>, Option<String>)
            = (row.get(0), row.get(1), row.get(3), row.get(4));

        if id.is_some() && title.is_some() && content.is_some() {
            let entry = BlogEntry {
                id: id.unwrap(),
                title: title.unwrap(),
                description: description.unwrap(),
                content: content.unwrap(),
            };
            println!("{}\n {}\n {}\n {}\n", &entry.id, &entry.title, &entry.description, &entry.content);
        }
    }
    Ok(())
}

fn update_entry(client: &mut Client) -> Result<(), Error> {
    Ok(())
}

fn delete_entry(client: &mut Client) -> Result<(), Error> {
    Ok(())
}

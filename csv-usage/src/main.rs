use csv::StringRecord;
use std::fs::File;
fn main() -> std::io::Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(File::open("books.csv")?);
    fmt(reader.headers()?);
    for record in reader.records() {
        fmt(&record?);
    }
    Ok(())
}
fn fmt(rec: &StringRecord) {
    println!(
        "{}",
        rec.into_iter()
            .map(|v| format!("{:20}", v))
            .collect::<Vec<String>>()
            .join("")
    )
}

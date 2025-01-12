use postgres::{Client, Error, NoTls, Transaction};
use rust_xlsxwriter::{Workbook};
use std::env::var;

struct FakeData {
    _id: i32,
    cols: Vec<String>,
}

fn setup_db_connection() -> Result<Client, Error> {
    let db_host = var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_name = var("DB_NAME").unwrap_or_else(|_| "test_db".to_string());
    let db_user = var("DB_USER").unwrap_or_else(|_| "user".to_string());
    let db_pass = var("DB_PASS").unwrap_or_else(|_| "password".to_string());

    Client::connect(
        &format!("postgresql://{db_user}:{db_pass}@{db_host}/{db_name}"),
        NoTls,
    )
}

fn fetch_chunk(transaction: &mut Transaction, chunk_size: usize) -> Result<Vec<FakeData>, Error> {
    let rows = transaction.query(&format!("FETCH {} FROM data_cursor", chunk_size), &[])?;

    Ok(rows
        .iter()
        .map(|row| FakeData {
            _id: row.get(0),
            cols: (1..row.len())
                .map(|i| row.get::<_, Option<String>>(i).unwrap_or_default())
                .collect(),
        })
        .collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CHUNK_SIZE: usize = 1000;

    // Set up the database connection and cursor
    let mut client = setup_db_connection()?;
    let mut transaction = client.transaction()?;
    transaction.execute(
        "DECLARE data_cursor CURSOR FOR SELECT * FROM test_table ORDER BY id",
        &[],
    )?;

    // Create a new Excel file object.
    let mut workbook = Workbook::new();
    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Write the header row.
    let header = [
        "_id", "col1", "col2", "col3", "col4", "col5", "col6", "col7", "col8", "col9", "col10",
    ];
    worksheet.write_row(0, 0, header)?;

    let mut row_index = 1; // Start writing data from the second row
    loop {
        let rows = fetch_chunk(&mut transaction, CHUNK_SIZE)?;
        if rows.is_empty() {
            break;
        }

        // Write the chunk to Excel
        for row in rows {
            let mut data: Vec<String> = vec![row._id.to_string()];
            data.extend(row.cols);
            worksheet.write_row(row_index, 0, data)?;
            row_index += 1;
        }
    }

    // Close the cursor and commit the transaction
    transaction.execute("CLOSE data_cursor", &[])?;
    transaction.commit()?;

    // Save the file to disk.
    workbook.save("hello.xlsx")?;

    Ok(())
}

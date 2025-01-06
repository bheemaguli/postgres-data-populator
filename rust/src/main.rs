use postgres::{Client, Error, NoTls};
use rust_xlsxwriter::{Workbook, XlsxError};
use std::env::var;

struct FakeData {
    _id: i32,
    col1: String,
    col2: String,
    col3: String,
    col4: String,
    col5: String,
    col6: String,
    col7: String,
    col8: String,
    col9: String,
    col10: String,
}

fn main() -> Result<(), XlsxError> {
    fn get_rows() -> Result<Vec<FakeData>, Error> {
        let db_host = var("DB_HOST").unwrap_or("localhost".to_string());
        let db_name = var("DB_NAME").unwrap_or("test_db".to_string());
        let db_user = var("DB_USER").unwrap_or("user".to_string());
        let db_pass = var("DB_PASS").unwrap_or("password".to_string());

        let mut client = Client::connect(
            &format!("postgresql://{db_user}:{db_pass}@{db_host}/{db_name}"),
            NoTls,
        )?;
        let result = client.query("SELECT * FROM test_table", &[])?;
        Ok(result
            .iter()
            .map(|row| FakeData {
                _id: row.get(0),
                col1: row.get(1),
                col2: row.get(2),
                col3: row.get(3),
                col4: row.get(4),
                col5: row.get(5),
                col6: row.get(6),
                col7: row.get(7),
                col8: row.get(8),
                col9: row.get(9),
                col10: row.get(10),
            })
            .collect())
    }

    let rows = match get_rows() {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Error fetching rows: {}", err);
            return Err(XlsxError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error fetching rows",
            )));
        }
    };

    // Create a new Excel file object.
    let mut workbook = Workbook::new();
    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    let _ = worksheet.write_row(
        0,
        0,
        [
            "_id", "col1", "col2", "col3", "col4", "col5", "col6", "col7", "col8", "col9", "col10",
        ],
    );
    for row in rows {
        // Write to excel
        let _ = worksheet.write_row(
            row._id as u32,
            0,
            [
                row._id.to_string(),
                row.col1,
                row.col2,
                row.col3,
                row.col4,
                row.col5,
                row.col6,
                row.col7,
                row.col8,
                row.col9,
                row.col10,
            ],
        );
    }

    // Save the file to disk.
    workbook.save("hello.xlsx")?;

    Ok(())
}

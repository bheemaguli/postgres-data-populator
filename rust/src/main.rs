use postgres::{Client, Error, NoTls};
use rust_xlsxwriter::{Workbook, XlsxError};
use std::env::var;

struct FakeData {
    _id: i32,
    cols: Vec<String>,
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
            .map(|row| {
                let _id: i32 = row.get(0);
                let cols: Vec<String> = (1..row.len())
                    .map(|i| row.get::<_, Option<String>>(i).unwrap_or_default())
                    .collect();
                FakeData { _id, cols }
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

    // Write the header row.
    let header = [
        "_id", "col1", "col2", "col3", "col4", "col5", "col6", "col7", "col8", "col9", "col10",
    ];
    worksheet.write_row(0, 0, header)?;

    // Write data rows.
    for (i, row) in rows.iter().enumerate() {
        let mut data = vec![row._id.to_string()];
        data.extend(row.cols.clone());
        worksheet.write_row(i as u32 + 1, 0, data)?;
    }

    // Save the file to disk.
    workbook.save("hello.xlsx")?;

    Ok(())
}

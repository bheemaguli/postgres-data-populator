import psycopg2
import os
import xlsxwriter

DB_HOST = os.getenv("DB_HOST", "localhost")
DB_PORT = os.getenv("DB_PORT", "5432")
DB_NAME = os.getenv("DB_NAME", "test_db")
DB_USER = os.getenv("DB_USER", "user")
DB_PASS = os.getenv("DB_PASS", "password")

conn = psycopg2.connect(
    host=DB_HOST, port=DB_PORT, dbname=DB_NAME, user=DB_USER, password=DB_PASS
)
cursor = conn.cursor()
cursor.execute('''SELECT * from test_table''')
result = cursor.fetchall();
cursor.close()
conn.close()

# Create an new Excel file and add a worksheet.
workbook = xlsxwriter.Workbook('demo.xlsx')
worksheet = workbook.add_worksheet()
worksheet.write_row(0, 0, ("col1", "col2", "col3", "col4", "col5", "col6", "col7", "col8", "col9", "col10"))
for row in result:
    worksheet.write_row(row[0], 0, row[1:])
workbook.close()

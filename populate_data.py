import os
import psycopg2
from faker import Faker
from tqdm import tqdm
import logging
from tqdm.contrib.logging import logging_redirect_tqdm
# Configure logging
logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")
logger = logging.getLogger(__name__)

DB_HOST = os.getenv("DB_HOST", "localhost")
DB_PORT = os.getenv("DB_PORT", "5432")
DB_NAME = os.getenv("DB_NAME", "test_db")
DB_USER = os.getenv("DB_USER", "user")
DB_PASS = os.getenv("DB_PASS", "password")

try:
    logger.info("Connecting to the database...")
    conn = psycopg2.connect(
        host=DB_HOST, port=DB_PORT, dbname=DB_NAME, user=DB_USER, password=DB_PASS
    )
    cursor = conn.cursor()
    logger.info("Connected to the database successfully.")
except Exception as e:
    logger.error(f"Failed to connect to the database: {e}")
    raise

fake = Faker()
current_rows = cursor.execute("SELECT COUNT(*) FROM test_table")
rows_to_insert = 1_000_000 - (current_rows.fetchone()[0] if current_rows else 0)
batch_size = 10_000

logger.info(f"Starting data population: {rows_to_insert} rows in batches of {batch_size}.")

try:
    with logging_redirect_tqdm():
        for batch_start in range(0, rows_to_insert, batch_size):
            data = [
                (
                    fake.text(max_nb_chars=20),
                    fake.text(max_nb_chars=20),
                    fake.text(max_nb_chars=20),
                    fake.text(max_nb_chars=20),
                    fake.text(max_nb_chars=20),
                    fake.text(max_nb_chars=20),
                    fake.text(max_nb_chars=20),
                    fake.text(max_nb_chars=20),
                    fake.text(max_nb_chars=20),
                    fake.text(max_nb_chars=20),
                )
                for _ in range(batch_size)
            ]
            args_str = ",".join(
                cursor.mogrify("(%s,%s,%s,%s,%s,%s,%s,%s,%s,%s)", row).decode("utf-8")
                for row in data
            )
            cursor.execute(
                f"INSERT INTO test_table (col1, col2, col3, col4, col5, col6, col7, col8, col9, col10) VALUES {args_str}"
            )
            conn.commit()
            logger.info(f"Inserted rows {batch_start + 1} to {batch_start + batch_size}")

    logger.info("Data population completed successfully!")
except Exception as e:
    logger.error(f"An error occurred during data population: {e}")
    conn.rollback()
finally:
    cursor.close()
    conn.close()
    logger.info("Database connection closed.")
    exit(0)

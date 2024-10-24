import sqlite3
import json
import argparse
import os
from faker import Faker
from datetime import datetime
from polycrypt.polycrypt import PolyCrypt, init_logger

# Initialize Faker and PolyCrypt
fake = Faker()
init_logger()
key = bytes([0] * 32)  # In a real scenario, use a proper key generation method
pc = PolyCrypt(key)

# Database connection
DB_PATH = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "polycrypt_benchmark.db"))

def connect_db():
    return sqlite3.connect(DB_PATH)

def create_tables(conn):
    cursor = conn.cursor()
    cursor.execute("""
    CREATE TABLE IF NOT EXISTS plain_records (
        id TEXT PRIMARY KEY,
        data TEXT NOT NULL
    )
    """)
    cursor.execute("""
    CREATE TABLE IF NOT EXISTS encrypted_records (
        id TEXT PRIMARY KEY,
        data TEXT NOT NULL
    )
    """)
    cursor.execute("""
    CREATE TABLE IF NOT EXISTS benchmark_results (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        benchmark_name TEXT NOT NULL,
        execution_time REAL NOT NULL,
        timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
    )
    """)
    conn.commit()

def generate_realistic_record(id):
    return {
        "id": f"P{id:05d}",
        "name": fake.name(),
        "dob": fake.date_of_birth(minimum_age=18, maximum_age=90).isoformat(),
        "gender": fake.random_element(elements=('Male', 'Female')),
        "blood_type": fake.random_element(elements=('A+', 'A-', 'B+', 'B-', 'AB+', 'AB-', 'O+', 'O-')),
        "height_cm": fake.random_int(min=150, max=200),
        "weight_kg": fake.random_int(min=45, max=120),
        "conditions": fake.random_elements(elements=('Hypertension', 'Diabetes', 'Asthma', 'Arthritis', 'Depression'), unique=True, length=fake.random_int(min=0, max=3)),
        "medications": fake.random_elements(elements=('Lisinopril', 'Metformin', 'Albuterol', 'Ibuprofen', 'Sertraline'), unique=True, length=fake.random_int(min=0, max=4)),
        "allergies": fake.random_elements(elements=('Peanuts', 'Penicillin', 'Latex'), unique=True, length=fake.random_int(min=0, max=2)),
        "last_visit": datetime.now().isoformat(),
        "notes": f"Patient has been doing well since last visit.",
        "sensitive_data": f"SSN: {fake.ssn()}"
    }

def populate_tables(conn, num_records):
    cursor = conn.cursor()
    fields_to_encrypt = ["conditions", "medications", "allergies", "notes", "sensitive_data", "name", "dob"]

    for i in range(num_records):
        record = generate_realistic_record(i)
        
        # Insert plain record
        cursor.execute("INSERT INTO plain_records (id, data) VALUES (?, ?)",
                       (record['id'], json.dumps(record)))
        
        # Encrypt sensitive fields and insert encrypted record
        encrypted_record = pc.encrypt_fields(record, fields_to_encrypt)
        cursor.execute("INSERT INTO encrypted_records (id, data) VALUES (?, ?)",
                       (record['id'], json.dumps(encrypted_record)))

    conn.commit()

def reset_tables(conn):
    cursor = conn.cursor()
    cursor.execute("DELETE FROM plain_records")
    cursor.execute("DELETE FROM encrypted_records")
    conn.commit()

def delete_db():
    if os.path.exists(DB_PATH):
        os.remove(DB_PATH)
        print(f"Database {DB_PATH} has been deleted.")
    else:
        print(f"Database {DB_PATH} does not exist.")

def count_records(conn):
    cursor = conn.cursor()
    cursor.execute("SELECT COUNT(*) FROM plain_records")
    plain_count = cursor.fetchone()[0]
    cursor.execute("SELECT COUNT(*) FROM encrypted_records")
    encrypted_count = cursor.fetchone()[0]
    return plain_count, encrypted_count

def main():
    parser = argparse.ArgumentParser(description="Manage polycrypt benchmark database")
    parser.add_argument("action", choices=["create", "reset", "delete", "count"], help="Action to perform")
    parser.add_argument("--size", type=int, default=1000, help="Number of records to generate (default: 1000)")

    args = parser.parse_args()

    if args.action == "create":
        conn = connect_db()
        create_tables(conn)
        populate_tables(conn, args.size)
        conn.close()
        print(f"Database setup complete. {args.size} records inserted into plain_records and encrypted_records tables.")
    elif args.action == "reset":
        conn = connect_db()
        reset_tables(conn)
        populate_tables(conn, args.size)
        conn.close()
        print(f"Tables reset and repopulated with {args.size} records.")
    elif args.action == "delete":
        delete_db()
    elif args.action == "count":
        conn = connect_db()
        plain_count, encrypted_count = count_records(conn)
        conn.close()
        print(f"Number of records in plain_records: {plain_count}")
        print(f"Number of records in encrypted_records: {encrypted_count}")

if __name__ == "__main__":
    main()

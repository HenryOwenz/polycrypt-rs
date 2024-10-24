# Database Setup and Management

This folder contains scripts for setting up, managing, and populating the benchmark database.

## Requirements

- Python 3.6 or higher
- pip (Python package installer)

## Setup

1. Create a virtual environment:
   ```
   python3 -m venv venv
   ```

2. Activate the virtual environment:
   - On Unix or MacOS:
     ```
     source venv/bin/activate
     ```
   - On Windows:
     ```
     venv\Scripts\activate
     ```

3. Install the required packages:
   ```
   pip install -r requirements.txt
   ```

## Usage

The `setup_db.py` script now supports the following commands:

1. Create a new database and populate it:
   ```
   python setup_db.py create --size 1000
   ```
   This will create a new database (if it doesn't exist) and populate it with 1000 records.

2. Reset existing tables and repopulate:
   ```
   python setup_db.py reset --size 500
   ```
   This will delete all records from the `plain_records` and `encrypted_records` tables and repopulate them with 500 new records.

3. Delete the entire database:
   ```
   python setup_db.py delete
   ```
   This will delete the database file.

The `--size` argument is optional and defaults to 1000 if not specified.

Note: The `benchmark_results` table is preserved across resets to maintain historical benchmark data.

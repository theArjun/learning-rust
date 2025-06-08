# 📄 Polars CSV Reader Exercise

**What it does:**

- Reads CSV file into a Polars DataFrame
- Uses `CsvReader` to parse employee data
- Displays the loaded DataFrame with formatting

**Key concepts:**

- `File::open()` - Open CSV file handle
- `CsvReader::new(file).finish()` - Parse CSV into DataFrame
- Polars automatically infers column types from CSV
- CSV is the most common data exchange format
- DataFrame represents tabular data in memory

**Sample data:** Employee records (id, name, salary, department)

**Dependencies:** `polars` with default features

**Run:** `cargo run`

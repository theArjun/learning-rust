# 🔢 Number to Words Converter

**What it does:**

- Converts integers into written English words
- Uses `num2words` crate for number-to-text conversion
- Applies title case formatting with `titlecase` crate
- Handles conversion errors gracefully with Result pattern

**Key concepts:**

- `Num2Words::new(number).to_words()` - Convert number to words string
- `Result<String, Error>` - Error handling for invalid numbers
- `titlecase()` - Capitalizes words properly (e.g., "One Thousand Four Hundred Fifty-Six")
- External crate integration and dependency management

**Example:** `1456` → `"One Thousand Four Hundred Fifty Six"`

**Dependencies:** `num2words`, `titlecase`

**Run:** `cargo run`

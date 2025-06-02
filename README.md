# xcut – A Flexible Field Extractor and Filter Tool for the Command Line

**xcut** is an extended version of the Unix `cut` command, with support for filtering, column extraction, and flexible delimiters.
It is written in Rust and supports cross-platform usage (Linux, macOS, Windows via CMD or Git Bash).

---

## Features

- Column selection (`--cols`)
- Flexible field delimiter (`--delim`, `--max-split`)
- Field filtering with boolean logic and regex (`--filter`)
- Output customization (`--out-delim`, `--output`)
- CSV-style header skipping (`--no-header`)
- Head/tail output restriction (`--head`, `--tail`)
- Accepts stdin when no input file is specified

---

## Usage

```sh
xcut [OPTIONS]
```

### Basic Options

- `-i`, `--input <INPUT>`
  Path to the input file. Reads from stdin if not specified

- `-f`, `--filter <FILTER>`
  Filter expression to match lines. Supports regex and boolean logic.
  Examples:
  - `col(3) == "INFO"`
  - `col(4) =~ "^CPU"`
  - `col(3) !~ "DEBUG" && col(4) =~ "error"`

- `-c`, `--cols <COLS>`
  List of column numbers to output (1-based index).
  Example: `--cols 1,3`

- `--delim <DELIM>`
  Delimiter used to split each line into columns. Default is whitespace

- `--max-split <N>`
  Maximum number of splits to perform when using `--delim`. Useful to preserve trailing content in the last field

- `--out-delim <OUT_DELIM>`
  Delimiter used to join output fields. Default is a space

- `-o`, `--output <OUTPUT>`
  Path to output file. Appends to the file if it exists. Defaults to stdout

- `--no-header`
  Skip the first line (e.g. header in CSV files)

- `--head <HEAD>`
  Output only the first N lines (like `head`)

- `--tail <TAIL>`
  Output only the last N lines (like `tail`)

- `-h`, `--help`
  Print help (see a summary with `-h`)

- `-V`, `--version`
  Print version

---

## Filtering Expression Examples

You can filter lines using column values. Examples:

| Expression                              | Description                                |
|-----------------------------------------|--------------------------------------------|
| `col(2) == "INFO"`                      | Select rows where column 2 equals "INFO"   |
| `col(2) != "INFO"`                      | Select rows where column 2 is not "INFO"   |
| `col(4) =~ "CPU"`                       | Regex match: column 4 contains "CPU"       |
| `col(3) !~ "DEBUG"`                     | Regex *not* match                          |
| `col(2) == "INFO" && col(4) =~ "CPU"`   | Logical AND of two conditions              |
| `col(2) == "INFO" || col(2) == "WARN"`  | Logical OR                                 |

---

## Examples

```bash
## Extract columns 2 and 4 with space as delimiter, max 4 splits
xcut --input sample_logs.txt --cols 2,4 --delim ' ' --max-split 4

# Filter lines where column 3 matches regex and output to a file
xcut --input sample_logs.txt --filter 'col(3) =~ "ERROR"' --output errors.txt

# Only take the first 10 lines, skip the header
xcut --input data.csv --no-header --head 10 --cols 1,2,3 --delim ','

# Use regex negation
xcut --input logs.txt --filter 'col(3) !~ "DEBUG"'
```

---

## PowerShell Note (Windows)

PowerShell has issues interpreting `!~`, quotes, and parentheses.
For best results on Windows:

- Use **cmd.exe** or **Git Bash**
---



# xcut – A Flexible Field Extractor and Filter Tool for the Command Line

`xcut` is a fast and flexible CLI tool for extracting and filtering fields from structured text, inspired by `cut`, `awk`, and `jq`.
It is written in Rust and supports cross-platform usage (Linux, macOS, Windows via CMD or Git Bash).

---

## Features

- Extract specific fields (columns) by index
- Flexible delimiter control (e.g., space, comma, tab)
- Field filtering with boolean expressions or regex
- Regex *and* logical expressions: `col(2) == "INFO" && col(4) =~ "CPU"`
- Configurable output delimiter
- Works with stdin and files

---

## Usage

```sh
xcut [OPTIONS]
```

### Basic Options

| Option           | Description                                                                 |
|------------------|-----------------------------------------------------------------------------|
| `--input <FILE>` | Input file path. If omitted, reads from stdin                               |
| `--cols <N,..>`  | Comma-separated list of 1-based field indices to output                     |
| `--delim <STR>`  | Input delimiter (default: whitespace)                                       |
| `--max-split N`  | Maximum number of fields to split per line (e.g., split into at most N)     |
| `--filter <EXPR>`| Filter expression (supports regex, boolean logic, `col(N)` substitution)    |
| `--out-delim <S>`| Output delimiter when printing selected columns (default: space)            |

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
# Extract column 3 and 4 from lines where column 2 is INFO
xcut --input sample_logs.txt --cols 3,4 --filter 'col(2) == "INFO"'

# Match lines where column 4 ends with CPU
xcut --cols 3,4 --delim ' ' --max-split 4 --filter 'col(4) =~ ".*CPU"'

# Exclude rows where column 2 is WARN
xcut --cols 2,3 --filter 'col(2) !~ "WARN"'

# Output selected fields with comma delimiter
xcut --cols 2,3 --out-delim ','
```

---

## PowerShell Note (Windows)

PowerShell has issues interpreting `!~`, quotes, and parentheses.
For best results on Windows:

- Use **cmd.exe** or **Git Bash**
---

## Planned Features

- [ ] Support for field range: `--cols 1-3,5`
- [ ] TSV, CSV, JSON format modes
- [ ] `--header` to skip or include headers
- [ ] External filter script via `--filter-file`
- [ ] Installable man page via `clap_mangen`

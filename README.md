# dbinfo

[![CI](https://github.com/keddad/dbinfo/actions/workflows/ci.yml/badge.svg)](https://github.com/keddad/dbinfo/actions)
[![Latest Release](https://img.shields.io/github/v/release/keddad/dbinfo?label=latest)](https://github.com/keddad/dbinfo/releases/latest)

A simple, fast, and easy-to-use Rust-based CLI for inspecting database metadata. `dbinfo` provides structural information like tables, column types, row counts, and indexes in a clean JSON format.

## Features

- **Multi-Database Support**: Works with **PostgreSQL**, **MySQL**, and **SQLite**.
- **Structural Insights**: Reports table sizes, row counts, column types, and indexes.
- **CI/CD Ready**: Outputs pure JSON for easy integration with scripts and monitoring tools.
- **Cross-Platform**: Binaries available for Linux, macOS, and Windows (amd64 & arm64).

## Installation

Download the latest binary for your platform from the [Releases Page](https://github.com/keddad/dbinfo/releases/latest).

## Usage

Simply provide the database URI as the first argument:

```bash
# PostgreSQL
dbinfo "postgres://user:password@localhost:5432/dbname"

# MySQL
dbinfo "mysql://user:password@localhost:3306/dbname"

# SQLite
dbinfo "./my_database.db"
dbinfo "file://my_database"
```

### Example Output

Running `dbinfo` against a PostgreSQL instance:

```json
{
  "tables": [
    {
      "name": "users",
      "size_bytes": 32768,
      "row_count": 1,
      "columns": [
        ["id", "integer"],
        ["name", "text"],
        ["role", "text"]
      ]
    }
  ],
  "indexes": [
    {
      "name": "users_pkey",
      "table_name": "users",
      "index_size": 16384,
      "index_scans": 0
    }
  ],
  "views": [
    {
      "name": "admin_users",
      "size": 0
    }
  ]
}
```

## Integration with `jq`

Since `dbinfo` outputs standard JSON, it pairs perfectly with `jq` for quick analysis:

**List all table names:**
```bash
dbinfo "postgres://..." | jq -r '.tables[].name'
```

**Find columns for a specific table (e.g., 'users'):**
```bash
dbinfo "postgres://..." | jq -r '.tables[] | select(.name=="users") | .columns[] | "\(.[0]) (\(.[1]))"'
```

**Get the total row count across all tables:**
```bash
dbinfo "postgres://..." | jq '[.tables[].row_count] | add'
```

**List large tables (> 1MB):**
```bash
dbinfo "postgres://..." | jq '.tables[] | select(.size_bytes > 1048576) | {name, size_bytes}'
```
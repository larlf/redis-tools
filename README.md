
# Introduction

This project provides a set of development tools for Redis, with current functions mainly focusing on saving and loading data from specified databases.

# Commands

## Save

```bash
# Basic command that exports the database with index 0 to data.json in the current directory
# The prefix "redis://" can be omitted
redis-tools save -u 127.0.0.1
# Specifies a password, database, and output file
redis-tools save -u redis://:password@127.0.0.1/2 -f db2.json
```

If you encounter the following error:
`[ERROR] Error getting string value: mybitmap, error: Cannot convert from UTF-8- TypeError`
It indicates that there is non-text data in your database. Try exporting in binary format using the command below.

```bash
# Export data in binary mode
redis-tools save -u redis://:password@127.0.0.1/2 -f db2.bin -m bin
```

## Load

```bash
# Load the exported file into the specified database
redis-tools load -u redis://127.0.0.1/3 -f db2.json
# Load data in binary format, specifying a password
redis-tools load -u redis://:password@127.0.0.1/3 -f db2.bin -m bin
```

## Help

```bash
# View help
redis-tools --help
```

# Contact

Mail: [larlf.wang@gmail.com](mailto://larlf.wang@gmail.com)

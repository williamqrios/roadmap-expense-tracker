### Expense Tracker
Project inspiration taken from [roadmap.sh](https://roadmap.sh/projects/expense-tracker). Built with Rust 1.84.0. 

Expenses are stored in a CSV file where the columns are separated by `;` to avoid issues with using `,` as a decimal separator. The dates are in the format %Y-%m-$d.

### Command list 
- `add --description <DESC> --amount <NUM> --date <DATE>` - adds a new expense with a given amount and description; the date is optional (defaults to today's date). When provided, the date must follow the format: %Y-%m-%d.
- `update --id <NUM> --description <DESC> --amount <NUM> --date <DATE>.` - updates an expense (can update only the description, amount, date) 
- `list` - lists all expenses
- `list --month <NUM>` - lists only expenses of the provided month and the current year 
- `summary` - computes the total of expenses 
- `summary --month <NUM>` - computes the total expenses for a given month and the current year 
- `delete --id <NUM>` - permanently deletes an expense by providing its ID

### Examples
No installation, building directly from source:
```
cargo run -- [COMMANDS] [ARGS]
```
Adding a new task:
```
cargo run -- add --description "dog surgery" --amount 3000 
# Output: Successfully added new expense with ID 1
```
Deleting a task: 
```
cargo run -- delete --id 1 
# Output: Successully deleted entry with ID 1
```
Updating a task: 
```
cargo run -- update --id 1 --amount 1000
# Output: Successfully updated expense with ID 1
```
Listing tasks:
```
cargo run -- list
# Output: 
# ID  | Date       | Amount     | Description
# 2   | 2024-05-03 | 3000.00    | dog surgery
# 3   | 2025-01-16 | 0.00       | nothing
# 4   | 2025-01-16 | 323.00     | phone
# 5   | 2025-01-16 | 3343.00    | new computer
```
Obtaining the summary: 
```
cargo run -- summary
# Output: Total expenses: 6666
```
Obtaining the summary given a month: 
```
cargo run -- summary --month 01
# Output: Total expenses for January: 3666
```

### Crates used 
- `clap` for easily parsing command line arguments 
- `csv` for easily writing and reading CSV files 
- `serde` for serializing/deserializing data, making it easier to read from and write to CSV files
- `chrono` for dealing with dates 

# minigrep

## Description
This is a simple project that use rust language. All of codes are from the [Rust documentation](https://rust-lang.tw/book-tw/ch12-00-an-io-project.html). I practice rust by this project. 

This project will return contents consistent with given "query_string" in the target file.

## Usage

Use the command below at the terminal

### For MacOS
```rust
// 1. With case senstitive
$ ./minigrep <query_string> <target_file>

// 2. With case insensitive
$ IGNORE_CASE=1 ./minigrep <query_string> <target_file>
```

### For Windows (powerShell)
```rust
// 1. With case sensitive
PS> .\minigrep <query_string> <target_file>

// 2. With case insensitive
PS> $Env:IGNORE_CASE=1
PS> .\minigrep <query_string> <target_file>
// Remove environment variable, if need
PS> Remove-Item Env:IGNORE_CASE
```
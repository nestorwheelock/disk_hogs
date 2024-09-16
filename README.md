
# disk_hogs

**disk_hogs** is a Rust-based tool that generates a report of large directories starting from a specified path. It scans directories, calculates their sizes, and provides a report of the largest directories. The report can either be displayed on the terminal or saved to a file.

## Features

- Generates a report of large directories starting from a specified path.
- Supports multi-threaded scanning using `rayon` for faster performance.
- Displays the report in the terminal or saves it to a file.
- Automatically excludes system directories like `/proc`, `/dev`, `/sys`, etc., to avoid unnecessary scanning.

## Requirements

- Rust toolchain installed (for compiling and running the program)
- Unix-based system (Linux/macOS)

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/disk_hogs.git
   cd disk_hogs
   ```

2. **Build the project using Cargo**:
   ```bash
   cargo build --release
   ```

## Usage

You can run `disk_hogs` to scan directories and generate a report. You can optionally save the report to a file.

### Example 1: Display the Report in the Terminal

To run the program and display the report of the largest directories:

```bash
./target/release/disk_hogs /path/to/start
```

The report will display the largest directories under `/path/to/start`, sorted by size.

### Example 2: Save the Report to a File

To save the report to a file named `directory_report.txt`, run the program with the `save` argument:

```bash
./target/release/disk_hogs save /path/to/start
```

After running the command, the report will be saved to `directory_report.txt` in the current directory.

### Additional Features

- The program will automatically paginate the output if the report is too long to fit on the screen. You can press `Enter` to continue viewing the next page of the report.
- If no directory is specified, the program will default to the home directory as the starting path.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

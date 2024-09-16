use rayon::prelude::*;
use std::env;
use std::fs::{self};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() -> io::Result<()> {
    // Check for "save" argument and optional custom path
    let args: Vec<String> = env::args().collect();
    let save_to_file = args.contains(&"save".to_string());
    let start_path = args.get(1).map_or_else(
        || dirs::home_dir().expect("Could not find home directory"),
        |p| PathBuf::from(p),
    );

    let output_file = "directory_report.txt";
    let mut file = if save_to_file {
        Some(fs::File::create(output_file)?)
    } else {
        None
    };

    // Convert PathBuf to &str
    let start_path_str = start_path.to_str().expect("Invalid path");

    // Large Directories Report
    let large_dirs_header = format!("Large Directories Report for {}", start_path_str);
    write_output(&mut file, &large_dirs_header)?;
    match find_large_directories(start_path_str, 20) {
        Ok(dir_sizes) => {
            for (size, path) in dir_sizes {
                let dir_info = format!("{}: {:.2} MB", path.display(), size as f64 / 1_048_576.0);
                write_output(&mut file, &dir_info)?;
            }
        }
        Err(e) => {
            eprintln!("Failed to find large directories: {}", e);
            return Err(e);
        }
    }

    let success_message = "\nReport generated successfully!";
    write_output(&mut file, success_message)?;

    if save_to_file {
        println!("Report saved to {}", output_file);
    } else {
        display_report_with_more(output_file)?;
    }

    Ok(())
}

fn write_output(file: &mut Option<fs::File>, output: &str) -> io::Result<()> {
    if let Some(ref mut f) = file {
        writeln!(f, "{}", output)?;
    }
    println!("{}", output);
    Ok(())
}

fn find_large_directories(path: &str, max_dirs: usize) -> io::Result<Vec<(u64, PathBuf)>> {
    let dir_sizes = Arc::new(Mutex::new(HashMap::new()));
    let excluded_dirs = vec!["/proc", "/dev", "/sys", "/run", "/tmp", "/var/run"];

    fn visit_dirs(
        dir: &Path,
        dir_sizes: Arc<Mutex<HashMap<PathBuf, u64>>>,
        excluded_dirs: &[&str],
    ) -> u64 {
        // Skip excluded directories
        if excluded_dirs.iter().any(|&ex_dir| dir.starts_with(ex_dir)) {
            return 0;
        }

        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries.filter_map(Result::ok).collect::<Vec<_>>(),
            Err(_) => return 0, // Silently skip directories that cannot be read
        };

        let total_size: u64 = entries.par_iter().map(|entry| {
            let path = entry.path();
            let metadata = fs::metadata(&path);
            if let Ok(metadata) = metadata {
                if metadata.is_dir() {
                    visit_dirs(&path, Arc::clone(&dir_sizes), excluded_dirs)
                } else {
                    metadata.len()
                }
            } else {
                0
            }
        }).sum();

        let mut sizes = dir_sizes.lock().unwrap();
        sizes.insert(dir.to_path_buf(), total_size);

        total_size
    }

    visit_dirs(Path::new(path), Arc::clone(&dir_sizes), &excluded_dirs);

    let mut dir_sizes_vec: Vec<(u64, PathBuf)> = dir_sizes.lock().unwrap()
        .iter()
        .map(|(path, &size)| (size, path.clone()))
        .collect();

    dir_sizes_vec.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by size, largest first

    Ok(dir_sizes_vec.into_iter().take(max_dirs).collect())
}

fn display_report_with_more(file_path: &str) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut line_count = 0;
    let page_size = 20; // Number of lines per page

    while let Some(line) = lines.next() {
        let line = line?;
        writeln!(stdout, "{}", line)?;
        line_count += 1;

        if line_count >= page_size {
            line_count = 0;
            writeln!(stdout, "-- More -- (press Enter to continue)")?;
            let _ = stdin.lock().lines().next();
        }
    }

    Ok(())
}


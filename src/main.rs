use std::path::{Path, PathBuf};
use std::process::exit;
use std::{env, fs, io};
use tempfile::NamedTempFile;

fn main() {
    let path = match find_path() {
        Some(path) => path,
        None => {
            print_help();
            exit(0);
        }
    };

    if let Err(err) = do_write(path) {
        eprintln!("{:?}", err);
        exit(1);
    }
}

fn find_path() -> Option<PathBuf> {
    let mut args = env::args();

    if args.len() != 2 {
        return None;
    }

    let arg = args.nth(1)?;

    if arg == "--help" || arg == "-h" {
        return None;
    }

    Some(PathBuf::from(arg))
}

fn print_help() {
    println!("usage: sp [file]")
}

fn do_write(path: impl AsRef<Path>) -> io::Result<()> {
    let mut tmpfile = NamedTempFile::new()?;
    let stdin = io::stdin();
    io::copy(&mut stdin.lock(), &mut tmpfile)?;

    if let Err(err) = tmpfile.persist(&path) {
        if err.error.kind() == io::ErrorKind::Other {
            fs::copy(err.file.path(), path.as_ref())?;
        } else {
            return Err(err.error);
        }
    }

    Ok(())
}

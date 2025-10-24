use std::fs;
use std::io;
use std::path::PathBuf;

use crate::hash;

pub fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf, hash: &String) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dest_path, hash)?;
        } else {
            copy_file(&entry.path(), &dest_path, hash);
        }
    }
    Ok(())
}

pub fn copy_file(source_file: &PathBuf, target_file: &PathBuf, hash: &String) {
    let start_md5: String = match hash::md5sum(source_file) {
        Ok(start_md5) => start_md5,
        Err(e) => {
            eprintln!("carbon: md5sum: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = fs::copy(source_file, target_file) {
        eprintln!("carbon: {e}");
        std::process::exit(1);
    }

    let end_md5: String = match hash::md5sum(target_file) {
        Ok(end_md5) => end_md5,
        Err(e) => {
            eprintln!("carbon: md5sum: {e}");
            std::process::exit(1);
        }
    };

    if start_md5 != end_md5 {
        eprintln!(
            "carbon: mismatch in hash after copy {:?}({}) -> {:?}({})",
            source_file, start_md5, target_file, end_md5
        )
    } else {
        println!(
            "carbon: match in hash after copy {:?}({}) -> {:?}({})",
            source_file, start_md5, target_file, end_md5
        )
    }
}

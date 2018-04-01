use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashSet;


// fn test(cb: &Fn(&DirEntry)) -> io::Result<()> {

// }


fn test(dir_entry: &DirEntry) {

}


/**
 * Gets called for every directory in the target path that has no
 * corresponding path in the source directory.
 */
fn target_extra_dir_handler() {
    println!("Extra directory found in target path: ");
}


/**
 * Gets called for every file in the target path that has no
 * corresponding path in the source directory.
 */
fn target_extra_file_handler() {
    println!("Extra file found in target path: ");
}


/**
 * In given path, replace the source path with tie target path
 */
fn source_path_to_target_path(full_source_path: &Path, source_path: &Path, target_path: &Path) -> PathBuf {
    assert!(full_source_path.starts_with(source_path));

    let striped_path     = full_source_path.clone().strip_prefix(source_path).unwrap();
    let full_target_path = PathBuf::from(target_path.clone()).join(striped_path);

    full_target_path
}


// one possible implementation of walking a directory only visiting files
fn recursive_walker(source_path: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if source_path.is_dir() {
        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let path  = entry.path();
            if path.is_dir() {
                recursive_walker(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}


fn main() {
    let args:     Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Too few arguments.");
    } else {
        let ref source_path = args[1];
        let ref target_path = args[2];

        println!("source_path: {}", source_path);
        println!("target_path: {}", target_path);

        let source_path = Path::new(source_path);
        let target_path = Path::new(target_path);
        
        recursive_walker(source_path, &test);
    }
}




#[test]
fn test_source_path_to_target_path() {
    let expected = PathBuf::from("b/c");
    let actual   = source_path_to_target_path(Path::new("a/c"), Path::new("a"), Path::new("b"));
    
    assert_eq!(actual, expected);
}

use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashSet;

// #[macro_use] extern crate maplit;

// fn test(cb: &Fn(&DirEntry)) -> io::Result<()> {

// }


#[derive(Hash, Eq, PartialEq)]
enum CachedFileType {
    Directory,
    File,
    SymbolicLink
}


#[derive(Hash, Eq, PartialEq)]
struct CachedDirEntry {
    file_name: String,
    file_type: CachedFileType
}


impl CachedDirEntry {
    fn new(file_name: String, file_type: CachedFileType) -> CachedDirEntry {
        CachedDirEntry { file_name:file_name, file_type:file_type }
    }

    fn from_dir_entry(dir_entry: &DirEntry) -> CachedDirEntry {
        let file_name     = dir_entry.file_name().into_string().unwrap();
        let file_type_obj = dir_entry.file_type().unwrap();
        let file_type     = match (
            file_type_obj.is_dir(),
            file_type_obj.is_file(),
            file_type_obj.is_symlink(),
        ) {
            (true,  false, false) => CachedFileType::Directory,
            (false, true,  false) => CachedFileType::File,
            (false, false, true ) => CachedFileType::SymbolicLink,
            _ => panic!(),
        };
        CachedDirEntry { file_name:file_name, file_type:file_type }
    }
}
    
// #[derive(PartialEq)]
// #[derive(Eq, PartialEq)]
#[derive(Hash, Eq, PartialEq)]
enum DiffEntry {
    OnlyInLeft       (CachedDirEntry),
    OnlyInRight      (CachedDirEntry),
    InBoth           (CachedDirEntry),
    UpToDateConverted(CachedDirEntry, CachedDirEntry),
    OutdatedConverted(CachedDirEntry, CachedDirEntry)
}


type CachedDirEntrySet = HashSet<CachedDirEntry>;
impl CachedDirEntrySet {
    fn from_dir_entry_set(dir_entry_set: &DirEntrySet) -> CachedDirEntrySet {
        // let cached_dir_entry_set = CachedDirEntrySet.new();
        let cached_dir_entry_set = dir_entry_set.map(|x| );
        
        // CachedDirEntry { file_name:file_name, file_type:file_type }
    }
}

type DirEntrySet       = HashSet<DirEntry>;



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


// fn is_dir(entry: &DirEntry) {
//     let base_path = entry.path();
//     let isdir     = base_path.is_dir();

//     isdir
// }


fn dir_entry_set(path: &Path) -> HashSet<PathBuf> {
    let mut entry_set: HashSet<PathBuf> = HashSet::new();

    for entry in fs::read_dir(path).unwrap() {
        // let entry     = entry;
        entry_set.insert(entry.unwrap().path().clone());
    }

    entry_set
}


fn extra_target_directory_handler() {
    
}


fn compare_2_dir_entry_sets() {

}


// one possible implementation of walking a directory only visiting files
fn recursive_walker(source_path: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if source_path.is_dir() {

        let source_set = dir_entry_set(source_path);
        println!("source_set: {:?}", source_set);
        
        // let source_set: HashSet<&DirEntry> =  fs::read_dir(source_path).iter().clone().collect();

        // if (is_dir(recursive_walker(&source_path, cb))) {
        //     recursive_walker(&path, cb)?;
        // } else {
        //     cb(&entry);
        // }
    }
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
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


#[cfg(test)]
fn test_path_pair(test_base_path: &Path) -> (PathBuf, PathBuf) {
    let (source_path, target_path) = (
        test_base_path.join("source"),
        test_base_path.join("target")
    );
    assert!(source_path.exists(), "Test source path does not exist: {:?}", source_path);
    assert!(target_path.exists(), "Test target path does not exist: {:?}", target_path);

    (source_path, target_path)
}

    
#[test]
fn test_source_path_to_target_path() {
    let expected = PathBuf::from("b/c");
    let actual   = source_path_to_target_path(Path::new("a/c"), Path::new("a"), Path::new("b"));
    
    assert_eq!(actual, expected);
}


#[test]
fn test_a_has_single_directory() {
    let test_base_path = Path::new("test_data/source_has_single_file");
    let (source_path, target_path) = test_path_pair(test_base_path);

    let expected = vec![
        CachedDirEntry::new(String::from("a"), CachedFileType::File),
    ];

    
    
    // let expected: HashSet<DiffEntry> = HashSet::new();
    //  = hashset!{
    //     Enum::OnlyInLeft("a")
    // };
    // expected.insert(OnlyInLeft("a"));
        
    // let actual   = source_path_to_target_path(Path::new("a/c"), Path::new("a"), Path::new("b"));
    
    // assert_eq!(actual, expected);
}

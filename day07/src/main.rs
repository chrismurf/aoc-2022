use std::fs::File;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::{self, prelude::*, BufReader};

#[derive(Default, Debug)]
struct DirectoryListing {
    dirs: HashMap<String, Box<DirectoryListing>>, 
    files : HashMap<String, usize>
}

impl DirectoryListing {
    fn new() -> Self {
        return DirectoryListing {dirs: HashMap::new(), files: HashMap::new() }
    }

    fn size(&self) -> usize {
        let mut total = 0;
        for (_, size) in self.files.iter() { total += size; }
        for (_, dir) in self.dirs.iter() { total += dir.size(); }
        total
    }

    fn set_file_size(&mut self, path: &mut std::slice::Iter<String>, name: &str, size: usize) {
        // We recurse through subdirectories, until we empty out path, then set the file size
        match path.next() {
            // No more subdirectories, so ensure this child file listing exists
            None => {
                self.files.entry(name.to_string()).or_insert(size);
            },

            // Follow subdirectory listing one level deeper, and recurse
            Some(subdir) => match self.dirs.entry(subdir.to_string()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().set_file_size(path, name, size);
                },

                Entry::Vacant(entry) => {
                    let listing = entry.insert(Box::new(DirectoryListing::new()));
                    listing.set_file_size(path, name, size);
                }
            }
        }
    }

    fn collect_subdirectory_sizes(&self, sizes: &mut Vec<usize>) {
        for dir in self.dirs.values() {
            sizes.push(dir.size());
            dir.collect_subdirectory_sizes(sizes);
        }
    }
}

fn day07() {
    let file = File::open("input.txt").expect("File 'input.txt' not readable.");
    let reader = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok().to_owned()); // Get Strings instead of Result

    let mut root = Box::new(DirectoryListing::new());
    let mut path : Vec<String> = Vec::new();

    for line in reader {
        if line.starts_with("$") {
            let args : Vec<&str> = line[2..].split(" ").collect();
            match args[0] {
                "cd" => match args[1] {
                    "/" => { path.clear(); },
                    ".." => { path.pop(); },
                    dir => { path.push(dir.to_string()); }
                },
                "ls" => { }, // We assume any unrecognized output is from 'ls'
                _ => unreachable!()
            }
        } else {
            let mut parts = line.split(" ");
            let size_or_dir = parts.next().unwrap();
            let name = parts.next().unwrap();
            match size_or_dir {
                // Ignore directory listings.  They have no size.
                "dir" => { },
                // Numerical file sizes - we care about these.  Store them.
                size_str => {
                    let size : usize = size_str.parse().unwrap();
                    root.set_file_size(&mut path.iter(), name, size);
                }
            }
        }
    }

    let mut all_subdirectory_sizes = Vec::new();
    root.collect_subdirectory_sizes(&mut all_subdirectory_sizes);
    all_subdirectory_sizes.sort();
    let total_under_100k: usize = all_subdirectory_sizes.iter().filter(|x| **x <= 100000).sum();
    println!("Total size of files at or under 100kB is {:?}", total_under_100k);

    let current_free_space = 70000000 - root.size();
    let need_to_free = 30000000 - current_free_space;

    let smallest_possible = all_subdirectory_sizes.iter().filter(|x| **x >= need_to_free).next();
    println!("Smallest directory to free up enough space is {:?}", smallest_possible.unwrap());
}

fn main() -> io::Result<()> {
    day07();
    Ok(())
}

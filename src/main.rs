#![feature(option_result_contains)]

use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::env;
use std::process::{Command};
use file_diff::{diff};

struct FileIterator {
    dirs: Vec<PathBuf>,
    files: Option<ReadDir>,
}

impl From<&str> for FileIterator {
    fn from(path: &str) -> Self {
        FileIterator {
            dirs: vec![PathBuf::from(path)],
            files: None,
        }
    }
}

impl Iterator for FileIterator {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        loop {
            while let Some(read_dir) = &mut self.files {
                match read_dir.next() {
                    Some(Ok(entry)) => {
                        let path = entry.path();
                        if let Ok(md) = entry.metadata() {
                            if md.is_dir() {
                               self.dirs.push(path.clone());
                                continue;
                            }
                        }
                        return Some(path);
                    }
                    None => {
                        self.files = None;
                        break;
                    }
                    _ => { }
                }
            }
            while let Some(dir) = self.dirs.pop() {
                let read_dir = fs::read_dir(&dir);
                if let Ok(files) = read_dir {
                    self.files = Some(files);
                    return Some(dir);
                }
            }
            break;
        }
        return None;
    }
}

fn create() {
    let mut iter = FileIterator::from("/");
    for _ in 0..100000 {
        println!("{:?}", iter.next().unwrap());
    }
}

fn filemon() {
    let args: Vec<String> = env::args().collect();
    let first_file = &args[2];
    let second_file = &args[3];
    let file_check = diff(first_file, second_file);

    if file_check == false {
        let file_changes = Command::new("comm")
                                  .arg("-1")
                                  .arg("-3")
                                  .arg(first_file)
                                  .arg(second_file)
                                  .output().unwrap_or_else(|e| {
                                  panic!("{}", e)
    });

    if file_changes.status.success() {
        let s = String::from_utf8_lossy(&file_changes.stdout);
        print!("\n{}", s);

    } else {
        let s = String::from_utf8_lossy(&file_changes.stderr);
        print!("Error:\n{}", s);
    }
}
}

fn refresh() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let current_result_file = &args[2];
    let new_file_name = &args[3];
    let new_result_file = &args[4];
    fs::rename(current_result_file, new_file_name);
    fs::rename(new_result_file, current_result_file);
    Ok(())
}

fn kernel() {
    let kernel_version = Command::new("uname")
                                .arg("-a")
                                .output()
                                .expect("Oh nooo failed to execute process!");

    let kernel_version_print = String::from_utf8(kernel_version.stdout).expect("Not a UTF-8 encoded file!");
    println!("{}", kernel_version_print);
}

fn id() {
    let id = Command::new("id")
                    .output()
                    .expect("Oh nooo failed to execute process!");

    let id_print = String::from_utf8(id.stdout).expect("Not a UTF-8 encoded file!");
    println!("{}", id_print);
}

fn tcp() {
    let tcp = Command::new("netstat")
                    .arg("-ntpl")
                    .arg("2>/dev/null")
                    .output()
                    .expect("Oh nooo failed to execute process!");

    let tcp_print = String::from_utf8(tcp.stdout).expect("Not a UTF-8 encoded file!");
    println!("{}", tcp_print);
}

fn udp() {
    let udp = Command::new("netstat")
                    .arg("-nupl")
                    .arg("2>/dev/null")
                    .output()
                    .expect("Oh nooo failed to execute process!");

    let udp_print = String::from_utf8(udp.stdout).expect("Not a UTF-8 encoded file!");
    println!("{}", udp_print);
}

fn search_suid() {
    let search_suid = Command::new("find")
                    .arg("/")
                    .arg("-type")
                    .arg("d")
                    .arg("-name")
                    .arg("proc")
                    .arg("-prune")
                    .arg("-o")
                    .arg("-type")
                    .arg("f")
                    .arg("-perm")
                    .arg("-4000")
                    .arg("-print")
                    .output().unwrap_or_else(|e| {
                    panic!("{}", e)
     });

    if search_suid.status.success() {
        let s = String::from_utf8_lossy(&search_suid.stdout);
        print!("\n{}", s);

    } else {
        let s = String::from_utf8_lossy(&search_suid.stderr);
        print!("Error:\n{}", s);
    }
}

fn external_storage() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let current_result_file = &args[2];
    let file_path = Path::new(current_result_file);
    let external_storage_input = File::open(file_path)?;
    let external_storage_buffered = BufReader::new(external_storage_input);

    for line in external_storage_buffered.lines() {
        let external = line.unwrap().replace('"', "");
        if external.contains(&"/storage") {
            println!("{}", external);
        }
    }
    Ok(())
}

fn apps() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let current_result_file = &args[2];
    let file_path = Path::new(current_result_file);
    let apps_input = File::open(file_path)?;
    let apps_buffered = BufReader::new(apps_input);

    for line in apps_buffered.lines() {
        let apps = line.unwrap().replace('"', "");
        if apps.contains(&"/data/data") {
            println!("{}", apps);
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Please enter a command :)");
    }

    if &args[1] == "create" {
        create();
    }

    if &args[1] == "filemon" {
        filemon();
    }

    if &args[1] == "refresh" {
        refresh();
    }

    if &args[1] == "kernel" {
        kernel();
    }

    if &args[1] == "id" {
        id();
    }

    if &args[1] == "tcp" {
        tcp();
    }

    if &args[1] == "udp" {
        udp();
    }

    if &args[1] == "search-suid" {
        search_suid();
    }

    if &args[1] == "external-storage" {
        external_storage();
    }

    if &args[1] == "apps" {
        apps();
    }
    Ok(())
}

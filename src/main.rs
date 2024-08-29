use home;
use sha256;
use std::{
    collections::{HashMap, VecDeque},
    fs::{metadata, read, read_dir},
    thread,
};

fn main() {
    let mut start = home::home_dir().unwrap();
    start.push("Downloads");
    start.push("love_decoded");

    let mut queue = VecDeque::from([start.into_os_string().into_string().unwrap()]);
    let mut files: Vec<_> = vec![];

    while queue.len() > 0 {
        let path = queue.pop_front();
        match read_dir(path.unwrap().clone()) {
            Ok(val) => {
                for i in val {
                    match i {
                        Ok(name) => {
                            let meta = metadata(name.path().clone());
                            match meta {
                                Ok(val) => {
                                    println!("{:?}", name.path());
                                    if val.is_dir() {
                                        queue.push_back(
                                            name.path().into_os_string().into_string().unwrap(),
                                        );
                                    } else if val.is_file() {
                                        files.push(name)
                                    }
                                }
                                Err(..) => {}
                            }
                        }
                        Err(..) => {}
                    };
                }
            }
            Err(..) => {}
        };
    }

    let mut hashes: HashMap<String, &mut Vec<String>> = HashMap::new();

    for i in files {
        let bytes = std::fs::read(i.path()).unwrap();
        let hash = sha256::digest(&bytes);

        match hashes.get(&hash) {
            Some(&mut items) => {
                items.push(i.path().into_os_string().into_string().unwrap());
            }
            _ => {
                hashes.insert(
                    hash,
                    &mut vec![i.path().into_os_string().into_string().unwrap()],
                );
            }
        }
    }
}

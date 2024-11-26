use chrono;
use ctrlc;

use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::exit;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
#[derive(Debug)]

struct Memo {
    id: String,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
}

fn main() {
    let memos = Arc::new(Mutex::new(load_from_file()));
    let memos_clone = Arc::clone(&memos);

    let (tx, rx) = mpsc::channel::<()>();

    ctrlc::set_handler(move || {
        println!("Interrupted");
        let memos = memos.lock().unwrap();
        save_to_file(&memos);
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        println!("Enter command: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let text = input.trim();

        match text {
            "exit" => {
                let memos = memos.lock().unwrap();
                save_to_file(&memos);
                break;
            }
            "add" => {
                println!("Enter title: ");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");

                println!("Enter content: ");
                let mut content = String::new();
                io::stdin()
                    .read_line(&mut content)
                    .expect("Failed to read line");

                let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let mut memos = memos.lock().unwrap();
                let new_memo = Memo {
                    id: (memos.len() + 1).to_string(),
                    title: title.trim().to_string(),
                    content: content.trim().to_string(),
                    created_at: now.clone(),
                    updated_at: now,
                };

                add_memo(&mut memos, new_memo)
            }
            "get" => {
                let memos = memos.lock().unwrap();
                println!("{:#?}", memos);
            }
            _ => println!("Invalid command"),
        }
    }
}

fn load_from_file() -> Vec<Memo> {
    let dir_path = "data";
    let file_path = "data/memo.txt";

    fs::create_dir_all(dir_path).expect("Failed to create directory.");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file.");
    let reader = BufReader::new(file);
    let mut memos = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let fields: Vec<&str> = line.split(',').collect();
        let memo = Memo {
            id: fields[0].into(),
            title: fields[1].into(),
            content: fields[2].into(),
            created_at: fields[3].into(),
            updated_at: fields[4].into(),
        };
        memos.push(memo);
    }
    memos
}

fn save_to_file(memos: &[Memo]) {
    let path = "data/memo.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Unreachable!()");

    for memo in memos {
        writeln!(
            file,
            "{},{},{},{},{}",
            memo.id, memo.title, memo.content, memo.created_at, memo.updated_at
        )
        .unwrap()
    }
}

fn add_memo(memos: &mut Vec<Memo>, new_memo: Memo) {
    memos.push(new_memo);
}

fn find_memo<'a>(memos: &'a [Memo], id: &str) -> Option<&'a Memo> {
    memos.iter().find(|memo| memo.id == id)
}

fn find_memo_mut<'a>(memos: &'a mut [Memo], id: &str) -> Option<&'a mut Memo> {
    memos.iter_mut().find(|memo| memo.id == id)
}

fn delete_memo(memos: &mut Vec<Memo>, id: &str) {
    memos.retain(|memo| memo.id != id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_memo_mut() {
        let mut memos: Vec<Memo> = Vec::new();
        let new_memo = Memo {
            id: "1".to_string(),
            title: "title1".to_string(),
            content: "content1".to_string(),
            created_at: "2020-01-01".to_string(),
            updated_at: "2020-01-01".to_string(),
        };

        add_memo(&mut memos, new_memo);

        assert_eq!(memos.len(), 1);
        assert_eq!(memos[0].id, "1");
    }

    #[test]
    fn test_find_memo() {
        let memos = vec![Memo {
            id: "1".to_string(),
            title: "title1".to_string(),
            content: "content1".to_string(),
            created_at: "2020-01-01".to_string(),
            updated_at: "2020-01-01".to_string(),
        }];

        if let Some(memo) = find_memo(&memos, "1") {
            assert_eq!(memo.title, "title1");
        } else {
            panic!("Memo not found");
        }
    }

    #[test]
    fn test_find_memo_mut() {
        let mut memos = vec![Memo {
            id: "1".to_string(),
            title: "title1".to_string(),
            content: "content1".to_string(),
            created_at: "2020-01-01".to_string(),
            updated_at: "2020-01-01".to_string(),
        }];

        if let Some(memo) = find_memo_mut(&mut memos, "1") {
            memo.content = "content updated".to_string();

            assert_eq!(memo.content, "content updated");
        } else {
            panic!("Memo not found");
        }
    }

    #[test]
    fn test_delete_memo() {
        let mut memos = vec![Memo {
            id: "1".to_string(),
            title: "title1".to_string(),
            content: "content1".to_string(),
            created_at: "2020-01-01".to_string(),
            updated_at: "2020-01-01".to_string(),
        }];

        delete_memo(&mut memos, "1");

        assert_eq!(memos.len(), 0);
    }
}

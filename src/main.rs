mod book;
use book::vec_to_json;
use book::Book;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

enum ProgMode {
    ADD,
    GEN,
}

fn get_args() -> Vec<String> {
    return env::args().collect();
}

fn read_file(file_path: &str) -> Option<String> {
    match fs::read_to_string(file_path) {
        Ok(val) => return Some(val),
        Err(_) => return None,
    }
}

fn user_input(promt: &str) -> String {
    let mut buffer: String = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle_stdin = stdin.lock();
    let mut handle_stdout = stdout.lock();
    print!("{}", promt);
    let _ = handle_stdout.flush();
    handle_stdin.read_line(&mut buffer).unwrap();
    return buffer;
}

fn book_form() -> Book {
    let p_name: String = user_input("Please Type the Book name : ");
    let p_author: String = user_input("Please Type the Author name : ");
    let p_synopsys: String = user_input("Please Type the synopsys : ");
    let p_path: String = user_input("Please Type the path : ");
    let p_image: String = user_input("Please Type the Image path : ");
    let p_tag: String = user_input("Please Type the tag : ");
    let parsed_tag: Vec<&str> = p_tag.split(";").collect();
    let mut final_tag: Vec<String> = Vec::new();
    for item in parsed_tag {
        final_tag.push(item.trim().to_owned().to_string());
    }

    return Book {
        name: p_name.trim().to_string(),
        author: p_author.trim().to_string(),
        synopsys: p_synopsys.trim().to_string(),
        path: p_path.trim().to_string(),
        image: p_image.trim().to_string(),
        tag: final_tag,
    };
}

fn main() {
    let user_args: Vec<String> = get_args();
    let mode: ProgMode;
    let user_args_len: usize = user_args.len();
    if user_args_len > 1 {
        match &user_args[1][..] {
            "add" | "a" => mode = ProgMode::ADD,
            "gen" | "g" => mode = ProgMode::GEN,
            _ => {
                eprintln!("ERR: Invalid program mode -> {}", user_args[1]);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("ERR: Did't get any program mode");
        std::process::exit(1);
    }
    match mode {
        ProgMode::ADD => {
            if user_args_len >= 3 {
                let file_content: String =
                    read_file(&user_args[2]).expect("ERR: Failed to read the specified file");
                let mut json_object: Vec<Book> = match Book::parse_json(&file_content) {
                    Some(val) => val,
                    None => {
                        eprintln!("ERR: Failed to parsed the json file");
                        std::process::exit(1);
                    }
                };
                json_object.push(book_form());
                let result_json: String = match vec_to_json(&json_object) {
                    Some(val) => val,
                    None => {
                        eprintln!("ERR: Failed to convert object to json");
                        std::process::exit(1);
                    }
                };
                println!("{result_json}");
            } else {
                eprintln!("ERR: Can't read the specified file.");
                std::process::exit(1);
            }
        }
        ProgMode::GEN => {
            let result: Book = book_form();
            let result_json: String = match result.to_json() {
                Some(val) => val,
                None => {
                    eprintln!("ERR: Failed to convert object to json");
                    std::process::exit(1);
                }
            };
            println!("{result_json}");
        }
    }
    /* let content = read_file(&user_args[1]);
    let mut json_object: Vec<Book> = Book::parse_json(&content).unwrap();

    let dummy: Book = Book {
    name: "test".to_string(),
    author: "me".to_string(),
    path: "".to_string(),
    image: "".to_string(),
    synopsys: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
    tag: vec!["Not".to_string(), "a".to_string(), "real".to_string(), "tag".to_string()]
    };

    json_object.push(dummy);
    let json_str: String = serde_json::to_string(&json_object).unwrap();
    println!("{}", json_str); */
}

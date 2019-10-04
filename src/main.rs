use std::fs;
use std::io::{stdin, stdout, ErrorKind, Write};

/*
** SHOPLIST :
**
** First Rust project, practice only.
** This code is free to use, have fun :)
*/

fn add_to_list(file_content: &mut String) {
    let mut readed = String::new();

    print!("What do you want to add to your list ? : ");

    stdout().flush().unwrap();
    stdin().read_line(&mut readed).expect("Error when reading.");
    file_content.push_str(readed.as_str());
    display_list(&file_content);
}

fn remove_line_in_content(file_content: &mut String, readed: String) {
    let split_content = file_content.split('\n');
    let mut vector_content: Vec<&str> = {
        let mut victor: Vec<&str> = split_content.collect();
        let victor_size: usize = victor.len();

        for i in 0..(victor_size - 1) {
            if victor[i] == "" || victor[i] == "\n" {
                victor.remove(i);
            }
        }
        victor
    };

    let selected_line: i32 = match readed.trim_end().parse() {
        Err(e) => {
            println!("There was an error : {}.", e);
            return;
        }
        Ok(e) => {
            if e > (vector_content.len() as i32 - 1) || e < 0 || vector_content[e as usize] == "\n" || vector_content[e as usize] == "" {
                println!("/!\\ Invalid line option.");
                return;
            } else {
                e
            }
        }
    };

    vector_content.remove(selected_line as usize);
    let yolo = vector_content.join("\n");
    *file_content = yolo;
}

fn delete_from_list(file_content: &mut String) {
    let mut readed = String::new();

    display_deletion_list(&file_content);
    print!("What do you want to remove from your list ? : ");
    stdout().flush().unwrap();
    stdin().read_line(&mut readed).expect("Error when reading.");

    remove_line_in_content(file_content, readed);
    display_list(&file_content);
}

fn get_file_content(filepath: &str) -> String {
    let f = fs::File::open(filepath);

    let _f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match fs::File::create(&filepath) {
                Ok(file_create) => file_create,
                Err(e) => panic!("Error whenn creating the file : {:?}", e),
            },
            other_error => panic!("Problem with file opening action : {:?}", other_error),
        },
    };

    let file_content = fs::read_to_string(filepath).expect("Error when reading.");
    file_content
}

fn display_deletion_list(file_content: &String) {
    let file_content = file_content.split('\n');
    let mut i = 0;

    println!("=== Your actual list =================\n");

    for element in file_content {
        match element {
            "" => {}
            "\n" => {}
            _ => {
                println!("{}\t{}", &i, element);
                i = i + 1;
            }
        }
    }
    println!("\n======================================\n");
}

fn display_list(file_content: &String) {
    let file_content = file_content.split('\n');
    println!("=== Your actual list =================\n");

    for element in file_content {
        match element {
            "" => {}
            "\n" => {}
            _ => {
                println!("- {}", element);
            }
        }
    }
    println!("\n======================================\n");
}

fn main() {
    let filepath = "shoplist.txt";
    let mut file_content = get_file_content(&filepath);
    let mut string = String::new();

    display_list(&file_content);
    println!("What do you want to do ?");
    loop {
        print!("[A]dd, [D]elete, [Q]uit : ");
        stdout().flush().unwrap();
        stdin().read_line(&mut string).expect("Error when reading.");

        match string.as_ref() {
            "A\n" => {
                add_to_list(&mut file_content);
                string.clear();
            }
            "D\n" => {
                delete_from_list(&mut file_content);
                string.clear();
            }
            "Q\n" => {
                let file = fs::File::create(filepath);

                file.unwrap()
                    .write_all(file_content.as_bytes())
                    .expect("Error when updating the file.");
                break;
            }
            "\n" => {
                string.clear();
            }
            _ => {
                println!("Not a valid input, please retry.\n");
                string.clear();
            }
        }
    }
}

use std::io;
use std::path::Path;
// use std::fs::{self, File};
use pwhash::bcrypt;
use std::io::prelude::*;
// use serde_json::Value as JsonValue;

mod user;
use user::user::User;

fn main() {
    let mut current_user: User = User::new(
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
    );
    let mut user_is_login: bool = false;

    loop {
        main_menu();
        let num = get_number_input("please enter a number: ");
        if num >= 3 {
            continue;
        } else {
            if num == 1 {
                let firts_name = get_string_input("Enter your first name: ");
                let last_name = get_string_input("Enter your last name: ");
                let email = get_string_input("Enter your email: ");
                let password = get_string_input("Enter a password: ");

                current_user = User::new(
                    firts_name,
                    last_name,
                    email,
                    bcrypt::hash(password).unwrap(),
                );

                if !Path::new("users.txt").is_file() {
                    let mut file = std::fs::File::create("users.txt").unwrap();
                    file.write_all(serde_json::to_string(&current_user).unwrap().as_bytes())
                        .unwrap();
                }
                user_is_login = true;
            } else if num == 2 {
                let users_path = Path::new("users.txt");
                if users_path.is_file() {
                    let username = get_string_input("Enter your email: ");
                    let password = get_string_input("Enter your password: ");
                    let file = std::fs::read_to_string("users.txt").unwrap();
                    let res: User = serde_json::from_str(&file).unwrap();
                    if *res.get_user_name() == username
                        && bcrypt::verify(&password, res.get_password())
                    {
                        println!("***( Welcome )***");
                        current_user = res;
                        user_is_login = true;
                    }
                }
            } else if num == 0 {
                break;
            }
            println!("user_is_login ==>  {}", user_is_login);
            while user_is_login {
                user_menu();
                let num = get_number_input("Choose an option: ");
                if num > 10 {
                    continue;
                } else {
                    if num == 1 {
                        let dir_name = get_string_input("Enter Directory Name: ");
                        match current_user.create_directory(&dir_name) {
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 2 {
                        let dir_name = get_string_input("Enter Directory Name: ");
                        match current_user.read_dir_content(&dir_name) {
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 3 {
                        let old_dir_name = get_string_input("Enter Directory Name: ");
                        let new_dir_name = get_string_input("Enter New Name: ");
                        match current_user.rename_file(&old_dir_name, &new_dir_name, false) {
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 4 {
                        let dir_name = get_string_input("Enter Directory Name: ");
                        match current_user.remove_directory(&dir_name){
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 5 {
                        let file_name = get_string_input("Enter File Name: ");
                        let file_content = get_string_input("Enter File Content: ");
                        match current_user.create_new_file(&file_name, &file_content){
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 6 {
                        let file_name = get_string_input("Enter File Name: ");
                        match current_user.remove_file(&file_name){
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 7 {
                        let old_file_name = get_string_input("Enter File Name: ");
                        let new_file_name = get_string_input("Enter New Name: ");
                        match current_user.rename_file(&old_file_name, &new_file_name, true){
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 8 {
                        let file_name = get_string_input("Enter File Name: ");
                        let file_content = get_string_input("Enter File Content: ");
                        match current_user.write_file(&file_name, &file_content){
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 9 {
                        let file_name = get_string_input("Enter File Name: ");
                        match current_user.read_file(&file_name){
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 10 {
                        let main_file_name = get_string_input("Enter Main File Name: ");
                        let copy_file_name = get_string_input("Enter Copy File Name: ");
                        match current_user.copy_file(&main_file_name, &copy_file_name){
                            Ok(()) => (),
                            Err(err) => {
                                println!("{}", err);
                                continue
                            }
                        }
                    }else if num == 0 {
                        println!("Goodbye :D");
                        break;
                    }
                }
            }
        }
    }
}

fn main_menu() {
    println!(
        r#"
1- Register
2- Login
0- Exit
    "#
    );
}

fn user_menu() {
    println!(
        r#"
1- Create Directory
2- Read Directory Contents
3- Rename Directory
4- Remove Directory
5- Create New File
6- Remove File
7- Rename File
8- Write File
9- Read File
10- Copy File
0- Exit
    "#
    );
}

fn get_number_input(s: &str) -> u32 {
    println!("{}", s);
    loop {
        let mut input = String::new();

        if let io::Result::Err(_) = io::stdin().read_line(&mut input) {
            println!("Please enter string:");
            continue;
        }

        let input: u32 = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a natural number:");
                continue;
            }
        };

        let min = 0;
        if input >= min {
            return input;
        }
    }
}

fn get_string_input(s: &str) -> String {
    println!("{}", s);
    loop {
        let mut input = String::new();

        if let io::Result::Err(_) = io::stdin().read_line(&mut input) {
            println!("Please enter string:");
            continue;
        }

        let input = input.trim();
        if input.parse::<u32>().is_ok() {
            println!("Please enter string, not number: ");
            continue;
        } else {
            return String::from(input);
        }
    }
}

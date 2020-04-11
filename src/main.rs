use std::io;
use pwhash::bcrypt;
use serde_json::Value as JsonValue;

mod user;
use crate::user::user::User;

fn main() {
    let user =  User::new(
        String::from("Mohsen"),
        String::from("Coder"),
        String::from("test@gmail.com"),
        bcrypt::hash(String::from("kdjsf@fjh")).unwrap()
    );



    /*let json_str = r#"
        {
            "name": "Mohsen",
            "age": 23,
            "is_maile": true
        }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let p: JsonValue = res.unwrap();
        println!("The name is {}", p["name"].as_str().unwrap());
    }else{
        println!("Sorry! Could not parse JSON");
    }*/

//    serde_json::
    /*if let std::io::Result::Err(e) = user.create_new_file("test.txt", "Mohsen I love you, You are my hurt beat."){
        eprintln!("{}", e);
    }*/

    /*if let std::io::Result::Err(e) = user.read_file("ehsgam_farah.txt"){
        eprintln!("{}", e);
    }*/

    /*if let std::io::Result::Err(e) = user.write_file("test2.txt", "lll"){
        eprintln!("{}", e);
    }*/

    /*if let std::io::Result::Err(e) = user.create_directory("test"){
        eprintln!("{}", e);
    }*/

    /*if let std::io::Result::Err(e) = user.remove_directory("test"){
        eprintln!("{}", e);
    }*/

    /*if let std::io::Result::Err(e) = user.remove_file("ehsgam_farah.txt"){
        eprintln!("{}", e);
    }*/

    if let std::io::Result::Err(e) = user.read_dir_content(""){
        eprintln!("{}", e);
    }

    /*match user.create_new_file("hello_2.txt") {
        Ok(x) => println!("File created successfully!"),
        Err(err_message) => println!("{}", err_message)
    }*/

//    main_menu();

//    let _num = get_number_input("please enter a number: ");

//    let s = get_string_input("Please enter your name: ");
//    println!("the string is => {}", s);

//    let digest = md5::compute(b"Hello World");
//    let digest = format!("{:x}", digest);

//    println!("{}", format!("{}/{}", "kos", "mos"));
}

fn main_menu() {
    println!(r#"
1- Register
2- Login
0- Exit
    "#);
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
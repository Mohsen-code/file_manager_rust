pub mod user {
    use std::fs::{self, File};
    use std::io::prelude::*;
    use std::path::Path;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    }

    impl User {
        pub fn get_user_name(&self) -> &String {
            &self.email
        }

        pub fn get_password(&self) -> &String {
            &self.password
        }

        pub fn new(first_name: String, last_name: String, email: String, password: String) -> User {
            let dir_name = User::hash_string(&email);
            let user = User {
                first_name,
                last_name,
                email,
                password,
            };
            if Path::new("users.txt").is_file() {
                fs::remove_file("users.txt").unwrap();
            }
            if !Path::new(&dir_name).is_dir() {
                fs::create_dir(&dir_name).unwrap();
            }
            user
        }

        pub fn create_directory(&self, dir_name: &str) -> Result<(), String> {
            let path = format!("{}/{}", User::hash_string(&self.email), dir_name);
            if Path::new(&path).is_dir() {
                Err(String::from("This directory is exist!"))
            } else {
                fs::create_dir(&path).unwrap();
                Ok(())
            }
        }

        pub fn remove_directory(&self, dir_name: &str) -> Result<(), String> {
            let path = format!("{}/{}", User::hash_string(&self.email), dir_name);
            if Path::new(&path).is_dir() {
                fs::remove_dir_all(&path).unwrap();
                Ok(())
            } else {
                Err(String::from("Directory not found!"))
            }
        }

        pub fn create_new_file(&self, file_name: &str, content: &str) -> Result<(), String> {
            let dir_name = User::hash_string(&self.email);
            if Path::new(&dir_name).is_dir() {
                let path = format!("{}/{}", &dir_name, file_name);
                if !Path::new(&path).exists() {
                    let mut file = File::create(&path).unwrap();
                    file.write_all(content.as_bytes()).unwrap();
                    return Ok(());
                }
                Err(String::from("file could not created!"))
            } else {
                Err(String::from("This directory is not exist!"))
            }
        }

        pub fn remove_file(&self, file_name: &str) -> Result<(), String> {
            let path = format!("{}/{}", User::hash_string(&self.email), file_name);
            let path = Path::new(&path);
            if path.is_file() {
                fs::remove_file(&path).unwrap();
                Ok(())
            } else {
                Err(String::from("File not found!"))
            }
        }

        pub fn read_file(&self, file_name: &str) -> Result<(), String> {
            let path = format!("{}/{}", User::hash_string(&self.email), file_name);
            let path = Path::new(&path);
            if path.is_file() {
                let file = fs::read_to_string(path).unwrap();
                println!("{}", &file);
                Ok(())
            } else {
                Err(String::from("File not found!"))
            }
        }

        pub fn write_file(&self, file_name: &str, content: &str) -> Result<(), String> {
            let path = format!("{}/{}", User::hash_string(&self.email), file_name);
            let path = Path::new(&path);
            if path.is_file() {
                /*
                * another way to write file in rust:
                * in this way if file not exist, rust create that file an then write content to it
                * let mut file = File::create(path)?;
                * file.write_all(content.as_bytes())?;
                */
                fs::write(path, content).unwrap();
                Ok(())
            } else {
                Err(String::from("File not found!"))
            }
        }

        pub fn copy_file(&self, file_one:&str, file_two:&str) -> Result<(), String> {
            let base_path = User::hash_string(&self.email);
            let path = format!("{}/{}", base_path, file_one);
            let path = Path::new(&path);
            if path.is_file() {
                println!("we are here");
                fs::copy(path, format!("{}/{}", base_path, file_two)).unwrap();
                Ok(())
            }else{
                Err(String::from("File not found!"))
            }
        }

        pub fn rename_file(&self, old_name: &str, new_name: &str, is_file: bool) -> Result<(), String> {
            let base_path = User::hash_string(&self.email);
            let path = format!("{}/{}", base_path, old_name);
            let path = Path::new(&path);
            if is_file {
                if path.is_file() {
                    fs::rename(path, format!("{}/{}", base_path, new_name)).unwrap();
                    Ok(())
                } else {
                    Err(String::from("File not found!"))
                }
            }else {
                if path.is_dir() {
                    fs::rename(path, format!("{}/{}", base_path, new_name)).unwrap();
                    Ok(())
                } else {
                    Err(String::from("Directory not found!"))
                }
            }
        }

        pub fn read_dir_content(&self, dir_name: &str) -> Result<(), String> {
            let path = format!("{}/{}", User::hash_string(&self.email), dir_name);
            let path = Path::new(&path);
            if path.is_dir() {
                User::read_directory(&path);
                Ok(())
            } else {
                Err(String::from("Directory not found!"))
            }
        }

        fn read_directory(path: &Path) {
            let kos = path.file_name().unwrap().to_str().unwrap();
            println!("{}/", kos);
            let entries = fs::read_dir(path).unwrap();
            let mut i = 1;
            for entry in entries {
                let entry = &entry.unwrap();
                let path = entry.path();
                let file_name = format!("\t {}- {}", i, entry.file_name().into_string().unwrap());
                /*if path.is_dir() {
                    file_name = format!("\t {}- {}/", i, entry.file_name().into_string().unwrap());
                }*/
                println!("{}", &file_name);
                if path.is_dir() {
                    User::read_directory(&path);
                }
                i += 1;
            }
        }

        fn hash_string(s: &str) -> String {
            let hash_bytes = md5::compute(s.as_bytes());
            let hash_string = format!("{:x}", hash_bytes);
            hash_string
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //    use crate::user::user::User;
    use pwhash::bcrypt;
    lazy_static::lazy_static! {
        pub static ref USER: user::User = user::User::new(
            String::from("Mohsen"),
            String::from("Coder"),
            String::from("test2@gmail.com"),
            bcrypt::hash(String::from("kdjsf@fjh")).unwrap()
        );
    }

    #[test]
    fn create_user(){
        let user = user::User::new(
            String::from("Mohsen"),
            String::from("Coder"),
            String::from("test2@gmail.com"),
            bcrypt::hash(String::from("kdjsf@fjh")).unwrap()
        );
        assert!(true);
    }

    #[test]
    fn test_create_directory() {
        assert_eq!(USER.create_directory("my_dir"), Ok(()));
    }

    #[test]
    fn test_rename_directory() {
        assert_eq!(USER.rename_file("Mohsen", "update_mohsen", false), Ok(()));
    }

    #[test]
    fn test_remove_directory() {
        assert_eq!(USER.remove_directory("my_dir"), Ok(()));
    }

    #[test]
    fn test_create_new_file() {
        assert_eq!(USER.create_new_file("my_file.txt", "Hello World"), Ok(()));
    }

    #[test]
    fn test_remove_file() {
        assert_eq!(USER.remove_file("my_file.txt"), Ok(()));
    }

    #[test]
    fn test_read_file() {
        assert_eq!(USER.read_file("my_file.txt"), Ok(()));
    }

    #[test]
    fn test_rename_file() {
        assert_eq!(USER.rename_file("my_file.txt", "my_new_file.txt", true), Ok(()));
    }

    #[test]
    fn test_write_file() {
        assert_eq!(USER.write_file("my_file.txt", "This is for test!"), Ok(()));
    }

    #[test]
    fn test_copy_file() {
        assert_eq!(USER.copy_file("my_new_file.txt", "b.txt"), Ok(()));
    }

    #[test]
    fn test_read_dir_content() {
        assert_eq!(USER.read_dir_content(""), Ok(()));
    }
}
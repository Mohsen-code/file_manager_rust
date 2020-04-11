pub mod user {
    use std::fs::{self, File};
    use std::io::prelude::*;
    use std::path::Path;

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
            if !Path::new(&dir_name).is_dir() {
                user.create_directory(&dir_name);
            }
            user
        }

        pub fn create_directory(&self, dir_name: &str) -> std::io::Result<()> {
            let path = format!("{}/{}", User::hash_string(&self.email), dir_name);
            if Path::new(&path).is_dir() {
                return std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                                "This directory is exist!"));
            } else {
                fs::create_dir(&path)?;
                Ok(())
            }
        }

        pub fn remove_directory(&self, dir_name: &str) -> std::io::Result<()> {
            let path = format!("{}/{}", User::hash_string(&self.email), dir_name);
            if Path::new(&path).is_dir() {
                fs::remove_dir_all(&path)?;
                Ok(())
            } else {
                return std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                                "Directory not found!"));
            }
        }

        pub fn create_new_file(&self, file_name: &str, content: &str) -> std::io::Result<()> {
            let dir_name = User::hash_string(&self.email);
            if Path::new(&dir_name).is_dir() {
                let path = format!("{}/{}", &dir_name, file_name);
                if !Path::new(&path).exists() {
                    let mut file = File::create(&path)?;
                    file.write_all(content.as_bytes())?;
                    return std::io::Result::Ok(());
                }
                return std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                                "file could not created!"));
            } else {
                return std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                                "This directory is not exist!"));
            }
        }

        pub fn remove_file(&self, file_name: &str) -> std::io::Result<()> {
            let path = format!("{}/{}", User::hash_string(&self.email), file_name);
            let path = Path::new(&path);
            if path.is_file() {
                fs::remove_file(&path)?;
                Ok(())
            } else {
                return std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                                "File not found!"));
            }
        }

        pub fn read_file(&self, file_name: &str) -> std::io::Result<()> {
            let path = format!("{}/{}", User::hash_string(&self.email), file_name);
            let path = Path::new(&path);
            if path.is_file() {
                let file = fs::read_to_string(path)?;
                println!("{}", &file);
                Ok(())
            } else {
                return std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::NotFound,
                                                                "File not found!"));
            }
        }

        pub fn write_file(&self, file_name: &str, content: &str) -> std::io::Result<()> {
            let path = format!("{}/{}", User::hash_string(&self.email), file_name);
            let path = Path::new(&path);
            if path.is_file() {
                /*
                * another way to write file in rust:
                * in this way if file not exist, rust create that file an then write content to it
                * let mut file = File::create(path)?;
                * file.write_all(content.as_bytes())?;
                */
                fs::write(path, content)?;
                Ok(())
            } else {
                return std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::NotFound,
                                                                "File not found!"));
            }
        }

        pub fn read_dir_content(&self, dir_name: &str) -> std::io::Result<()> {
            let path = format!("{}/{}", User::hash_string(&self.email), dir_name);
            let path = Path::new(&path);
            if path.is_dir() {
                let entries = fs::read_dir(path)?;
                for entry in entries{
                    let entry = &entry?;
                    let file_name = entry.file_name().into_string().unwrap();
                    println!("{}", &file_name);
                }
//                println!("{:?}", entries);
                Ok(())
            }else {
                return std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::NotFound,
                                                                "Directory not found!"));
            }
        }

        fn hash_string(s: &str) -> String {
            let hash_bytes = md5::compute(s.as_bytes());
            let hash_string = format!("{:x}", hash_bytes);
            hash_string
        }
    }
}
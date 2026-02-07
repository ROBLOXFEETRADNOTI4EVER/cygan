use regex::Regex;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let cz_file_results = File::open("main.cz");

    let _ = match cz_file_results {
        Ok(mut file) => {
            // print!("{:#?}",file);
            let mut buff = Vec::new();

            let read_cz_file = file.read_to_end(&mut buff);
            match read_cz_file {
                Ok(_) => {
                    let s: String = String::from_utf8(buff).expect("There was an error");
                    // println!("before checking syntax {:#?}\n after: {:#?}, math operation: {:#?}",s,check_syntax(&s),math_operations_version_0(&check_syntax(&s)));
                    println!("{}", math_operations_version_0(&check_syntax(&s)));
                }
                Err(error) => {
                    panic!("Couldn't decode the file something is wrong with it error:{error} ");
                }
            }
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("File was not found can't interpet it ");
            }
            _ => {
                panic!("There was a problem while reading the file");
            }
        },
    };
}

fn check_syntax(file_read: &str) -> String {
    // later we will require main fn
    // need to check if its ascii only no wierd symbols later on handle it to
    file_read.replace(" ", "\n")
}

fn math_operations_version_0(file_read: &str) -> isize {
    for i in 0..file_read.len() - 1 {
        file_read.chars().nth(i).unwrap();
    }

    let re = Regex::new(r"^\s*add\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)\s*;\s*$").unwrap();
    if re.is_match(&file_read.replace(" ", "")) {
        // println!("ok");

        if let Some(caps) = re.captures(&file_read) {
            let num1 = caps.get(1).unwrap().as_str();
            let num2 = caps.get(2).unwrap().as_str();
            // println!("First: {}, Second: {}", num1, num2);
            // println!("First num + secs num is ")
            return num1.parse::<isize>().unwrap() + num2.parse::<isize>().unwrap();
        } else {
            panic!("FAILED TO DO MY JOB");
        }
    } else {
        panic!("FAILED TO DO MY JOB");
    }
}

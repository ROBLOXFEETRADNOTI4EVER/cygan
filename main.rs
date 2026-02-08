use fsize::fsize;
use regex::Regex;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    let cz_file_results = File::open("./main.cz");

    let _ = match cz_file_results {
        Ok(mut file) => {
            // print!("{:#?}",file);
            let mut buff = Vec::new();

            let read_cz_file = file.read_to_end(&mut buff);
            match read_cz_file {
                Ok(_) => {
                    let s: String = String::from_utf8(buff).expect("There was an error");
                    // println!("before checking syntax {:#?}\n after: {:#?}, math operation: {:#?}",s,check_syntax(&s),math_operations_version_0(&check_syntax(&s)));
                    println!("{}", check_syntax(&s));
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
    // Here a logic thing that deletes lines that have // since its commenting out
    // file_read.replace(" ", "\n")
    // file_read.replace(" ", "\n");

    // we will do a thing we count how many valid things are there and then how many instuctions to run and we give it to the logic part

    let mut new_vec: Vec<char> = Vec::new();
    let mut found_shit = false;
    for i in 0..file_read.len() {
        if file_read.chars().nth(i).unwrap() == '/' && file_read.chars().nth(i + 1).unwrap() == '/'
        {
            found_shit = true;
        }
        if file_read.chars().nth(i).unwrap() == '\n' {
            found_shit = false;
            continue;
        }

        if !found_shit {
            let a: char = file_read.chars().nth(i).unwrap();
            new_vec.push(a);
        }

        if file_read.chars().nth(i).unwrap() == ';' && found_shit == false {
            // println!("found");
            let result: String = new_vec.iter().collect();
            // println!("{result}");
            // return result

            println!("{} :{}", result, math_operations_version_0(&result));
            new_vec.clear();
        }
    }
    // println!("vec is {:#?}",new_vec);
    let result: String = new_vec.iter().collect();
    // println!("result is {:#?}",result);

    result
}

fn math_operations_version_0(file_read: &str) -> fsize {
    // need to check for line by line
    for i in 0..file_read.len() - 1 {
        file_read.chars().nth(i).unwrap();
    }

    let re_add: Regex = Regex::new(r"^\s*add\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)\s*;\s*$").unwrap();
    let re_remove: Regex =
        Regex::new(r"^\s*remove\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)\s*;\s*$").unwrap();
    let re_multiply: Regex =
        Regex::new(r"^\s*multiply\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)\s*;\s*$").unwrap();
    let re_devide: Regex =
        Regex::new(r"^\s*devide\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)\s*;\s*$").unwrap();

    if re_add.is_match(&file_read.replace(" ", "")) {
        // println!("ok");

        if let Some(caps) = re_add.captures(&file_read) {
            let num1 = caps.get(1).unwrap().as_str();
            let num2 = caps.get(2).unwrap().as_str();
            // println!("First: {}, Second: {}", num1, num2);
            // println!("First num + secs num is ")
            return num1.parse::<fsize>().unwrap() + num2.parse::<fsize>().unwrap();
        } else {
            panic!("FAILED TO DO MY JOB");
        }
    } else if re_remove.is_match(&file_read.replace(" ", "")) {
        if let Some(caps) = re_remove.captures(&file_read) {
            let num1 = caps.get(1).unwrap().as_str();
            let num2 = caps.get(2).unwrap().as_str();
            // println!("First: {}, Second: {}", num1, num2);
            // println!("First num + secs num is ")
            return num1.parse::<fsize>().unwrap() - num2.parse::<fsize>().unwrap();
        } else {
            panic!("FAILED TO DO MY JOB");
        }
    } else if re_multiply.is_match(&file_read.replace(" ", "")) {
        if let Some(caps) = re_multiply.captures(&file_read) {
            let num1 = caps.get(1).unwrap().as_str();
            let num2 = caps.get(2).unwrap().as_str();
            // println!("First: {}, Second: {}", num1, num2);
            // println!("First num + secs num is ")
            return num1.parse::<fsize>().unwrap() * num2.parse::<fsize>().unwrap();
        } else {
            panic!("FAILED TO DO MY JOB");
        }
    } else if re_devide.is_match(&file_read.replace(" ", "")) {
        if let Some(caps) = re_devide.captures(&file_read) {
            let num1 = caps.get(1).unwrap().as_str();
            let num2 = caps.get(2).unwrap().as_str();
            // println!("First: {}, Second: {}", num1, num2);
            // println!("First num + secs num is ")
            return num1.parse::<fsize>().unwrap() / num2.parse::<fsize>().unwrap();
        } else {
            panic!("FAILED TO DO MY JOB");
        }
    } else {
        panic!("FAILED TO DO MY JOB");
    }
}

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
    let re_variable_name: Regex =
        Regex::new(r#"^let [a-zA-Z_][a-zA-Z0-9_]* = (?:"[^"]*"|-?\d+(?:\.\d+)?);$"#).unwrap();
    // we will do a thing we count how many valid things are there and then how many instuctions to run and we give it to the logic part

    let mut new_vec: Vec<char> = Vec::new();
    let mut found_shit: bool = false;
    let mut is_a_variable: bool = false;
    for i in 0..file_read.len() {
        if file_read.chars().nth(i).unwrap() == '/' && file_read.chars().nth(i + 1).unwrap() == '/'
        {
            found_shit = true;
        } else if file_read.chars().nth(i).unwrap() == 'l'
            && file_read.chars().nth(i + 1).unwrap() == 'e'
            && file_read.chars().nth(i + 2).unwrap() == 't'
            && found_shit == false
        {
            println!(
                "found a let thing checking if its a variable,{}",
                &file_read
            );

            is_a_variable = true;
        }
        if file_read.chars().nth(i).unwrap() == '\n' {
            found_shit = false;
            is_a_variable = false;
            continue;
        }

        if !found_shit {
            let a: char = file_read.chars().nth(i).unwrap();
            new_vec.push(a);
        }

        if file_read.chars().nth(i).unwrap() == ';' && found_shit == false && !is_a_variable {
            // println!("found");
            let result: String = new_vec.iter().collect();
            println!("{result}");
            // return result

            println!("{} :{}", result, math_operations_version_0(&result));
            new_vec.clear();
        }
        let result: String = new_vec.iter().collect();
        // println!("Result before the shit if statment {}",result);
        if file_read.chars().nth(i).unwrap() == ';'
            && found_shit == false
            && is_a_variable
            && re_variable_name.is_match(&result)
        {
            // println!("found");
            let result: String = new_vec.iter().collect();
            println!("{result}");
            // return result
            println!(
                "IS A VARIABLE AND NOT COMMENTED OUT AND ENDS WITH ; {}",
                result
            );
            let re_integer: Regex = Regex::new(r"^let [a-zA-Z_][a-zA-Z0-9_]* = -?\d+;$").unwrap();
            let re_float: Regex =
                Regex::new(r"^let [a-zA-Z_][a-zA-Z0-9_]* = -?\d*\.\d*;$").unwrap();
            let re_string: Regex =
                Regex::new(r#"^let [a-zA-Z_][a-zA-Z0-9_]* = "[^"]*";$"#).unwrap();

            if re_integer.is_match(&result) {
                println!("{result} is an Integer");
            } else if re_float.is_match(&result) {
                println!("{result} is a Float");
            } else if re_string.is_match(&result) {
                println!("{result} is a String");
            }

            // println!("{} :{}", result, math_operations_version_0(&result));
            new_vec.clear();
        }
    }
    let result: String = new_vec.iter().collect();
    println!("vec is {:#?}", new_vec);
    println!("result is {:#?}", result);

    if re_variable_name.is_match(&result) {
        println!("MATCH");
    }
    result
}

// fn variables(file_read: &str) {
//     let re_variable_name: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
//     let collecting_data: Vec<&str> = Vec::new();

//     if re_variable_name.is_match(&file_read.replace(" ", "")) {
//         let collecting_data_for_varible: Vec<&str> = Vec::new();
//     }

//     // what will we feed to it? like let a = 4;
//     // this will need to find the let and after we found a let we know its a varible;
//     // we turn it into a=4; = a[int,4];
// }

fn math_operations_version_0(file_read: &str) -> fsize {
    // need to check for line by line
    for i in 0..file_read.len() - 1 {
        file_read.chars().nth(i).unwrap();
    }

    let re_add: Regex =
        Regex::new(r"^\s*add\(\s*(-?\d+(?:\.\d+)?(?:\s*,\s*-?\d+(?:\.\d+)?)*)\s*\)\s*;\s*$")
            .unwrap();
    let re_remove: Regex =
        Regex::new(r"^\s*remove\(\s*(-?\d+(?:\.\d+)?(?:\s*,\s*-?\d+(?:\.\d+)?)*)\s*\)\s*;\s*$")
            .unwrap();
    let re_multiply: Regex =
        Regex::new(r"^\s*multiply\(\s*(-?\d+(?:\.\d+)?(?:\s*,\s*-?\d+(?:\.\d+)?)*)\s*\)\s*;\s*$")
            .unwrap();
    let re_devide: Regex =
        Regex::new(r"^\s*devide\(\s*(-?\d+(?:\.\d+)?(?:\s*,\s*-?\d+(?:\.\d+)?)*)\s*\)\s*;\s*$")
            .unwrap();
    // let re_complex: Regex =
    // Regex::new(r"^\s*complex\(\s*(-?\d+(?:\.\d+)?(?:\s*,\s*-?\d+(?:\.\d+)?)*)\s*\)\s*;\s*$")
    //     .unwrap();

    if re_add.is_match(&file_read.replace(" ", "")) {
        // println!("ok");

        if let Some(caps) = re_add.captures(&file_read) {
            // let num1 = caps.get(1).unwrap().as_str();
            // let num2 = caps.get(2).unwrap().as_str();
            // // println!("First: {}, Second: {}", num1, num2);
            // // println!("First num + secs num is ")
            // return num1.parse::<fsize>().unwrap() + num2.parse::<fsize>().unwrap();

            let mut total_num: f64 = 0.;
            let num = caps.get(1).unwrap().as_str();
            total_num = num
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .sum();
            return total_num;
        } else {
            panic!("FAILED TO DO MY JOB");
        }
    } else if re_remove.is_match(&file_read.replace(" ", "")) {
        if let Some(caps) = re_remove.captures(&file_read) {
            // let num1 = caps.get(1).unwrap().as_str();
            // let num2 = caps.get(2).unwrap().as_str();
            // // println!("First: {}, Second: {}", num1, num2);
            // // println!("First num + secs num is ")
            // return num1.parse::<fsize>().unwrap() - num2.parse::<fsize>().unwrap();

            let mut total_num: f64 = 0.;
            let mut vv: Vec<f64> = Vec::new();
            let num = caps.get(1).unwrap().as_str();
            vv = num
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();
            total_num = vv[0];
            total_num += vv[0];

            for i in vv {
                total_num -= i;
            }

            return total_num;
        } else {
            panic!("FAILED TO DO MY JOB");
        }
    } else if re_multiply.is_match(&file_read.replace(" ", "")) {
        if let Some(caps) = re_multiply.captures(&file_read) {
            let mut total_num: f64 = 0.;
            let mut vv: Vec<f64> = Vec::new();
            let num = caps.get(1).unwrap().as_str();
            vv = num
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();
            total_num = 1.;
            // total_num += vv[0];

            for i in vv {
                total_num *= i;
            }

            return total_num;
        } else {
            panic!("FAILED TO DO MY JOB");
        }
    } else if re_devide.is_match(&file_read.replace(" ", "")) {
        if let Some(caps) = re_devide.captures(&file_read) {
            let mut total_num: f64 = 0.;
            let mut vv: Vec<f64> = Vec::new();
            let num = caps.get(1).unwrap().as_str();
            vv = num
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();
            total_num = vv[0];
            // total_num += vv[0];
            let mut a = 0;
            // println!("{:#?}",vv);
            for i in vv {
                if i == total_num {
                    continue;
                }
                total_num = total_num / i;
            }

            return total_num;
        } else {
            panic!("FAILED TO DO MY JOB");
        }
    }
    // else if re_complex.is_match(&file_read.replace(" ", "")){
    //     if let Some(caps) = re_complex.captures(&file_read){
    //         let mut total_num: f64 = 0.;
    //         let num = caps.get(1).unwrap().as_str();
    //         total_num = num.split(',').map(|s| s.trim().parse::<f64>().unwrap()).sum();
    //         return total_num;

    //     }else{
    //         panic!("FAILED TO DO MY JOB");

    //     }
    // }
    else {
        panic!("FAILED TO DO MY JOB else");
    }
}

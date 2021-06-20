use std::io;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

pub fn task3() {

    let mut department: HashMap<String, Vec<String>> = HashMap::new();

     'hh: loop {
        let command: Vec<String> = get_command();

        match command[0].to_lowercase().trim() {
            "quit" => break 'hh,
            "add" => add_employer(&command, &mut department),
            "get-in-dep" => get_employer_by_department(&mut department, &command[1]),
            "get-all" => get_employers_in_company(&mut department),
            _ => println!("wrong command"),
        }
    }
}

fn get_command() -> Vec<String> {

    println!("Please write a command (add, sort, quit):");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let split1 = input.split(" ").map(|s| s.to_string());
    let command: Vec<String> = split1.collect();

    return command;

}

// fn quit_program(command: &String) -> bool {
//     if command.to_lowercase() == "quit" {
//         return true;
//     }
//     return false;
// }

fn add_employer(command :&Vec<String>, department: &mut HashMap<String, Vec<String>>) {

    // let name = match get_name(&command) {
    //     Ok(name) => name,
    //     Err(e) => panic!("missing a name"),
    // };

    // let name = match command.get(1) {
    //     Some(name) => name,
    //     None    =>  Err("ffff".to_string())
    // };
    let mut name = String::new();

    match get_name(&command) {
        Ok(str) => name.push_str(&str),
        Err(e) => { println!("error: {}", e); return; }
    };

    let mut depart = String::new();

    match get_depart(&command) {
        Ok(dep) => depart.push_str(&dep),
        Err(e) => { println!("error2: {}", e); return; }
    };

    if department.contains_key(&depart) {
        department.get_mut(&depart).unwrap().push(name.clone());
        return;
    }

    department.insert(depart.clone(), vec![name.clone()]);
}

fn get_name(commands: &Vec<String>) -> Result<String, io::Error> {
    // Some(commands[1].clone()).ok_or(Error::from_raw_os_error(55))
    match commands.get(1) {
        Some(name) => Ok(name.to_string()),
        None    =>  Err(Error::from(ErrorKind::NotFound))
    }
}

fn get_depart(commands: &Vec<String>) -> Result<String, io::Error> {
    // Some(commands[1].clone()).ok_or(Error::from_raw_os_error(55))
    match commands.get(3) {
        Some(depart) => Ok(depart.to_string()),
        None    =>  Err(Error::from(ErrorKind::NotFound))
    }
}

fn get_employer_by_department(department: &mut HashMap<String, Vec<String>>, dep_name: &String) {
    println!("{:?}", department.get(dep_name));
}

fn get_employers_in_company(department: &mut HashMap<String, Vec<String>>) {
    for (name, employers) in department {
        employers.sort();
        println!("name: {} \n employers : {:?}", name, employers);
    }
}

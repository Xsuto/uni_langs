use std::fs;
use std::process::exit;

fn main() {
    let file_name = "courses.txt";
    let file = match fs::read_to_string(file_name) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("{file_name} does not exist");
            exit(1);
        }
    };
    let mut courses = vec![String::new()];
    let mut current_index = 0;
    for (i, line) in file.split("\n").into_iter().enumerate() {
        if i % 6 == 0 && i >= 6 {
            courses.push(String::new());
            current_index += 1;
        }
        if i % 6 != 0 {
            courses[current_index].push(' ');
        }
        courses[current_index].push_str(line);
    }
    println!("Kod kursu Kod grupy nazwa termin Prowadzacy Miejsca zzu stajonarne stopien");
    for it in courses {
        if it.contains("(pt)") && it.contains("(wt)") {
            println!("{it}");
        }
    }
}

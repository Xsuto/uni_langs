use std::fs;
use std::io::stdin;
use std::process::exit;

fn find_course(criteria: &[&str], courses: &[String]) {
    for it in courses {
        let is_valid_course = criteria.iter().all(|x| {
            it.contains(x)
        });
        if is_valid_course {
            println!("{it}");
        }
    }
}

fn format_into_courses_vector(file_as_string: String) -> Vec<String> {
    let mut courses = vec![String::new()];
    let mut current_index = 0;
    let size_of_course = 5;
    for (i, line) in file_as_string.split('\n').into_iter().enumerate() {
        if i % size_of_course == 0 && i >= size_of_course {
            courses.push(String::new());
            current_index += 1;
        }
        if i % size_of_course != 0 {
            courses[current_index].push(' ');
        }
        courses[current_index].push_str(line);
    }
    return courses;
}

fn main() {
    let file_name = "courses.txt";
    let file = match fs::read_to_string(file_name) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("{file_name} does not exist");
            exit(1);
        }
    };
    let courses = format_into_courses_vector(file);
    let mut input = String::new();
    loop {
        input = "".to_owned();
        stdin().read_line(&mut input).expect("Failed to read user input");
        input = input.trim().to_owned();
        println!("Kod kursu Kod grupy nazwa termin Prowadzacy Miejsca zzu stajonarne stopien");
        let mut criteria = vec![];
        for it in input.split(',') {
            criteria.push(it);
        }
        println!("{criteria:?}");
        find_course(&criteria, &courses);
    }
}

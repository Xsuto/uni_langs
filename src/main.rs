use std::fs;
use std::process::exit;

fn find_course(criteria: &[&str], courses: &[String]) {
    for it in courses {
        let is_valid_course = criteria.into_iter().all(|x| {
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
    for (i, line) in file_as_string.split("\n").into_iter().enumerate() {
        if i % 6 == 0 && i >= 6 {
            courses.push(String::new());
            current_index += 1;
        }
        if i % 6 != 0 {
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
    println!("Kod kursu Kod grupy nazwa termin Prowadzacy Miejsca zzu stajonarne stopien");
    find_course(&["(wt)", "(pt)"], &courses);
}

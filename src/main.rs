use std::io::stdin;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

const FILE: &str = include_str!("../swimming.txt");

trait FromMins {
    fn from_mins(mins: u64) -> Duration;
}

impl FromMins for Duration {
    fn from_mins(mins: u64) -> Duration {
        Duration::from_secs(60 * mins)
    }
}

struct IdleProgramChecker {
    start_time: Arc<Mutex<Instant>>,
}

impl IdleProgramChecker {
    fn new() -> Self {
        Self {
            start_time: Arc::new(Mutex::new(Instant::now()))
        }
    }
    fn start_timer(&self, allowed_idle_time_in_mins: u64) {
        let time = self.start_time.clone();
        std::thread::spawn(move || {
            loop {
                if time.lock().unwrap().elapsed() >= Duration::from_mins(allowed_idle_time_in_mins) {
                    exit(0);
                }
                sleep(Duration::from_secs(1));
            }
        });
    }
    fn reset_timer(&mut self) {
        *self.start_time.lock().unwrap() = Instant::now();
    }
}

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

fn format_into_courses_vector(file_as_string: &str) -> Vec<String> {
    let mut courses = vec![String::new()];
    let mut current_index = 0;
    for (i, line) in file_as_string.split('\n').enumerate() {
        if i != 0 && (line.contains("WFW") || line.contains("JZL")) {
            courses.push(String::new());
            current_index += 1;
        } else {
            courses[current_index].push(' ');
        }
        courses[current_index].push_str(line);
    }
    courses
}

fn main() {
    let courses = format_into_courses_vector(FILE);
    let mut input = String::new();
    let mut checker = IdleProgramChecker::new();
    checker.start_timer(10);

    loop {
        stdin().read_line(&mut input).expect("Failed to read user input");
        input = input.trim().to_owned();
        println!("Kod kursu Kod grupy nazwa termin Prowadzacy Miejsca zzu stajonarne stopien");
        let mut criteria = vec![];
        for it in input.split(',') {
            criteria.push(it);
        }
        find_course(&criteria, &courses);
        checker.reset_timer();
        input.clear();
    }
}

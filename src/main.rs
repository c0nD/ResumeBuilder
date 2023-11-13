use printpdf::*;
use std::io::{self, Write};
use std::fs::File;
use std::path::Path;

mod resume_builder;
use resume_builder::ResumeBuilder;

fn main() {
    let name = prompt("Enter your name: ");
    let education = prompt("Enter your education: ");
    let experience = prompt("Enter your experience: ");
    let skills = prompt("Enter your skills: ");
    let filename = prompt("Enter filename for your resume (e.g., resume.pdf): ");

    let resume = ResumeBuilder::new()
        .name(&name)
        .education(&education)
        .experience(&experience)
        .skills(&skills)
        .build();

    resume.save(Path::new(&filename));
}

fn prompt(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

use std::io::{self, Write};
use std::path::Path;

mod resume_builder;
use resume_builder::ResumeBuilder;

fn main() {
    // Basic Info
    let name = prompt("Enter your full name: ");
    let phone = prompt("Enter your phone number: ");
    let email = prompt("Enter your email address: ");
    let location = prompt("Enter your location (city/state): ");

    // Education
    let mut education_entries = Vec::new();
    println!("Enter your education details (type 'Done' when finished):");
    loop {
        let degree = prompt("Degree (e.g., B.S. in Computer Science): ");
        if degree.eq_ignore_ascii_case("done") {
            break;
        }
        let school = prompt("School name: ");
        let date = prompt("Graduation date (month/year): ");
        let gpa = prompt("GPA (optional, leave blank if not applicable): ");
        education_entries.push((degree, school, date, gpa));
    }

    // Skills
    let skills_input = prompt("Enter a comma separated list of your skills: ");
    let skills = skills_input.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();

    // Projects
    let mut projects = Vec::new();
    println!("Enter details for each project you want to include (type 'Done' when finished):");
    loop {
        let project_name = prompt("Project name: ");
        if project_name.eq_ignore_ascii_case("done") {
            break;
        }
        let role = prompt("Your role in the project: ");
        let start_date = prompt("Project start date (month/year): ");
        let end_date = prompt("Project end date (month/year) or 'Present': ");
        let location = prompt("Location: ");
        let mut bullet_points = Vec::new();
        println!("Enter bullet points for this project (type 'Done' for each bullet point to finish):");
        loop {
            let bullet = prompt("Bullet point: ");
            if bullet.eq_ignore_ascii_case("done") {
                break;
            }
            bullet_points.push(bullet);
        }
        projects.push((project_name, role, start_date, end_date, location, bullet_points));
    }

    // Work Experience
    let mut experiences = Vec::new();
    println!("Enter details for each position of your work experience (type 'Done' when finished):");
    loop {
        let job_title = prompt("Job title: ");
        if job_title.eq_ignore_ascii_case("done") {
            break;
        }
        let company_name = prompt("Company name: ");
        let start_date = prompt("Start date (month/year): ");
        let end_date = prompt("End date (month/year) or 'Present': ");
        let job_location = prompt("Location: ");
        let mut job_bullet_points = Vec::new();
        println!("Enter bullet points for this position (type 'Done' for each bullet point to finish):");
        loop {
            let bullet = prompt("Bullet point: ");
            if bullet.eq_ignore_ascii_case("done") {
                break;
            }
            job_bullet_points.push(bullet);
        }
        experiences.push((job_title, company_name, start_date, end_date, job_location, job_bullet_points));
    }

    // Resume Filename
    let filename = prompt("Enter filename for your resume (e.g., 'resume.pdf'): ");

    // Construct the resume
    let resume = ResumeBuilder::new()
        .name(&name)
        .phone(&phone)
        .email(&email)
        .location(&location)
        .education_entries(&education_entries)
        .skills(&skills)
        .projects(&projects)
        .work_experience(&experiences)
        .build();

    // Save the resume to a file
    resume.save(Path::new(&filename));
    println!("Resume saved to {}", filename);
}

fn prompt(message: &str) -> String {
    let mut input = String::new();
    print!("{} ", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

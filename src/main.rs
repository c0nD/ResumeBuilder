use printpdf::*;
use std::io::{self, Write, BufWriter};
use std::fs::File;

fn prompt(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn create_resume(name: &str, education: &str, experience: &str, skills: &str, filename: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Resume", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let mut y_position = 270.0;

    // Name
    current_layer.use_text(name.to_string(), 20.0, Mm(10.0), Mm(y_position), &font);
    y_position -= 30.0;

    // Education
    current_layer.use_text(format!("Education: {}", education), 14.0, Mm(10.0), Mm(y_position), &font);
    y_position -= 20.0;

    // Experience
    current_layer.use_text(format!("Experience: {}", experience), 14.0, Mm(10.0), Mm(y_position), &font);
    y_position -= 20.0;

    // Skills
    current_layer.use_text(format!("Skills: {}", skills), 14.0, Mm(10.0), Mm(y_position), &font);

    doc.save(&mut BufWriter::new(File::create(filename).unwrap())).unwrap();
}

fn main() {
    let name = prompt("Enter your name: ");
    let education = prompt("Enter your education: ");
    let experience = prompt("Enter your experience: ");
    let skills = prompt("Enter your skills: ");
    let filename = prompt("Enter filename for your resume (e.g., resume.pdf): ");

    create_resume(&name, &education, &experience, &skills, &filename);
}

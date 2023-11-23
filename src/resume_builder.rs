use printpdf::*;
use std::{fs::File, io::BufWriter, path::Path};

pub struct ResumeBuilder {
    name: String,
    phone: String,
    email: String,
    location: String,
    education_entries: Vec<(String, String, String, String)>,
    skills: Vec<String>,
    projects: Vec<(String, String, String, String, String, Vec<String>)>,
    work_experience: Vec<(String, String, String, String, String, Vec<String>)>,
}

impl ResumeBuilder {
    pub fn new() -> Self {
        ResumeBuilder {
            name: String::new(),
            phone: String::new(),
            email: String::new(),
            location: String::new(),
            education_entries: Vec::new(),
            skills: Vec::new(),
            projects: Vec::new(),
            work_experience: Vec::new(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn phone(mut self, phone: &str) -> Self {
        self.phone = phone.to_string();
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = email.to_string();
        self
    }

    pub fn location(mut self, location: &str) -> Self {
        self.location = location.to_string();
        self
    }

    pub fn education_entries(mut self, entries: &[(String, String, String, String)]) -> Self {
        self.education_entries = entries.to_vec();
        self
    }

    pub fn skills(mut self, skills: &[String]) -> Self {
        self.skills = skills.to_vec();
        self
    }

    pub fn projects(mut self, projects: &[(String, String, String, String, String, Vec<String>)]) -> Self {
        self.projects = projects.to_vec();
        self
    }

    pub fn work_experience(mut self, experience: &[(String, String, String, String, String, Vec<String>)]) -> Self {
        self.work_experience = experience.to_vec();
        self
    }

    pub fn build(self) -> Resume {
        Resume {
            name: self.name,
            phone: self.phone,
            email: self.email,
            location: self.location,
            education_entries: self.education_entries,
            skills: self.skills,
            projects: self.projects,
            work_experience: self.work_experience,
        }
    }
}

pub struct Resume {
    name: String,
    phone: String,
    email: String,
    location: String,
    education_entries: Vec<(String, String, String, String)>,
    skills: Vec<String>,
    projects: Vec<(String, String, String, String, String, Vec<String>)>,
    work_experience: Vec<(String, String, String, String, String, Vec<String>)>,
}

impl Resume {
    pub fn save(&self, path: &Path) {
        let (doc, page1, layer1) = PdfDocument::new("Resume", Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Define initial y_position (top of the page)
        let mut y_position = 297.0 - 20.0; // Page height - top margin
        let left_margin = 20.0;
        let line_height = 6.0;
        let font_size_header = 18.0;
        let font_size_section_title = 16.0;
        let font_size_text = 12.0;

        // Helper function to add text to the layer
        let mut add_text = |text: &str, size: f64, y_pos: &mut f64, is_bold: bool, new_section: bool| {
            if new_section {
                *y_pos -= line_height * 2.0; // Add extra space before new section
            }
            let font = if is_bold {
                doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap()
            } else {
                doc.add_builtin_font(BuiltinFont::Helvetica).unwrap()
            };
            let text_len = (text.len() as f64 * (size / 2.0)).round(); // Rough estimate of text length
            if text_len + left_margin > 210.0 - left_margin { // Check if text exceeds the margin
                *y_pos -= line_height; // Move to new line if it does
            }
            current_layer.use_text(text.to_string(), size as f32, Mm(left_margin as f32), Mm(*y_pos as f32), &font);
            *y_pos -= line_height;
        };

        // Name and Contact Information
        add_text(&self.name, font_size_header, &mut y_position, true, false);
        add_text(&format!("{} | {}", self.phone, self.email), font_size_text, &mut y_position, false, false);
        add_text(&self.location, font_size_text, &mut y_position, false, false);

        // Add a section header for Education
        add_text("EDUCATION", font_size_section_title, &mut y_position, true, true);

        // Iterate over education entries and add them to the PDF
        for (degree, school, date, gpa) in &self.education_entries {
            add_text(&format!("{} - {}", degree, school), font_size_text, &mut y_position, false, false);
            add_text(&format!("{} | GPA: {}", date, gpa), font_size_text, &mut y_position, false, false);
        }

        // Add a section header for Skills
        add_text("SKILLS", font_size_section_title, &mut y_position, true, true);
        add_text(&self.skills.join(", "), font_size_text, &mut y_position, false, false);

        // Add a section header for Projects
        add_text("PROJECTS", font_size_section_title, &mut y_position, true, true);
        for (project_name, role, start_date, end_date, description, bullet_points) in &self.projects {
            add_text(&format!("{} | {}", project_name, role), font_size_text, &mut y_position, true, false);
            add_text(&format!("{} - {}", start_date, end_date), font_size_text, &mut y_position, false, false);
            add_text(description, font_size_text, &mut y_position, false, false);
            for point in bullet_points {
                add_text(&format!("• {}", point), font_size_text, &mut y_position, false, false);
            }
        }

        // Add a section header for Work Experience
        add_text("WORK EXPERIENCE", font_size_section_title, &mut y_position, true, true);
        for (job_title, company_name, start_date, end_date, job_location, job_bullet_points) in &self.work_experience {
            add_text(company_name, font_size_text, &mut y_position, true, false);
            add_text(&format!("{} | {} - {}", job_title, start_date, end_date), font_size_text, &mut y_position, false, false);
            for point in job_bullet_points {
                add_text(&format!("• {}", point), font_size_text, &mut y_position, false, false);
            }
        }

        // Save the PDF
        doc.save(&mut BufWriter::new(File::create(path).unwrap())).unwrap();
        println!("Resume saved as {:?}", path);
    }
}
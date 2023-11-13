use printpdf::*;
use std::path::Path;
use std::io::BufWriter;
use std::fs::File;

pub struct ResumeBuilder {
    name: String,
    education: String,
    experience: String,
    skills: String,
}

impl ResumeBuilder {
    pub fn new() -> Self {
        ResumeBuilder {
            name: String::new(),
            education: String::new(),
            experience: String::new(),
            skills: String::new(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn education(mut self, education: &str) -> Self {
        self.education = education.to_string();
        self
    }

    pub fn experience(mut self, experience: &str) -> Self {
        self.experience = experience.to_string();
        self
    }

    pub fn skills(mut self, skills: &str) -> Self {
        self.skills = skills.to_string();
        self
    }

    pub fn build(self) -> Resume {
        Resume {
            name: self.name,
            education: self.education,
            experience: self.experience,
            skills: self.skills,
        }
    }
}

pub struct Resume {
    name: String,
    education: String,
    experience: String,
    skills: String,
}

impl Resume {
    pub fn save(self, path: &Path) {
        let (doc, page1, layer1) = PdfDocument::new("Resume", Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);
        let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

        let mut y_position = 270.0;

        // Name
        current_layer.use_text(self.name, 20.0, Mm(10.0), Mm(y_position), &font);
        y_position -= 30.0;

        // Education
        current_layer.use_text(format!("Education: {}", self.education), 14.0, Mm(10.0), Mm(y_position), &font);
        y_position -= 20.0;

        // Experience
        current_layer.use_text(format!("Experience: {}", self.experience), 14.0, Mm(10.0), Mm(y_position), &font);
        y_position -= 20.0;

        // Skills
        current_layer.use_text(format!("Skills: {}", self.skills), 14.0, Mm(10.0), Mm(y_position), &font);

        doc.save(&mut BufWriter::new(File::create(path).unwrap())).unwrap();
    }
}

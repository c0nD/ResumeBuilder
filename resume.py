from pylatex import Document, Section, NewPage, Command
from pylatex.utils import bold

class Resume:
    def __init__(self, name, email, skills, experience):
        self.name = name
        self.email = email
        self.skills = skills
        self.experience = experience

    def generate_pdf(self):
        doc = Document()

        with doc.create(Section('Resume', numbering=False)):
            doc.append(bold("Name: "))
            doc.append(self.name)
            doc.append(NewPage())

            doc.append(bold("Email: "))
            doc.append(self.email)
            doc.append(NewPage())

            doc.append(bold("Skills: "))
            doc.append(", ".join(self.skills))
            doc.append(NewPage())

            doc.append(bold("Experience: "))
            doc.append(self.experience)
            doc.append(NewPage())

        doc.generate_pdf("resume", clean_tex=False)

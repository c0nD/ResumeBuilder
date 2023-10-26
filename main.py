from resume import Resume
from user_interface import UserInterface

def main():
    ui = UserInterface()
    resume_data = ui.collect_resume_data()
    
    resume = Resume(**resume_data)
    resume.generate_pdf()

if __name__ == "__main__":
    main()

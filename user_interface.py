class UserInterface:
    def collect_resume_data(self):
        name = input("Enter your name: ")
        email = input("Enter your email: ")
        skills = input("Enter your skills (comma-separated): ").split(',')
        experience = input("Enter your experience: ")
        
        return {
            'name': name,
            'email': email,
            'skills': skills,
            'experience': experience
        }
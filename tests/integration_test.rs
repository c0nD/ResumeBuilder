use std::io::Cursor;

fn main() {

}

// Mock input for the test case
let test_input = "\
Colin Tiller
919-924-7895
colintiller21@gmail.com
Boone, NC
B.S. Computer Science
Appalachian State University
May, 2024
3.17
Certificate of Data Science
Appalachian State University
May, 2024

Done
Python, Rust, C++, Java, Management, Git, Docker
FluffyBOT
Lead Open-Source Developer
October 2022
Present
Online
Develop and manage an open-source, documented Discord bot...
Integrate a data tracking system...
Collaborate with the communities of 500+ users...
Maintain integration of new features...
Done
CreampuffBOT
Lead Open-Source Developer
June 2023
Present
Online
Engineer an open-source OCR-based tool...
Design a user-friendly GUI...
Reduce data collection and analysis time...
Done
Done
Manager
Aquatech Pool Management
May 2022
August 2023
Apex, NC
Oversaw the safety and well-being of patrons...
Efficiently managed, scheduled, and led a team...
Served as the primary point of contact...
Done
Done
resume.pdf
";

// Create a cursor to simulate user input
let input_cursor = Cursor::new(test_input);

main(input_cursor);

use std::io;
use project::project::create_project;
use project::project::Project;
fn main() {
    // let mut guess = String::new();

    // println!("Guess: ");
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    // println!("{}",guess)
    let test_project: Project = create_project();
}

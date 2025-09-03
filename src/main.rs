use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Stdin;
use std::process;
use csv::Reader;
use project::project::create_project;
use project::project::Project;
use csv;

fn new_file(file_path: Option<&str>)-> Result<File, Box<dyn Error>>{
    //create a File using file_path
    let file = File::create(file_path.unwrap())?;
   let mut wtr = csv::Writer::from_path(file_path.unwrap())?;
    // Since we're writing records manually, we must explicitly write our
    // header record. A header record is written the same way that other
    // records are written.
    wtr.write_record(&["Project","Size","Cost","Whole Army/Warband",
    "Assembly Required","Kitbash rating","Painting level","Complexity rating",
    "Preference modifier","Priority","Status","Is Owned"])?;

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    wtr.flush()?;
    Ok(file)
}
fn read_file()-> Result<(), Box<dyn Error>>{
    let filepath = get_first_arg()?;
    //if file does not exist, make a new one!
    let file = File::open(&filepath).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            new_file(filepath.to_str()).unwrap()
            
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    let mut rdr = csv::Reader::from_reader(file);

    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        let record =result?;
        // Print a debug version of the record.
        println!("{:?}", record);
    }
    Ok(())
}
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
fn main() {
    //TODO: Look at reading CSV and error checking properly again
    let test_project: Project = create_project();
      if let Err(err) = read_file() {
        println!("{}", err);
        process::exit(1);
    }
    


}

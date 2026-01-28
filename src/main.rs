use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::ErrorKind;
use std::process;
use std::vec::Vec;
use project::project::Project;
use csv;
use tabled::{Table};
use tabled::settings::{Style};

fn new_file(file_path: Option<&str>, headers: [&str; 12])-> Result<File, Box<dyn Error>>{
    //create a File using file_path
    let file = File::create(file_path.unwrap())?;
   let mut wtr = csv::Writer::from_path(file_path.unwrap())?;
    // Add header to file
    wtr.write_record(headers)?;

    wtr.flush()?;
    Ok(file)
}

fn read_file()-> Result<(Vec<Project>), Box<dyn Error>>{
    let filepath= OsString::from("project_priorities.csv");

    let headers = ["Project","Size","Cost","Whole Army/Warband",
    "Assembly Required","Kitbash Rating","Painting Level","Complexity Rating",
    "Preference Modifier","Priority","Status","Is Owned"];

    //if file does not exist, make a new one!
    let file = File::open(&filepath).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            new_file(filepath.to_str(),headers).unwrap()
            
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    let mut rdr = csv::Reader::from_reader(file);
    let mut project_list: Vec<Project> = Vec::new();

    for result in rdr.deserialize(){
        let record: Project = result?;
        // Print a debug version of the record.
        //TEMPORARILY COMMENTED
        // println!("{:?}", &record);
        //add each record to a vector, so it can be returned.
        project_list.push(record);
    }
    Ok(project_list)
}

fn add_project() -> Result<(), Box<dyn Error>> { //IMPLEMENT: add record to project_priorities.csv
    Ok(())
}

fn generate_table (data: Vec<Project>) -> Table {
    let styling = Style::modern();
    let mut table = Table::new(data);
    table.with(styling);
    table
}

fn render_table(data: Vec<Project>) {
    //iterate through each element in data, adding escape characters
    let table = generate_table(data);

    println!("{}", table.to_string());
    //make it so table can be scrolled through vertically and horizontally, instead of wrapping
    //once that is done, change certain row backgrounds based on their status
    //then add a function to display basic metrics.
    //create add_entry button / ctrl+A type commands?
}

fn get_first_arg() -> Result<(), Box<dyn Error>>  {
    //maybe change to accept different arguments, display to show project priorities table,
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(argument )=> {if argument == OsString::from("display") {
            //run function for displaying csv
            let data = read_file()?;
            render_table(data);
            Ok(())
        }
        //UNIMPLEMENTED
        else if argument == OsString::from("add"){
            add_project()
        }
        else{
            Err(From::from("Invalid argument"))
        }},
    }
}

fn main() {
    // let test_project: Project = create_from_existing();
      if let Err(err) = get_first_arg() {
        println!("{}", err);
        process::exit(1);
    }
}

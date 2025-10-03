use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Stdin;
use std::process;
use std::vec::Vec;
use csv::Reader;
use project::project::create_from_existing;
use project::project::Project;
use csv;

fn new_file(file_path: Option<&str>, headers: &[&str])-> Result<File, Box<dyn Error>>{
    //create a File using file_path
    let file = File::create(file_path.unwrap())?;
   let mut wtr = csv::Writer::from_path(file_path.unwrap())?;
    // Add header to file
    wtr.write_record(headers)?;

    wtr.flush()?;
    Ok(file)
}

fn read_file(headers: &[&str])-> Result<(Vec<Project>), Box<dyn Error>>{
    let filepath= OsString::from("project_priorities.csv");
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
        println!("{:?}", &record);
        //add each record to a vector, so it can be returned.
        project_list.push(record);
    }
    Ok(project_list)
}

fn add_project() -> Result<(), Box<dyn Error>> { //IMPLEMENT: add record to project_priorities.csv
    Ok(())
}

fn render_table(data: Vec<Project>, headers: &[&str]) { //TODO: now that vector is returned, figure out how to actually display it as a matrix/grid.
    //flatten headers into a single string with tabs to space out each element.
    let flat_headers: String = headers.iter()
                            .map(|s| s.chars())
                            .flatten()
                            .collect();
    println!("{}", flat_headers)

    //iterate through each element in data, adding escape characters

    //after this version is done, style it with ASCII characters,
    //once that is done, change certain row backgrounds based on their status
    //then add a function to display basic metrics.
}

fn get_first_arg(headers: &[&str]) -> Result<(), Box<dyn Error>>  {
    //maybe change to accept different arguments, display to show project priorities table,
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(argument )=> {if argument == OsString::from("display") {
            //run function for displaying csv
            let data = read_file(headers)?;
            render_table(data, headers);
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
    let headers = ["Project","Size","Cost","Whole Army/Warband",
    "Assembly Required","Kitbash rating","Painting level","Complexity rating",
    "Preference modifier","Priority","Status","Is Owned"];
    // let test_project: Project = create_from_existing();
      if let Err(err) = get_first_arg(&headers) {
        println!("{}", err);
        process::exit(1);
    }
}

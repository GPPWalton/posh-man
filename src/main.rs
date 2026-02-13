use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::ErrorKind;
use std::process;
use std::vec::Vec;
use project::project::Project;
use csv;
mod app;
use crate::app::App;
fn new_file(file_path: Option<&str>, headers: [&str; 11])-> Result<File, Box<dyn Error>>{
    //create a File using file_path
    let file = File::create(file_path.unwrap())?;
   let mut wtr = csv::Writer::from_path(file_path.unwrap())?;
    // Add header to file
    wtr.write_record(headers)?;

    wtr.flush()?;
    Ok(file)
}

fn read_file(headers: [&str; 11])-> Result<Vec<Project>, Box<dyn Error>>{
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
        //add each record to a vector, so it can be returned.
        project_list.push(record);
    }
    Ok(project_list)
}

fn add_project() -> Result<(), Box<dyn Error>> {
    //TODO: add record to project_priorities.csv
    todo!();
}

fn get_first_arg(headers: [&str; 11]) -> Result<(), Box<dyn Error>> {
    //maybe change to accept different arguments, display to show project priorities table,
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(argument )=> {if argument == OsString::from("display") {
            //run function for displaying csv
            let data = read_file(headers)?;
            Ok(ratatui::run(|terminal| App::new(data).run(terminal,headers))?)
        }
        else if argument == OsString::from("add"){
            add_project()
        }
        else{
            Err(From::from("Invalid argument"))
        }},
    }
}

fn main(){
    let headers = ["Project","Size","Cost","Whole Army / Warband",
    "Assembly Required","Kitbash Rating","Painting Level","Complexity Rating",
    "Priority","Status","Is Owned"];
    if let Err(err) = get_first_arg(headers) {
        println!("{}", err);
        process::exit(1);
    }    
}
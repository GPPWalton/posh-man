use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Stdin;
use std::process;
use csv::Reader;
use project::project::create_from_existing;
use project::project::Project;
use csv;

fn new_file(file_path: Option<&str>)-> Result<File, Box<dyn Error>>{
    //create a File using file_path
    let file = File::create(file_path.unwrap())?;
   let mut wtr = csv::Writer::from_path(file_path.unwrap())?;
    // Add header to file
    // old header-> keep for later 'prettying-up'
    // wtr.write_record(&["Project","Size","Cost","Whole Army/Warband",
    // "Assembly Required","Kitbash rating","Painting level","Complexity rating",
    // "Preference modifier","Priority","Status","Is Owned"])?;
    wtr.write_record(&["project_name", "size","cost", "whole_army",
                        "needs_assembly", "kitbash_rating", "paint_level",
                        "complexity_rating","preference_modifier","priority", "status",])?;

    wtr.flush()?;
    Ok(file)
}

fn read_file()-> Result<(), Box<dyn Error>>{
    let filepath= OsString::from("project_priorities.csv");
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
    // for result in rdr.records() {
    //     // An error may occur, so abort the program in an unfriendly way.
    //     let record =result?;
    //     // Print a debug version of the record.
    //     println!("{:?}", record);
    // }

    //TODO:this should work to parse the records, but you'll need to rename/change headers or struct fields!
    for result in rdr.deserialize(){
        let record: Project = result?;
        // Print a debug version of the record.
        println!("{:?}", record);
    }
    Ok(())
}

fn add_project() -> Result<(), Box<dyn Error>> { //IMPLEMENT: add record to project_priorities.csv
    Ok(())
}


//-> Result<OsString, Box<dyn Error>>
fn get_first_arg() -> Result<(), Box<dyn Error>>  {
    //maybe change to accept different arguments, display to show project priorities table,
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(argument )=> {if argument == OsString::from("display") {
            //run function for displaying csv
            read_file()
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

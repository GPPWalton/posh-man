use crate::app::App;
use project::project::{Project,PaintLevel,Cost};

//Contains helper/convenience functions for testing
pub fn generate_test_table() ->(App,usize){
    let mut test_projects = vec![];

    for i in 0..29 {
        test_projects.push(Project::new(String::from("Dangle No. ".to_owned() + &i.to_string() ), 1,Cost::None,true,false,4,PaintLevel::Character,1.0f64,1.0f64,false,true));
    }
    let test_len = &test_projects.len()-1;
    let test_app = App::new(test_projects);
    return  (test_app,test_len);
}
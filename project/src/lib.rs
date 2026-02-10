pub mod project{
    use tabled::{Tabled};
    use std::{fmt, vec};
    #[derive(Debug, serde::Serialize, serde::Deserialize,Copy,Clone)]
    pub enum Cost{
        None,
        Low,
        Medium,
        High,        
    }
    impl Cost {
         pub fn to_string(&self)->String{
            match self {
                Self::None => return String::from("None"),
                Self::Low => return String::from("Low"),
                Self::Medium => return String::from("Medium"),
                Self::High => return String::from("High"),
            }
        }
    }
    impl fmt::Display for Cost {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }
    #[derive(Debug, serde::Serialize, serde::Deserialize,Copy,Clone)]
    pub enum PaintLevel{
        Simple,
        Battle,
        Character,
    }
    //TODO: perhaps condense this and the one for Cost into a single function?
    impl PaintLevel {
        pub fn to_string(&self) -> String {
            match self {
                Self::Simple => return String::from("Simple"),
                Self::Battle => return  String::from("Battle"),
                Self::Character => return  String::from("Character")
            }
        }
    }

    //Resolve this duplication?
    impl fmt::Display for PaintLevel {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize, Tabled)]
    #[tabled(rename_all = "Upper Title Case")]
    pub struct Project{
        project_name: String,
        size: u8,
        cost: Cost,
        whole_army: bool,
        needs_assembly: bool,
        kitbash_rating: u8,
        paint_level: PaintLevel,
        priority: f64,
        status: bool,
    }
    impl Project {

        pub fn new (project_name: String,size: u8,cost: Cost,
            whole_army: bool,needs_assembly: bool,kitbash_rating: u8,
            paint_level: PaintLevel,priority: f64, status: bool) -> Project {
            Project
            {
                project_name: project_name,
                size: size,
                cost: cost,
                whole_army: whole_army,
                needs_assembly: needs_assembly,
                kitbash_rating: kitbash_rating,
                paint_level: paint_level,
                priority: priority,
                status: status
            }
        }
        //get methods - 
        pub fn project_name (&self) -> &str{&self.project_name} 
        pub fn size (&self)-> u8 {self.size}
        pub fn cost (&self)-> Cost {self.cost}
        pub fn whole_army (&self)-> bool {self.whole_army}
        pub fn needs_assembly (&self)-> bool {self.needs_assembly}
        pub fn kitbash_rating (&self)-> u8 {self.kitbash_rating}
        pub fn paint_level (&self)-> PaintLevel {self.paint_level}
        pub fn priority (&self)-> f64 {self.priority}
        pub fn status (&self)-> bool {self.status}

        //return object as string array
        pub fn as_str_array(&self) -> [String; 9] {
        [self.project_name().to_string(), self.size.to_string() ,
        self.cost().to_string(), self.whole_army().to_string(),
        self.needs_assembly().to_string(),
        self.kitbash_rating().to_string(),self.paint_level().to_string(),
        self.priority().to_string(),self.status.to_string()]
    }
    }
    //To do calculate priority using fibonacci storypointing
    //fn calc_priority, should this be calculated after all projects are generated??
}
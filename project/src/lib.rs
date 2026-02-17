pub mod project{
    use std::{error::Error, fmt, str::FromStr, u8};

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
    impl FromStr for Cost {
        type Err = Box<dyn std::error::Error>;
        fn from_str(s: &str) -> Result<Cost, Self::Err> {
            //convert to lowercase to remove typing variation
            match s.trim().to_lowercase().as_str() {
                "none"=> Ok(Cost::None),
                "low" => Ok(Cost::Low),
                "medium"=>Ok(Cost::Medium),
                "high" => Ok(Cost::High),
                _ => Err(From::from("Invalid value for cost"))
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

    impl FromStr for PaintLevel {
        type Err = Box<dyn std::error::Error>;
        fn from_str(s: &str) -> Result<PaintLevel, Self::Err> {
            //convert to lowercase to remove typing variation
            match s.trim().to_lowercase().as_str() {
                "simple"=> Ok(PaintLevel::Simple),
                "battle" => Ok(PaintLevel::Battle),
                "character"=>Ok(PaintLevel::Character),
                _ => Err(From::from("Invalid value for paint level"))
            }
        }
    }

    //Resolve this duplication?
    impl fmt::Display for PaintLevel {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Project{
        project_name: String,
        size: u8,
        cost: Cost,
        whole_army: bool,
        needs_assembly: bool,
        kitbash_rating: u8,
        paint_level: PaintLevel,
        complexity_rating: f64,
        priority: f64,
        status: bool,
        is_owned: bool,
    }
    impl Project {

        pub fn new (project_name: String,size: u8,cost: Cost,
            whole_army: bool,needs_assembly: bool,kitbash_rating: u8,
            paint_level: PaintLevel,complexity_rating: f64, 
            priority: f64, status: bool,
            is_owned: bool) -> Project {
            Project
            {
                project_name: project_name,
                size: size,
                cost: cost,
                whole_army: whole_army,
                needs_assembly: needs_assembly,
                kitbash_rating: kitbash_rating,
                paint_level: paint_level,
                complexity_rating: complexity_rating,
                priority: priority,
                status: status,
                is_owned: is_owned,
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
        pub fn complexity_rating (&self)-> f64 {self.complexity_rating}
        pub fn priority (&self)-> f64 {self.priority}
        pub fn status (&self)-> bool {self.status}
        pub fn is_owned (&self)-> bool {self.is_owned}

        //set methods
        pub fn set_project_name (&mut self, input: String){self.project_name = input;}
        pub fn set_size(&mut self, input: u8) {self.size = input;}
        pub fn set_cost(&mut self, input: Cost) {self.cost = input;}
        pub fn set_whole_army(&mut self, input: bool) {self.whole_army = input;}
        pub fn set_needs_assembly(&mut self, input: bool) {self.needs_assembly = input;}
        pub fn set_kitbash_rating(&mut self, input: u8) {self.kitbash_rating = input;}
        pub fn set_paint_level(&mut self, input: PaintLevel) {self.paint_level = input;}
        pub fn set_complexity_rating(&mut self, input: f64) {self.complexity_rating = input;}
        pub fn set_priority(&mut self, input: f64) {self.priority = input;}
        pub fn set_status(&mut self, input: bool) {self.status = input;}
        pub fn set_is_owned(&mut self, input: bool) {self.is_owned = input;}

        //return object as string array
        pub fn as_str_array(&self) -> [String; 11] {
            [self.project_name().to_string(), self.size.to_string() ,
            self.cost().to_string(), self.whole_army().to_string(),
            self.needs_assembly().to_string(),
            self.kitbash_rating().to_string(),self.paint_level().to_string(),
            self.complexity_rating().to_string(),
            self.priority().to_string(),self.status.to_string(),self.is_owned().to_string()]
        }
        pub fn from_arr(separated_items: [&str;11]) -> Result<Project, Box<dyn Error>> {
            Ok(Project { project_name: separated_items[0].to_string(),
                //TODO: handle trimming whitespace and value size-constraints at input for simplicity
                size: separated_items[1].parse::<u8>()?,
                cost: Cost::from_str(separated_items[2])?,
                //TODO: handle lowercasing at input?
                whole_army: separated_items[3].to_lowercase().parse::<bool>()?,
                needs_assembly: separated_items[4].to_lowercase().parse::<bool>()?,
                //TODO: not here, but should normalise kitbash rating to specific range 1-5?
                kitbash_rating: separated_items[5].parse::<u8>()?,
                paint_level: PaintLevel::from_str(separated_items[6])?,
                complexity_rating: separated_items[7].parse::<f64>()?,
                priority: separated_items[8].parse::<f64>()?,
                status: separated_items[9].parse::<bool>()?,
                is_owned: separated_items[10].parse::<bool>()?
            })
        }
    }
        
    }
    //TODO: calculate priority using fibonacci storypointing
    //fn calc_priority, should this be calculated after all projects are generated??

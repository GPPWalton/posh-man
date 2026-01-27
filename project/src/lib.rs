pub mod project{
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub enum Cost{
        None,
        Low,
        Medium,
        High,        
    }
    impl Cost {
         pub fn as_str(&self)-> &str{
            match self {
                Self::None => return "None",
                Self::Low => return "Low",
                Self::Medium => return "Medium",
                Self::High => return "High",
            }
        }
    }
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub enum PaintLevel{
        Simple,
        Battle,
        Character,
    }
    //TODO: perhaps condense this and the one for Cost into a single function?
    impl PaintLevel {
        pub fn as_str(&self) -> &str {
            match self {
                Self::Simple => return "Simple",
                Self::Battle => return  "Battle",
                Self::Character => return  "Character"
            }
        }
    }
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Project{
        pub project_name: String,
        pub size: u8,
        pub cost: Cost,
        pub whole_army: bool,
        pub needs_assembly: bool,
        pub kitbash_rating: u8,
        pub paint_level: PaintLevel,
        pub priority: f64,
        pub status: bool,

    }
    pub fn create_from_existing(project_name: String, size: u8,
        cost: Cost, whole_army: bool, needs_assembly: bool,
        kitbash_rating: u8, paint_level: PaintLevel, priority: f64,
        status: bool) -> Project{   //create project struct from an existing record
        let test = Project{
            project_name,
            size,
            cost,
            whole_army,
            needs_assembly,
            kitbash_rating,
            paint_level,
            priority, //this should probably call calc_priority?
            status,
        };
    return  test;
    }
    //To do calculate priority using fibonacci storypointing
    //fn calc_priority, should this be calculated after all projects are generated??
}
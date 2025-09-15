pub mod project{
   pub enum Cost{
        None,
        Low,
        Medium,
        High,
    }
    pub enum PaintLevel{
        Simple,
        Battle,
        Character,
    }
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
            status: status,
        };
    return  test;
    }
    //To do calculate priority using fibonacci storypointing
    //fn calc_priority, should this be calculated after all projects are generated??
}
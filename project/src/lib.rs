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
    pub fn create_from_existing(project_name: &str, size: u8,
        cost: Cost, whole_army: bool, needs_assembly: bool,
        kitibash_rating: u8, paint_level: PaintLevel, priority: f64,
        status: bool) -> Project{
        let test = Project{
            project_name: project_name.to_string(),
            size: size,
            cost: cost,
            whole_army: whole_army,
            needs_assembly: needs_assembly,
            kitbash_rating: kitibash_rating,
            paint_level: paint_level,
            priority: priority,
            status: status,
        };
    return  test;
    }
}
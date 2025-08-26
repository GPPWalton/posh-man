pub mod project{
    enum Cost{
        None,
        Low,
        Medium,
        High,
    }
    enum PaintLevel{
        Simple,
        Battle,
        Character,
    }
    struct Project{
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
    pub fn create_project() -> Project{
        let test = Project{
        project_name: "Dark Angels Centurion".to_string(),
        size: 1,
        cost: Cost::None,
        whole_army: false,
        needs_assembly: false,
        kitbash_rating: 1,
        paint_level: PaintLevel::Character,
        priority: 1.0,
        status: true,
    };
    return  test;
    }
}
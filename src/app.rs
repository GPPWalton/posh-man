
use std::{io};
use project::project::{Project};
use crossterm::event::{self, Event as crossEvent, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, layout::{Constraint, Layout}
    ,widgets::{ScrollbarState, TableState}
};
use unicode_width::UnicodeWidthStr;
use crate::ui::{TableColors,PALETTES,main_ui::render_main_ui};

const ITEM_HEIGHT: usize = 4;

fn max_width<F, T>(items: &[Project], field_fn: F) -> u16 where F: Fn(&Project) -> T, T: ToString,{
    items
        .iter()
        .map(|x| field_fn(x).to_string())
        .map(|x| UnicodeWidthStr::width(x.as_str()))
        .max()
        //TODO: Eliminate this unwrap_or?
        .unwrap_or(0) as u16
}
pub struct App <'a>{
    table_state: TableState,
    data: Vec<Project>,
    exit: bool,
    longest_item_lens: [u16;11],
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
    current_screen: CurrentScreen,
    currently_editing: Option<CurrentlyEditing>,
    input_array: [&'a str;11],
}



pub enum CurrentScreen {
    Main,       //main table
    Editing,    //when editing entry
    Adding,     //when adding entry
    Exiting,    //confirmation of edit or add
}
pub enum CurrentlyEditing {
    Project,
    Size,
    Cost,
    WholeArmy,
    AssemblyRequired,
    KitbashRating,
    PaintingLevel,
    ComplexityRating,
    Priority
    ,Status,
    IsOwned
}

impl<'a> App<'a> {
    //constructor
    pub fn new(data: Vec<Project>) -> App<'a > {
        App {
            table_state: TableState::default(),
            longest_item_lens: Self::get_constraints(&data),
            data: data,
            exit: false,
            scroll_state: ScrollbarState::default(),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            input_array: ["","","","","","","","","","",""],
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal, headers: [&str;11]) -> io::Result<()> {
        while !self.exit {
            //main loop goes here
            terminal.draw(|frame| render_main_ui(frame, self,headers))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            crossEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            KeyCode::Enter => self.select_entry(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn move_up(&mut self){
        //wrap around to bottom
        let i = match self.table_state.selected() {
            Some(i) =>{
                if i ==  0 {
                    self.data.len() -1
                }
                else{
                    i - 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn move_down(&mut self){
        //wrap around to top  
        let i = match self.table_state.selected() {
            Some(i) =>{
                if i ==  (self.data.len()-1) {
                    0
                }
                else{
                    i + 1
                }}
            None => 0,
        };

        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
        }

    fn select_entry(&mut self){
        //TODO: implement properly later
        //change colour of the selected row
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }
    fn save_input (&mut self) {
        // let tmp = Project::from_arr(self.input_array)?;
        //add new project to data vector based on input array
        self.data.push(match Project::from_arr(self.input_array) {
            Ok(input) =>input,
            Err(e) => panic!("the following error occured on input {:?}",e),
        });

        //clear the input array
        self.input_array = ["","","","","","","","","","",""];
        self.currently_editing = None;
    }
    pub fn edit_next(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Project => self.currently_editing = Some(CurrentlyEditing::Size),
                CurrentlyEditing::Size => self.currently_editing = Some(CurrentlyEditing::Cost),
                CurrentlyEditing::Cost => self.currently_editing = Some(CurrentlyEditing::WholeArmy),
                CurrentlyEditing::WholeArmy => self.currently_editing = Some(CurrentlyEditing::AssemblyRequired),
                CurrentlyEditing::AssemblyRequired => self.currently_editing = Some(CurrentlyEditing::KitbashRating),
                CurrentlyEditing::KitbashRating => self.currently_editing = Some(CurrentlyEditing::PaintingLevel),
                CurrentlyEditing::PaintingLevel => self.currently_editing = Some(CurrentlyEditing::ComplexityRating),
                CurrentlyEditing::ComplexityRating => self.currently_editing = Some(CurrentlyEditing::Priority),
                CurrentlyEditing::Priority => self.currently_editing = Some(CurrentlyEditing::Status),
                CurrentlyEditing::Status => self.currently_editing = Some(CurrentlyEditing::IsOwned),
                CurrentlyEditing::IsOwned => self.currently_editing = Some(CurrentlyEditing::Project)
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Project);
        }
    }


    fn get_constraints(items: &Vec<Project>) -> [u16;11] {
        //find out mac unicode width of each column, or the header if the header is longer
        [
            max_width(items, |x| x.project_name().to_string()),
            max_width(items, |x| x.size().to_string()),
            max_width(items, |x| x.cost().to_string()),
            max_width(items, |x| x.whole_army().to_string()),
            max_width(items, |x| x.needs_assembly().to_string()),
            max_width(items, |x| x.kitbash_rating().to_string()),
            max_width(items, |x| x.paint_level().to_string()),
            max_width(items, |x| x.complexity_rating().to_string()),
            max_width(items, |x| x.priority().to_string()),
            max_width(items, |x| x.status().to_string()),
            max_width(items, |x| x.is_owned().to_string()),
        ]

    
}

    //getters
    pub fn get_table_state(&self) -> &TableState {&self.table_state}
    pub fn get_data(&self) -> &Vec<Project> {&self.data}
    pub fn get_exit(&self) -> bool {self.exit}
    pub fn get_longest_item_lens(&self) -> &[u16; 11] {&self.longest_item_lens}
    pub fn get_scroll_state(&self) -> &ScrollbarState {&self.scroll_state}
    pub fn get_colors(&self) -> &TableColors {&self.colors}
    pub fn get_color_index(&self) -> usize {self.color_index}
    pub fn get_current_screen(&self) -> &CurrentScreen {&self.current_screen}
    pub fn get_currently_editing(&self) -> &Option<CurrentlyEditing> {&self.currently_editing}
    pub fn get_input_array(&self) -> &[&'a str; 11] {&self.input_array}

    //mutable getters for states
    pub fn get_mut_table_state(&mut self) -> &mut TableState {&mut self.table_state}
    pub fn get_mut_scroll_state(&mut self) -> &mut ScrollbarState {&mut self.scroll_state}

    //setters
    pub fn set_table_state(&mut self, table_state: TableState) {self.table_state = table_state;}
    pub fn set_data(&mut self, data: Vec<Project>) {self.data = data;}
    pub fn set_longest_item_lens(&mut self, longest_item_lens: [u16; 11]) {self.longest_item_lens = longest_item_lens;}
    pub fn set_scroll_state(&mut self, scroll_state: ScrollbarState) {self.scroll_state = scroll_state;}
    pub fn set_input_array(&mut self, input_array: [&'a str; 11]) {self.input_array = input_array;}
    pub fn set_exit(&mut self, exit: bool) {self.exit = exit;}
    pub fn set_colors(&mut self, colors: TableColors) {self.colors = colors;}
    pub fn set_color_index(&mut self, color_index: usize) {self.color_index = color_index;}
    pub fn set_current_screen(&mut self, current_screen: CurrentScreen) {self.current_screen = current_screen;}
    pub fn set_currently_editing(&mut self, currently_editing: Option<CurrentlyEditing>) {self.currently_editing = currently_editing;}
}

#[cfg(test)]
mod tests {
    use super::*;
    use project::project::{Cost,PaintLevel};
    #[test]
    fn up_test() {
        let mut test_projects = vec![];

        for i in 0..29 {
            test_projects.push(Project::new(String::from("Dangle No. ".to_owned() + &i.to_string() ), 1,Cost::None,true,false,4,PaintLevel::Character,1.0f64,1.0f64,false,true));
        }
        let test_len = &test_projects.len()-1;
        let mut test_app = App::new(test_projects);
        for i in 2..0 {
            test_app.move_up();
            if i != 0{
                //TODO: remove unwrap
                assert_eq!(i -1, test_app.table_state.selected().unwrap())
            }
            else {
                //TODO: remove unwrap
                assert_eq!(test_len, test_app.table_state.selected().unwrap())
            }
        }
    }
    #[test]
    fn down_test() {
        let mut test_projects = vec![];

        for i in 0..29 {
test_projects.push(Project::new(String::from("Dangle No. ".to_owned() + &i.to_string() ), 1,Cost::None,true,false,4,PaintLevel::Character,1.0f64,1.0f64,false,true));
        }
        let test_len = &test_projects.len()-1;
        let mut test_app = App::new(test_projects);
        for i in 2..0 {
            test_app.move_up();
            if i != test_len{
                //TODO: remove unwrap
                assert_eq!(i+1, test_app.table_state.selected().unwrap())
            }
            else {
                //TODO: remove unwrap
                assert_eq!(0, test_app.table_state.selected().unwrap())
            }
        }
    }
   
}
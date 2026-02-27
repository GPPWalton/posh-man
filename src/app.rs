
use std::{io};
use project::project::{Project};
use ratatui::{
    DefaultTerminal ,widgets::{ScrollbarState, TableState}
};
use unicode_width::UnicodeWidthStr;
use crate::{event_handlers::{handle_editing_key_event, handle_main_key_event,handle_events},
    ui::{PALETTES, TableColours, main_ui::render_main_ui, render_editing_ui,}
};

use strum_macros::EnumIter;


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
    colours: TableColours,
    colour_index: usize,
    current_screen: CurrentScreen,
    currently_editing: Option<CurrentlyEditing>,
    //TODO: Hashmap instead?
    input_array: [&'a str;11],
}

pub enum CurrentScreen {
    Main,       //main table
    Editing,    //when editing entry
    Adding,     //when adding entry
    Exiting,    //confirmation of edit or add
}
#[derive(EnumIter, Clone,Copy,PartialEq)]
pub enum CurrentlyEditing {
    Project,
    Size,
    Cost,
    WholeArmy,
    AssemblyRequired,
    KitbashRating,
    PaintingLevel,
    ComplexityRating,
    Priority,
    Status,
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
            colours: TableColours::new(&PALETTES[0]),
            colour_index: 0,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            input_array: ["","","","","","","","","","",""],
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal, headers: [&str;11]) -> io::Result<()> {
        while !self.exit {
            //main loop goes here
                        
            match self.get_current_screen() {
                CurrentScreen::Adding => todo!(),
                CurrentScreen::Editing => {
                    let selected_index = match self.get_table_state().selected(){
                        Some(i)=> i,
                        None => panic!("No entry selected")
                    };
                    terminal.draw(|frame| render_editing_ui(frame, self,selected_index))?;
                    handle_events(self,handle_editing_key_event)?;
                },
                CurrentScreen::Main => {
                    terminal.draw(|frame| render_main_ui(frame, self,headers))?;
                    handle_events(self,handle_main_key_event)?;},
                CurrentScreen::Exiting => todo!()
            }
        }
        Ok(())
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
    pub fn get_colours(&self) -> &TableColours {&self.colours}
    pub fn get_colour_index(&self) -> usize {self.colour_index}
    pub fn get_current_screen(&self) -> &CurrentScreen {&self.current_screen}
    pub fn get_currently_editing(&self) -> &Option<CurrentlyEditing> {&self.currently_editing}
    pub fn get_input_array(&self) -> &[&'a str; 11] {&self.input_array}

    //mutable getters for states and the data, as fully setting it might not be appropriate
    pub fn get_mut_table_state(&mut self) -> &mut TableState {&mut self.table_state}
    pub fn get_mut_scroll_state(&mut self) -> &mut ScrollbarState {&mut self.scroll_state}
    pub fn get_mut_data(&mut self) -> &mut Vec<Project> {&mut self.data}

    //setters
    pub fn set_table_state(&mut self, table_state: TableState) {self.table_state = table_state;}
    pub fn set_longest_item_lens(&mut self, longest_item_lens: [u16; 11]) {self.longest_item_lens = longest_item_lens;}
    pub fn set_scroll_state(&mut self, scroll_state: ScrollbarState) {self.scroll_state = scroll_state;}
    pub fn set_input_array(&mut self, input_array: [&'a str; 11]) {self.input_array = input_array;}
    pub fn set_exit(&mut self, exit: bool) {self.exit = exit;}
    pub fn set_colours(&mut self, colours: TableColours) {self.colours = colours;}
    pub fn set_colour_index(&mut self, colour_index: usize) {self.colour_index = colour_index;}
    pub fn set_current_screen(&mut self, current_screen: CurrentScreen) {self.current_screen = current_screen;}
    pub fn set_currently_editing(&mut self, currently_editing: Option<CurrentlyEditing>) {self.currently_editing = currently_editing;}
}

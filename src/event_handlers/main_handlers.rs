use std::fs::File;
use crate::{app::{CurrentScreen, CurrentlyEditing}, event_handlers::*};

/// Handles key events for navigating through and manipulating the project table.
pub fn handle_main_key_event( app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => exit(app),
        KeyCode::Up => move_up(app),
        KeyCode::Down => move_down(app),
        KeyCode::Enter => select_entry(app),
        KeyCode::Char('n') => add_new_entry(app),
        _ => {}
    }
}

/// Exits the program, cuttently saves changes on exit.
fn exit(app: &mut App) {
    let _test = save_data(app, "project_priorities.csv".to_string());
    app.set_exit(true);
    app.set_current_screen(CurrentScreen::Main);
}

/// Move upwards once on the table.
fn move_up(app: &mut App){
    //wrap around to bottom
    let i = match app.get_table_state().selected() {
        Some(i) =>{
            if i ==  0 {
                app.get_data().len() -1
            }
            else{
                i - 1
            }
        }
        None => 0,
    };

    app.get_mut_table_state().select(Some(i));
    app.set_scroll_state(app.get_scroll_state().position(i * ITEM_HEIGHT));
}

/// Move downwards once on the table.
fn move_down(app: &mut App){
    //wrap around to top  
    let i = match app.get_table_state().selected() {
        Some(i) =>{
            if i ==  (app.get_data().len()-1) {
                0
            }
            else{
                i + 1
            }}
        None => 0,
    };

    app.get_mut_table_state().select(Some(i));
    app.set_scroll_state(app.get_scroll_state().position(i * ITEM_HEIGHT));
    }

/// Edit the currently selected entry.
fn select_entry(app: &mut App){
    //change colour of the selected row
    app.set_colour_index((app.get_colour_index() + 1) % PALETTES.len());
    app.set_current_screen(CurrentScreen::Editing);
    //get index of selected row
    let selected_index = app.get_selected_index();

    //load record into input for editing
    app.set_input_array(app.get_data()[selected_index].as_str_array());
    app.set_currently_editing(Some(CurrentlyEditing::Project));
}

/// Add a new entry to the project table.
fn add_new_entry (app: &mut App){
    //change colour of the selected row
    app.set_colour_index((app.get_colour_index() + 1) % PALETTES.len());
    //switch current screen to adding
    app.set_current_screen(CurrentScreen::Adding);

    //set a blank project
    app.set_currently_editing(Some(CurrentlyEditing::Project));
}

//TODO: handle ? shortcut so I don't need to return a result

/// Saves the `Vec<Project>` that stores the data back to the CSV file.
fn save_data(app: &mut App, file_path: String)-> Result<(), Box<dyn std::error::Error>>{
    match File::create(&file_path){
        Ok(_)=>{
            let mut wtr = match csv::Writer::from_path(&file_path){
                Ok( wtr) => wtr,
                Err(e) => panic!("{:?}",e)
            };
            for row in app.get_data() {
                // Add header to file
                wtr.serialize(row)?;
            }
            wtr.flush()?;
        },
        Err(e) => panic!("{:?}",e)
    };
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use project::project::{Cost,PaintLevel,Project};
    #[test]
    fn up_test() {
        let mut test_projects = vec![];

        for i in 0..29 {
            test_projects.push(Project::new(String::from("Dangle No. ".to_owned() + &i.to_string() ), 1,Cost::None,true,false,4,PaintLevel::Character,1.0f64,1.0f64,false,true));
        }
        let test_len = &test_projects.len()-1;
        let mut test_app = App::new(test_projects);
        for i in 2..0 {
            move_up(&mut test_app);
            if i != 0{
                //TODO: remove unwrap
                assert_eq!(i -1, test_app.get_mut_table_state().selected().unwrap())
            }
            else {
                //TODO: remove unwrap
                assert_eq!(test_len, test_app.get_mut_table_state().selected().unwrap())
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
            move_down(&mut test_app);
            if i != test_len{
                //TODO: remove unwrap
                assert_eq!(i+1, test_app.get_mut_table_state().selected().unwrap())
            }
            else {
                //TODO: remove unwrap
                assert_eq!(0, test_app.get_mut_table_state().selected().unwrap())
            }
        }
    }
   
}
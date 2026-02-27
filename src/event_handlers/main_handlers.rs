use crate::{app::{CurrentScreen, CurrentlyEditing}, event_handlers::*};

pub fn handle_main_key_event( app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => exit(app),
        KeyCode::Up => move_up(app),
        KeyCode::Down => move_down(app),
        KeyCode::Enter => select_entry(app),
        _ => {}
    }
}

fn exit(app: &mut App) {
    app.set_exit(true);
    app.set_current_screen(CurrentScreen::Main);
}

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

fn select_entry(app: &mut App){
    //TODO: implement properly later
    //change colour of the selected row
    app.set_colour_index((app.get_colour_index() + 1) % PALETTES.len());
    app.set_current_screen(CurrentScreen::Editing);
    app.set_currently_editing(Some(CurrentlyEditing::Project));
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
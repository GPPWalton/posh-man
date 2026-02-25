
    use super::*;
    use project::project::Project;
    use crate::app::{CurrentScreen, CurrentlyEditing};
    use strum::{IntoEnumIterator};

    //enum to represent direction user moves through attributes when editing
    enum StepDirection {
        Left,
        Right
    }
    //TODO: implement event_handling for editing pop-up
    pub fn handle_editing_key_event( app: &mut App, key_event: KeyEvent){
        match key_event.code {
            //TODO: change keycode to Ctrl+C or Esc
            KeyCode::Char('c') => close_popup(app),
            KeyCode::Left => move_to(StepDirection::Left,app),
            KeyCode::Right => move_to(StepDirection::Right,app),
            KeyCode::Enter => confirm(app),
            _ => {}
        }
    }
    fn move_to(direction: StepDirection,app: &mut App){
        if let Some(edit_mode) = &app.get_currently_editing() {
            //convert enum into vector
            let currently_editing_vec: Vec<_> = CurrentlyEditing::iter().collect();
            //find index of the current value of the enum
            let current_index = match currently_editing_vec.iter().position(|v| v == edit_mode){
                Some(i) => i,
                None => panic!("CurrentlyEditing value not found in vector")
            };
            let len = currently_editing_vec.len();

            let next_index = match direction {
                //wrap around the vector, using modulo for remainder,
                StepDirection::Left => (current_index + len - 1) % len,
                StepDirection::Right => (current_index + 1) % len,
            };

            app.set_currently_editing(Some(currently_editing_vec[next_index]));
        }
    }

    fn input_listener (app: &mut App){

    }
    fn close_popup(app: &mut App){
        app.set_current_screen(CurrentScreen::Main);
        //change colour of selected row back to 'unselected' colour
        app.set_color_index((app.get_color_index() + 1) % PALETTES.len());
    }
    fn confirm(app: &mut App){
        todo!()
    }
    fn save_input (app: &mut App) {
        //add new project to data vector based on input array
        let new_record = match Project::from_arr(app.get_input_array()) {
            Ok(input) =>input,
            Err(e) => panic!("the following error occured on input {:?}",e),
        };
        app.get_mut_data().push(new_record);

        //clear the input array
        app.set_input_array(["","","","","","","","","","",""]);
        app.set_currently_editing(None);
    }


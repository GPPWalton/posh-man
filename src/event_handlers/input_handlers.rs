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
    pub fn handle_input_key_event( app: &mut App, key_event: KeyEvent){
        match key_event.code {
            KeyCode::Esc => close_popup(app),
            KeyCode::BackTab => move_to(StepDirection::Left, app),
            KeyCode::Tab => move_to(StepDirection::Right, app),
            //TODO:  implement cursor and changing input position with left and right
            // KeyCode::Left => todo!(),
            // KeyCode::Right => todo!(),
            KeyCode::Enter => confirm(app),
            KeyCode::Backspace => delete_char(app),
            KeyCode::Char(value) => insert_char(value,app),
            _ =>{}
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

            //might be a better way to do this
            app.set_currently_editing(Some(currently_editing_vec[next_index].clone()));
        }
    }

    fn delete_char(app: &mut App){
        if let Some(editing) = &mut app.get_mut_currently_editing() {
            match editing {
                CurrentlyEditing::Project => app.get_mut_input_array()[0].pop(),
                CurrentlyEditing::Size => app.get_mut_input_array()[1].pop(),
                CurrentlyEditing::Cost => app.get_mut_input_array()[2].pop(),
                CurrentlyEditing::WholeArmy => app.get_mut_input_array()[3].pop(),
                CurrentlyEditing::AssemblyRequired => app.get_mut_input_array()[4].pop(),
                CurrentlyEditing::KitbashRating => app.get_mut_input_array()[5].pop(),
                CurrentlyEditing::PaintingLevel => app.get_mut_input_array()[6].pop(),
                CurrentlyEditing::ComplexityRating => app.get_mut_input_array()[7].pop(),
                CurrentlyEditing::Priority => app.get_mut_input_array()[8].pop(),
                CurrentlyEditing::Status => app.get_mut_input_array()[9].pop(),
                CurrentlyEditing::IsOwned => app.get_mut_input_array()[10].pop()
            };
        }
    }
    
    fn insert_char(value: char, app: &mut App) {
        // Step 1: Get the current editing state (mutable borrow starts and ends here)
        if let Some(editing) = app.get_mut_currently_editing() {
            match editing {
                CurrentlyEditing::Project => app.get_mut_input_array()[0].push(value),
                CurrentlyEditing::Size => app.get_mut_input_array()[1].push(value),
                CurrentlyEditing::Cost => app.get_mut_input_array()[2].push(value),
                CurrentlyEditing::WholeArmy => app.get_mut_input_array()[3].push(value),
                CurrentlyEditing::AssemblyRequired => app.get_mut_input_array()[4].push(value),
                CurrentlyEditing::KitbashRating => app.get_mut_input_array()[5].push(value),
                CurrentlyEditing::PaintingLevel => app.get_mut_input_array()[6].push(value),
                CurrentlyEditing::ComplexityRating => app.get_mut_input_array()[7].push(value),
                CurrentlyEditing::Priority => app.get_mut_input_array()[8].push(value),
                CurrentlyEditing::Status => app.get_mut_input_array()[9].push(value),
                CurrentlyEditing::IsOwned => app.get_mut_input_array()[10].push(value)
            }
        }
        
    }
    fn close_popup(app: &mut App){
        match app.get_current_screen() {
            CurrentScreen::Adding => {
                let last_index = app.get_data().len()-1;
                //when adding new entry, ensure that table has new row selected when it returns
                app.get_mut_table_state().select(Some(last_index));
                app.set_scroll_state(app.get_scroll_state().position(last_index * ITEM_HEIGHT));
            },
            _ =>{}
        }
        app.set_current_screen(CurrentScreen::Main);
        //change colour of selected row back to 'unselected' colour
        app.set_colour_index((app.get_colour_index() + 1) % PALETTES.len());
    }
    fn confirm(app: &mut App){
        //save input to new record
        save_input(app);
        //TODO: add a confirmation pop-up here, so user can double check input
        //close pop-up
        close_popup(app);
        app.set_currently_editing(None);
    }
    fn save_input (app: &mut App) {
        //add new project to data vector based on input array
        let new_record = match Project::from_arr(app.get_input_array()) {
            Ok(input) =>input,
            Err(e) => panic!("the following error occured on input {:?}",e),
        };
        match app.get_current_screen() {
            CurrentScreen::Adding => {
                //if adding new entry, add a new element to data vector
                app.get_mut_data().push(new_record);
            }
            CurrentScreen::Editing => {
                let selected_index = app.get_selected_index();
                //otherwise, if editing, overwrite existing data with new record
                app.get_mut_data()[selected_index] =new_record;
            }
            _ => {}//in any other case, do nothing
        }
        //clear the input array
        app.set_input_array([
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ]);
    }


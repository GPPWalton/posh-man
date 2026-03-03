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
            KeyCode::Esc => close_popup(app),
            KeyCode::BackTab => move_to(StepDirection::Left, app),
            KeyCode::Tab => move_to(StepDirection::Right, app),
            //TODO:  implement cursor and changing input position with left and right
            // KeyCode::Left => todo!(),
            // KeyCode::Right => todo!(),
            //TODO: enter should also shift to next attr, then confirm at the end before closing pup-up
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

    //TODO: needs reworking now input method was reverted
    fn delete_char(app: &mut App){
        if let Some(editing) = &mut app.get_mut_currently_editing() {
            match editing {
                CurrentlyEditing::Project(_) => app.get_mut_input_array()[0].pop(),
                CurrentlyEditing::Size(_) => app.get_mut_input_array()[1].pop(),
                CurrentlyEditing::Cost(_) => app.get_mut_input_array()[2].pop(),
                CurrentlyEditing::WholeArmy(_) => app.get_mut_input_array()[3].pop(),
                CurrentlyEditing::AssemblyRequired(_) => app.get_mut_input_array()[4].pop(),
                CurrentlyEditing::KitbashRating(_) => app.get_mut_input_array()[5].pop(),
                CurrentlyEditing::PaintingLevel(_) => app.get_mut_input_array()[6].pop(),
                CurrentlyEditing::ComplexityRating(_) => app.get_mut_input_array()[7].pop(),
                CurrentlyEditing::Priority(_) => app.get_mut_input_array()[8].pop(),
                CurrentlyEditing::Status(_) => app.get_mut_input_array()[9].pop(),
                CurrentlyEditing::IsOwned(_) => app.get_mut_input_array()[10].pop()
            };
        }
    }
    //TODO: needs reworking now input method was reverted
    fn insert_char(value: char, app: &mut App) {
        // Step 1: Get the current editing state (mutable borrow starts and ends here)
        if let Some(editing) = app.get_mut_currently_editing() {
            match editing {
                CurrentlyEditing::Project(_) => app.get_mut_input_array()[0].push(value),
                CurrentlyEditing::Size(_) => app.get_mut_input_array()[1].push(value),
                CurrentlyEditing::Cost(_) => app.get_mut_input_array()[2].push(value),
                CurrentlyEditing::WholeArmy(_) => app.get_mut_input_array()[3].push(value),
                CurrentlyEditing::AssemblyRequired(_) => app.get_mut_input_array()[4].push(value),
                CurrentlyEditing::KitbashRating(_) => app.get_mut_input_array()[5].push(value),
                CurrentlyEditing::PaintingLevel(_) => app.get_mut_input_array()[6].push(value),
                CurrentlyEditing::ComplexityRating(_) => app.get_mut_input_array()[7].push(value),
                CurrentlyEditing::Priority(_) => app.get_mut_input_array()[8].push(value),
                CurrentlyEditing::Status(_) => app.get_mut_input_array()[9].push(value),
                CurrentlyEditing::IsOwned(_) => app.get_mut_input_array()[10].push(value)
            }
        }
        
    }
    fn close_popup(app: &mut App){
        app.set_current_screen(CurrentScreen::Main);
        //change colour of selected row back to 'unselected' colour
        app.set_colour_index((app.get_colour_index() + 1) % PALETTES.len());
    }
    fn confirm(app: &mut App){
        todo!()
    }
    // fn save_input (app: &mut App) {
    //     //add new project to data vector based on input array
    //     let new_record = match Project::from_arr(app.get_input_array()) {
    //         Ok(input) =>input,
    //         Err(e) => panic!("the following error occured on input {:?}",e),
    //     };
    //     app.get_mut_data().push(new_record);

    //     //clear the input array
    //     app.set_input_array(["","","","","","","","","","",""]);
    //     app.set_currently_editing(None);
    // }


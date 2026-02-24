
    use super::*;
    use project::project::Project;
    use crate::app::{CurrentScreen, CurrentlyEditing};

    //TODO: implement event_handling for editing pop-up
    pub fn handle_editing_key_event( app: &mut App, key_event: KeyEvent){
        match key_event.code {
            KeyCode::Char('c') => close_popup(app),
            KeyCode::Left => move_left(app),
            KeyCode::Right => move_right(app),
            KeyCode::Enter => confirm(app),
            _ => {}
        }
    }
    fn move_left(app: &mut App){
        todo!()
    }
    fn move_right(app: &mut App){
        todo!()
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
    fn edit_next(app: &mut App) {
        if let Some(edit_mode) = &app.get_currently_editing() {
            match edit_mode {
                CurrentlyEditing::Project => app.set_currently_editing(Some(CurrentlyEditing::Size)),
                CurrentlyEditing::Size => app.set_currently_editing(Some(CurrentlyEditing::Cost)),
                CurrentlyEditing::Cost => app.set_currently_editing(Some(CurrentlyEditing::WholeArmy)),
                CurrentlyEditing::WholeArmy => app.set_currently_editing(Some(CurrentlyEditing::AssemblyRequired)),
                CurrentlyEditing::AssemblyRequired => app.set_currently_editing(Some(CurrentlyEditing::KitbashRating)),
                CurrentlyEditing::KitbashRating => app.set_currently_editing(Some(CurrentlyEditing::PaintingLevel)),
                CurrentlyEditing::PaintingLevel => app.set_currently_editing(Some(CurrentlyEditing::ComplexityRating)),
                CurrentlyEditing::ComplexityRating => app.set_currently_editing(Some(CurrentlyEditing::Priority)),
                CurrentlyEditing::Priority => app.set_currently_editing(Some(CurrentlyEditing::Status)),
                CurrentlyEditing::Status => app.set_currently_editing(Some(CurrentlyEditing::IsOwned)),
                CurrentlyEditing::IsOwned => app.set_currently_editing(Some(CurrentlyEditing::Project))
            };
        } else {
            app.set_currently_editing(Some(CurrentlyEditing::Project));
        }
    }


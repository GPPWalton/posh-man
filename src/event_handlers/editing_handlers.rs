
    use super::*;
    use project::project::Project;
    use crate::app::CurrentlyEditing;

    fn save_input (app: &mut App) {
        // let tmp = Project::from_arr(app.input_array)?;
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
    pub fn edit_next(app: &mut App) {
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


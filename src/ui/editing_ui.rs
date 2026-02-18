    use crate::ui::*;
    use ratatui::{Frame, 
   
};
    use crate::app::App;

    pub fn render_editing_ui(frame: &mut Frame, app: &App) {
        let outer_edit_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(3),
                //TODO: replace min here with a constant later
                Constraint::Min(15),
                Constraint::Length(3)])
            .split(frame.area());
        let inner_edit_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(1),
            ])
            .split(outer_edit_layout[1]);
    }


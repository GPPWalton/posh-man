    use crate::{app::CurrentlyEditing, ui::*};
    use project::project::Project;
    use ratatui::{Frame, style::Style, widgets::{Block, BorderType, Borders, Paragraph}
};
    use crate::app::App;


    pub fn render_editing_ui(frame: &mut Frame, app: &mut App) {
        if let Some(editing) = &app.get_currently_editing() {
            let editing_block = Block::default()
                .title("Add a new project")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let area = centered_rect(60, 25, frame.area());
            frame.render_widget(editing_block, area);

            let outer_edit_layout = Layout::default().direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![
                Constraint::Length(3),
                //TODO: replace min here with a constant later
                Constraint::Min(15),
                Constraint::Length(3)])
            .split(area);
            let inner_edit_layout = Layout::default().direction(Direction::Vertical)
                .margin(1)
                .constraints(vec![
                    Constraint::Length(1),
                    Constraint::Min(3),
                    Constraint::Length(1),
                ])
                .split(outer_edit_layout[1]);
            let left_arrow = Paragraph::new("<").block(Block::default()
                .borders(Borders::NONE));
            frame.render_widget(left_arrow, outer_edit_layout[0]);

            let right_arrow =Paragraph::new(">").block(Block::default()
                .borders(Borders::NONE));
            frame.render_widget(right_arrow, outer_edit_layout[2]);

            let input_block = Block::default().border_type(BorderType::Double);

            match app.get_table_state().selected() {
                Some(i) => {
                   let current_project = &app.get_data()[i];
                   match editing {
                        CurrentlyEditing::Project => input_block.title(current_project.project_name()),
                        CurrentlyEditing::Size => input_block.title(current_project.size().to_string()),
                        CurrentlyEditing::Cost => input_block.title(current_project.cost().to_string()),
                        CurrentlyEditing::WholeArmy =>input_block.title(current_project.whole_army().to_string()),
                        CurrentlyEditing::AssemblyRequired => input_block.title(current_project.needs_assembly().to_string()),
                        CurrentlyEditing::KitbashRating => input_block.title(current_project.kitbash_rating().to_string()),
                        CurrentlyEditing::PaintingLevel => input_block.title(current_project.paint_level().to_string()),
                        CurrentlyEditing::ComplexityRating => input_block.title(current_project.complexity_rating().to_string()),
                        CurrentlyEditing::Priority => input_block.title(current_project.priority().to_string()),
                        CurrentlyEditing::Status => input_block.title(current_project.status().to_string()),
                        CurrentlyEditing::IsOwned => input_block.title(current_project.is_owned().to_string()),
                    };
                },
                None  => panic!("No row selected.")
            };

            //TODO: Add text input box
            //TODO: flashing cursor?
            //TODO: Add info tooltip/footer
        }
    }

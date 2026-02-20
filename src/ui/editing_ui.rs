use crate::{app::CurrentlyEditing, ui::*};
use ratatui::{Frame, style::Style, widgets::{Block, BorderType, Borders, Paragraph}
};
use crate::app::App;

const INFO_TEXT: [&str; 2] = [
    "<C> Cancel | <←> Previous Attribute | <→> Next Attribute",
    "<Enter> Confirm",
];
const INPUT_WIDTH: u16 = 15;

//TODO: editing_ui event handlers
pub fn render_editing_ui(frame: &mut Frame, app: &mut App, selected_index: usize) {
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
            Constraint::Min(INPUT_WIDTH),
            Constraint::Length(3)])
        .split(area);
        let inner_edit_layout = Layout::default().direction(Direction::Vertical)
            .margin(1)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ])
            .split(outer_edit_layout[1]);
        let left_arrow = Paragraph::new("←").block(Block::default()
            .borders(Borders::NONE));
        frame.render_widget(left_arrow, outer_edit_layout[0]);

        let right_arrow =Paragraph::new("→").block(Block::default()
            .borders(Borders::NONE));
        frame.render_widget(right_arrow, outer_edit_layout[2]);
        
        // get currently selected project,
        let current_project = &app.get_data()[selected_index];
        // use currently editing enum to determine the title displayed in input block
        let title = match editing {
            CurrentlyEditing::Project => current_project.project_name(),
            CurrentlyEditing::Size => &current_project.size().to_string(),
            CurrentlyEditing::Cost => &current_project.cost().to_string(),
            CurrentlyEditing::WholeArmy => &current_project.whole_army().to_string(),
            CurrentlyEditing::AssemblyRequired => &current_project.needs_assembly().to_string(),
            CurrentlyEditing::KitbashRating => &current_project.kitbash_rating().to_string(),
            CurrentlyEditing::PaintingLevel => &current_project.paint_level().to_string(),
            CurrentlyEditing::ComplexityRating => &current_project.complexity_rating().to_string(),
            CurrentlyEditing::Priority => &current_project.priority().to_string(),
            CurrentlyEditing::Status => &current_project.status().to_string(),
            CurrentlyEditing::IsOwned => &current_project.is_owned().to_string(),
        };
        //render input_block
        let input_block = Block::default().title(title).border_type(BorderType::Double);
        frame.render_widget(input_block, inner_edit_layout[0]);
        //TODO: flashing cursor?
        let input_footer = Paragraph::new(Text::from_iter(INFO_TEXT));
        frame.render_widget(input_footer, inner_edit_layout[1]);
    }
}

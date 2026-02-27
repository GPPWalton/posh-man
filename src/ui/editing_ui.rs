use crate::{app::CurrentlyEditing, ui::*};
use ratatui::{Frame, symbols::border, widgets::{Block, BorderType, Borders, Paragraph}
};
use crate::app::App;

const INFO_TEXT: [&str; 2] = [
    "<Esc> Cancel | <Shift+Tab> Previous Attribute | <Tab> Next Attribute",
    "<Enter> Confirm",
];
const INPUT_WIDTH: u16 = 15;

pub fn render_editing_ui(frame: &mut Frame, app: &mut App, selected_index: usize) {
    if let Some(editing) = &app.get_currently_editing() {
        let area = centered_rect(50, 50, frame.area());
        let editing_block = Block::bordered()
            .title("Add a new project")
            .borders(Borders::ALL)
            .border_set(border::DOUBLE);

        
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

        // use currently editing enum to determine the title displayed in input block
        let title = match editing {
            CurrentlyEditing::Project(_) => "Project Name",
            CurrentlyEditing::Size(_) => "Size",
            CurrentlyEditing::Cost(_) => "Cost",
            CurrentlyEditing::WholeArmy(_) => "Whole Army/Warband",
            CurrentlyEditing::AssemblyRequired(_) => "Assembly Required",
            CurrentlyEditing::KitbashRating(_) => "Kitbash Rating",
            CurrentlyEditing::PaintingLevel(_) => "Painting Level",
            CurrentlyEditing::ComplexityRating(_) => "Complexity Rating",
            CurrentlyEditing::Priority(_) => "Priority",
            CurrentlyEditing::Status(_) => "Status",
            CurrentlyEditing::IsOwned(_) => "Is Owned",
        };
        //render input_block
        let input_block = Block::bordered().title(title).border_type(BorderType::Double);
        if let Some(edit_mode) = app.get_currently_editing(){
            let input_text =match edit_mode {
            CurrentlyEditing::Project(current_input) => current_input,
            CurrentlyEditing::Size(current_input) => current_input,
            CurrentlyEditing::Cost(current_input) => current_input,
            CurrentlyEditing::WholeArmy(current_input) => current_input,
            CurrentlyEditing::AssemblyRequired(current_input) => current_input,
            CurrentlyEditing::KitbashRating(current_input) => current_input,
            CurrentlyEditing::PaintingLevel(current_input) => current_input,
            CurrentlyEditing::ComplexityRating(current_input) => current_input,
            CurrentlyEditing::Priority(current_input) => current_input,
            CurrentlyEditing::Status(current_input) => current_input,
            CurrentlyEditing::IsOwned(current_input) => current_input,
           };
           let input_field = Paragraph::new(Text::from(input_text.to_string())).block(input_block);
            frame.render_widget(input_field, inner_edit_layout[0]);
        }
        //TODO: flashing cursor!
        let input_footer = Paragraph::new(Text::from_iter(INFO_TEXT));
        frame.render_widget(input_footer, inner_edit_layout[1]);
    }
}

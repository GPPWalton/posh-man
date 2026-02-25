use crate::{app::CurrentlyEditing, ui::*};
use ratatui::{Frame, style::Style, style::Stylize, symbols::border,widgets::Widget, widgets::{Block, BorderType, Borders, Paragraph}
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
            CurrentlyEditing::Project => "Project Name",
            CurrentlyEditing::Size => "Size",
            CurrentlyEditing::Cost => "Cost",
            CurrentlyEditing::WholeArmy => "Whole Army/Warband",
            CurrentlyEditing::AssemblyRequired => "Assembly Required",
            CurrentlyEditing::KitbashRating => "Kitbash Rating",
            CurrentlyEditing::PaintingLevel => "Painting Level",
            CurrentlyEditing::ComplexityRating => "Complexity Rating",
            CurrentlyEditing::Priority => "Priority",
            CurrentlyEditing::Status => "Status",
            CurrentlyEditing::IsOwned => "Is Owned",
        };
        //render input_block
        let input_block = Block::bordered().title(title).border_type(BorderType::Double);
        frame.render_widget(input_block, inner_edit_layout[0]);
        //TODO: flashing cursor?
        let input_footer = Paragraph::new(Text::from_iter(INFO_TEXT));
        frame.render_widget(input_footer, inner_edit_layout[1]);
    }
}

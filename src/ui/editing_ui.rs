use crate::{app::CurrentlyEditing, ui::*};
use ratatui::{Frame, symbols::border, widgets::{Block, BorderType, Borders, Paragraph}
};
use crate::app::App;

const INFO_TEXT: [&str; 2] = [
    "<Esc> Cancel | <Shift+Tab> Previous Attribute | <Tab> Next Attribute",
    "<Enter> Confirm",
];
const INPUT_WIDTH: u16 = 15;

pub fn render_editing_ui(frame: &mut Frame, app: &mut App) {
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
        //TODO: In a later version, Add some input validation!
        if let Some(edit_mode) = app.get_currently_editing(){
            let input_text =match edit_mode {
            CurrentlyEditing::Project => app.get_input_array()[0].to_string(),
            CurrentlyEditing::Size => app.get_input_array()[1].to_string(),
            CurrentlyEditing::Cost => app.get_input_array()[2].to_string(),
            CurrentlyEditing::WholeArmy => app.get_input_array()[3].to_string(),
            CurrentlyEditing::AssemblyRequired => app.get_input_array()[4].to_string(),
            CurrentlyEditing::KitbashRating => app.get_input_array()[5].to_string(),
            CurrentlyEditing::PaintingLevel => app.get_input_array()[6].to_string(),
            CurrentlyEditing::ComplexityRating => app.get_input_array()[7].to_string(),
            CurrentlyEditing::Priority => app.get_input_array()[8].to_string(),
            CurrentlyEditing::Status => app.get_input_array()[9].to_string(),
            CurrentlyEditing::IsOwned => app.get_input_array()[10].to_string(),
           };
           let input_field = Paragraph::new(Text::from(input_text.to_string())).block(input_block);
            frame.render_widget(input_field, inner_edit_layout[0]);
        }
        //TODO: flashing cursor!
        let input_footer = Paragraph::new(Text::from_iter(INFO_TEXT));
        frame.render_widget(input_footer, inner_edit_layout[1]);
    }
}

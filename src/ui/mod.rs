 use ratatui::{style::{palette::tailwind, Color},
     text::{Line, Text},
     layout::{Direction,Constraint,Layout,Rect}
};

//TODO: consider replacing this with a dynamic value calculated in the process_cell_content function
const CELL_WRAP_LIMIT: u16 = 16;
const CELL_PADDING: u16 = 1;
const HEADER_WRAP_LIMIT: u16 = 10;
pub const PALETTES: [tailwind::Palette; 2] = [
    tailwind::BLUE,
    tailwind::EMERALD,
];
const INFO_TEXT: [&str; 2] = [
    "<Q> quit | <↑> move up | <↓> move down",
    "<Enter> select",
];

pub mod main_ui;
pub mod editing_ui;

pub use main_ui::*;
pub use editing_ui::*;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn process_cell_content<'a> (content: String, limit: u16) -> Text<'a>{
    //find how many lines
    let isolated_words: Vec<&str> = content.split_whitespace().collect();

    let mut lines: Vec<Line> = Vec::new();
    let mut current_line = String::new();

    for word in isolated_words {
        // Check if adding this word would exceed the limit
        let potential_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        if potential_line.chars().count() > limit.into() {
            // Current line is full, push it and start a new line
            if !current_line.is_empty() {
                lines.push(Line::from(current_line));
            }
            current_line = word.to_string();
        } else {
            // Add word to current line
            current_line = potential_line;
        }
    }

    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }
    Text::from(lines)
}
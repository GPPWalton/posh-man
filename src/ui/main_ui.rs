    use crate::ui::*;
    use ratatui::{Frame, layout::{Margin},
    style::{Modifier, Style, Stylize, Color},
    widgets::{Block, BorderType, Cell, HighlightSpacing,
        Paragraph, Row, Table,Scrollbar,ScrollbarOrientation}
};
    use crate::app::App;

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
#[derive(Debug, Default)]
pub struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
     pub const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
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

pub fn render_main_ui(frame: &mut Frame, app: &mut App, headers: [&str;11]){
    let main_layout = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
    let rects = main_layout.split(frame.area());

    set_colors(app);

    render_table(app,frame, rects[0],headers);
    render_scrollbar(app,frame, rects[0]);
    render_footer(app,frame, rects[1]);
}

fn set_colors(app: &mut App) {
    app.set_colors(TableColors::new(&PALETTES[app.get_color_index()]));
}

fn render_table(app: &mut App, frame: &mut Frame, area: Rect,headers: [&str;11]) {
    let header_style = Style::default()
        .fg(app.get_colors().header_fg)
        .bg(app.get_colors().header_bg);
    let selected_row_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.get_colors().selected_row_style_fg);
    let selected_col_style = Style::default().fg(app.get_colors().selected_column_style_fg);
    let selected_cell_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.get_colors().selected_cell_style_fg);

    let header = headers
        .into_iter()
        .map(|content| Cell::from(Text::from(
            process_cell_content(String::from(content),HEADER_WRAP_LIMIT))))
        .collect::<Row>()
        .style(header_style)
        .height(3);
    let rows = app.get_data().iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.get_colors().normal_row_color,
            _ => app.get_colors().alt_row_color,
        };
        let item = data.as_str_array();
        //TODO: consider centering each column beyond the first, use slice?
        item.into_iter()
            .map(|content| Cell::from(Text::from(process_cell_content(content, CELL_WRAP_LIMIT))))
            .collect::<Row>()
            .style(Style::new().fg(app.get_colors().row_fg).bg(color))
            .height(4)
    });
    let bar = " █ ";
    let t = Table::new(
        rows,
        [
            Constraint::Min(CELL_WRAP_LIMIT+CELL_PADDING),
            Constraint::Min(app.get_longest_item_lens()[1].max(HEADER_WRAP_LIMIT)+ CELL_PADDING),
            Constraint::Min(app.get_longest_item_lens()[2].max(HEADER_WRAP_LIMIT)),
            Constraint::Min(app.get_longest_item_lens()[3].max(HEADER_WRAP_LIMIT)),
            Constraint::Min(app.get_longest_item_lens()[4].max(HEADER_WRAP_LIMIT)),
            Constraint::Min(app.get_longest_item_lens()[5].max(HEADER_WRAP_LIMIT)),
            Constraint::Min(app.get_longest_item_lens()[6].max(HEADER_WRAP_LIMIT)),
            Constraint::Min(app.get_longest_item_lens()[7].max(HEADER_WRAP_LIMIT)),
            Constraint::Min(app.get_longest_item_lens()[8].max(HEADER_WRAP_LIMIT)),
            Constraint::Min(app.get_longest_item_lens()[9].max(HEADER_WRAP_LIMIT)),
            Constraint::Min(app.get_longest_item_lens()[10].max(HEADER_WRAP_LIMIT)),
        
        ],
    )
    .header(header)
    .row_highlight_style(selected_row_style)
    .column_highlight_style(selected_col_style)
    .cell_highlight_style(selected_cell_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.get_colors().buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(t, area, app.get_mut_table_state());
}

fn render_scrollbar(app: &mut App, frame: &mut Frame, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        app.get_mut_scroll_state(),
    );
}

fn render_footer(app: &mut App, frame: &mut Frame, area: Rect) {       
    let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
        .style(
            Style::new()
                .fg(app.get_colors().row_fg)
                .bg(app.get_colors().buffer_bg),
        )
        .centered()
        .block(
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(app.get_colors().footer_border_color)),
        );
    frame.render_widget(info_footer, area);
}


    
#[cfg(test)]
mod test{
    use super::*;
     #[test]
    fn line_wrap_test(){
        let line_content = String::from("Lord-Veritant on Gryph-Stalker");

        let cell_text = process_cell_content(line_content,CELL_WRAP_LIMIT);
        assert_eq!(cell_text.lines.len(),2);
        assert_eq!(cell_text.lines[0],Line::from("Lord-Veritant on"));
        assert_eq!(cell_text.lines[1],Line::from("Gryph-Stalker"));
    }
}
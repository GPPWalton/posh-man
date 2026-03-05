    use crate::ui::*;
    use ratatui::{Frame, layout::{Margin},
    style::{Modifier, Style, Stylize, Color},
    widgets::{Block, BorderType, Cell, HighlightSpacing,
        Paragraph, Row, Table,Scrollbar,ScrollbarOrientation}
};
    use crate::app::App;
const CELL_WRAP_LIMIT: u16 = 16;
const CELL_PADDING: u16 = 1;
const HEADER_WRAP_LIMIT: u16 = 10;
const INFO_TEXT: [&str; 2] = [
    "<Esc> quit | <↑> move up | <↓> move down",
    "<Enter> edit | <n> create new entry",
];
#[derive(Debug, Default)]
pub struct TableColours {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_colour: Color,
    alt_row_colour: Color,
    footer_border_colour: Color,
}

impl TableColours {
     pub const fn new(colour: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: colour.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: colour.c400,
            selected_column_style_fg: colour.c400,
            selected_cell_style_fg: colour.c600,
            normal_row_colour: tailwind::SLATE.c950,
            alt_row_colour: tailwind::SLATE.c900,
            footer_border_colour: colour.c400,
        }
    }
}

pub fn render_main_ui(frame: &mut Frame, app: &mut App, headers: [&str;11]){
    let main_layout = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
    let rects = main_layout.split(frame.area());

    set_colours(app);

    render_table(app,frame, rects[0],headers);
    render_scrollbar(app,frame, rects[0]);
    render_footer(app,frame, rects[1]);
}

fn set_colours(app: &mut App) {
    app.set_colours(TableColours::new(&PALETTES[app.get_colour_index()]));
}

fn render_table(app: &mut App, frame: &mut Frame, area: Rect,headers: [&str;11]) {
    let header_style = Style::default()
        .fg(app.get_colours().header_fg)
        .bg(app.get_colours().header_bg);
    let selected_row_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.get_colours().selected_row_style_fg);
    let selected_col_style = Style::default().fg(app.get_colours().selected_column_style_fg);
    let selected_cell_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.get_colours().selected_cell_style_fg);

    let header = headers
        .into_iter()
        .map(|content| Cell::from(Text::from(
            process_cell_content(String::from(content),HEADER_WRAP_LIMIT))))
        .collect::<Row>()
        .style(header_style)
        .height(3);
    let rows = app.get_data().iter().enumerate().map(|(i, data)| {
        let colour = match i % 2 {
            0 => app.get_colours().normal_row_colour,
            _ => app.get_colours().alt_row_colour,
        };
        let item = data.as_str_array();
        //TODO: consider centering each column beyond the first, use slice?
        item.into_iter()
            .map(|content| Cell::from(Text::from(process_cell_content(content, CELL_WRAP_LIMIT))))
            .collect::<Row>()
            .style(Style::new().fg(app.get_colours().row_fg).bg(colour))
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
    .bg(app.get_colours().buffer_bg)
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
                .fg(app.get_colours().row_fg)
                .bg(app.get_colours().buffer_bg),
        )
        .centered()
        .block(
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(app.get_colours().footer_border_colour)),
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
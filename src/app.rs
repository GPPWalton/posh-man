
use std::{io, vec};
use project::project::{Project};
use crossterm::event::{self, Event as crossEvent, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize, palette::tailwind},
     text::{Line, Text}, widgets::{Block, BorderType, Cell, HighlightSpacing,
        Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState}
};
use unicode_width::UnicodeWidthStr;

const PALETTES: [tailwind::Palette; 2] = [
    tailwind::BLUE,
    tailwind::EMERALD,
];
#[derive(Debug, Default)]
struct TableColors {
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
    const fn new(color: &tailwind::Palette) -> Self {
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
const ITEM_HEIGHT: usize = 4;

//TODO: consider replacing this with a dynamic value calculated in the process_cell_content function
const CELL_WRAP_LIMIT: u16 = 16;
const HEADER_WRAP_LIMIT: u16 = 10;
#[derive(Debug, Default)]
pub struct App {
    table_state: TableState,
    data: Vec<Project>,
    exit: bool,
    longest_item_lens: [u16;12],
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

const INFO_TEXT: [&str; 2] = [
    "<Q> quit | <↑> move up | <↓> move down",
    "<Enter> select",
];
pub enum CurrentScreen {
    Main,       //main table
    Editing,    //when editing entry
    Exiting,    //confirmation of edit
}
fn max_width<F, T>(items: &[Project], field_fn: F) -> u16 where F: Fn(&Project) -> T, T: ToString,
{
    items
        .iter()
        .map(|x| field_fn(x).to_string())
        .map(|x| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0) as u16
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
impl App {

    //constructor
    pub fn new(data: Vec<Project>) -> App {
        App {
            table_state: TableState::default(),
            longest_item_lens: Self::get_constraints(&data),
            data: data,
            exit: false,
            scroll_state: ScrollbarState::default(),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0 }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal, headers: [&str;12]) -> io::Result<()> {
        while !self.exit {
            //main loop goes here
            terminal.draw(|frame| self.draw(frame,headers))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, headers: [&str;12]) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(frame.area());

        self.set_colors();

        self.render_table(frame, rects[0],headers);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            crossEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            KeyCode::Enter => self.select_entry(),
            _ => {}
        }
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect,headers: [&str;12]) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        //TODO: Figure out how to widen columns
        let header = headers
            .into_iter()
            .map(|content| Cell::from(Text::from(
                process_cell_content(String::from(content),HEADER_WRAP_LIMIT))))
            .collect::<Row>()
            .style(header_style)
            .height(3);
        let rows = self.data.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.as_str_array();
            //TODO: consider centering each column beyond the first, use slice?
            item.into_iter()
                .map(|content| Cell::from(Text::from(process_cell_content(content, CELL_WRAP_LIMIT))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                //these should be at minimum based on header length, except for the first one.
                Constraint::Length(CELL_WRAP_LIMIT+1),
                Constraint::Min(self.longest_item_lens[1] + 1),
                Constraint::Min(self.longest_item_lens[2]),
                Constraint::Min(self.longest_item_lens[3]),
                Constraint::Min(self.longest_item_lens[4]),
                Constraint::Min(self.longest_item_lens[5]),
                Constraint::Min(self.longest_item_lens[6]),
                Constraint::Min(self.longest_item_lens[7]),
                Constraint::Min(self.longest_item_lens[8]),
                Constraint::Min(self.longest_item_lens[9]),
                Constraint::Min(self.longest_item_lens[10]),
                Constraint::Min(self.longest_item_lens[11]),
            
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
        .bg(self.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.table_state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {       
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg)
                    .bg(self.colors.buffer_bg),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(self.colors.footer_border_color)),
            );
        frame.render_widget(info_footer, area);
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn move_up(&mut self){
        //wrap around to bottom
        let i = match self.table_state.selected() {
            Some(i) =>{
                if i ==  0 {
                    self.data.len() -1
                }
                else{
                    i - 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn move_down(&mut self){
        //wrap around to top  
        let i = match self.table_state.selected() {
            Some(i) =>{
                if i ==  (self.data.len()-1) {
                    0
                }
                else{
                    i + 1
                }}
            None => 0,
        };

        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
        }

    fn select_entry(&mut self){
        //TODO: implement properly later
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }

    fn get_constraints(items: &Vec<Project>) -> [u16;12] {
        [
            max_width(items, |x| x.project_name().to_string()),
            max_width(items, |x| x.size().to_string()),
            max_width(items, |x| x.cost().to_string()),
            max_width(items, |x| x.whole_army().to_string()),
            max_width(items, |x| x.needs_assembly().to_string()),
            max_width(items, |x| x.kitbash_rating().to_string()),
            max_width(items, |x| x.paint_level().to_string()),
            max_width(items, |x| x.complexity_rating().to_string()),
            max_width(items, |x| x.preference_modifier().to_string()),
            max_width(items, |x| x.priority().to_string()),
            max_width(items, |x| x.status().to_string()),
            max_width(items, |x| x.is_owned().to_string()),
        ]

    
}
}

#[cfg(test)]

mod tests {
    use super::*;
    use project::project::{Cost,PaintLevel};
    #[test]
    fn up_test() {
        let mut test_projects = vec![];

        for i in 0..29 {
            test_projects.push(Project::new(String::from("Dangle No. ".to_owned() + &i.to_string() ), 1,Cost::None,true,false,4,PaintLevel::Character,0.01f64,1.0f64,1.0f64,false,true));
        }
        let test_len = &test_projects.len()-1;
        let mut test_app = App::new(test_projects);
        for i in 2..0 {
            test_app.move_up();
            if i != 0{
                assert_eq!(i -1, test_app.table_state.selected().unwrap())
            }
            else {
                assert_eq!(test_len, test_app.table_state.selected().unwrap())
            }
        }
    }
    #[test]
    fn down_test() {
        let mut test_projects = vec![];

        for i in 0..29 {
test_projects.push(Project::new(String::from("Dangle No. ".to_owned() + &i.to_string() ), 1,Cost::None,true,false,4,PaintLevel::Character,0.01f64,1.0f64,1.0f64,false,true));
        }
        let test_len = &test_projects.len()-1;
        let mut test_app = App::new(test_projects);
        for i in 2..0 {
            test_app.move_up();
            if i != test_len{
                assert_eq!(i+1, test_app.table_state.selected().unwrap())
            }
            else {
                assert_eq!(0, test_app.table_state.selected().unwrap())
            }
        }
    }
    #[test]
    fn line_wrap_test(){
        let line_content = String::from("Lord-Veritant on Gryph-Stalker");

        let cell_text = process_cell_content(line_content,CELL_WRAP_LIMIT);
        for line in &cell_text.lines {
            println!("{}", line.to_string());
        }
        assert_eq!(cell_text.lines.len(),2);
        assert_eq!(cell_text.lines[0],Line::from("Lord-Veritant on"));
        assert_eq!(cell_text.lines[1],Line::from("Gryph-Stalker"));
    }
}
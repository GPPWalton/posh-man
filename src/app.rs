
use std::io;

use project::project::{Cost,Project,PaintLevel};
use crossterm::event::{self, Event as crossEvent, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Constraint,
    layout::{Layout, Rect,Margin},
    style::{Style, Stylize, Modifier,palette::tailwind, Color},
    text::{Text},
    widgets::{Block, Paragraph,BorderType, Table,
        TableState, Row, Cell,
        ScrollbarState, HighlightSpacing,
        Scrollbar,ScrollbarOrientation,},
    DefaultTerminal, Frame,
};

use unicode_width::UnicodeWidthStr;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
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
#[derive(Debug, Default)]
pub struct App {
    table_state: TableState,
    row_pos: usize,
    data: Vec<Project>,
    exit: bool,
    longest_item_lens: (u16, u16, u16,u16, u16, u16,u16, u16, u16),
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
impl App {

    //constructor
    pub fn new(data: Vec<Project>) -> App {
        App {
            table_state: TableState::default(),
            row_pos: 0,
            longest_item_lens: Self::constraint_len_calculator(&data),
            data: data,
            exit: false,
            scroll_state: ScrollbarState::default(),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0 }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            //main loop goes here
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(frame.area());

        self.set_colors();

        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
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

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
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

        let header = ["Project","Size","Cost","Whole Army/Warband",
    "Assembly Required","Kitbash Rating","Painting Level","Complexity Rating",
    "Preference Modifier","Priority","Status","Is Owned"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.data.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.as_str_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.2),
                Constraint::Min(self.longest_item_lens.3),
                Constraint::Min(self.longest_item_lens.4),
                Constraint::Min(self.longest_item_lens.5),
                Constraint::Min(self.longest_item_lens.6),
                Constraint::Min(self.longest_item_lens.7),
                Constraint::Min(self.longest_item_lens.8),
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

        println!("index is upward {}",&i);
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn move_down(&mut self){
        //wrap around to top  
        //TODO: Header seems to be overwriten by next row, probably okat but could be resol
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

        println!("index is downward {}",&i);
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

    fn constraint_len_calculator(items: &Vec<Project>) -> (u16, u16, u16,u16, u16, u16,u16, u16, u16) {
    //TODO: simplify this as each one is the same.
    let project_name_len = items
        .iter()
        .map(|x| x.project_name().to_string())
        .map(|x| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0);
    let size_len = items
        .iter()
        .map(|x| x.size().to_string())
        .map(|x: String| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0);
    let cost_len = items
        .iter()
        .map(|x| x.cost().to_string())
        .map(|x: String| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0);
    let whole_army_len = items
        .iter()
        .map(|x| x.whole_army().to_string())
        .map(|x: String| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0);
    let needs_assembly_len = items
        .iter()
        .map(|x| x.needs_assembly().to_string())
        .map(|x: String| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0);
    let kitbash_rating_len = items
        .iter()
        .map(|x| x.kitbash_rating().to_string())
        .map(|x: String| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0);
    let paint_level_len = items
        .iter()
        .map(|x| x.paint_level().to_string())
        .map(|x: String| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0);
    let priority_len= items
        .iter()
        .map(|x| x.priority().to_string())
        .map(|x: String| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0);
    let status_len = items
        .iter()
        .map(|x| x.status().to_string())
        .map(|x: String| UnicodeWidthStr::width(x.as_str()))
        .max()
        .unwrap_or(0);

    (project_name_len as u16, size_len as u16, cost_len as u16,
    whole_army_len as u16, needs_assembly_len as u16, kitbash_rating_len as u16,
    paint_level_len as u16, priority_len as u16, status_len as u16)
}
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn up_test() {
        let mut test_projects = vec![];

        for i in 0..29 {
            test_projects.push(Project::new(String::from("Dangle No. ".to_owned() + &i.to_string() ), 1,Cost::None,true,false,4,PaintLevel::Character,1.0f64,false));
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
            test_projects.push(Project::new(String::from("Dangle No. ".to_owned() + &i.to_string() ), 1,Cost::None,true,false,4,PaintLevel::Character,1.0f64,false));
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
}
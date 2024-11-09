use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    Frame,
};
use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/main/ratatui/examples

    // TODO: Split the layout
    // let [area1, area2, area3 ...] =

    // TODO: get the list of cities
    // let cities: Vec<ListItem> =
    // let list_component =

    // TODO: render the list of cities
    // frame.render_widget(list_component, area);


    // TODO: Create the weather info component
    // let weather_info =

    // TODO: Render the weather info component
    // frame.render_widget(weather_info, area);
    // let title = Line::from(" Counter App Tutorial ".bold());
    // let instructions = Line::from(vec![
    //     " Decrement ".into(),
    //     "<Down/Left>".blue().bold(),
    //     " Increment ".into(),
    //     "<Up/Right>".blue().bold(),
    //     " Quit ".into(),
    //     "<q> ".blue().bold(),
    // ]);
    // let block = Block::default()
    //     .title(title)
    //     .border_set(border::THICK);

    // let area = Rect::new(0, 0, 50, 5);
    // frame.render_widget(block, area);
    let greeting = Paragraph::new("Weather App")
                .white()
                .on_blue();
    frame.render_widget(greeting, frame.size());
}

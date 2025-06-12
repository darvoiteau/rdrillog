use std::io::{self, stdout};
use std::time::Duration;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    event::{self, Event, KeyCode},
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{BarChart, Block, Borders, Paragraph, Sparkline},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    text::{Span, Line},
};


enum View {
    Bar,
    Trend,
    Gauge,
}

struct TerminalCleanup;
impl Drop for TerminalCleanup{
    fn drop(&mut self){
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
    }
}

struct Bars{
    title: &'static str,
    border: ratatui::widgets::Borders,
    data: Vec<(String, u64)>,
    bwidth: u16,
    bcolor: ratatui::style::Color,
    vfgcolor: ratatui::style::Color,
    vbgcolor: ratatui::style::Color,
}

impl Bars{
    fn chart_create(&self) -> BarChart<'_>{
         let data_str: Vec<(&str, u64)> = self.data.iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();

        let chart = BarChart::default()
            .block(Block::default().title(self.title).borders(self.border))
            .data(&data_str) // ← Its already the good data type
            .bar_width(self.bwidth)
            .bar_style(Style::default().fg(self.bcolor))
            .value_style(Style::default().fg(self.vfgcolor).bg(self.vbgcolor));

        chart
    }
}

struct Gauges{
    title: &'static str,
    border: ratatui::widgets::Borders,
    percent: u16,
    gwidth: u16,
    bcolor: ratatui::style::Color,
    vfgcolor: ratatui::style::Color,
    vbgcolor: ratatui::style::Color,
}

impl Gauges{
    fn create_chart(&self) -> BarChart<'_>{
        let data = vec![("Pattern", self.percent as u64)];
        let chart = BarChart::default()
            .block(Block::default().title(self.title).borders(self.border))
            .data(&data) 
            .max(100)
            .bar_width(self.gwidth)
            .bar_style(Style::default().fg(self.bcolor))
            .value_style(Style::default().fg(self.vfgcolor).bg(self.vbgcolor));


        chart
    }
}

struct Sparklines{
    title: &'static str,
    border: ratatui::widgets::Borders,
    data: Vec<u64>,
    bcolor: ratatui::style::Color,
}

impl Sparklines{

    fn create_chart(&self) -> Sparkline{
        let data: &[u64] = &self.data;
        let chart = Sparkline::default()
            .block(Block::default().title(self.title).borders(self.border))
            .data(data)
            .style(Style::default().fg(self.bcolor));

        chart
    }
}

#[allow(deprecated)]
pub fn graph_display(data_bchart: Vec<(String, u64)>, data_schart: Vec<u64>, data_gchart: u16) -> Result <(), io::Error>{
    // Enables terminal raw mode to capture keyboard input
    enable_raw_mode()?;
    let mut stdout = stdout();
     // Switches to an alternative screen (full screen, without affecting the main terminal)
    execute!(stdout, EnterAlternateScreen)?;
    let _cleanup = TerminalCleanup;
    // Initializes the terminal with the Crossterm backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Sets default view as bar chart
    let mut current_view = View::Bar;

    // Sorts data by descending value to display the 5 largest
    let mut bars = data_bchart.clone();
    bars.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending by value
    let bars: Vec<(String, u64)> = bars.into_iter().take(5).collect();

    loop {
    // Build interface content according to the active view
    terminal.draw(|f| {
        let size = f.size();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                    Constraint::Percentage(90),
                    Constraint::Percentage(10),
            ])
            .split(size);
        
    match current_view{
        // Create Bars chart Object
        View::Bar => {
            let graph_bar = Bars{
                title: "Bars Graph: Display top founds matches by number of occurences with the given pattern",
                border: Borders::ALL,
                data: bars.clone(),
                bwidth: 20,
                bcolor: Color::Green,
                vfgcolor: Color::Black,
                vbgcolor: Color::Green,
            };

           // Create Bars chart
           let bars_chart = graph_bar.chart_create();

           // Display the Bars chart
            f.render_widget(bars_chart, layout[0]);
        
        
        }
        // Create Sparkline chart Object
        View::Trend => {
            let graph_sprkline = Sparklines{
                title: "Trend graph: Show founds matches with the given pattern in the time. From the begin to the end of the given logfile",
                border: Borders::ALL,
                data: data_schart.clone(),
                bcolor: Color::Green,
            };

            // Create Sparline chart
            let sprkl_chart = graph_sprkline.create_chart();

            // Display the sparkline chart
            f.render_widget(sprkl_chart, layout[0]);
            
        }

        View::Gauge => {
            // Create the Gauge chart Object
            let graph_gauge = Gauges{
                title:"Gauge Graph: Gives the percentage of log lines that match the pattern compared with the total number of log lines.",
                border: Borders::ALL,
                percent: data_gchart,
                gwidth: 100,
                bcolor: Color::Green,
                vfgcolor: Color::Black,
                vbgcolor: Color::Green,
            };

            // Create the Gauge chart
            let gauge_chart = graph_gauge.create_chart();

            // Display the Gauge chart
            f.render_widget(gauge_chart, layout[0]);

        }
    }
     let help = Paragraph::new(Line::from(Span::raw("q: quit    b: bars chart view  t: trend view   g: gauge view")))
            .block(Block::default().borders(Borders::ALL).title("Options"));
        f.render_widget(help, layout[1]);
        
    })?;
    // Keyboard input management (events)
    if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('t') => current_view = View::Trend,
                    KeyCode::Char('b') => current_view = View::Bar,
                    KeyCode::Char('g') => current_view = View::Gauge,
                        _ => {}
                }
            }
        }
}
     // Restores normal terminal state at end
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
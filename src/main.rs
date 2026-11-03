use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint};
use ratatui::text::{Line};
use std::time::{Duration, Instant};
use ratatui::style::{Modifier, Style, Color};

const REG_MESSAGES: [&str; 6] = [
    "PILOT EJECTING", "EJECT NOW EJECT", "CORE BREAK IN PROGRESS",
    "DESTRUCT SEQUENCE ENGAGED", "EJECT-SYSTEM ENGAGED" ,"PROTOCOL [03]"
];
const RARE_MESSAGES: [&str; 38] = [
    "It was fun", 
    "Ain't over till it's over", "Until next time, then",
    "Second star to the right", "Go get 'em tiger", "Make it so",
    "See you on the other side", "Wait for the wheel",
    "Do not throw your shot", "Drop and burn 'em up",
    "Exhibit no restraint", "Rip and saw",
    "Code zero zero zero. Destruct. Zero.",
    "Authorization Alpha-Alpha 3-0-5", 
    "Have just the greatest day", "Avenge me",
    "Stay safe", "Situation normal", "PROTOCOL <3",
    "Fly, you fool", "Never give up. Never surrender", "In case of doubt, attack",
    "You're never beaten until you admit it",
    "Wrong us, shall we not revenge?", "Till all are one",
    "Take care pilot", "We had a good run",
    "Nothin' but the rain", "Sometimes, you have to roll a hard six",
    "And sometimes, when you fall, you fly", "See you space cowboy...",
    "Just another day at the office", "End of line", "Fortune favors the bold",
    "A leaf on the wind", "You are who you choose to be", "Always",
    "Finish the fight"
];

// reg 85%, rare 15%

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let time_limit = Duration::from_secs(1);
    let start_time = Instant::now();
    terminal.draw(render)?;
    loop { 
        let elapsed = start_time.elapsed();
        if elapsed >= time_limit {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let message: &str;
    if rand::random_bool(0.85) {
        let ridx = rand::random_range(0..REG_MESSAGES.len());
        message = REG_MESSAGES[ridx];
    } else {
        let ridx = rand::random_range(0..RARE_MESSAGES.len());
        message = RARE_MESSAGES[ridx];
    }

    let text = Line::styled(
        message,
        Style::default()
            .fg(Color::Red)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD)
    ).centered();
    let area = frame.area().centered_vertically(
        Constraint::Length(1),
    );
    frame.render_widget(text, area);
}


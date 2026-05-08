//! Welcome screen content for onboarding.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::palette;

/// Hammerstein gold/brass accent — matches the `mode_agent` / `status_working`
/// color in `HAMMERSTEIN_UI_THEME`.
const GOLD: Color = Color::Rgb(201, 169, 64);

/// Portrait of Kurt Freiherr von Hammerstein-Equord, as rendered in
/// the main `hammerstein` CLI README.  Top ~20 lines so it fits most
/// terminal heights during onboarding.
const PORTRAIT: &[&str] = &[
    "====================++++++++++++++++++++++++++*+********+",
    "==================+%%%%%%%%%#++++++++++++++++++++++*+++++",
    "===============%%%%%%%%%%%%#%%###+#++++++++++++++++++++++",
    "============#%%%%%%%%%%%%%%#***=+++#*++++++++++++++++++++",
    "======++===%%%%%%%%%%%%%%#*##*+=:::::=+++++=+++++++++++++",
    "==========%%%%%%%%##**++==:-:::.......:=++++=++++++++++++",
    "=========%%%%%##%#####**=-::............+++++=+++++++++++",
    "=========%%%%%%%%%%%##**+=-::..:.........+++++==+++++++++",
    "========+%%%%%%%%%%###**++--::::.:::.....++++++++++++++++",
    "=========%%%%%%%%%%%#%##***--::..::......=+++++=+++++++++",
    "=========%%%%%%%%%%%%##+==++*--:-:-::...:=++++=++++=+=+++",
    "=========%%%%%%%%%%%%%%###*++***+--=***+===++=+++++++++++",
    "=========+%%%%%%%%%%%%%%%%:#+%%%*:.+#@+.#===+++++++=+++++",
    "========%%%%%%%%%%%%%%@%%%.--#%%%=.:-=-:.-+++++++=+++++++",
    "========%%%%%%%%%%%%%%%%%#+-=%%%%*.:.-:.::++=+++++==+++++",
    "=======-%%%%%%%%%%%%#+=***+*#%%%%#-.::::::++++++++++=++++",
    "========+%%%%%%%%%%%%%******#%@%%%+:::::::=++++++++++++++",
    "=========#%%%%%%%%%%%%%%%##*#@%%%%*=::::::=++++++++++++++",
    "++++======#%%%%%%%%%%%%%%#**%%%%%+-::::::=+++++++++++++++",
    "++++++=+====@%%%%%%%%%%%%*%%*###+-:::-:::++=++=++++++++++",
];

pub fn lines() -> Vec<Line<'static>> {
    let mut out = Vec::new();

    // Portrait block — gold accent
    for line in PORTRAIT {
        out.push(Line::from(Span::styled(
            *line,
            Style::default().fg(GOLD),
        )));
    }
    out.push(Line::from(""));

    // Title
    out.push(Line::from(Span::styled(
        "Hammerstein TUI",
        Style::default()
            .fg(GOLD)
            .add_modifier(Modifier::BOLD),
    )));
    out.push(Line::from(Span::styled(
        format!("Version {}", env!("CARGO_PKG_VERSION")),
        Style::default().fg(palette::TEXT_MUTED),
    )));
    out.push(Line::from(""));
    out.push(Line::from(Span::styled(
        "A focused terminal workspace for longer model sessions.",
        Style::default().fg(palette::TEXT_PRIMARY),
    )));
    out.push(Line::from(Span::styled(
        "You'll add an API key, review trust for this directory, and then land in the chat.",
        Style::default().fg(palette::TEXT_MUTED),
    )));
    out.push(Line::from(Span::styled(
        "The main composer is multi-line, so you can write full prompts instead of squeezing everything into one line.",
        Style::default().fg(palette::TEXT_MUTED),
    )));
    out.push(Line::from(""));
    out.push(Line::from(Span::styled(
        "Press Enter to continue.",
        Style::default().fg(palette::TEXT_PRIMARY),
    )));
    out.push(Line::from(Span::styled(
        "Ctrl+C exits at any point.",
        Style::default().fg(palette::TEXT_MUTED),
    )));

    out
}

/************************************************************************************************/

pub enum Text {
    CliBuildAbout,
    CliVerboseHelp,
    CliCleanAbout,
}

/************************************************************************************************/

pub fn s(text: Text) -> &'static str {
    match text {
        Text::CliBuildAbout => "Builds the site.",
        Text::CliVerboseHelp => "Use verbose output.",
        Text::CliCleanAbout => "Cleans up previously generated site.",
    }
}

/************************************************************************************************/

/************************************************************************************************/

pub enum Text {
    /*------------------------------------------------------------------------------------------*/
    CliBuildAbout,
    CliCleanAbout,
    CliVerboseHelp,
    /*------------------------------------------------------------------------------------------*/
    ErrorParseErrorFor,
    ErrorNoClass,
    ErrorNoForClass,
    ErrorNoTitle,
    ErrorNoDescription,
    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

pub fn s(text: Text) -> &'static str {
    match text {
        /*--------------------------------------------------------------------------------------*/
        Text::CliBuildAbout => "Builds the site.",
        Text::CliCleanAbout => "Cleans up previously generated site.",
        Text::CliVerboseHelp => "Use verbose output.",
        /*--------------------------------------------------------------------------------------*/
        Text::ErrorParseErrorFor => "Parse error for {1}.",
        Text::ErrorNoClass => "No class specified.",
        Text::ErrorNoForClass => "No for-class or invalid for-class specified.",
        Text::ErrorNoTitle => "No title specified.",
        Text::ErrorNoDescription => "No description specified.",
        /*--------------------------------------------------------------------------------------*/
    }
}

/************************************************************************************************/

pub fn so(text: Text) -> String {
    s(text).to_string()
}

/************************************************************************************************/

pub fn sr(text: Text, values: &[&str]) -> String {
    let mut msg = String::from(s(text));

    for (i, v) in values.iter().enumerate() {
        let mut place_holder = String::new();
        place_holder.push('{');
        place_holder.push_str(&(i + 1).to_string());
        place_holder.push('}');
        msg = msg.replace(&place_holder, v);
    }

    msg
}

/************************************************************************************************/

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
    ErrorSiteConfigShouldContainTitle,
    ErrorInputDirectoryNotExisting,
    ErrorInputIsNotDirectory,
    ErrorOutputDirectoryNotExisting,
    ErrorOutputIsNotDirectory,
    ErrorOutputIsNotEmpty,
    /*------------------------------------------------------------------------------------------*/
    VerboseBuilding,
    VerboseDone,
    VerboseReadingSiteConfig,
    VerboseBuildingFileList,
    VerboseProcessingFiles,
    VerboseProcessingPages,
    VerboseCopying,
    VerboseCompiling,
    VerboseCleaning,
    VerboseDeletingDirectory,
    VerboseCreatingOutputDirectory,
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
        Text::ErrorSiteConfigShouldContainTitle => "Site.yaml should contain a title field.",
        Text::ErrorInputDirectoryNotExisting => "Input directory '{1}' does not exist.",
        Text::ErrorInputIsNotDirectory => "Input '{1}' is not a directory.",
        Text::ErrorOutputDirectoryNotExisting => "Output directory '{1}' does not exist.",
        Text::ErrorOutputIsNotDirectory => "Output '{1}' is not a directory.",
        Text::ErrorOutputIsNotEmpty => "Output directory '{1}' is not empty.",
        /*--------------------------------------------------------------------------------------*/
        Text::VerboseBuilding => "Building...",
        Text::VerboseDone => "Done!",
        Text::VerboseReadingSiteConfig => "Reading site configuration from Site.yaml.",
        Text::VerboseBuildingFileList => "Building file list.",
        Text::VerboseProcessingFiles => "Processing files.",
        Text::VerboseProcessingPages => "Processing pages.",
        Text::VerboseCopying => "Copying {1}.",
        Text::VerboseCompiling => "Compiling {1}.",
        Text::VerboseCleaning => "Cleaning...",
        Text::VerboseDeletingDirectory => "Deleting directory {1}.",
        Text::VerboseCreatingOutputDirectory => "Creating output directory {1}.",
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

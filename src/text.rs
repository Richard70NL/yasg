/************************************************************************************************/

pub enum Text {
    /*------------------------------------------------------------------------------------------*/
    CliBuildAbout,
    CliCleanAbout,
    CliVerboseHelp,
    /*------------------------------------------------------------------------------------------*/
    ErrorYasgExit,
    ErrorParseErrorFor,
    ErrorInputDirectoryNotExisting,
    ErrorInputIsNotDirectory,
    ErrorOutputDirectoryNotExisting,
    ErrorOutputIsNotDirectory,
    ErrorOutputIsNotEmpty,
    ErrorWriteLongHelp,
    ErrorWhileReadingSiteYaml,
    ErrorNoValidValueField,
    ErrorValidatingSiteYaml,
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
        Text::ErrorYasgExit => "YASG exits with the following error(s):",
        Text::ErrorParseErrorFor => "Parse error for {1}.",
        Text::ErrorInputDirectoryNotExisting => "Input directory '{1}' does not exist.",
        Text::ErrorInputIsNotDirectory => "Input '{1}' is not a directory.",
        Text::ErrorOutputDirectoryNotExisting => "Output directory '{1}' does not exist.",
        Text::ErrorOutputIsNotDirectory => "Output '{1}' is not a directory.",
        Text::ErrorOutputIsNotEmpty => "Output directory '{1}' is not empty.",
        Text::ErrorWriteLongHelp => "An error occured while writing the help information.",
        Text::ErrorWhileReadingSiteYaml => "An error occured while reading Site.yaml.",
        Text::ErrorNoValidValueField => "No valid value has been provided for the '{1}' field.",
        Text::ErrorValidatingSiteYaml => "Site.yaml contains invalid information.",
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

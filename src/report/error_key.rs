use strum_macros::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, Hash)]
#[strum(serialize_all = "kebab-case")]
pub enum ErrorKey {
    Config,
    ReadError,
    ParseError,
    BracePlacement,
    LocalValues,
    Packaging,
    Validation,
    Filename,
    Encoding,
    Localization,
    Markup,
    DuplicateItem,
    ExactDuplicateItem,
    DuplicateField,
    NameConflict,
    EventNamespace,
    MissingLocalization,
    MissingFile,
    MissingSound,
    MissingItem,
    MissingPerspective,
    WrongGender,
    Conflict,
    ImageFormat,
    Unneeded,
    Scopes,
    /// This seems like it solves the same problem that `Confidence` solves.
    StrictScopes,
    Crash,
    Range,
    Tooltip,
    IfElse,
    Rivers,
    Modifiers,
    Macro,
    History,
    Logic,
    Bugs,
    Datafunctions,
    Removed,
    FieldMissing,
    UnknownField,
    TitleTier,
    Colors,
    UnusedLocalization,
    UnusedFile,
    UnknownList,
    Choice,
    UseOfThis,
    CharacterId,
    Loop,

    PrincesOfDarkness,

    Internal,
}

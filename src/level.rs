//! [`Level`] constants for easy importing

use crate::renderer::stylesheet::Stylesheet;
use crate::snippet::{ERROR_TXT, HELP_TXT, INFO_TXT, NOTE_TXT, WARNING_TXT};
use crate::{Element, Group, Message, Title};
use anstyle::Style;

/// Default `error:` [`Level`]
pub const ERROR: Level<'_> = Level {
    name: None,
    level: LevelInner::Error,
};

/// Default `warning:` [`Level`]
pub const WARNING: Level<'_> = Level {
    name: None,
    level: LevelInner::Warning,
};

/// Default `info:` [`Level`]
pub const INFO: Level<'_> = Level {
    name: None,
    level: LevelInner::Info,
};

/// Default `note:` [`Level`]
pub const NOTE: Level<'_> = Level {
    name: None,
    level: LevelInner::Note,
};

/// Default `help:` [`Level`]
pub const HELP: Level<'_> = Level {
    name: None,
    level: LevelInner::Help,
};

/// [`Message`] or [`Title`] severity level
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Level<'a> {
    pub(crate) name: Option<Option<&'a str>>,
    pub(crate) level: LevelInner,
}

impl<'a> Level<'a> {
    pub const ERROR: Level<'a> = ERROR;
    pub const WARNING: Level<'a> = WARNING;
    pub const INFO: Level<'a> = INFO;
    pub const NOTE: Level<'a> = NOTE;
    pub const HELP: Level<'a> = HELP;

    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn text(self, text: Option<&'a str>) -> Level<'a> {
        Level {
            name: Some(text),
            level: self.level,
        }
    }
}

impl<'a> Level<'a> {
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn header(self, header: &'a str) -> Message<'a> {
        Message {
            id: None,
            groups: vec![Group::new().element(Element::Title(Title {
                level: self,
                title: header,
            }))],
        }
    }

    /// <div class="warning">
    ///
    /// Text passed to this function is allowed to be pre-styled, as such all
    /// text is considered "trusted input" and has no normalizations applied to
    /// it. [`normalize_untrusted_str`](crate::normalize_untrusted_str) can be
    /// used to normalize untrusted text before it is passed to this function.
    ///
    /// </div>
    pub fn title(self, title: &'a str) -> Title<'a> {
        Title { level: self, title }
    }

    pub(crate) fn as_str(&self) -> &'a str {
        match (self.name, self.level) {
            (Some(Some(name)), _) => name,
            (Some(None), _) => "",
            (None, LevelInner::Error) => ERROR_TXT,
            (None, LevelInner::Warning) => WARNING_TXT,
            (None, LevelInner::Info) => INFO_TXT,
            (None, LevelInner::Note) => NOTE_TXT,
            (None, LevelInner::Help) => HELP_TXT,
        }
    }

    pub(crate) fn style(&self, stylesheet: &Stylesheet) -> Style {
        self.level.style(stylesheet)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LevelInner {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

impl LevelInner {
    pub(crate) fn style(self, stylesheet: &Stylesheet) -> Style {
        match self {
            LevelInner::Error => stylesheet.error,
            LevelInner::Warning => stylesheet.warning,
            LevelInner::Info => stylesheet.info,
            LevelInner::Note => stylesheet.note,
            LevelInner::Help => stylesheet.help,
        }
    }
}

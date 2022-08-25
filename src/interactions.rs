use fnv::FnvHashMap;
use std::path::{Path, PathBuf};

use crate::block::{Block, DefinitionItem, Token};
use crate::desc::verify_desc_locas;
use crate::errorkey::ErrorKey;
use crate::errors::{error, error_info, info, will_log, LogPauseRaii};
use crate::everything::FileHandler;
use crate::fileset::{FileEntry, FileKind};
use crate::localization::Localization;
use crate::pdxfile::PdxFile;

#[derive(Clone, Debug, Default)]
pub struct Interactions {
    interactions: FnvHashMap<String, Interaction>,
}

impl Interactions {
    pub fn load_interaction(&mut self, key: Token, block: &Block, values: Vec<(Token, Token)>) {
        if let Some(other) = self.interactions.get(key.as_str()) {
            if other.key.loc.kind == key.loc.kind && will_log(&key, ErrorKey::Duplicate) {
                error(
                    &key,
                    ErrorKey::Duplicate,
                    "interaction redefines an existing interaction",
                );
                info(
                    &other.key,
                    ErrorKey::Duplicate,
                    "the other interaction is here",
                );
            }
        }
        self.interactions.insert(
            key.to_string(),
            Interaction::new(key, block.clone(), values),
        );
    }

    pub fn check_have_localizations(&self, locas: &Localization) {
        for interaction in self.interactions.values() {
            interaction.check_have_localizations(locas);
        }
    }
}

impl FileHandler for Interactions {
    fn subpath(&self) -> PathBuf {
        PathBuf::from("common/character_interactions")
    }

    fn config(&mut self, _config: &Block) {}

    fn handle_file(&mut self, entry: &FileEntry, fullpath: &Path) {
        if !entry.filename().to_string_lossy().ends_with(".txt") {
            return;
        }

        let _pause = LogPauseRaii::new(entry.kind() != FileKind::ModFile);

        let block = match PdxFile::read(entry.path(), entry.kind(), fullpath) {
            Ok(block) => block,
            Err(e) => {
                error_info(
                    entry,
                    ErrorKey::ReadError,
                    "could not read file",
                    &format!("{:#}", e),
                );
                return;
            }
        };

        let mut values: Vec<(Token, Token)> = Vec::new();

        for def in block.iter_definitions_warn() {
            match def {
                DefinitionItem::Keyword(key) => error_info(
                    key,
                    ErrorKey::Validation,
                    "unexpected token",
                    "Did you forget an = ?",
                ),
                DefinitionItem::Assignment(key, value) => {
                    if key.as_str().starts_with('@') {
                        values.push((key.clone(), value.clone()));
                    } else {
                        error(
                            key,
                            ErrorKey::Validation,
                            "unknown setting in interaction file",
                        );
                    }
                }
                DefinitionItem::Definition(key, b) => {
                    self.load_interaction(key.clone(), b, values.clone());
                }
            }
        }
    }

    fn finalize(&mut self) {}
}

#[derive(Clone, Debug)]
pub struct Interaction {
    key: Token,
    values: Vec<(Token, Token)>,
    block: Block,
}

impl Interaction {
    pub fn new(key: Token, block: Block, values: Vec<(Token, Token)>) -> Self {
        Interaction { key, values, block }
    }

    pub fn check_have_localizations(&self, locas: &Localization) {
        // TODO: The ai_ name check is a heuristic. It would be better to check if the
        // is_shown trigger requires scope:actor to be is_ai = yes. But that's a long way off.
        if !self.key.as_str().starts_with("ai_") {
            locas.verify_have_key(self.key.as_str(), &self.key, "interaction");
        }
        if self.block.get_field_value("extra_icon").is_some()
            && self
                .block
                .get_field_block("should_use_extra_icon")
                .is_some()
        {
            locas.verify_have_key(
                &format!("{}_extra_icon", self.key),
                self.block.key_token("extra_icon").unwrap(),
                "interaction extra_icon localization",
            );
        }
        if let Some(desc) = self.block.get_field("desc") {
            verify_desc_locas(desc, locas, "interaction description");
        }
        if let Some(desc) = self.block.get_field("prompt") {
            verify_desc_locas(desc, locas, "interaction prompt");
        }
        if let Some(desc) = self.block.get_field("notification_text") {
            verify_desc_locas(desc, locas, "interaction notification");
        }
        if let Some(desc) = self.block.get_field("on_decline_summary") {
            verify_desc_locas(desc, locas, "interaction on_decline");
        }
        if let Some(key) = self.block.get_field_value("answer_accept_key") {
            locas.verify_have_key(key.as_str(), &key, "interaction acceptance");
        }
        if let Some(key) = self.block.get_field_value("answer_reject_key") {
            locas.verify_have_key(key.as_str(), &key, "interaction rejection");
        }
        if let Some(key) = self.block.get_field_value("highlighted_reason") {
            locas.verify_have_key(key.as_str(), &key, "interaction reason");
        }
        if let Some(key) = self.block.get_field_value("options_heading") {
            locas.verify_have_key(key.as_str(), &key, "interaction options");
        }
        if let Some(key) = self.block.get_field_value("pre_answer_maybe_breakdown_key") {
            locas.verify_have_key(key.as_str(), &key, "interaction");
        }
        if let Some(key) = self.block.get_field_value("pre_answer_maybe_key") {
            locas.verify_have_key(key.as_str(), &key, "interaction");
        }
        if let Some(key) = self.block.get_field_value("pre_answer_no_key") {
            locas.verify_have_key(key.as_str(), &key, "interaction");
        }
        if let Some(key) = self.block.get_field_value("pre_answer_yes_key") {
            locas.verify_have_key(key.as_str(), &key, "interaction");
        }
        if let Some(key) = self.block.get_field_value("send_name") {
            locas.verify_have_key(key.as_str(), &key, "interaction name");
        }
        for b in self.block.get_field_blocks("send_option") {
            if let Some(key) = self.block.get_field_value("localization") {
                locas.verify_have_key(key.as_str(), &key, "interaction option");
            }
        }
    }
}

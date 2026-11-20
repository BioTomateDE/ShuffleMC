//! Structs to represent the JSON `pack.mcmeta` file.
//!
//! Information taken from <https://minecraft.wiki/w/Pack.mcmeta>

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

/// A `pack.mcmeta` file.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackMeta {
    /// Holds the pack information.
    pub pack: Pack,

    /// too lazy to implement this
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<serde_json::Value>,

    /// too lazy to implement this
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<serde_json::Value>,

    /// too lazy to implement this
    #[serde(skip_serializing_if = "Option::is_none")]
    overlay: Option<serde_json::Value>,

    /// Contains additional languages to add to the language menu.
    ///
    /// Map keys are IETF language codes (all lowercase, underscore instead of dash), i.e. `en_us`.
    /// They correspond to a JSON file with the same name in the directory `assets/<namespace>/lang`.
    ///
    /// Only present in resource packs. Optional.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub language: HashMap<String, Language>,
}

/// Holds the pack information.
///
/// This is the most relevant struct.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pack {
    /// A text component that appears when hovering over the pack's name in the
    /// list given by the `/datapack list` command, or when viewing the pack in the *Create World* screen.
    ///
    /// NOTE: The wiki specifies that this can also be an object or a list???
    pub description: String,

    /// The main format version of this pack.
    ///
    /// Deprecated since resource pack format v65 / data pack format v82.
    /// However, this value should still be added to support older game versions (if desirable).
    pub pack_format: i32,

    /// The minimum supported format version.
    pub min_format: FormatVersion,

    /// The maximum supported format version.
    pub max_format: FormatVersion,

    /// too lazy to implement this garbage
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_formats: Option<serde_json::Value>,
}

/// An additional language to add to the language menu.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Language {
    /// The full name of the language.
    pub name: String,

    /// The country or region name.
    pub region: String,

    /// Whether the language reads right to left.
    pub bidirectional: bool,
}

/// A format version.
///
/// This struct does not correspond to a JSON object:
/// It can either be a single integer or a list of two integers,
/// where the first int is the pack (major) version and the second int is the minor version.
/// Specifying a single integer is interpreted as that major version.
/// For example, `82` or `[82]` is equivalent to `[82, 0]`.
#[derive(Debug, Clone)]
pub struct FormatVersion {
    pub major: i32,
    pub minor: i32,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum FormatVersionRepr {
    Bare(i32),
    Pair(i32, i32),
}

impl<'de> Deserialize<'de> for FormatVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match FormatVersionRepr::deserialize(deserializer)? {
            FormatVersionRepr::Bare(major) => FormatVersion { major, minor: 0 },
            FormatVersionRepr::Pair(major, minor) => FormatVersion { major, minor },
        })
    }
}

impl Serialize for FormatVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.minor == 0 {
            serializer.serialize_i32(self.major)
        } else {
            (self.major, self.minor).serialize(serializer)
        }
    }
}

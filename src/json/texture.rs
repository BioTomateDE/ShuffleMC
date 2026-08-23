//! Structs to represent the JSON `*.png.mcmeta` files.
//!
//! Information taken from <https://minecraft.wiki/w/Resource_pack#Texture_animation>

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A texture animation `*.png.mcmeta` file.
///
/// These files are found in `assets/textures/block` and more.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextureProperties {
    /// Animation properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    /// Rendering properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<Texture>,
}

/// Texture rendering properties.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Texture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mipmap_strategy: Option<MipmapStrategy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_cutoff_bias: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MipmapStrategy {
    DarkCutout,
    StrictCutout,
    Mean,
}

/// Texture animation properties.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Animation {
    /// Whether to generate additional frames between frames if
    /// the frametime is greater than 1 between them.
    ///
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<bool>,

    /// The width of a single frame in pixels.
    ///
    /// Defaults to the image's height if it's defined, or the smaller of the image's dimensions if it isn't.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,

    /// The height of a single frame in pixels.
    ///
    /// Defaults to the image's width if it's defined, or the smaller of the image's dimensions if it isn't.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,

    /// The default time for each frame in increments of one game tick. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frametime: Option<i32>,

    /// Contains a list of frames.
    ///
    /// Defaults to displaying all the frames from first to last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<Vec<Frame>>,
}

/// A texture animation frame.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub index: i32,
    pub time: Option<i32>,
}

impl Serialize for Frame {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.time {
            Some(time) => {
                use serde::ser::SerializeStruct;
                let mut s = serializer.serialize_struct("Frame", 2)?;
                s.serialize_field("index", &self.index)?;
                s.serialize_field("time", &time)?;
                s.end()
            }
            None => serializer.serialize_i32(self.index),
        }
    }
}

impl<'de> Deserialize<'de> for Frame {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum FrameRepr {
            Simple(i32),
            Full { index: i32, time: i32 },
        }

        match FrameRepr::deserialize(deserializer)? {
            FrameRepr::Simple(index) => Ok(Frame { index, time: None }),
            FrameRepr::Full { index, time } => Ok(Frame { index, time: Some(time) }),
        }
    }
}

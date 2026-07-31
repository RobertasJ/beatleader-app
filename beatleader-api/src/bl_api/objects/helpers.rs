use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Pp(Option<f32>);

impl Pp {
    pub fn new(pp: Option<f32>) -> Self {
        Self(pp)
    }

    pub fn pp(&self) -> Option<f32> {
        self.0
    }
}

impl Serialize for Pp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.0.unwrap_or(0.0))
    }
}

impl<'de> Deserialize<'de> for Pp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;

        Ok(Self(match value {
            0.0 => None,
            x => Some(x),
        }))
    }
}

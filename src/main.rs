use bincode::{config, Decode, Encode};
use serde::{ser::SerializeStruct, Serialize};

static BINCODE_CONF: config::Configuration = config::standard();
#[derive(Copy, Hash, Debug, Clone, Eq, PartialEq, Encode, Decode, PartialOrd, Ord)]
struct Entity {
    x: i32,
    y: i32,
}

impl Serialize for Entity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Window", 2)?;

        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;

        state.end()
    }
}

#[derive(Hash, Debug, Clone, Eq, PartialEq, Encode, Decode, PartialOrd, Ord)]
struct World(Vec<Entity>);

impl World {
    pub fn to_bytes(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(self, BINCODE_CONF)
    }
    pub fn from_bytes(bs: &[u8]) -> Result<World, bincode::error::DecodeError> {
        let (record, len) = bincode::decode_from_slice(bs, BINCODE_CONF)?;

        if len != bs.len() {
            return Err(bincode::error::DecodeError::ArrayLengthMismatch {
                required: bs.len(),
                found: len,
            });
        }

        Ok(record)
    }
}

fn main() {
    let world = World(vec![Entity { x: 0, y: 4 }, Entity { x: 10, y: 20 }]);

    let encoded: Vec<u8> = world.to_bytes().unwrap();

    let decoded: World = World::from_bytes(&encoded[..]).unwrap();

    assert_eq!(world, decoded);
}

//! Module to deal with Scratch block

use crate::prelude::*;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use utils::{deserialize_json_str, serialize_json_str};

/// Scratch scripting block
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockNormal {
    /// A string naming the block.
    pub opcode: OpCode,

    /// Wiki says nothing about this, probably comment id that this block attached to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<Uid>,

    /// The Id of the next block or null.
    pub next: Option<Uid>,

    /// If the block is a stack block and is preceded, this is the Id of the preceding block.
    /// If the block is the first stack block in a C mouth, this is the Id of the C block.
    /// If the block is an input to another block, this is the Id of that other block.
    /// Otherwise it is none.
    pub parent: Option<Uid>,

    /// See [`BlockInput`]
    pub inputs: StringHashMap<BlockInput>,

    /// See [`BlockField`]
    pub fields: StringHashMap<BlockField>,

    /// True if this is a shadow block and false otherwise.
    pub shadow: bool,

    /// False if the block has a parent and true otherwise.
    pub top_level: bool,

    /// Mutations are present some blocks that has a certain opcode.
    /// See [`BlockMutationEnum`] for availiable mutations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation: Option<BlockMutation>,

    /// X Position of the top level block.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub x: Option<Number>,

    /// Y Position of the top level block.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub y: Option<Number>,
}

/// This is a reporter of list or variable when it's at the top.
#[derive(Debug, PartialEq, Clone, Deserialize_tuple, Serialize_tuple)]
pub struct BlockVarListReporterTop {
    pub kind: ListOrVariable,
    /// Name of the variable
    pub name: Name,
    /// Id of the variable
    pub id: Uid,
    /// Position X of the reporter
    pub x: Number,
    /// Position y of the reporter
    pub y: Number,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ListOrVariable {
    Variable,
    List,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Block {
    Normal(BlockNormal),
    VarList(BlockVarListReporterTop),
}

/// A struct representing inputs into which other blocks may be dropped, including C mouths.
/// idk if is this possible without vec
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockInput {
    /// See [`ShadowInputType`]
    pub shadow: ShadowInputType,

    /// Inputs
    pub inputs: Vec<Option<UidOrValue>>,
}

/// Used for [`BlockInput`]
/// When the input could be either [`Uid`] or [`BlockInputValue`]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UidOrValue {
    /// When it's [`Uid`]
    Uid(Uid),
    /// When it's [`BlockInputValue`]
    Value(BlockInputValue),
}

/// Field of the block
#[derive(Debug, Clone, PartialEq)]
pub enum BlockField {
    /// Field when Id are sometimes needed
    WithId {
        /// Value of the field
        value: Value,

        /// For certain fields,
        /// such as variable and broadcast dropdown menus,
        /// there is also a second element, which is the Id of the field's value.
        id: Option<Uid>,
    },
    /// Field with no Id needed
    NoId {
        /// Value of the field
        value: Value,
    },
}

/// Mutation for procedural block (custom block) or stop block
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockMutation {
    /// Always equal to "mutation".
    /// Don't know if there any other tag
    /// Wiki says there's only "mutation" though
    pub tag_name: String,

    /// Seems to always be an empty array.
    pub children: Vec<Json>,

    /// See [`BlockMutationEnum`]
    #[serde(flatten)]
    pub mutation_enum: BlockMutationEnum,
}

/// Different mutation has different properties.
/// This enum define them.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockMutationEnum {
    /// opcode is `"procedures_prototype"` mutations have the following additional properties
    ProceduresPrototype {
        /// The name of the custom block, including inputs: %s for string/number inputs and %b for boolean inputs.
        proccode: String,

        /// An array of the ids of the arguments; these can also be found in the input property of the main block.
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str"
        )]
        argumentids: Vec<Uid>,

        /// An array of the names of the arguments.
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str"
        )]
        argumentnames: Vec<Name>,

        /// An array of the defaults of the arguments.
        ///  - String default is an empty string
        ///  - bool default is `false`
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str"
        )]
        argumentdefaults: Vec<Value>,

        /// Whether to run the block without screen refresh or not.
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str"
        )]
        warp: Option<bool>,
    },

    /// opcode is `"procedures_call"` mutations have the following additional properties
    ProceduresCall {
        /// The name of the custom block, including inputs: %s for string/number inputs and %b for boolean inputs.
        proccode: String,

        /// An array of the ids of the arguments; these can also be found in the input property of the main block.
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str"
        )]
        argumentids: Vec<Uid>,

        /// Whether to run the block without screen refresh or not.
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str"
        )]
        warp: Option<bool>,
    },

    /// opcode is `"control_stop"` mutations have the following additional property
    ControlStop {
        /// Whether the block has a block following it or not
        ///  - false for stop all and stop all in sprite
        ///  - true for stop other scripts in sprite)
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str"
        )]
        hasnext: bool,
    },
}

/// Shadow enum for [`BlockInput`]
///
/// Shadow is area inside block input/arg/param or whatever you wanted to call it.
/// It's consisting of:
///  - raw input field where you just type stuff in and optionally can put a reporter in
///  - menu that you can choose but cannot put a reporter in
///  - menu that you can chose and optionally can put a reporter in
///  - or others I might not catch while developing this
///
/// This documentation might not be completed or is completed, idk.
/// Scratch wiki didn't tell anything about this.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ShadowInputType {
    /// There is a shadow
    Shadow = 1,

    /// There is no shadow
    #[default]
    NoShadow = 2,

    /// There is a shadow but obscured by the input.
    /// The shadow is obscured when reporter is inserted.
    ShadowObscured = 3,
}

/// Input of the BlockInput
#[derive(Debug, Clone, PartialEq)]
pub enum BlockInputValue {
    /// Number input
    Number {
        /// The value
        value: Value,
    },

    /// Postive number input
    PositiveNumber {
        /// The value
        value: Value,
    },

    /// Postive integer input
    PositiveInteger {
        /// The value
        value: Value,
    },

    /// Integer input
    Integer {
        /// The value
        value: Value,
    },

    /// Angle input
    Angle {
        /// The value
        value: Value,
    },

    /// Color input
    Color {
        /// Value, a `#` followed by a hexadecimal numeral representing the color
        value: Value,
    },

    /// String input
    String {
        /// The value
        value: Value,
    },

    /// Broadcast input
    Broadcast {
        /// Name of the broadcast
        name: Name,

        /// Id of the broadcast
        id: Uid,
    },

    /// Variable input
    Variable {
        /// Name of the variable
        name: Name,
        /// Id of the variable
        id: Option<Uid>,
    },

    /// List input
    List {
        /// Name of the list
        name: Name,
        /// Id of the list
        id: Uid,
    },
}

impl Default for BlockNormal {
    /// This create new block that act like it's a top most block
    fn default() -> Self {
        BlockNormal {
            opcode: OpCode::default(),
            comment: None,
            next: None,
            parent: None,
            inputs: StringHashMap::default(),
            fields: StringHashMap::default(),
            shadow: false,
            top_level: true,
            mutation: None,
            x: Some(0.into()),
            y: Some(0.into()),
        }
    }
}

// Serde impl ==================================================================

macro_rules! list_or_variable_vistor_types {
    ($($fn_name:ident, $ty:ty, $sign:ident, $unexpty:ty;)*) => {
        $(
            fn $fn_name<E>(self, v: $ty) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    12 => Ok(ListOrVariable::Variable),
                    13 => Ok(ListOrVariable::List),
                    v => Err(E::invalid_value(
                        Unexpected::$sign(v as $unexpty),
                        &"12 for variable, 13 for list",
                    )),
                }
            }
        )*
    };
}

struct ListOrVariableVisitor;

impl<'de> Visitor<'de> for ListOrVariableVisitor {
    type Value = ListOrVariable;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("12 for variable, 13 for list")
    }

    list_or_variable_vistor_types! {
        visit_i8, i8, Signed, i64;
        visit_i16, i16, Signed, i64;
        visit_i32, i32, Signed, i64;
        visit_i64, i64, Signed, i64;

        visit_u8, u8, Unsigned, u64;
        visit_u16, u16, Unsigned, u64;
        visit_u32, u32, Unsigned, u64;
        visit_u64, u64, Unsigned, u64;
    }
}

impl<'de> Deserialize<'de> for ListOrVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(ListOrVariableVisitor)
    }
}

impl Serialize for ListOrVariable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(match self {
            ListOrVariable::Variable => 12,
            ListOrVariable::List => 13,
        })
    }
}

impl BlockInput {
    /// Use for serializing
    fn size_hint(&self) -> usize {
        1 + self.inputs.len()
    }
}

struct BlockInputVisitor;

impl<'de> Visitor<'de> for BlockInputVisitor {
    type Value = BlockInput;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("list that is a block input")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;

        let shadow = seq.next_element::<ShadowInputType>()?.ok_or_else(|| {
            A::Error::invalid_length(0, &"Expected 2 or more elements for block input")
        })?;

        let mut inputs = vec![];
        while let Some(v) = seq.next_element::<Option<UidOrValue>>()? {
            inputs.push(v)
        }

        Ok(BlockInput { shadow, inputs })
    }
}

impl<'de> Deserialize<'de> for BlockInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(BlockInputVisitor)
    }
}

impl Serialize for BlockInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut s = serializer.serialize_seq(Some(self.size_hint()))?;
        s.serialize_element(&self.shadow)?;
        for v in &self.inputs {
            s.serialize_element(&v)?;
        }
        s.end()
    }
}

impl BlockInputValue {
    fn get_id(&self) -> u8 {
        use BlockInputValue::*;

        match self {
            Number { value: _ } => 4,
            PositiveNumber { value: _ } => 5,
            PositiveInteger { value: _ } => 6,
            Integer { value: _ } => 7,
            Angle { value: _ } => 8,
            Color { value: _ } => 9,
            String { value: _ } => 10,
            Broadcast { name: _, id: _ } => 11,
            Variable { name: _, id: _ } => 12,
            List { name: _, id: _ } => 13,
        }
    }

    fn hint_size(&self) -> usize {
        use BlockInputValue::*;

        match self {
            Number { value: _ } => 1,
            PositiveNumber { value: _ } => 1,
            PositiveInteger { value: _ } => 1,
            Integer { value: _ } => 1,
            Angle { value: _ } => 1,
            Color { value: _ } => 1,
            String { value: _ } => 1,
            Broadcast { name: _, id: _ } => 2,
            Variable { name: _, id: _ } => 2,
            List { name: _, id: _ } => 2,
        }
    }
}

struct BlockInputValueVisitor;

impl<'de> Visitor<'de> for BlockInputValueVisitor {
    type Value = BlockInputValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("list that is a block input value")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;
        use BlockInputValue::{
            Angle, Broadcast, Color, Integer, List, Number as BlockInputNumber, PositiveInteger,
            PositiveNumber, String, Variable,
        };

        fn seq_next_element_error<'de, T, A>(
            seq: &mut A,
            len: usize,
            error: &str,
        ) -> Result<T, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
            T: Deserialize<'de>,
        {
            seq.next_element::<T>()?
                .ok_or_else(|| A::Error::invalid_length(len, &error))
        }

        let vtype: u8 = seq_next_element_error(
            &mut seq,
            0,
            "Expecting 2 or more elements for block input value with any Id",
        )?;

        let value = seq_next_element_error(
            &mut seq,
            1,
            "Expecting 2 or more elements for block input value with any Id",
        )?;

        let res = match vtype {
            4 => BlockInputNumber { value },
            5 => PositiveNumber { value },
            6 => PositiveInteger { value },
            7 => Integer { value },
            8 => Angle { value },
            9 => Color { value },
            10 => String { value },
            11 => {
                let id = seq_next_element_error(
                    &mut seq,
                    3,
                    "Expecting 3 or more elements for block input value with Id 11",
                )?;

                let name = match value {
                    Value::Text(s) => s,
                    Value::Number(_) => {
                        return Err(A::Error::invalid_value(
                            serde::de::Unexpected::Other("number"),
                            &"a string",
                        ))
                    }
                    Value::Bool(_) => {
                        return Err(A::Error::invalid_value(
                            serde::de::Unexpected::Other("bool"),
                            &"a string",
                        ))
                    }
                };

                Broadcast { name, id }
            }
            12 => {
                let id = seq_next_element_error(
                    &mut seq, 3,
                    "Expecting 3 or 5 or more elements for block input value with Id 12 - 13 inclusive"
                )?;
                let _x = seq.next_element::<Number>()?;
                let _y = seq.next_element::<Number>()?;
                let name = match value {
                    Value::Text(s) => s,
                    Value::Number(_) => {
                        return Err(A::Error::invalid_value(
                            serde::de::Unexpected::Other("number"),
                            &"a string",
                        ))
                    },
                    Value::Bool(_) => {
                        return Err(A::Error::invalid_value(
                            serde::de::Unexpected::Other("bool"),
                            &"a string",
                        ))
                    }
                };
                Variable { name, id }
            }
            13 => {
                let id = seq_next_element_error(
                    &mut seq, 3,
                    "Expecting 3 or 5 or more elements for block input value with Id 12 - 13 inclusive"
                )?;
                let _x = seq.next_element::<Number>()?;
                let _y = seq.next_element::<Number>()?;
                let name = match value {
                    Value::Text(s) => s,
                    Value::Number(_) => {
                        return Err(A::Error::invalid_value(
                            serde::de::Unexpected::Other("number"),
                            &"a string",
                        ))
                    },
                    Value::Bool(_) => {
                        return Err(A::Error::invalid_value(
                            serde::de::Unexpected::Other("bool"),
                            &"a string",
                        ))
                    }
                };
                List { name, id }
            }
            v => {
                return Err(A::Error::invalid_value(
                    serde::de::Unexpected::Unsigned(v.into()),
                    &"Expecting a type id between 4 - 13 inclusive",
                ))
            }
        };

        Ok(res)
    }
}

impl<'de> Deserialize<'de> for BlockInputValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(BlockInputValueVisitor)
    }
}

impl Serialize for BlockInputValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        use BlockInputValue::*;

        let mut s = serializer.serialize_seq(Some(self.hint_size()))?;
        s.serialize_element(&self.get_id())?;
        match self {
            Number { value }
            | PositiveNumber { value }
            | PositiveInteger { value }
            | Integer { value }
            | Angle { value }
            | Color { value }
            | String { value } => {
                s.serialize_element(value)?;
            }
            Broadcast { name, id } => {
                s.serialize_element(name)?;
                s.serialize_element(id)?;
            }
            Variable { name, id } => {
                s.serialize_element(name)?;
                s.serialize_element(id)?;
            },
            List { name, id } => {
                s.serialize_element(name)?;
                s.serialize_element(id)?;
            }
        }
        s.end()
    }
}

impl BlockField {
    /// Value of the field
    #[inline(always)]
    pub fn value(&self) -> &Value {
        match self {
            BlockField::WithId { value, id: _ } => value,
            BlockField::NoId { value } => value,
        }
    }

    /// For certain fields,
    /// such as variable and broadcast dropdown menus,
    /// there is also a second element, which is the Id of the field's value.
    #[inline(always)]
    pub fn id(&self) -> Option<&Uid> {
        match self {
            BlockField::WithId { value: _, id } => id.as_ref(),
            BlockField::NoId { value: _ } => None,
        }
    }
}

struct BlockFieldVisitor;

impl<'de> Visitor<'de> for BlockFieldVisitor {
    type Value = BlockField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("sequence of values that is a blockfield")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;

        let value = seq
            .next_element::<Value>()?
            .ok_or_else(|| A::Error::invalid_length(1, &"length 1 or 2 for BlockField"))?;
        let id = seq.next_element::<Option<Uid>>()?;

        Ok(match id {
            Some(id) => BlockField::WithId { value, id },
            None => BlockField::NoId { value },
        })
    }
}

impl<'de> Deserialize<'de> for BlockField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(BlockFieldVisitor)
    }
}

impl Serialize for BlockField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        match self {
            BlockField::WithId { value, id } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(value)?;
                seq.serialize_element(id)?;
                seq.end()
            }
            BlockField::NoId { value } => {
                let mut seq = serializer.serialize_seq(Some(1))?;
                seq.serialize_element(value)?;
                seq.end()
            }
        }
    }
}

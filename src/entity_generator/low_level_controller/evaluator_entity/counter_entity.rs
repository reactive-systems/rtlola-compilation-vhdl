use crate::entity_generator::GenerateVhdlCode;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct CounterEntity {}

impl CounterEntity {
    pub(crate) fn new() -> CounterEntity {
        CounterEntity {}
    }
}

impl GenerateVhdlCode for CounterEntity {
    fn template_name(&self) -> String {
        "counter_entity.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        panic!("should not happen.")
    }
}

impl Serialize for CounterEntity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = serializer.serialize_struct("Counter", 0)?;
        s.end()
    }
}

use crate::entity_generator::GenerateVhdlCode;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct CounterComponent {}

impl CounterComponent {
    pub(crate) fn new() -> CounterComponent {
        CounterComponent {}
    }
}

impl GenerateVhdlCode for CounterComponent {
    fn template_name(&self) -> String {
        "counter_component.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        panic!("should not happen.")
    }
}

impl Serialize for CounterComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = serializer.serialize_struct("Counter", 0)?;
        s.end()
    }
}

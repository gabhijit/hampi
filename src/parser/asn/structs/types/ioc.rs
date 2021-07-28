//! Structures related to Information Object Class, Objects etc.

use std::collections::HashMap;

use super::Asn1Type;

#[derive(Debug, Clone)]
pub(crate) struct FixedTypeValueFieldSpec {
    pub(crate) id: String,

    pub(crate) field_type: Asn1Type,
    pub(crate) unique: bool,
    pub(crate) default: Option<String>,
    pub(crate) optional: bool,
    pub(crate) with_syntax: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypeFieldSpec {
    pub(crate) id: String,
    pub(crate) optional: bool,
    pub(crate) default: Option<Asn1Type>,
    pub(crate) with_syntax: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum ObjectClassFieldSpec {
    Type {
        id: String,
        default: Option<Asn1Type>,
        optional: bool,
        with_syntax: Option<String>,
        resolved: bool,
    },
    FixedTypeValue {
        id: String,

        field_type: Asn1Type,
        unique: bool,
        default: Option<String>,
        optional: bool,
        with_syntax: Option<String>,
        resolved: bool,
    },
    // TODO: Following Field Specs are not implemented right now
    // VariableTypeValue(VariableTypeValueFieldSpec),
    // FixedTypeValueSet(FixedTypeValueSetFieldSpec),
    // VariableTypeValueSet(VariableTypeValueSetSpec),
    // Object(ObjectFieldSpec),
    // ObjectSet(ObjectSetFieldSpec)
}

impl ObjectClassFieldSpec {
    pub(crate) fn id(&self) -> String {
        match self {
            Self::Type { id, .. } | Self::FixedTypeValue { id, .. } => id.clone(),
        }
    }

    pub(crate) fn resolved(&self) -> bool {
        match self {
            Self::Type { resolved, .. } | Self::FixedTypeValue { resolved, .. } => *resolved,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ObjectClass {
    pub(crate) fields: HashMap<String, ObjectClassFieldSpec>,
}

impl Asn1ObjectClass {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut output = vec![];
        for field in self.fields.values() {
            match field {
                ObjectClassFieldSpec::FixedTypeValue { field_type, .. } => {
                    let mut field_references = field_type.dependent_references();
                    output.append(&mut field_references);
                }
                _ => {}
            }
        }
        output
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ObjectSet {
    pub(crate) class: String,      // Class for which this Object Set is defined
    pub(crate) objects: ObjectSet, // Actual Object Set
}

impl Asn1ObjectSet {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut output = vec![self.class.clone()];
        output.append(&mut self.objects.dependent_references());
        output
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1Object {
    pub(crate) class: String, // Class for which this Object Set is defined
    pub(crate) value: String, // For now just a string,
}

#[derive(Debug, Clone)]
pub(crate) struct ObjectSet {
    pub(crate) root_elements: Vec<ObjectSetElement>,
    pub(crate) additional_elements: Vec<ObjectSetElement>,
}

impl ObjectSet {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut output = vec![];
        for e in &self.root_elements {
            if let Some(element) = match e {
                ObjectSetElement::ObjectSetReference(ref r)
                | ObjectSetElement::ObjectReference(ref r) => Some(r.clone()),
                _ => None,
            } {
                output.push(element);
            }
        }
        for e in &self.additional_elements {
            if let Some(element) = match e {
                ObjectSetElement::ObjectSetReference(ref r)
                | ObjectSetElement::ObjectReference(ref r) => Some(r.clone()),
                _ => None,
            } {
                output.push(element);
            }
        }
        output
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ObjectSetElement {
    ObjectSetReference(String), // A Reference to a defined Object Set
    ObjectReference(String),    // A reference to a defined Object
    Object(String),             // An object defined Inline
}
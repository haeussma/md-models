use crate::attribute::Attribute;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
/// Represents an object with a name, attributes, docstring, and an optional term.
pub struct Object {
    /// Name of the object.
    pub name: String,
    /// List of attributes associated with the object.
    pub attributes: Vec<Attribute>,
    /// Documentation string for the object.
    pub docstring: String,
    /// Optional term associated with the object.
    pub term: Option<String>,
}

impl Object {
    /// Creates a new `Object` with the given name and optional term.
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the name of the object.
    /// * `term` - An optional string representing a term associated with the object.
    ///
    /// # Returns
    ///
    /// * `Object` - A new instance of the `Object` struct.
    pub fn new(name: String, term: Option<String>) -> Self {
        Object {
            name,
            attributes: Vec::new(),
            docstring: String::new(),
            term,
        }
    }

    /// Adds an attribute to the object.
    ///
    /// # Arguments
    ///
    /// * `attribute` - An instance of `Attribute` to be added to the object's attributes.
    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    /// Sets the docstring for the object.
    ///
    /// # Arguments
    ///
    /// * `docstring` - A string representing the documentation string for the object.
    pub fn set_docstring(&mut self, docstring: String) {
        self.docstring = docstring;
    }

    /// Retrieves the last attribute added to the object.
    ///
    /// # Returns
    ///
    /// * `&mut Attribute` - A mutable reference to the last attribute.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no attributes in the object.
    pub fn get_last_attribute(&mut self) -> &mut Attribute {
        self.attributes.last_mut().unwrap()
    }

    /// Creates and adds a new attribute to the object.
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the name of the attribute.
    /// * `required` - A boolean indicating whether the attribute is required.
    pub fn create_new_attribute(&mut self, name: String, required: bool) {
        let attribute = Attribute::new(name, required);
        self.attributes.push(attribute);
    }

    /// Checks if the object has any attributes.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the object has attributes, `false` otherwise.
    pub fn has_attributes(&self) -> bool {
        !self.attributes.is_empty()
    }

    /// Sets the name of the object.
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the new name of the object.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Checks if any attribute of the object has a term.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if any attribute has a term, `false` otherwise.
    pub fn has_any_terms(&self) -> bool {
        self.attributes.iter().any(|attr| attr.has_term())
    }

    /// Sorts the attributes of the object by their `required` field in descending order.
    pub fn sort_attrs_by_required(&mut self) {
        self.attributes.sort_by(|a, b| b.required.cmp(&a.required))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Represents an enumeration with a name and mappings.
pub struct Enumeration {
    /// Name of the enumeration.
    pub name: String,
    /// Mappings associated with the enumeration.
    pub mappings: BTreeMap<String, String>,
    /// Documentation string for the enumeration.
    pub docstring: String,
}

impl Enumeration {
    /// Checks if the enumeration has any values.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the enumeration has values, `false` otherwise.
    pub fn has_values(&self) -> bool {
        !self.mappings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_create_new_object() {
        let object = Object::new("Person".to_string(), None);
        assert_eq!(object.name, "Person");
        assert_eq!(object.attributes.len(), 0);
        assert_eq!(object.docstring, "");
        assert_eq!(object.term, None);
    }

    #[test]
    fn test_add_attribute() {
        let mut object = Object::new("Person".to_string(), None);
        let attribute = Attribute::new("name".to_string(), false);
        object.add_attribute(attribute);
        assert_eq!(object.attributes.len(), 1);
        assert_eq!(object.attributes[0].name, "name");
    }

    #[test]
    fn test_set_docstring() {
        let mut object = Object::new("Person".to_string(), None);
        object.set_docstring("This is a person object".to_string());
        assert_eq!(object.docstring, "This is a person object");
    }

    #[test]
    fn test_get_last_attribute() {
        let mut object = Object::new("Person".to_string(), None);
        let attribute = Attribute::new("name".to_string(), false);
        object.add_attribute(attribute);
        let last_attribute = object.get_last_attribute();
        assert_eq!(last_attribute.name, "name");
    }

    #[test]
    fn test_create_new_attribute() {
        let mut object = Object::new("Person".to_string(), None);
        object.create_new_attribute("name".to_string(), false);
        assert_eq!(object.attributes.len(), 1);
        assert_eq!(object.attributes[0].name, "name");
    }
}

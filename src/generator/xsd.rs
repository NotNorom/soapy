use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Xsd {
    #[serde(rename = "{http://www.w3.org/2001/XMLSchema}schema")]
    schema: Schema,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Schema {
    #[serde(rename = "$attr:version")]
    version: String,
    #[serde(rename = "$attr:targetNamespace")]
    target_namespace: String,

    #[serde(rename = "{http://www.w3.org/2001/XMLSchema}import")]
    import: Vec<Import>,
    #[serde(rename = "{http://www.w3.org/2001/XMLSchema}element")]
    element: Vec<Element>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Import {
    #[serde(rename = "$attr:namespace")]
    namespace: String,
    #[serde(rename = "$attr:schemaLocation")]
    schema_location: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Element {
    #[serde(rename = "$attr:name")]
    name: String,
    #[serde(rename = "$attr:type")]
    r#type: Option<SimpleElementType>,
    #[serde(rename = "$attr:default")]
    default: Option<String>,
    #[serde(rename = "$attr:fixed")]
    fixed: Option<String>,

    #[serde(rename = "$attr:minOccurs")]
    min_occurs: Option<String>,
    #[serde(rename = "$attr:maxOccurs")]
    max_occurs: Option<String>,

    #[serde(rename = "$attr:substitutionGroup")]
    substitution_group: Option<String>,
    #[serde(rename = "$attr:block")]
    block: Option<String>,
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.r#type == other.r#type
            && self.default == other.default
            && self.fixed == other.fixed
    }
}

impl Element {
    pub fn value<T>(&self) -> T {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum SimpleElementType {
    String,
    Decimal,
    Integer,
    Boolean,
    Date,
    Time,
}

mod indicator {
    struct All;
    struct Choice;
    struct Sequence;

    struct Group;

    struct Attribute;
}

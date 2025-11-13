use std::collections::HashMap;

///I think this should be a type
pub enum GeneratorType {
    ///Single constant value
    Constant,
    ///Set of values, randomly selected
    RandomSet,
    ///Set of values, selected in order
    OrderedSet,
    ///Random alphabetic characters
    RandomCharacters,
    ///Random numbers
    RandomNumbers,
    ///Random alphanumeric
    RandomAlphaNumeric,
    ///Generate a string that matches a regular expression
    RandomRegex,
}

///What kind of values can a column have
pub struct ColumnGenerator {
    generator_type: GeneratorType,
    length: usize,
    available_values: Option<Vec<String>>,
    //meta tags used to possibly change or generate
    meta_values: Option<Vec<String>>,
}

pub struct Column {
    name: String,
    generator: ColumnGenerator,
    value: Option<String>,
}

///The actual data model...do we need this? should be not just keep generating?
///TOIDO think about this a bit more
pub struct DataModel {
    //what are the names of our csv columns, eg name, surname, telephone, email, customer_code
    columns: Vec<String>
    rows: Vec<String>,
}

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
}
pub struct Column {
    name: String,
    generator: ColumnGenerator,
}

pub struct DataModel {
    //what are the names of our csv columns, eg name, surname, telephone, email, customer_code
    column_names: Vec<String>,
    column_valus: Vec,
}

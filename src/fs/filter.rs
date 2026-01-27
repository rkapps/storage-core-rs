#[derive(Debug, Clone)]
pub struct Filter {
    pub conditions: Vec<FilterCondition>,
    pub sort: Option<Vec<SortField>>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct FilterCondition {
    pub field: String,
    pub operator: FilterOp,
    pub value: FilterValue,
}

#[derive(Debug, Clone)]
pub enum FilterOp {
    Eq,           // Equality
    Gte,          // Greater than or equal
    Lte,          // Less than or equal
    Gt,           // Greater than
    Lt,           // Less than
    In,           // Value in array
    Contains,     // String contains
    StartsWith,   // String starts with
}

#[derive(Debug, Clone)]
pub enum FilterValue {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Array(Vec<String>),  // For "In" operator
}

#[derive(Debug, Clone)]
pub struct SortField {
    pub field: String,
    pub ascending: bool,  // true = ascending, false = descending
}
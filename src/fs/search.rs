use crate::core::Searchable;

#[derive(Debug, Clone)]
pub struct SearchCriteria {
    pub conditions: Vec<SearchCondition>,
    pub sort_fields: Option<Vec<SortField>>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct SearchCondition {
    pub field: String,
    pub operator: SearchOp,
    pub value: SearchValue,
}

#[derive(Debug, Clone)]
pub enum SearchOp {
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
pub enum SearchValue {
    String(String),
    Decimal(rust_decimal::Decimal),
    Int(i64),
    Bool(bool),
    Array(Vec<String>),  // For "In" operator
}

#[derive(Debug, Clone)]
pub struct SortField {
    pub field: String,
    pub ascending: bool,  // true = ascending, false = descending
}

impl SearchCriteria {

    pub fn new() -> SearchCriteria {
        SearchCriteria {
            conditions: Vec::new(),
            sort_fields: None,
            limit: None
        }
    }

    // add_condition
    pub fn add_condition(&mut self, field: &str, operator: SearchOp, value: SearchValue) {
        self.conditions.push( SearchCondition { field: field.to_string(), operator, value })
    }

    // add sort
    pub fn add_sort(&mut self, field: &str, ascending: bool) {

        let sort_field = SortField {
            field: field.to_string(),
            ascending,
        };        

        self.sort_fields.get_or_insert(Vec::new()).push(sort_field);
    }

    // add limit
    pub fn add_limit(&mut self, limit: usize) {
        self.limit.get_or_insert(limit);
    }
    

}


pub fn apply_sort<M: Searchable>(mut items: Vec<M>, sort_fields: &[SortField]) -> Vec<M>{

    items.sort_by(|a, b| {
        for sort_field in sort_fields {

            let val_a = a.get_field_value(&sort_field.field);
            let val_b = b.get_field_value(&sort_field.field);
            
            match (val_a, val_b) {
                (Some(a), Some(b)) => {
                    let ordering = if sort_field.ascending {
                        a.cmp(&b)
                    } else {
                        b.cmp(&a)
                    };
                    
                    if ordering != std::cmp::Ordering::Equal {
                        return ordering;
                    }
                    // Continue to next sort field if equal
                }
                _ => {} // S
            }
        }
        std::cmp::Ordering::Equal

    });

    items
}
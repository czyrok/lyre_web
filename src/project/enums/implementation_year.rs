use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum ImplementationYear {
    CurrentYear,
    LastYear,
    TwoYearsAgo,
    MoreThanTwoYears,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub enum ImplementationYear {
    CurrentYear,
    LastYear,
    TwoYearsAgo,
    MoreThanTwoYears,
}

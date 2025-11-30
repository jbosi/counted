use std::fmt;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub enum EventSSE {
    UserCreated,
    UserDeleted,
    UserModified,
    ProjectCreated,
    ProjectDeleted,
    ProjectModified,
    ExpenseCreated,
    ExpenseDeleted,
    ExpenseModified,
    PaymentCreated,
    PaymentDeleted,
    PaymentModified,
    None
}

impl fmt::Display for EventSSE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventSSE::UserCreated => write!(f, "UserCreated"),
            EventSSE::UserDeleted => write!(f, "UserDeleted"),
            EventSSE::UserModified => write!(f, "UserModified"),
            EventSSE::ProjectCreated => write!(f, "ProjectCreated"),
            EventSSE::ProjectDeleted => write!(f, "ProjectDeleted"),
            EventSSE::ProjectModified => write!(f, "ProjectModified"),
            EventSSE::ExpenseCreated => write!(f, "ExpenseCreated"),
            EventSSE::ExpenseDeleted => write!(f, "ExpenseDeleted"),
            EventSSE::ExpenseModified => write!(f, "ExpenseModified"),
            EventSSE::PaymentCreated => write!(f, "PaymentCreated"),
            EventSSE::PaymentDeleted => write!(f, "PaymentDeleted"),
            EventSSE::PaymentModified => write!(f, "PaymentModified"),
            EventSSE::None => write!(f, ""),
        }
    }
}

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    None,
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
            EventSSE::None => write!(f, "None"),
        }
    }
}

impl std::str::FromStr for EventSSE {
    type Err = ();
    fn from_str(input: &str) -> Result<EventSSE, Self::Err> {
        match input {
            "UserCreated" => Ok(EventSSE::UserCreated),
            "UserDeleted" => Ok(EventSSE::UserDeleted),
            "UserModified" => Ok(EventSSE::UserModified),
            "ProjectCreated" => Ok(EventSSE::ProjectCreated),
            "ProjectDeleted" => Ok(EventSSE::ProjectDeleted),
            "ProjectModified" => Ok(EventSSE::ProjectModified),
            "ExpenseCreated" => Ok(EventSSE::ExpenseCreated),
            "ExpenseDeleted" => Ok(EventSSE::ExpenseDeleted),
            "ExpenseModified" => Ok(EventSSE::ExpenseModified),
            "PaymentCreated" => Ok(EventSSE::PaymentCreated),
            "PaymentDeleted" => Ok(EventSSE::PaymentDeleted),
            "PaymentModified" => Ok(EventSSE::PaymentModified),
            "None" => Ok(EventSSE::None),
            _ => Err(()),
        }
    }
}

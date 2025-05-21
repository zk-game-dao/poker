use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
struct Trace {
    location: &'static str,
    line: u32,
    message: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TracedError<E> {
    error: E,
    traces: Vec<Trace>,
}

impl<E> TracedError<E> {
    pub fn new(error: E) -> Self {
        Self {
            error,
            traces: Vec::new(),
        }
    }

    pub fn trace(mut self, location: &'static str, line: u32, message: impl Into<String>) -> Self {
        self.traces.push(Trace {
            location,
            line,
            message: message.into(),
        });
        self
    }

    pub fn into_inner(self) -> E {
        self.error
    }
}

pub trait IntoTracedError<E> {
    fn into_traced(self) -> TracedError<E>;
}

impl<E: Error, F> IntoTracedError<E> for TracedError<F>
where
    E: From<F>,
{
    fn into_traced(self) -> TracedError<E> {
        let mut new_traced = TracedError::new(E::from(self.error));
        new_traced.traces = self.traces;
        new_traced
    }
}

impl<E: fmt::Display> fmt::Display for TracedError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error: {}", self.error)?;
        if !self.traces.is_empty() {
            writeln!(f, "\nTrace:")?;
            for trace in self.traces.iter().rev() {
                writeln!(
                    f,
                    "  at {}:{} - {}",
                    trace.location, trace.line, trace.message
                )?;
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! trace_err {
    ($error:expr, $message:expr) => {
        $error.trace(file!(), line!(), $message)
    };
    ($error:expr) => {
        $error.trace(file!(), line!(), "")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fmt;

    // Test error types
    #[derive(Debug)]
    struct ErrorA(String);

    impl fmt::Display for ErrorA {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ErrorA: {}", self.0)
        }
    }

    impl Error for ErrorA {}

    #[derive(Debug)]
    struct ErrorB(String);

    impl fmt::Display for ErrorB {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ErrorB: {}", self.0)
        }
    }

    impl Error for ErrorB {}

    impl From<ErrorA> for ErrorB {
        fn from(error: ErrorA) -> Self {
            ErrorB(error.0)
        }
    }

    // Helper functions to simulate error propagation
    fn inner_function() -> Result<(), TracedError<ErrorA>> {
        Err(trace_err!(
            TracedError::new(ErrorA("inner error".to_string())),
            "error in inner_function"
        ))
    }

    fn middle_function() -> Result<(), TracedError<ErrorA>> {
        inner_function().map_err(|e| trace_err!(e, "error in middle_function"))?;
        Ok(())
    }

    fn outer_function() -> Result<(), TracedError<ErrorB>> {
        middle_function()
            .map_err(|e| e.into_traced())
            .map_err(|e| trace_err!(e, "error in outer_function"))?;
        Ok(())
    }

    #[test]
    fn test_basic_error_creation() {
        let error = TracedError::new(ErrorA("test error".to_string()));
        assert!(error.traces.is_empty());
    }

    #[test]
    fn test_single_trace() {
        let error = TracedError::new(ErrorA("test error".to_string()));
        let traced = trace_err!(error, "trace message");

        assert_eq!(traced.traces.len(), 1);
        assert_eq!(traced.traces[0].message, "trace message");
    }

    #[test]
    fn test_error_propagation() {
        let result = outer_function();

        assert!(result.is_err());
        let err = result.unwrap_err();

        // Should have 3 traces: inner, middle, and outer
        assert_eq!(err.traces.len(), 3);

        // Check trace order (most recent first when displayed)
        let display = format!("{}", err);

        println!("{}", display);

        assert!(display.contains("error in outer_function"));
        assert!(display.contains("error in middle_function"));
        assert!(display.contains("error in inner_function"));
    }

    #[test]
    fn test_error_conversion() {
        let error_a = TracedError::new(ErrorA("test error".to_string()));
        let traced_a = trace_err!(error_a, "trace for A");

        let error_b: TracedError<ErrorB> = traced_a.into_traced();
        assert_eq!(error_b.traces.len(), 1);
        assert_eq!(error_b.traces[0].message, "trace for A");
    }

    #[test]
    fn test_display_format() {
        let error = TracedError::new(ErrorA("test error".to_string()));
        let traced = trace_err!(trace_err!(error, "first trace"), "second trace");

        let display = format!("{}", traced);

        println!("{}", display);

        assert!(display.contains("ErrorA: test error"));
        assert!(display.contains("first trace"));
        assert!(display.contains("second trace"));
    }

    #[test]
    fn test_error_extraction() {
        let original_error = ErrorA("test error".to_string());
        let traced = TracedError::new(original_error);
        let extracted = traced.into_inner();

        assert_eq!(format!("{}", extracted), "ErrorA: test error");
    }

    #[test]
    fn test_empty_message_trace() {
        let error = TracedError::new(ErrorA("test error".to_string()));
        let traced = trace_err!(error); // Uses empty string variant

        assert_eq!(traced.traces.len(), 1);
        assert_eq!(traced.traces[0].message, "");
    }
}

#[cfg(test)]
mod tests_with_game_error {
    use super::*;
    use std::collections::HashMap;

    // Mock GameError for testing
    #[derive(Debug, PartialEq)]
    enum GameError {
        PlayerNotFound,
        Other(String),
    }

    impl std::fmt::Display for GameError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                GameError::PlayerNotFound => write!(f, "Player not found"),
                GameError::Other(msg) => write!(f, "Other error: {}", msg),
            }
        }
    }

    impl std::error::Error for GameError {}

    // Mock struct to simulate your table/user structure
    struct MockTable {
        users: HashMap<String, User>,
    }

    #[derive(Debug, PartialEq)]
    struct User {
        name: String,
    }

    impl MockTable {
        fn new() -> Self {
            Self {
                users: HashMap::new(),
            }
        }

        fn get_user(&self, id: &str) -> Result<&User, TracedError<GameError>> {
            self.users.get(id).ok_or_else(|| {
                trace_err!(
                    TracedError::new(GameError::PlayerNotFound),
                    "User not found in get_user"
                )
            })
        }

        // Helper function that calls get_user to test error propagation
        fn process_user(&self, id: &str) -> Result<(), TracedError<GameError>> {
            let user = self
                .get_user(id)
                .map_err(|e| trace_err!(e, "Failed to process user"))?;

            // Simulate some processing
            if user.name.is_empty() {
                return Err(trace_err!(
                    TracedError::new(GameError::Other("Invalid user name".to_string())),
                    "Name validation failed"
                ));
            }

            Ok(())
        }
    }

    #[test]
    fn test_user_not_found() {
        let table = MockTable::new();
        let result = table.get_user("nonexistent");

        assert!(result.is_err());
        let err = result.unwrap_err();

        println!("test_user_not_found: {:?}", err);

        // Check the error type
        assert!(matches!(err.into_inner(), GameError::PlayerNotFound));
    }

    #[test]
    fn test_user_found() {
        let mut table = MockTable::new();
        table.users.insert(
            "test_id".to_string(),
            User {
                name: "Test User".to_string(),
            },
        );

        let result = table.get_user("test_id");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Test User");
    }

    #[test]
    fn test_error_trace_information() {
        let table = MockTable::new();
        let err = table.get_user("nonexistent").unwrap_err();

        println!("test_error_trace_information: {:?}", err);

        let error_string = format!("{}", err);
        assert!(error_string.contains("User not found in get_user"));
        assert!(error_string.contains(file!())); // Should contain filename
    }

    #[test]
    fn test_error_propagation() {
        let table = MockTable::new();
        let err = table.process_user("nonexistent").unwrap_err();

        let error_string = format!("{}", err);
        // Should contain both error messages in the trace
        assert!(error_string.contains("Failed to process user"));
        assert!(error_string.contains("User not found in get_user"));
    }

    #[test]
    fn test_multiple_error_types() {
        let mut table = MockTable::new();
        table.users.insert(
            "empty_name".to_string(),
            User {
                name: "".to_string(),
            },
        );

        let err = table.process_user("empty_name").unwrap_err();
        let error_string = format!("{}", err);

        // Should show the validation error
        assert!(error_string.contains("Name validation failed"));
        assert!(matches!(err.into_inner(), GameError::Other(_)));
    }

    #[test]
    fn test_trace_order() {
        let table = MockTable::new();
        let err = table.process_user("nonexistent").unwrap_err();
        let error_string = format!("{}", err);

        println!("test_trace_order:\n{}", error_string);

        // Get positions of trace messages
        let pos1 = error_string.find("Failed to process user").unwrap_or(0);
        let pos2 = error_string.find("User not found in get_user").unwrap_or(0);

        // The process_user error should appear before the get_user error in the trace
        // (most recent first)
        assert!(pos1 < pos2);
    }

    #[test]
    fn test_nested_error_handling() {
        let table = MockTable::new();

        // Create a function that uses multiple levels of error handling
        fn deep_process(table: &MockTable, id: &str) -> Result<(), TracedError<GameError>> {
            let user = table
                .get_user(id)
                .map_err(|e| trace_err!(e, "Level 3 error"))?;

            if user.name.len() < 10 {
                return Err(trace_err!(
                    TracedError::new(GameError::Other("Name too short".to_string())),
                    "Validation error"
                ));
            }

            Ok(())
        }

        let err = deep_process(&table, "nonexistent").unwrap_err();
        let error_string = format!("{}", err);

        println!("test_nested_error_handling:\n{}", error_string);

        assert!(error_string.contains("Level 3 error"));
        assert!(error_string.contains("User not found in get_user"));
        assert_eq!(err.traces.len(), 2);
    }
}

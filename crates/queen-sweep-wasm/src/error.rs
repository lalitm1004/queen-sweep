//! Typed build errors exposed to JavaScript.

use queen_sweep_core::dynamic::DynamicSolverError;
use wasm_bindgen::prelude::*;

/// A stable reason a solver could not be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildErrorCode {
    /// The color regions were empty.
    Empty,
    /// The grid was not square.
    NonSquare,
    /// A flat list was not a perfect square.
    NotPerfectSquare,
    /// The implied board size is not supported.
    UnsupportedSize,
    /// The grid input could not be deserialized.
    InvalidInput,
    /// The board failed initialization (bad color index or count).
    BoardInitFailed,
}

impl BuildErrorCode {
    /// The stable string code exposed to JavaScript.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "EMPTY",
            Self::NonSquare => "NON_SQUARE",
            Self::NotPerfectSquare => "NOT_PERFECT_SQUARE",
            Self::UnsupportedSize => "UNSUPPORTED_SIZE",
            Self::InvalidInput => "INVALID_INPUT",
            Self::BoardInitFailed => "BOARD_INIT_FAILED",
        }
    }
}

/// An error thrown when a solver cannot be constructed.
///
/// JavaScript receives a plain object `{ code, message }`; see the generated
/// `BuildError` interface in the `.d.ts`.
#[derive(Debug, Clone)]
pub struct BuildError {
    /// The stable reason code.
    pub code: BuildErrorCode,
    /// A human-readable description.
    pub message: String,
}

impl BuildError {
    /// Creates a new build error.
    pub fn new(code: BuildErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl From<DynamicSolverError> for BuildError {
    fn from(err: DynamicSolverError) -> Self {
        let code = match &err {
            DynamicSolverError::Empty => BuildErrorCode::Empty,
            DynamicSolverError::NonSquare { .. } => BuildErrorCode::NonSquare,
            DynamicSolverError::NotPerfectSquare(_) => BuildErrorCode::NotPerfectSquare,
            DynamicSolverError::UnsupportedSize(_) => BuildErrorCode::UnsupportedSize,
            DynamicSolverError::BoardInit(_) => BuildErrorCode::BoardInitFailed,
        };

        Self::new(code, err.to_string())
    }
}

impl From<BuildError> for JsValue {
    fn from(err: BuildError) -> JsValue {
        let object = js_sys::Object::new();

        let _ = js_sys::Reflect::set(
            &object,
            &JsValue::from_str("code"),
            &JsValue::from_str(err.code.as_str()),
        );
        let _ = js_sys::Reflect::set(
            &object,
            &JsValue::from_str("message"),
            &JsValue::from_str(&err.message),
        );

        object.into()
    }
}

#[wasm_bindgen(typescript_custom_section)]
const BUILD_ERROR_TYPES: &'static str = r#"
/** A stable reason a solver could not be built. */
export type BuildErrorCode =
    | "EMPTY"
    | "NON_SQUARE"
    | "NOT_PERFECT_SQUARE"
    | "UNSUPPORTED_SIZE"
    | "INVALID_INPUT"
    | "BOARD_INIT_FAILED";

/** Thrown when a solver cannot be constructed. */
export interface BuildError {
    code: BuildErrorCode;
    message: string;
}
"#;

#[cfg(test)]
mod tests {
    use super::{BuildError, BuildErrorCode};
    use queen_sweep_core::{
        board::BoardInitError,
        dynamic::DynamicSolverError,
    };

    #[test]
    fn build_error_codes_map_to_stable_strings() {
        assert_eq!(BuildErrorCode::Empty.as_str(), "EMPTY");
        assert_eq!(BuildErrorCode::NonSquare.as_str(), "NON_SQUARE");
        assert_eq!(BuildErrorCode::NotPerfectSquare.as_str(), "NOT_PERFECT_SQUARE");
        assert_eq!(BuildErrorCode::UnsupportedSize.as_str(), "UNSUPPORTED_SIZE");
        assert_eq!(BuildErrorCode::InvalidInput.as_str(), "INVALID_INPUT");
        assert_eq!(BuildErrorCode::BoardInitFailed.as_str(), "BOARD_INIT_FAILED");
    }

    #[test]
    fn maps_every_dynamic_error_to_its_code() {
        let cases = [
            (DynamicSolverError::Empty, BuildErrorCode::Empty),
            (
                DynamicSolverError::NonSquare { rows: 2, cols: 3 },
                BuildErrorCode::NonSquare,
            ),
            (
                DynamicSolverError::NotPerfectSquare(5),
                BuildErrorCode::NotPerfectSquare,
            ),
            (
                DynamicSolverError::UnsupportedSize(2),
                BuildErrorCode::UnsupportedSize,
            ),
            (
                DynamicSolverError::BoardInit(BoardInitError::ColorOutOfRange {
                    color: 9,
                    max_colors: 4,
                }),
                BuildErrorCode::BoardInitFailed,
            ),
        ];

        for (err, expected) in cases {
            let build_err = BuildError::from(err);
            assert_eq!(build_err.code, expected);
        }
    }

    #[test]
    fn build_error_preserves_underlying_message() {
        let err = DynamicSolverError::UnsupportedSize(3);
        let build_err = BuildError::from(err);
        assert_eq!(
            build_err.message,
            "board size 3 is not supported (supported sizes: 1 and 4..=18)"
        );
    }

    #[test]
    fn build_error_new_sets_code_and_message() {
        let err = BuildError::new(BuildErrorCode::InvalidInput, "bad payload");
        assert_eq!(err.code, BuildErrorCode::InvalidInput);
        assert_eq!(err.message, "bad payload");
    }
}

//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyParameter;
impl Format for JsAnyParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyFormalParameter(node) => node.format(formatter),
            Self::JsRestParameter(node) => node.format(formatter),
            Self::TsThisParameter(node) => node.format(formatter),
        }
    }
}
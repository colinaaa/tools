use crate::{
	format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::LabelledStmt;

impl ToFormatElement for LabelledStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let label = formatter.format_node(self.label()?)?;
		let statement = formatter.format_node(self.stmt()?)?;
		let colon = formatter.format_token(&self.colon_token()?)?;

		Ok(format_elements![label, colon, space_token(), statement])
	}
}

//! Handles Cloudflare email protection.
use crate::imports::html::Element;

extern crate alloc;
use alloc::string::String;

/// Parses `data-cfemail` attribute and returns the email address.
///
/// # Examples
/// ```
/// use aidoku::helpers::cfemail::parse_cfemail;
/// assert_eq!(
///     parse_cfemail("98d1fcf7f4f5d8ebecfdea"),
///     "Idolm@ster",
/// );
/// ```
pub fn parse_cfemail<T: AsRef<str>>(data: T) -> String {
	let data = data.as_ref();
	if let Ok(key) = u32::from_str_radix(&data[0..2], 16) {
		let mut email = String::with_capacity(data.len() / 2 - 1);
		let mut n = 2;

		while n < data.len() {
			if let Ok(chrcode) = u32::from_str_radix(&data[n..n + 2], 16) {
				let chrcode = chrcode ^ key;
				email.push(char::from_u32(chrcode).unwrap_or_default());
			}
			n += 2;
		}
		email
	} else {
		"[email protected]".into()
	}
}

/// Replaces all `[email protected]` elements with their contents in-place.
pub fn decode_cfemail(html: &Element) {
	let Some(elements) = html.select(".__cf_email__") else {
		return;
	};
	for mut elem in elements {
		let email = parse_cfemail(elem.attr("data-cfemail").unwrap_or_default());
		elem.set_text(email).ok();
	}
}

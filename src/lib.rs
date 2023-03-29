/*!
Simple ANSI graphics code formatter.

# Examples

Set attributes:

```
println!(
	"{}Hello {}world!{}",
	ansi_gfx::BOLD, ansi_gfx::UNDERLINE, ansi_gfx::RESET);
```

<pre style="background-color: black; color: lightgray;"><span style="font-weight: bold;">Hello <span style="text-decoration: underline;">world!</span></span></pre>

Change text foreground and background color:

```
println!(
	concat!(
		"Foreground {}Red {}Green {}and {}Blue{}!\n",
		"Background {}Red {}Green {}and {}Blue{}!"),
	ansi_gfx::RED, ansi_gfx::GREEN, ansi_gfx::RESET, ansi_gfx::BLUE, ansi_gfx::RESET,
	ansi_gfx::RED_BG, ansi_gfx::GREEN_BG, ansi_gfx::RESET, ansi_gfx::BLUE_BG, ansi_gfx::RESET);
```

<pre style="background-color: black; color: lightgray;">Foreground <span style="color: red;">Red </span><span style="color: green;">Green </span>and <span style="color: blue;">Blue</span>!
Foreground <span style="background-color: red;">Red </span><span style="background-color: green;">Green </span>and <span style="background-color: blue;">Blue</span>!</pre>

Combine attributes, colors and extended colors using the [`mode!`] macro:

```
println!(
	"{}Here comes the sun!{}",
	ansi_gfx::mode!(UNDERLINE; FG RGB 243, 159, 24; BG PAL 28),
	ansi_gfx::RESET);
```

<pre style="background-color: black; color: lightgrey;"><span style="background-color: rgb(0, 135, 0); color: rgb(243, 159, 24); text-decoration: underline;">Hello world!</span></pre>

*/

#![cfg_attr(not(test), no_std)]


use core::{fmt, slice, str};

/// ANSI graphics mode builder for complex codes.
///
/// Returns an instance of [`Print`].
///
/// The macro accepts any number of arguments separated by a semicolon `;`.
///
/// A single argument can be:
/// * A code name identifier (e.g. `BOLD`). See [`codes`] for a list of all codes.
/// * A runtime [`Code`] value (e.g. `{ansi_gfx::BOLD}`).
/// * A foreground palette color (e.g. `FG PAL 28`).
/// * A background palette color (e.g. `BG PAL 28`).
/// * A foreground RGB color (e.g. `FG RGB 255, 0, 0`).
/// * A background RGB color (e.g. `BG RGB 255, 0, 0`).
///
/// # Examples
///
/// ```
/// println!("{}Bold and underlined{}\n", ansi_gfx::mode!(BOLD; UNDERLINE), ansi_gfx::RESET);
/// println!("{}Red on yellow{}\n", ansi_gfx::mode!(FG PAL 9; BG PAL 11), ansi_gfx::RESET);
/// let style = ansi_gfx::INVERSE;
/// println!("{}Inverted{}\n", ansi_gfx::mode!({style}; BOLD), ansi_gfx::RESET);
/// ```
///
/// <pre style="background-color: black; color: lightgray;"><span style="font-weight: bold; text-decoration: underline;">Bold and underlined</span>
/// <span style="color: rgb(239, 41, 41); background-color: rgb(252, 233, 79);">Red on yellow</span>
/// <span style="background-color: lightgray; color: black;">Inverted</span></pre>
#[macro_export]
macro_rules! mode {
	($($tt:tt)*) => {
		$crate::__mode!([] $($tt)*)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __mode {
	// Palette
	([$($code:expr,)*] $ground:ident $space:ident $index:expr; $($tail:tt)*) => {
		$crate::__mode!([
			$($code,)*
			$crate::__FG_or_BG::$ground.__byte,
			$crate::__RGB_or_PAL::$space.__byte,
			{ let index: u8 = $index; index },
		] $($tail)*)
	};
	([$($code:expr,)*] $ground:ident $space:ident $index:expr) => {
		$crate::__mode!([
			$($code,)*
			$crate::__FG_or_BG::$ground.__byte,
			$crate::__RGB_or_PAL::$space.__byte,
			{ let index: u8 = $index; index },
		])
	};

	// RGB
	([$($code:expr,)*] $ground:ident $space:ident $red:expr, $green:expr, $blue:expr; $($tail:tt)*) => {
		$crate::__mode!([
			$($code,)*
			$crate::__FG_or_BG::$ground.__byte,
			$crate::__RGB_or_PAL::$space.__byte,
			{ let red: u8 = $red; red },
			{ let green: u8 = $green; green },
			{ let blue: u8 = $blue; blue },
		] $($tail)*)
	};
	([$($code:expr,)*] $ground:ident $space:ident $red:expr, $green:expr, $blue:expr) => {
		$crate::__mode!([
			$($code,)*
			$crate::__FG_or_BG::$ground.__byte,
			$crate::__RGB_or_PAL::$space.__byte,
			{ let red: u8 = $red; red },
			{ let green: u8 = $green; green },
			{ let blue: u8 = $blue; blue },
		])
	};

	// Identifier
	([$($code:expr,)*] $name:ident; $($tail:tt)*) => {
		$crate::__mode!([
			$($code,)*
			$crate::codes::$name.__byte,
		] $($tail)*)
	};
	([$($code:expr,)*] $name:ident) => {
		$crate::__mode!([
			$($code,)*
			$crate::codes::$name.__byte,
		])
	};

	// Runtime value
	([$($code:expr,)*] {$v:expr}; $($tail:tt)*) => {
		$crate::__mode!([
			$($code,)*
			{ let v: $crate::Code = $v; v.__byte },
		] $($tail)*)
	};
	([$($code:expr,)*] {$v:expr}) => {
		$crate::__mode!([
			$($code,)*
			{ let v: $crate::Code = $v; v.__byte },
		])
	};

	// Term
	([$($code:expr,)*]) => {
		$crate::Print { __codes: [$($code),*] }
	};
}

/// Single ANSI graphics code.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Code {
	#[doc(hidden)]
	pub __byte: u8,
}

#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __FG_or_BG {
	use super::Code;

	/// Sets the extended foreground color.
	pub const FG: Code = Code { __byte: 38 };
	/// Sets the extended background color.
	pub const BG: Code = Code { __byte: 48 };
}

#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __RGB_or_PAL {
	use super::Code;

	/// Use the 256-bit color palette.
	pub const PAL: Code = Code { __byte: 5 };
	/// Use the true color mode.
	pub const RGB: Code = Code { __byte: 2 };
}

/// Color codes and attributes.
///
/// These constants are re-exported at the crate root for convenience.
pub mod codes {
	use super::Code;

	/// Set bold mode.
	pub const BOLD: Code = Code { __byte: 1 };
	/// Set dim/faint mode.
	pub const DIM: Code = Code { __byte: 2 };
	/// Set italic mode.
	pub const ITALIC: Code = Code { __byte: 3 };
	/// Set underline mode.
	pub const UNDERLINE: Code = Code { __byte: 4 };
	/// Set blinking mode.
	pub const BLINK: Code = Code { __byte: 5 };
	/// Flip foreground and background colors.
	pub const INVERSE: Code = Code { __byte: 7 };
	/// Set hidden/invisible mode.
	pub const HIDDEN: Code = Code { __byte: 8 };
	/// Set strikethrough mode.
	pub const STRIKE: Code = Code { __byte: 9 };

	/// Reset all attributes.
	pub const RESET: Code = Code { __byte: 0 };
	/// Reset bold/dim mode.
	pub const RESET_WEIGHT: Code = Code { __byte: 22 };
	/// Reset italic mode.
	pub const RESET_ITALIC: Code = Code { __byte: 23 };
	/// Reset underline mode.
	pub const RESET_UNDERLINE: Code = Code { __byte: 24 };
	/// Reset blinking mode.
	pub const RESET_BLINK: Code = Code { __byte: 25 };
	/// Reset inverse mode.
	pub const RESET_INVERSE: Code = Code { __byte: 27 };
	/// Reset hidden mode.
	pub const RESET_HIDDEN: Code = Code { __byte: 28 };
	/// Reset strikethrough mode.
	pub const RESET_STRIKE: Code = Code { __byte: 29 };

	/// Black foreground color.
	pub const BLACK: Code = Code { __byte: 30 };
	/// Red foreground color.
	pub const RED: Code = Code { __byte: 31 };
	/// Green foreground color.
	pub const GREEN: Code = Code { __byte: 32 };
	/// Yellow foreground color.
	pub const YELLOW: Code = Code { __byte: 33 };
	/// Blue foreground color.
	pub const BLUE: Code = Code { __byte: 34 };
	/// Magenta foreground color.
	pub const MAGENTA: Code = Code { __byte: 35 };
	/// Cyan foreground color.
	pub const CYAN: Code = Code { __byte: 36 };
	/// White foreground color.
	pub const WHITE: Code = Code { __byte: 37 };
	/// Default foreground color.
	pub const DEFAULT: Code = Code { __byte: 39 };

	/// Black background color.
	pub const BLACK_BG: Code = Code { __byte: 40 };
	/// Red background color.
	pub const RED_BG: Code = Code { __byte: 41 };
	/// Green background color.
	pub const GREEN_BG: Code = Code { __byte: 42 };
	/// Yellow background color.
	pub const YELLOW_BG: Code = Code { __byte: 43 };
	/// Blue background color.
	pub const BLUE_BG: Code = Code { __byte: 44 };
	/// Magenta background color.
	pub const MAGENTA_BG: Code = Code { __byte: 45 };
	/// Cyan background color.
	pub const CYAN_BG: Code = Code { __byte: 46 };
	/// White background color.
	pub const WHITE_BG: Code = Code { __byte: 47 };
	/// Default background color.
	pub const DEFAULT_BG: Code = Code { __byte: 49 };

	/// Bright black foreground color.
	pub const BRIGHT_BLACK: Code = Code { __byte: 90 };
	/// Bright red foreground color.
	pub const BRIGHT_RED: Code = Code { __byte: 91 };
	/// Bright green foreground color.
	pub const BRIGHT_GREEN: Code = Code { __byte: 92 };
	/// Bright yellow foreground color.
	pub const BRIGHT_YELLOW: Code = Code { __byte: 93 };
	/// Bright blue foreground color.
	pub const BRIGHT_BLUE: Code = Code { __byte: 94 };
	/// Bright magenta foreground color.
	pub const BRIGHT_MAGENTA: Code = Code { __byte: 95 };
	/// Bright cyan foreground color.
	pub const BRIGHT_CYAN: Code = Code { __byte: 96 };
	/// Bright white foreground color.
	pub const BRIGHT_WHITE: Code = Code { __byte: 97 };

	/// Bright black background color.
	pub const BRIGHT_BLACK_BG: Code = Code { __byte: 100 };
	/// Bright red background color.
	pub const BRIGHT_RED_BG: Code = Code { __byte: 101 };
	/// Bright green background color.
	pub const BRIGHT_GREEN_BG: Code = Code { __byte: 102 };
	/// Bright yellow background color.
	pub const BRIGHT_YELLOW_BG: Code = Code { __byte: 103 };
	/// Bright blue background color.
	pub const BRIGHT_BLUE_BG: Code = Code { __byte: 104 };
	/// Bright magenta background color.
	pub const BRIGHT_MAGENTA_BG: Code = Code { __byte: 105 };
	/// Bright cyan background color.
	pub const BRIGHT_CYAN_BG: Code = Code { __byte: 106 };
	/// Bright white background color.
	pub const BRIGHT_WHITE_BG: Code = Code { __byte: 107 };
}

pub use self::codes::*;

impl fmt::Display for Code {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut buf = [0u8; 8];
		f.write_str(display(slice::from_ref(&self.__byte), &mut buf).ok_or(fmt::Error)?)?;
		Ok(())
	}
}

impl fmt::Debug for Code {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\"\\x1b[{}m\"", self.__byte)
	}
}

/// Format graphics codes as an ANSI escape sequence.
///
/// Create an instance using the [`mode!`] macro.
pub struct Print<T: AsRef<[u8]>> {
	#[doc(hidden)]
	pub __codes: T,
}

impl<T: AsRef<[u8]>> Print<T> {
	/// Normalizes the generic type to `&[u8]`.
	pub fn erase<'a>(&'a self) -> Print<&'a [u8]> {
		Print { __codes: self.__codes.as_ref() }
	}
}

impl<T: AsRef<[u8]>> fmt::Display for Print<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let codes = self.__codes.as_ref();
		if codes.len() > 0 {
			let mut buf = [0u8; 64];
			f.write_str(display(codes, &mut buf).ok_or(fmt::Error)?)?;
		}
		Ok(())
	}
}

impl<T: AsRef<[u8]>> fmt::Debug for Print<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		debug(self.__codes.as_ref(), f)
	}
}

#[inline(never)]
fn debug(codes: &[u8], f: &mut fmt::Formatter) -> fmt::Result {
	if codes.len() > 0 {
		write!(f, "\"\\x1b[")?;
		for i in 0..codes.len() {
			let suffix = if i + 1 == codes.len() { 'm' } else { ';' };
			write!(f, "{}{}", codes[i], suffix)?;
		}
		write!(f, "\"")?;
	}
	Ok(())
}

#[inline]
fn display_code(mut code: u8, suffix: u8, buf: &mut [u8]) -> usize {
	if buf.len() < 4 {
		return 0;
	}

	let mut i = 0;
	if code >= 100 {
		buf[i] = b'0' + code / 100;
		code = code % 100;
		i += 1;
	}
	if code >= 10 {
		buf[i] = b'0' + code / 10;
		code = code % 10;
		i += 1;
	}
	buf[i] = b'0' + code;
	i += 1;
	buf[i] = suffix;
	i += 1;
	return i;
}

#[inline(never)]
fn display<'a>(codes: &[u8], buf: &'a mut [u8]) -> Option<&'a str> {
	if buf.len() < 3 {
		return None;
	}
	buf[0] = 0x1b;
	buf[1] = b'[';
	let mut total = 2;
	{
		let mut buf = &mut buf[2..];
		for i in 0..codes.len() {
			let suffix = if i + 1 == codes.len() { b'm' } else { b';' };
			let skip = display_code(codes[i], suffix, buf);
			if skip == 0 {
				return None;
			}
			total += skip;
			buf = &mut buf[skip..];
		}
	}
	let buf = &buf.get(..total)?;
	unsafe { Some(str::from_utf8_unchecked(buf)) }
}

#[cfg(test)]
mod tests;

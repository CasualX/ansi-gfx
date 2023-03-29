use super::*;

#[test]
fn simple() {
	assert_eq!(format!("{}", BOLD), "\x1b[1m");
	assert_eq!(format!("{}", RED), "\x1b[31m");
}

#[test]
fn mode() {
	let style = UNDERLINE;
	assert_eq!(format!("{}", mode!(BOLD; {style}; FG PAL 9; BG RGB 255, 0, 0)), "\u{1b}[1;4;38;5;9;48;2;255;0;0m");
}

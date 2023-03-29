
fn main() {
	println!("Attributes:\n");

	let attrs = [
		("BOLD", ansi_gfx::BOLD),
		("DIM", ansi_gfx::DIM),
		("ITALIC", ansi_gfx::ITALIC),
		("UNDERLINE", ansi_gfx::UNDERLINE),
		("BLINK", ansi_gfx::BLINK),
		("INVERSE", ansi_gfx::INVERSE),
		("HIDDEN", ansi_gfx::HIDDEN),
		("STRIKE", ansi_gfx::STRIKE),
	];
	for &(text, attr) in &attrs {
		println!("{}{}{}", attr, text, ansi_gfx::RESET);
	}


	println!("\nColors:\n");

	let colors = [
		("BLACK", ansi_gfx::BLACK, ansi_gfx::BLACK_BG, ansi_gfx::BRIGHT_BLACK, ansi_gfx::BRIGHT_BLACK_BG),
		("RED", ansi_gfx::RED, ansi_gfx::RED_BG, ansi_gfx::BRIGHT_RED, ansi_gfx::BRIGHT_RED_BG),
		("GREEN", ansi_gfx::GREEN, ansi_gfx::GREEN_BG, ansi_gfx::BRIGHT_GREEN, ansi_gfx::BRIGHT_GREEN_BG),
		("YELLOW", ansi_gfx::YELLOW, ansi_gfx::YELLOW_BG, ansi_gfx::BRIGHT_YELLOW, ansi_gfx::BRIGHT_YELLOW_BG),
		("BLUE", ansi_gfx::BLUE, ansi_gfx::BLUE_BG, ansi_gfx::BRIGHT_BLUE, ansi_gfx::BRIGHT_BLUE_BG),
		("MAGENTA", ansi_gfx::MAGENTA, ansi_gfx::MAGENTA_BG, ansi_gfx::BRIGHT_MAGENTA, ansi_gfx::BRIGHT_MAGENTA_BG),
		("CYAN", ansi_gfx::CYAN, ansi_gfx::CYAN_BG, ansi_gfx::BRIGHT_CYAN, ansi_gfx::BRIGHT_CYAN_BG),
		("WHITE", ansi_gfx::WHITE, ansi_gfx::WHITE_BG, ansi_gfx::BRIGHT_WHITE, ansi_gfx::BRIGHT_WHITE_BG),
	];
	for &(text, fg, bg, fg_bright, bg_bright) in &colors {
		print!("{}{:<8}{} ", fg, text, ansi_gfx::RESET);
		print!("{}{:<8}{} ", fg_bright, text, ansi_gfx::RESET);
		print!("{}{:<8}{} ", bg, text, ansi_gfx::RESET);
		print!("{}{:<8}{} ", bg_bright, text, ansi_gfx::RESET);
		println!();
	}


	println!("\nPalette colors:\n");

	for i in 0..16 {
		if i == 8 {
			println!();
		}
		let fg = if i == 0 { ansi_gfx::WHITE } else { ansi_gfx::BLACK };
		let mode = ansi_gfx::mode!({fg}; BG PAL i);
		print!("{}{:>4} {}", mode.erase(), i, ansi_gfx::RESET);
	}
	println!("\n");

	let mut n = 0;
	for i in 16..232 {
		if n == 18 {
			n = 0;
			println!();
		}
		n += 1;
		let fg = if i < 34 { ansi_gfx::WHITE } else { ansi_gfx::BLACK };
		print!("{}{:>4} {}", ansi_gfx::mode!({fg}; BG PAL i), i, ansi_gfx::RESET);
	}

	println!("\n");
	for i in 232..244 {
		print!("{}{:>4} {}", ansi_gfx::mode!(WHITE; BG PAL i), i, ansi_gfx::RESET);
	}
	println!();
	for i in 244..256 {
		let i = i as u8;
		print!("{}{:>4} {}", ansi_gfx::mode!(BLACK; BG PAL i), i, ansi_gfx::RESET);
	}

	println!();
}

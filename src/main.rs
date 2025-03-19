use clap::{arg, command, Parser, Subcommand};
use turing_smart_screen::screen::{Orientation, Screen};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	/// Portrait orientation
	#[clap(short, long)]
	portrait: bool,

	/// Brightness level (optional)
	#[clap(short, long, default_value = "100")]
	brightness: u8,

	#[command(subcommand)]
	command: Commands
}

#[derive(Subcommand)]
enum Commands {
	/// Test for a screen
	/// This command will check if it detects a screen
	Test {},
	/// Clear the screen
	Clear {},
	/// Draw an image
	Draw {
		/// Image path
		#[arg(short, long)]
		path: String,
	},
	/// Draw a text
	Text {
		/// Text to draw
		#[arg(short, long)]
		text: String,
		/// Font file path
		#[arg(short, long)]
		font: String,
	},
}

fn main() {
	let args = Cli::parse();

	let port = Screen::find_port().expect("No port found");
	let mut screen = Screen::new(port).expect("Failed to open port");

	if let Commands::Test {} = args.command {
		println!("Screen detected");
		return;
	}

	screen.brightness(args.brightness).expect("Failed to set brightness");

	if let Commands::Clear {} = args.command {
		screen.orientation(Orientation::Portrait).expect("Failed to set orientation");
		screen.clear().expect("Failed to clear screen");
		return;
	}

	if args.portrait {
		screen.orientation(Orientation::Portrait).expect("Failed to set orientation");
	} else {
		screen.orientation(Orientation::Landscape).expect("Failed to set orientation");
	}

	if let Commands::Draw { path } = args.command {
		let img = image::ImageReader::open(path).unwrap().decode().unwrap();
		screen.draw(img.into()).expect("Failed to draw image");
	} else if let Commands::Text { text, font } = args.command {
		let mut image = ril::Image::new(480, 320, ril::Rgb::black());
		// Open the font at the given path. You can try using `Font::from_bytes` along with the `include_bytes!` macro
		// since fonts can usually be statically loaded.
		let font = ril::Font::open(
				font,
				// Do note that the following is a specified optimal size
				// and not a fixed size for the font. It specifies what size
				// to optimize rasterizing for. You do not have to load the same
				// font multiple times for different sizes.
				22.0,
		).expect("Failed to load font!");

		let lines: Vec<&str> = text.split("\\n").collect();
		for (i, line) in lines.iter().enumerate() {
			let text_segment = ril::TextSegment::new(&font, line, ril::Rgb::white())
				.with_position(20, 20 + i as u32 * 30); // Adjust the vertical spacing as needed
			image.draw(&text_segment);
		}
		image.save_inferred("/tmp/text.png").expect("Failed to save");

		let img = image::ImageReader::open("/tmp/text.png").unwrap().decode().unwrap();
		screen.draw(img.into()).expect("Failed to draw image");

		// Clean up the temporary image
		std::fs::remove_file("/tmp/text.png").expect("Failed to remove temporary image");
	}
}

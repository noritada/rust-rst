use structopt::StructOpt;
use clap::arg_enum;
use quicli::{
	fs::read_file,
	prelude::{CliResult,Verbosity},
};

use rst_parser::parse;
use rst_renderer::{
	render_json,
	render_xml,
	render_html,
};

arg_enum! {
	#[derive(Debug)]
	#[allow(non_camel_case_types)]
	enum Format { json, xml, html }
}

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
	#[structopt(
		long = "format", short = "f", default_value = "html",  // xml is pretty defunct…
		raw(possible_values = "&Format::variants()", case_insensitive = "true"),
	)]
	format: Format,
	file: String,
	#[structopt(flatten)]
	verbosity: Verbosity,
}

fn main() -> CliResult {
	let args = Cli::from_args();
	args.verbosity.setup_env_logger("rst")?;
	
	let content = read_file(args.file)?;
	let document = parse(&content)?;
	let stdout = std::io::stdout();
	match args.format {
		Format::json => render_json(&document, stdout)?,
		Format::xml  => render_xml (&document, stdout)?,
		Format::html => render_html(&document, stdout, true)?,
	}
	Ok(())
}

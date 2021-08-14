/*
 *  This one is dedicated to all the sexy sysadmoms out there.
 */

use structopt::StructOpt;
use std::process;

// TODO: Populate these from config file
const NBHOST:     &str = "";
const NBTESTHOST: &str = "";

#[derive(Debug, StructOpt)]
#[structopt(name = "nba", about = "netbox agent")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Run against netbox-test deployment
    #[structopt(short, long)]
    test: bool,

    // / Set speed
    // we don't want to name it "speed", need to look smart
    //#[structopt(short = "v", long = "velocity", default_value = "42")]
    //speed: f64,

    // / Input file
    //#[structopt(parse(from_os_str))]
    //input: PathBuf,

    // / Output file, stdout if not present
    //#[structopt(parse(from_os_str))]
    //output: Option<PathBuf>,

    // / Where to write the output: to `stdout` or `file`
    //#[structopt(short)]
    //out_type: String,

    // / File name: only required when `out-type` is set to `file`
    //#[structopt(name = "FILE", required_if("out-type", "file"))]
    //file_name: Option<String>,
}

fn main() {
    // Process command line options
    let opt = Opt::from_args();
    if opt.debug {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    log::debug!("{:?}", opt);

    let usehost: &str;
    match opt.test {
        true  => { usehost = NBTESTHOST; }
        false => { usehost = NBHOST; }
    }

    // Run the nba program
    if let Err(e) = nba::run(usehost) {
        eprintln!("oh noes {}", e);
        process::exit(1);
    }
}

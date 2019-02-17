extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;
extern crate serde_json;
extern crate bars;

use std::io::Error;
use clap::App;
use clap::Arg;
use std::collections::BTreeMap;
use util::parser;
use time::PreciseTime;
use core::stats;
use core::stats::Stats;
use bars::domain::bar::Bar;
use bars::domain::period::Period;

mod util;
mod core;


fn main() -> Result<(), Box<Error>> {
    env_logger::init();
    let matches = App::new("Stats")
        .version("0.1.0")
        .author("Maksim Vakhnik <m.vakhnik.dev@gmail.com>")
        .usage("stats FILES||(FILE1 FILE2)")
        .about(
            "Get stats of bars",
        ).arg(
        Arg::with_name("file1")
            .takes_value(true)
            .required(true)
            .index(1)
            .help(r#"combinations of files in format [file1] [file2]"#),
    ).arg(
        Arg::with_name("file2")
            .takes_value(true)
            .required(false)
            .index(2)
            .help(r#"combinations of files in format [file1] [file2]"#),
    ).arg(
        Arg::with_name("both")
            .long("both")
            .takes_value(false)
            .short("b")
            .help("Both direction")
    ).arg(
        Arg::with_name("verbose")
            .long("verbose")
            .takes_value(true)
            .short("v")
            .help("Verbosity level: 1 (default), 2")
    ).get_matches();
    let (file1, file2) = match matches.value_of("file2") {
        Some(file) => (matches.value_of("file1").expect("file1 is not defined"), file),
        None => {
            let files = matches.value_of("file1").expect("files are not defined");
            let paths: Vec<&str> = serde_json::from_str(files).expect("incorrect files format");
            (*paths.get(0).expect("file1 is not defined"), *paths.get(1).expect("file2 is not defined"))
        }
    };
    let both = matches.is_present("both");
    debug!("file1:{}", file1);
    debug!("file2:{}", file2);
    debug!("both:{}", both);
//    let all_bars = &mut BTreeMap::new();

    let t1= ticker(file1);
    let t2= ticker(file2);
    let bars1 = &parser::parse(file1, t1);
    debug!("bars1:{}", bars1.len());
    debug!("bars1.head:{:?}", bars1.iter().next());
    let bars2 = &parser::parse(file2, t2);
    debug!("bars2:{}", bars2.len());
    debug!("bars2.head:{:?}", bars2.iter().next());
    let stat1 = stats_next(bars1, bars2);
    print_stats(t1, t2, stat1);
    if both {
        let stat2 = stats_next(bars2, bars1);
        print_stats(t2, t1, stat2);
    }
    Ok(())
}
fn print_stats(t1: &str, t2: &str, stats: Stats) {
    println!("{}-{}\t{}", t1, t2, format(stats));
}
fn stats_next(bars1: &BTreeMap<Period, Bar>, bars2: &BTreeMap<Period, Bar>) -> Stats {
    let mut s = PreciseTime::now();
    let norm1 = &util::normalizer::norm(bars1, bars2);
    debug!(
        "normalize:{}, ms",
        s.to(PreciseTime::now()).num_milliseconds()
    );
    debug!("normalized.len:{}", norm1.len());
    debug!("normalized.head:{:?}", norm1.iter().next());
    s = PreciseTime::now();
    let stat = stats::direction_next(norm1);
    debug!(
        "stats:{}, ms",
        s.to(PreciseTime::now()).num_milliseconds()
    );
    stat

}
fn format(stats: Stats) -> String{
    format!(
        "same:{}\topp:{}\tunclear:{}\tsame_perc:{:.*}",
        stats.same.len(),
        stats.opp.len(),
        stats.unclear.len(),
        2, stats.same_perc()
    )
}
fn ticker(path: &str) -> &str {
    path.rsplit("/").next().expect(format!("Cannot parse ticker from path:{}", path).as_str())
}

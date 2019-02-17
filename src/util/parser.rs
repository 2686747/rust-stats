use std::collections::BTreeMap;
use domain::period::Period;
use domain::bar::Bar;
use std::fs::File;
use std::io::Read;
use time::PreciseTime;
use std::error::Error;
use std::iter::FromIterator;
use std::io::BufReader;
use std::io::BufRead;

pub fn parse(path: &str, ticker: &str) -> BTreeMap<Period, Bar> {
    let mut s = PreciseTime::now();

    let res = parse_with_vec(read_file_buff(path), ticker);
    debug!(
        "parse_with_vec1:{}, ms",
        s.to(PreciseTime::now()).num_milliseconds()
    );
    BTreeMap::from(res)
}

fn read_file_buff(path: &str) -> BufReader<File> {
    let mut f = File::open(path).expect(format!("cannot find file:{}", path).as_str());
    BufReader::new(f)
}

fn parse_with_vec(csv: BufReader<File>, ticker: &str) -> BTreeMap<Period, Bar> {

    let bars = csv.lines().filter_map(|line| {
        match bar(&line.unwrap(), ticker) {
            Ok(b) => Some((Period::from(&b), b)),
            Err(e) => None
        }
    });
    BTreeMap::from_iter(bars.into_iter())
}
fn bar(csv_row: &str, ticker: &str) -> Result<Bar, Box<Error>> {
    Bar::from_1(csv_row, ticker)
}


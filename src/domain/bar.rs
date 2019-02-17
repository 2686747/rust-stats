use std::error::Error;
use std::hash::Hash;
use std::hash::Hasher;
use std::str::Split;

#[derive(Debug, Clone)]
pub struct Bar {
    pub ticker: String,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}
impl Bar {
    pub fn price(&self) -> f32 {
        self.open
    }
    //Date,Open,High,Low,Close,Volume,OpenInt
    // 2011-08-15,20.44,20.46,20.44,20.46,440,0
    pub fn from_1(csv_row: &str, ticker: &str) -> Result<Bar, Box<Error>> {
        let mut split: Split<&str> = csv_row.split(",");
        let v = split
            .next()
            .expect(format!("cannot get date from string:{}", csv_row).as_str());
        let mut date = v.split("-");
        let y = try!(date.next().expect("cannot get year").parse::<u16>());
        let m = try!(date.next().expect("cannot get month").parse::<u8>());
        let d = try!(date.next().expect("cannot get day").parse::<u8>());
        let open = try!(split.next().expect("cannot get open").parse::<f32>());
        let high = try!(split.next().expect("cannot get high").parse::<f32>());
        let low = try!(split.next().expect("cannot get low").parse::<f32>());
        let close = try!(split.next().expect("cannot get close").parse::<f32>());
        Ok(Bar {
            ticker: ticker.to_string(),
            year: y,
            month: m,
            day: d,
            hour: 0,
            min: 0,
            open: open,
            high: high,
            low: low,
            close: close,
        })
    }
    pub fn from(csv_row: &str) -> Result<Bar, Box<Error>> {
        let mut split: Split<&str> = csv_row.split(";");

        let t = String::from(split.next().expect("cannot get ticker"));
        split.next();
        let v = split
            .next()
            .expect(format!("cannot get date from string:{}", csv_row).as_str());
        let y = try!(v[0..4].parse::<u16>());
        let m = try!(v[4..6].parse::<u8>());
        let d = try!(v[6..8].parse::<u8>());
        let time = split.next().expect("cannot get time");
        let hh = try!(time[0..2].parse::<u8>());
        let mm = try!(time[2..4].parse::<u8>());
        let open = try!(split.next().expect("cannot get open").parse::<f32>());
        let high = try!(split.next().expect("cannot get high").parse::<f32>());
        let low = try!(split.next().expect("cannot get low").parse::<f32>());
        let close = try!(split.next().expect("cannot get close").parse::<f32>());
        Ok(Bar {
            ticker: t,
            year: y,
            month: m,
            day: d,
            hour: hh,
            min: mm,
            open: open,
            high: high,
            low: low,
            close: close,
        })
    }
    pub fn from_bar(other: &Bar) -> Bar {
        Bar {
            ticker: other.ticker.to_owned(),
            year: other.year,
            month: other.month,
            day: other.day,
            hour: other.hour,
            min: other.min,
            open: other.open,
            high: other.high,
            low: other.low,
            close: other.close,
        }
    }
    pub fn value(&self) -> f32 {
        if self.open <= self.close {
            self.high - self.low
        } else {
            self.low - self.high
        }
    }
}

//impl<'a> From<&'a Bar> for Bar {
//    fn from(other: &'a Bar) -> Bar {
//        Bar{
//            ticker: String::from(*other.ticker),
//            year: other.year,
//            month: other.month,
//            day: other.day,
//            hour: other.hour,
//            min: other.min,
//            open: other.open,
//            high: other.high,
//            low: other.low,
//            close: other.close
//        }
//    }
//}
impl PartialEq for Bar {
    fn eq(&self, other: &Bar) -> bool {
        self.ticker == other.ticker
            && self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.min == other.min
    }
}
impl Eq for Bar {}
impl Hash for Bar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ticker.hash(state);
        self.year.hash(state);
        self.month.hash(state);
        self.day.hash(state);
        self.hour.hash(state);
        self.min.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn up_bar_volume() {
        let b = Bar {
            ticker: "test".to_string(),
            year: 2012,
            month: 1,
            day: 1,
            hour: 0,
            min: 0,
            open: 2.0,
            high: 3.0,
            low: 1.0,
            close: 2.5,
        };
        assert_eq!(2.0, b.value());
    }
    #[test]
    fn down_bar_volume() {
        let b = Bar {
            ticker: "test".to_string(),
            year: 2012,
            month: 1,
            day: 1,
            hour: 0,
            min: 0,
            open: 3.0,
            high: 3.0,
            low: 1.0,
            close: 2.5,
        };
        assert_eq!(-2.0, b.value());
    }
    #[test]
    fn from_1() {
        //Date,Open,High,Low,Close,Volume,OpenInt
        let b = Bar::from_1(
            "2011-08-15,20.44,20.46,20.47,20.46,440,0",
            "test",
        ).unwrap();
        assert_eq!(2011, b.year);
        assert_eq!(8, b.month);
        assert_eq!(15, b.day);
        assert_eq!(20.44, b.open);
        assert_eq!(20.46, b.high);
        assert_eq!(20.47, b.low);
        assert_eq!(20.46, b.close);
    }
}

use domain::bar::Bar;

#[derive(Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct Period {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8

}

impl<'a> From<&'a Period> for Period {
    fn from(p: &'a Period) -> Period {
        Period {
            year: p.year,
            month: p.month,
            day: p.day,
            hour: p.hour,
            min: p.min,
        }
    }
}
impl<'a> From<&'a Bar> for Period {
    fn from(p: &'a Bar) -> Period {
        Period {
            year: p.year,
            month: p.month,
            day: p.day,
            hour: p.hour,
            min: p.min,
        }
    }
}
impl Period {
    pub fn as_month(p: &Period) -> Period {
        let init = Period {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            min: 0,
        };
        Period {
            year: p.year,
            month: p.month,
            ..init
        }
    }
}

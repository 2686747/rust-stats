use std::collections::BTreeMap;
use bars::domain::bar::Bar;
use bars::domain::period::Period;
#[derive(Debug, Clone)]
pub struct Stats {
//    pub missing: Vec<(Option<Bar>, Option<Bar>)>,
    pub same: Vec<(Option<Bar>, Option<Bar>)>,
    pub opp: Vec<(Option<Bar>, Option<Bar>)>,
    pub unclear: Vec<(Option<Bar>, Option<Bar>)>
}
impl Stats {

    pub fn same_perc(&self) -> f32 {
        (self.same.len() * 100) as f32 / self.total() as f32
    }
    pub fn total(&self) -> usize{
        self.same.len() + self.opp.len() + self.unclear.len()
    }
}
//второй зависит от первого
pub fn direction_next(bars: &BTreeMap<Period, (Option<Bar>, Option<Bar>)>) -> Stats{
//    let mut missing = Vec::new();
    let mut same = Vec::new();
    let mut opp = Vec::new();
    let mut unclear = Vec::new();
    let mut primary = None;

    for pair in bars {
        if primary.is_none() {
            //присвоить и ждать следующего
            primary = (pair.1).0.clone();
        }
            else {
            //сравнить с текущим
            let depend = (pair.1).1.clone();
            if depend.is_some() {
                let v1 = primary.clone().unwrap().value();
                let v2 = depend.clone().unwrap().value();
  //                //в одну сторону
                if (v1 < 0.0 && v2 < 0.0) || (v1 > 0.0 && v2 > 0.0) {
                    same.push((primary.clone(), depend.clone()));
                } else if v1 == 0.0 && v2 == 0.0 {
                    unclear.push((primary.clone(), depend.clone()));
                } else {
                    opp.push((primary.clone(), depend.clone()));
                }
                primary = (pair.1).0.clone();
            } else {
                primary = None;
            }
        }
    }
    Stats{
        same,
        opp,
        unclear
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_stats_opp_direction() {
        let mut bars = BTreeMap::new();
        for x in 1..5 {
            let b1 = Bar {
                ticker: "t1".to_string(),
                year: 2012,
                month: 1,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 2.5,
            };
            let b2 = Bar {
                ticker: "t2".to_string(),
                year: 2012,
                month: 1,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 1.5,
            };
            let p = Period::from(&b1);
            bars.insert(p, (Some(b1.clone()), Some(b2.clone())));
        }
        //пропустить парочку
        for x in 1..5 {
            let b1 = Bar {
                ticker: "t1".to_string(),
                year: 2012,
                month: 2,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 1.5,
            };

            let p = Period::from(&b1);
            bars.insert(p, (Some(b1.clone()), None));
        }
        for x in 1..5 {
            let b1 = Bar {
                ticker: "t1".to_string(),
                year: 2012,
                month: 3,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 1.5,
            };
            let b2 = Bar {
                ticker: "t2".to_string(),
                year: 2012,
                month: 3,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 2.5,
            };
            let p = Period::from(&b1);
            bars.insert(p, (Some(b1.clone()), Some(b2.clone())));
        }
        let res = direction_next(&bars);
        assert_eq!(bars.len() - 5, res.opp.len());
        assert_eq!(0, res.same.len());
        assert_eq!(0, res.unclear.len());

    }
    #[test]
    fn correct_stats_same_direction() {
        let mut bars = BTreeMap::new();
        for x in 1..5 {
            let b1 = Bar {
                ticker: "t1".to_string(),
                year: 2012,
                month: 1,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 2.5,
            };
            let b2 = Bar {
                ticker: "t2".to_string(),
                year: 2012,
                month: 1,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 2.5,
            };
            let p = Period::from(&b1);
            bars.insert(p, (Some(b1.clone()), Some(b2.clone())));
        }
        //пропустить парочку
        for x in 1..5 {
            let b1 = Bar {
                ticker: "t1".to_string(),
                year: 2012,
                month: 2,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 1.5,
            };

            let p = Period::from(&b1);
            bars.insert(p, (Some(b1.clone()), None));
        }
        for x in 1..5 {
            let b1 = Bar {
                ticker: "t1".to_string(),
                year: 2012,
                month: 3,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 1.5,
            };
            let b2 = Bar {
                ticker: "t2".to_string(),
                year: 2012,
                month: 3,
                day: x,
                hour: 0,
                min: 0,
                open: 2.0,
                high: 3.0,
                low: 1.0,
                close: 2.5,
            };
            let p = Period::from(&b1);
            bars.insert(p, (Some(b1.clone()), Some(b2.clone())));
        }
        let res = direction_next(&bars);
        let res = direction_next(&bars);
        assert_eq!(3, res.same.len());
        assert_eq!(4, res.opp.len());
        assert_eq!(0, res.unclear.len());

    }

}

use std::collections::BTreeMap;
use domain::period::Period;
use domain::bar::Bar;
use std::iter::Map;

//заполняет периоды полностью
pub fn norm(b1: &BTreeMap<Period, Bar>, b2: &BTreeMap<Period, Bar>)
                     -> BTreeMap<Period, (Option<Bar>, Option<Bar>)> {
    let mut res = BTreeMap::new();
    for bar in b1 {
        let p = bar.0;
        let b = bar.1;
        res.insert(Period::from(p), (Some(b.clone()), None));
    }
    for bar in b2 {
        let t1 = bar.0;
        let b = bar.1;
        let e =
            res.entry(Period::from(t1)).or_insert((None, None));
        e.1 = Some(b.clone());
    }
    res
}


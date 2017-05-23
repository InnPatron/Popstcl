use std::ops::Range;

#[macro_use]
macro_rules! range {
    ($start: expr, $exclusive: expr) => {LineInfo { range: $start..$exclusive } }
}

#[macro_use]
macro_rules! location {
    ($location: expr) => { LineInfo { range: $location..$location+1 } }
}

#[derive(Clone, Debug)]
pub struct LineInfo {
    pub range: Range<usize>
}

impl LineInfo {
    //TODO: check for overlap of ranges while folding?
    pub fn collapse(list: &[LineInfo]) -> LineInfo {
        assert!(list.len() > 0);
        let mut iter = list.iter();
        let first = iter.next().unwrap().clone();

        iter.fold(first, |accu, next| LineInfo::span(&accu, next))
    }
    pub fn span(lhs: &LineInfo, rhs: &LineInfo) -> LineInfo {
        let mut info = LineInfo { range: 0..1};

        if lhs.range.start < rhs.range.start {
            info.range.start = lhs.range.start;
        } else {
            info.range.start = rhs.range.start;
        }

        if lhs.range.end > rhs.range.end {
            info.range.end = lhs.range.end;
        } else {
            info.range.end = rhs.range.end;
        }

        info
    }
}

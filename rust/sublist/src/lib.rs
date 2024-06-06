use std::fmt::Debug as DebugFmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + DebugFmt>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.eq(_second_list) {
        return Comparison::Equal;
    }

    let first_list_count = _first_list.iter().count();
    let second_list_count = _second_list.iter().count();
    if first_list_count > second_list_count {
        if _second_list.is_empty() {
            return Comparison::Superlist;
        }
        for (n, higher) in _first_list.iter().enumerate() {
            if higher == _second_list.iter().next().unwrap() {
                let matching_slice = _first_list.iter().skip(n).take(second_list_count);
                if matching_slice.eq(_second_list) {
                    return Comparison::Superlist;
                }
            }
        }
    } else if first_list_count < second_list_count {
        if _first_list.is_empty() {
            return Comparison::Sublist;
        }
        for (n, higher) in _second_list.iter().enumerate() {
            if higher == _first_list.iter().next().unwrap() {
                let matching_slice = _second_list.iter().skip(n).take(first_list_count);
                if matching_slice.eq(_first_list) {
                    return Comparison::Sublist;
                }
            }
        }
    }

    Comparison::Unequal
}

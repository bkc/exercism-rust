#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_len = _first_list.len();
    let second_len = _second_list.len();
    if first_len == 0 {
        if second_len == 0 {
            return Comparison::Equal;
        }
        return Comparison::Sublist;
    } else if second_len == 0 {
        return Comparison::Superlist;
    }
    if first_len < second_len {
        for chunk in _second_list.windows(first_len) {
            if chunk == _first_list {
                return Comparison::Sublist;
            }
        }
    } else if first_len > second_len {
        for chunk in _first_list.windows(second_len) {
            if chunk == _second_list {
                return Comparison::Superlist;
            }
        }
    } else if _second_list == _first_list {
        return Comparison::Equal;
    }
    Comparison::Unequal
}

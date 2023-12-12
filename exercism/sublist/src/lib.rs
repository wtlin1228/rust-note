#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_contain<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    // phase 1: build the LSP
    let mut lsp = vec![0; needle.len()];
    let mut ptr = 0;
    let mut i = 1;
    while i < needle.len() {
        if needle[i] == needle[ptr] {
            lsp[i] = ptr + 1;
            i += 1;
            ptr += 1;
        } else if ptr == 0 {
            lsp[i] = 0;
            i += 1;
        } else {
            ptr = lsp[ptr - 1];
        }
    }

    // phase 2: find needle from the haystack
    ptr = 0;
    i = 0;
    while i < haystack.len() {
        if haystack[i] == needle[ptr] {
            i += 1;
            ptr += 1;
        } else if ptr == 0 {
            i += 1;
        } else {
            ptr = lsp[ptr - 1];
        }

        if ptr == needle.len() {
            return true;
        }
    }

    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        for i in 0.._first_list.len() {
            if _first_list[i] != _second_list[i] {
                return Comparison::Unequal;
            }
        }
        return Comparison::Equal;
    } else if _first_list.len() == 0 {
        return Comparison::Sublist;
    } else if _second_list.len() == 0 {
        return Comparison::Superlist;
    } else if _first_list.len() > _second_list.len() {
        if is_contain(_first_list, _second_list) {
            return Comparison::Superlist;
        } else {
            return Comparison::Unequal;
        }
    } else {
        if is_contain(_second_list, _first_list) {
            return Comparison::Sublist;
        } else {
            return Comparison::Unequal;
        }
    }
}
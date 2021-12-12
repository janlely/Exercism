#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        let size = _first_list.len();
        let res = (0..size).find(|idx: &usize| _first_list[*idx] != _second_list[*idx]);
        match res {
            Some(_) => Comparison::Unequal,
            None => Comparison::Equal
        }
     }else if _first_list.len() < _second_list.len() {
         for i in 0.._second_list.len() - _first_list.len() {
             if _first_list == &_second_list[i..i+_first_list.len()] {
                 return Comparison::Sublist;
             }
         }
         return Comparison::Unequal;
     }else {
         for i in 0.._first_list.len() - _second_list.len() {
             if _second_list== &_first_list[i..i+_second_list.len()] {
                 return Comparison::Superlist;
             }
         }
         return Comparison::Unequal;
     }
}

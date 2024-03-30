pub fn has_duplicates<T: Eq + std::hash::Hash>(list: &[T]) -> bool {
    let mut set = std::collections::HashSet::new();
    for item in list {
        if !set.insert(item) {
            return true;
        }
    }
    false
}

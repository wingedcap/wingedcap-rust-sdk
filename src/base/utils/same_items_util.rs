pub fn same_items<T>(vec1: &Vec<T>, vec2: &Vec<T>, cmp: fn(&T, &T) -> bool) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }

    for a in vec1 {
        let mut found = false;

        for b in vec2 {
            if cmp(a, b) {
                found = true;
                break;
            }
        }

        if !found {
            return false;
        }
    }

    for b in vec2 {
        let mut found = false;

        for a in vec1 {
            if cmp(a, b) {
                found = true;
                break;
            }
        }

        if !found {
            return false;
        }
    }

    true
}

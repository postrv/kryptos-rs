// transposition.rs

pub fn columnar_transposition(text: &str, key: &str) -> String {
    let key_len = key.len();
    let text_len = text.len();
    let mut result = vec![' '; text_len];

    let mut key_indices: Vec<_> = key.chars().enumerate().collect();
    key_indices.sort_by_key(|&(_, c)| c);

    let mut col = 0;
    for &(i, _) in &key_indices {
        let mut row = i;
        while row < text_len {
            result[col] = text.chars().nth(row).unwrap();
            row += key_len;
            col += 1;
        }
    }

    result.into_iter().collect()
}

pub fn route_transposition(text: &str, key: &str) -> String {
    let key_len = key.len();
    let text_len = text.len();
    let mut result = vec![' '; text_len];

    let mut index: isize = 0;
    let mut direction: isize = 1;
    for c in text.chars() {
        result[index as usize] = c;
        index += direction * key_len as isize;

        if index >= text_len as isize || index < 0 {
            direction *= -1;
            index += direction * key_len as isize;
            index += direction;
        }
    }

    result.into_iter().collect()
}

pub fn solve(input: Vec<String>) -> (String, String) {
    let mut list_a: Vec<u32> = Vec::new();
    let mut list_b: Vec<u32> = Vec::new();

    for line in &input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse(), parts[1].parse()) {
                list_a.push(a);
                list_b.push(b);
            }
        }
    }

    assert!(list_a.len() == list_b.len() && list_b.len() == input.len());

    let list_c = list_a.clone();
    let list_d = list_b.clone();

    // part 1
    let mut total_distance: u64 = 0;
    while !list_a.is_empty() {
        let (index_a, &min_a) = list_a
            .iter()
            .enumerate()
            .min_by_key(|&(_, item)| item)
            .unwrap();

        let (index_b, &min_b) = list_b
            .iter()
            .enumerate()
            .min_by_key(|&(_, item)| item)
            .unwrap();

        total_distance += u64::from(min_a.max(min_b) - min_a.min(min_b));

        list_a.remove(index_a);
        list_b.remove(index_b);
    }

    // part 2
    let mut similarity_score: u64 = 0;
    for item_c in &list_c {
        for item_d in &list_d {
            if item_c == item_d {
                similarity_score += *item_c as u64;
            }
        }
    }

    (total_distance.to_string(), similarity_score.to_string())
}

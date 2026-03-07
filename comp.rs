// took 4700-4800 ms [3816ms, ]
fn get_max_direct_access(line: &str) -> u32 {
    let mut max = 0;
    let chars = line.as_bytes();
    let len = line.len();
    for n1 in 0..len-1 {
        let ch1 = chars[n1] as char;
        for n2 in (n1+1)..len {
            let ch2 = chars[n2] as char;
            let pair_number = [ch1, ch2].into_iter().collect::<String>().parse::<u32>().unwrap();
            max = pair_number.max(max);
        }
    }
    max
}

// took 1680 ms
fn get_max_indices(line: &str) -> u32 {
    let mut max = 0;
    let len = line.len();
    for n1 in 0..len-1 {
        for n2 in (n1+1)..len {
            let ch1 = line.get(n1..=n1).unwrap();
            let ch2 = line.get(n2..=n2).unwrap();
            let pair = [ch1, ch2].into_iter().collect::<String>();
            let pair_number = pair.parse::<u32>().unwrap();
            max = pair_number.max(max);
        }
    }
    max
}
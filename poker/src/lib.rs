use std::collections::HashMap;

pub fn parse_card(c: &str) -> (u8, &str) {
    //  the two parts: v (the prefix) and n (the suffix).
    let (v, n) = c.split_at(c.len() - 1);

    (
        match v.parse::<u8>() {
            Ok(v) => v,
            Err(_) => "JQKA".find(v).unwrap() as u8 + 11
        },
        n
    )
}

pub fn score(hand: &str) -> (Vec<usize>, Vec<u8>) {
    // it unzips these tuples into two separate vectors: vs containing u8 values and ns containing string slices.
    let (vs, ns): (Vec<u8>, Vec<&str>) = hand.split_whitespace().map(parse_card).unzip();

    let mut hs: Vec<_> = vs.iter().fold(HashMap::<u8, usize>::new(), |mut h, &v| {
        *h.entry(v).or_default() += 1;
        h
    }).into_iter().map(|(k, v)| (v, k)).collect();

    hs.sort_by(|u, v| v.cmp(u));

    // This line unzips the vector hs into two separate vectors: cs containing the counts of each value and vs containing the values themselves.
    let (mut cs, mut vs): (Vec<_>, Vec<_>) = hs.into_iter().unzip();

    if cs.len() == 5 {
        if vs == [14, 5, 4, 3, 2] {
            vs = vec![5, 4, 3, 2, 1];
        }

        let is_straight: bool = vs[0] - vs[4] == 4;
        let is_flush = ns.windows(2).all(|w| w[0] == w[1]);

        match (is_straight, is_flush) {
            (true, true) => cs = vec![5],
            (false, true) => cs = vec![3, 1, 3],
            (true, false) => cs = vec![3, 1, 2],
            _ => {}
        }
    }

    (cs, vs)
    
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let scores = hands.iter().map(|&h| (score(h), h)).collect::<Vec<_>>();
    let (max_score, _) = scores.iter().max_by_key(|(score, _)| score).unwrap();

    scores.iter().filter(|(score, _)| score == max_score).map(|(_, hand)| *hand).collect::<Vec<_>>()
}

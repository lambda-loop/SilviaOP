use std::collections::HashSet;

pub fn swap(r: &[u8]) -> Vec<u8> {
    if r.len() < 2 {
        return r.to_vec();
    }

    let mut candidate = r.to_vec();

    let i = rand::random_range(0..candidate.len());
    let mut j = rand::random_range(0..candidate.len());

    while i == j {
        j = rand::random_range(0..candidate.len());
    }

    candidate.swap(i, j);

    candidate
}

pub fn two_opt(r: &[u8]) -> Vec<u8> {
    if r.len() < 2 {
        return r.to_vec();
    }

    let mut candidate = r.to_vec();

    let i = rand::random_range(0..candidate.len());
    let j = rand::random_range(i..candidate.len());

    candidate[i..=j].reverse();

    candidate
}

pub fn relocate(r: &[u8]) -> Vec<u8> {
    if r.len() < 2 {
        return r.to_vec();
    }

    let mut candidate = r.to_vec();

    let i = rand::random_range(0..candidate.len());
    let x = candidate.remove(i);

    let j = rand::random_range(0..=candidate.len());
    candidate.insert(j, x);

    candidate
}

pub fn insert(r: &[u8], unvisited: &HashSet<u8>, max: u8) -> (Vec<u8>, HashSet<u8>) {
    let mut candidate = r.to_vec();
    let mut candidate_unvisited = unvisited.clone();

    if candidate_unvisited.is_empty() {
        return (candidate, candidate_unvisited);
    }

    let x = loop {
        let x = rand::random_range(0..max);

        if candidate_unvisited.contains(&x) {
            break x;
        }
    };

    let i = rand::random_range(0..=candidate.len());

    candidate.insert(i, x);
    candidate_unvisited.remove(&x);

    (candidate, candidate_unvisited)
}

pub fn remove(r: &[u8], unvisited: &HashSet<u8>) -> (Vec<u8>, HashSet<u8>) {
    let mut candidate = r.to_vec();
    let mut candidate_unvisited = unvisited.clone();

    if candidate.len() <= 1 {
        return (candidate, candidate_unvisited);
    }

    let i = rand::random_range(0..candidate.len());
    let x = candidate.remove(i);

    candidate_unvisited.insert(x);

    (candidate, candidate_unvisited)
}
pub fn replace(r: &[u8], unvisited: &HashSet<u8>, max: u8) -> (Vec<u8>, HashSet<u8>) {
    let mut candidate = r.to_vec();
    let mut candidate_unvisited = unvisited.clone();

    if candidate.is_empty() || candidate_unvisited.is_empty() {
        return (candidate, candidate_unvisited);
    }

    let x = loop {
        let x = rand::random_range(0..max);

        if candidate_unvisited.contains(&x) {
            break x;
        }
    };

    let i = rand::random_range(0..candidate.len());
    let old = std::mem::replace(&mut candidate[i], x);

    candidate_unvisited.remove(&x);
    candidate_unvisited.insert(old);

    (candidate, candidate_unvisited)
}

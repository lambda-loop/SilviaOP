// im doing the change inside the local searchs provided a way to choose if the change will or not be performed after all.

use std::collections::HashSet;

type Validator = fn(&[u8]) -> bool;

pub fn swap(r: &mut Vec<u8>, p: Validator) {
    if r.len() < 2 {
        return;
    }

    let mut candidate = r.clone();

    let i = rand::random_range(0..candidate.len());
    let mut j = rand::random_range(0..candidate.len());

    while i == j {
        j = rand::random_range(0..candidate.len());
    }

    candidate.swap(i, j);
    if p(&candidate) {
        *r = candidate;
    }
}

pub fn two_opt(r: &mut Vec<u8>, p: Validator) {
    if r.len() < 2 {
        return;
    }

    let mut candidate = r.clone();

    let i = rand::random_range(0..candidate.len());
    let j = rand::random_range(i..candidate.len());

    candidate[i..=j].reverse();

    if p(&candidate) {
        *r = candidate
    }
}

pub fn relocate(r: &mut Vec<u8>, p: Validator) {
    if r.len() < 2 {
        return;
    }

    let mut candidate = r.to_vec();

    let i = rand::random_range(0..candidate.len());
    let x = candidate.remove(i);

    let j = rand::random_range(0..=candidate.len());
    candidate.insert(j, x);

    if p(&candidate) {
        *r = candidate;
    }
}

pub fn insert(r: &mut Vec<u8>, unvisited: &mut HashSet<u8>, max: u8, p: Validator) {
    let mut candidate = r.clone();
    if unvisited.is_empty() {
        return;
    }

    let x = loop {
        let x = rand::random_range(0..max);

        if unvisited.contains(&x) {
            break x;
        }
    };

    let i = rand::random_range(0..=candidate.len());
    candidate.insert(i, x);

    if p(&candidate) {
        unvisited.remove(&x);
        *r = candidate;
    }
}

pub fn remove(r: &mut Vec<u8>, unvisited: &mut HashSet<u8>, p: Validator) {
    let mut candidate = r.clone();

    if candidate.len() <= 1 {
        return;
    }

    let i = rand::random_range(0..candidate.len());
    let x = candidate.remove(i);

    if p(&candidate) {
        *r = candidate;
        unvisited.insert(x);
    }
}
pub fn replace(r: &mut Vec<u8>, unvisited: &mut HashSet<u8>, max: u8, p: Validator) {
    let mut candidate = r.clone();

    if candidate.is_empty() || unvisited.is_empty() {
        return;
    }

    let x = loop {
        let x = rand::random_range(0..max);

        if unvisited.contains(&x) {
            break x;
        }
    };

    let i = rand::random_range(0..candidate.len());
    let old = std::mem::replace(&mut candidate[i], x);

    if p(&candidate) {
        unvisited.remove(&x);
        unvisited.insert(old);
        *r = candidate;
    }
}

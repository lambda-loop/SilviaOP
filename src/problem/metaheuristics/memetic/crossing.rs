
use crate::problem::route::TourStatus;
use rand::{
    *, rngs::ThreadRng,
};

pub type MatchMaker = Box<
    dyn for<'pop> Fn(
        &'pop [(Vec<u8>, f32, TourStatus)],
        &mut ThreadRng,
    ) -> (&'pop [u8], &'pop [u8])
        >;
pub type Mate = Box<dyn Fn(&[u8], &[u8], &mut ThreadRng) -> Vec<u8>>;

pub fn rejoin(
    l: &[u8],
    r: &[u8],
    rng: &mut ThreadRng,
) -> Vec<u8> {
    let (l_idx, r_idx) = if l.is_empty() || r.is_empty() {
        (l.len(), 0)
    } else {
        (
            rng.random_range(0..=l.len() / 2),
            rng.random_range(r.len() / 2..r.len()),
        )
    };

    let mut child = Vec::with_capacity(l_idx + r.len() - r_idx);
    let mut visited = [false; 256];

    for &v in l[..l_idx].iter().chain(r[r_idx..].iter()) {
        let idx = v as usize;

        if !visited[idx] {
            visited[idx] = true;
            child.push(v);
        }
    }

    child
}

pub fn random_match<'pop> (
    pop: &'pop [(Vec<u8>, f32, TourStatus)],
    rng: &mut ThreadRng,
) -> (&'pop [u8], &'pop [u8]) {
    let pop_len = pop.len();
    let i = rng.random_range(0..pop_len);
    let j = rng.random_range(0..pop_len);
    (&pop[i].0, &pop[j].0)
}

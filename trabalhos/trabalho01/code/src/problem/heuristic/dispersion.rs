use std::collections::HashSet;

type Arc = (u8, u8);

fn arcs(
    route: &[u8],
    start: u8,
    end: u8,
) -> HashSet<Arc> {
    let mut arcs =
        HashSet::with_capacity(route.len() + 1);

    let mut previous = start;

    for &u in route {
        arcs.insert((previous, u));
        previous = u;
    }

    arcs.insert((previous, end));

    arcs
}

/// Normalized adjacency / broken-pairs-style distance.
///
/// 0.0 -> identical adjacency structure
/// 1.0 -> no arcs in common
pub fn route_distance(
    a: &[u8],
    b: &[u8],
    start: u8,
    end: u8,
) -> f64 {
    let a = arcs(a, start, end);
    let b = arcs(b, start, end);

    let intersection =
        a.intersection(&b).count();

    let union =
        a.len() + b.len() - intersection;

    if union == 0 {
        return 0.0;
    }

    1.0 - intersection as f64 / union as f64
}

pub fn average_pairwise_distance(
    routes: &[Vec<u8>],
    start: u8,
    end: u8,
) -> f64 {
    if routes.len() < 2 {
        return 0.0;
    }

    let mut total = 0.0;
    let mut count = 0usize;

    for i in 0..routes.len() {
        for j in i + 1..routes.len() {
            total += route_distance(
                &routes[i],
                &routes[j],
                start,
                end,
            );

            count += 1;
        }
    }

    total / count as f64
}

pub fn average_knn_distance(
    routes: &[Vec<u8>],
    k: usize,
    start: u8,
    end: u8,
) -> f64 {
    if routes.len() < 2 {
        return 0.0;
    }

    let k = k.min(routes.len() - 1);

    let mut total = 0.0;

    for i in 0..routes.len() {
        let mut distances =
            Vec::with_capacity(routes.len() - 1);

        for j in 0..routes.len() {
            if i == j {
                continue;
            }

            distances.push(route_distance(
                &routes[i],
                &routes[j],
                start,
                end,
            ));
        }

        distances.sort_by(f64::total_cmp);

        let local =
            distances[..k].iter().sum::<f64>()
                / k as f64;

        total += local;
    }

    total / routes.len() as f64
}

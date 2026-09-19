use std::collections::HashSet;

// pub fn route_dispersion(routes: &[Vec<u8>]) -> (f32, usize) {
//     let mut distances = Vec::new();
//     let mut distinct_nodes = HashSet::new();

//     for route in routes {
//         distinct_nodes.extend(route.iter().copied());
//     }

//     let edges: Vec<HashSet<(u8, u8)>> = routes
//         .iter()
//         .map(|route| route.windows(2).map(|w| (w[0], w[1])).collect())
//         .collect();

//     for i in 0..edges.len() {
//         for j in (i + 1)..edges.len() {
//             let intersection = edges[i].intersection(&edges[j]).count();
//             let union = edges[i].union(&edges[j]).count();

//             let distance = if union == 0 {
//                 0.0
//             } else {
//                 1.0 - intersection as f32 / union as f32
//             };

//             distances.push(distance);
//         }
//     }

//     distances.sort_by(|a, b| a.total_cmp(b));

//     let dispersion = match distances.len() {
//         0 => 0.0,
//         n if n % 2 == 1 => distances[n / 2],
//         n => (distances[n / 2 - 1] + distances[n / 2]) / 2.0,
//     };

//     (dispersion, distinct_nodes.len())
// }

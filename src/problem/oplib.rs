use super::*;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeWeightType {
    Euc2D,
    Ceil2D,
    Geo,
    Att,
    Explicit,
}

impl EdgeWeightType {
    fn parse(s: &str) -> Self {
        match s.trim() {
            "EUC_2D" => Self::Euc2D,
            "CEIL_2D" => Self::Ceil2D,
            "GEO" => Self::Geo,
            "ATT" => Self::Att,
            "EXPLICIT" => Self::Explicit,
            other => panic!("unsupported EDGE_WEIGHT_TYPE: {other}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Section {
    None,
    NodeCoord,
    NodeScore,
    EdgeWeight,
    Depot,
    Ignore,
}

/*
 * Public interface.
 *
 * Receives the CONTENT of an OPLib file and directly returns
 * the Problem representation used by the rest of the project.
 */
pub fn parse(content: &str) -> Problem {
    let mut dimension: Option<usize> = None;
    let mut cost_limit: Option<f64> = None;
    let mut edge_weight_type: Option<EdgeWeightType> = None;
    let mut edge_weight_format: Option<String> = None;

    /*
     * First pass: header.
     */
    for raw_line in content.lines() {
        let line = raw_line.trim();

        if let Some(value) = header_value(line, "DIMENSION") {
            dimension = Some(
                value
                    .parse()
                    .expect("invalid DIMENSION")
            );
        }

        if let Some(value) = header_value(line, "COST_LIMIT") {
            cost_limit = Some(
                value
                    .parse()
                    .expect("invalid COST_LIMIT")
            );
        }

        if let Some(value) = header_value(line, "EDGE_WEIGHT_TYPE") {
            edge_weight_type = Some(
                EdgeWeightType::parse(value)
            );
        }

        if let Some(value) = header_value(line, "EDGE_WEIGHT_FORMAT") {
            edge_weight_format = Some(value.to_owned());
        }
    }

    let dimension =
        dimension.expect("missing DIMENSION");

    let cost_limit =
        cost_limit.expect("missing COST_LIMIT");

    let edge_weight_type =
        edge_weight_type.expect("missing EDGE_WEIGHT_TYPE");

    /*
     * Original OPLib representation.
     */
    let mut points =
        vec![None; dimension];

    let mut scores =
        vec![None; dimension];

    let mut depot: Option<usize> = None;

    let mut explicit_weights =
        Vec::<f64>::new();

    /*
     * Second pass: sections.
     */
    let mut section = Section::None;

    for raw_line in content.lines() {
        let line = raw_line.trim();

        if line.is_empty() {
            continue;
        }

        match line {
            "NODE_COORD_SECTION" => {
                section = Section::NodeCoord;
                continue;
            }

            "NODE_SCORE_SECTION" => {
                section = Section::NodeScore;
                continue;
            }

            "EDGE_WEIGHT_SECTION" => {
                section = Section::EdgeWeight;
                continue;
            }

            "DEPOT_SECTION" => {
                section = Section::Depot;
                continue;
            }

            "EOF" => {
                break;
            }

            _ => {}
        }

        /*
         * If another TSPLIB section that we do not care about
         * appears, just ignore it.
         */
        if line.ends_with("_SECTION") {
            section = Section::Ignore;
            continue;
        }

        match section {
            Section::None | Section::Ignore => {}

            Section::NodeCoord => {
                let mut values =
                    line.split_whitespace();

                let id: usize =
                    values
                        .next()
                        .expect("missing node id")
                        .parse()
                        .expect("invalid node id");

                let x: f64 =
                    values
                        .next()
                        .expect("missing x coordinate")
                        .parse()
                        .expect("invalid x coordinate");

                let y: f64 =
                    values
                        .next()
                        .expect("missing y coordinate")
                        .parse()
                        .expect("invalid y coordinate");

                assert!(
                    values.next().is_none(),
                    "too many values in NODE_COORD_SECTION line"
                );

                assert!(
                    id >= 1 && id <= dimension,
                    "node id out of range"
                );

                points[id - 1] =
                    Some(Point { x, y });
            }

            Section::NodeScore => {
                let mut values =
                    line.split_whitespace();

                let id: usize =
                    values
                        .next()
                        .expect("missing node id")
                        .parse()
                        .expect("invalid node id");

                let score: u32 =
                    values
                        .next()
                        .expect("missing node score")
                        .parse()
                        .expect("invalid node score");

                assert!(
                    values.next().is_none(),
                    "too many values in NODE_SCORE_SECTION line"
                );

                assert!(
                    id >= 1 && id <= dimension,
                    "node id out of range"
                );

                scores[id - 1] =
                    Some(score);
            }

            Section::EdgeWeight => {
                for value in line.split_whitespace() {
                    explicit_weights.push(
                        value
                            .parse()
                            .expect("invalid EDGE_WEIGHT_SECTION value")
                    );
                }
            }

            Section::Depot => {
                let id: i64 =
                    line
                        .parse()
                        .expect("invalid DEPOT_SECTION value");

                if id == -1 {
                    section = Section::None;
                    continue;
                }

                assert!(
                    id >= 1 && id as usize <= dimension,
                    "depot id out of range"
                );

                assert!(
                    depot.is_none(),
                    "more than one depot in OPLib instance"
                );

                /*
                 * OPLib ids are 1-based.
                 * Internally we use 0-based ids.
                 */
                depot = Some(id as usize - 1);
            }
        }
    }

    let depot =
        depot.expect("missing depot");

    let scores: Vec<u32> =
        scores
            .into_iter()
            .enumerate()
            .map(|(i, score)| {
                score.unwrap_or_else(|| {
                    panic!("missing score for node {}", i + 1)
                })
            })
            .collect();

    /*
     * Build the cost matrix in ORIGINAL OPLib indexing.
     */
    let original_costs =
        match edge_weight_type {
            EdgeWeightType::Explicit => {
                let format =
                    edge_weight_format
                        .as_deref()
                        .expect(
                            "EXPLICIT instance without EDGE_WEIGHT_FORMAT"
                        );

                build_explicit_matrix(
                    dimension,
                    format,
                    &explicit_weights,
                )
            }

            _ => {
                let points: Vec<Point> =
                    points
                        .into_iter()
                        .enumerate()
                        .map(|(i, point)| {
                            point.unwrap_or_else(|| {
                                panic!(
                                    "missing coordinates for node {}",
                                    i + 1
                                )
                            })
                        })
                        .collect();

                build_coordinate_matrix(
                    &points,
                    edge_weight_type,
                )
            }
        };

    /*
     * Convert from OPLib:
     *
     *     DEPOT -> ... -> DEPOT
     *
     * to the representation expected by this project:
     *
     *     0 (START) -> ... -> 1 (END)
     *
     * START and END are two internal copies of the same
     * physical OPLib depot.
     */

    let mut node_map =
        Vec::with_capacity(dimension + 1);

    node_map.push(depot); // internal START = 0
    node_map.push(depot); // internal END   = 1

    for node in 0..dimension {
        if node != depot {
            node_map.push(node);
        }
    }

    let len =
        node_map.len();

    assert_eq!(
        len,
        dimension + 1
    );

    /*
     * Scores in the internal representation.
     *
     * START and END get score 0.
     */
    let mut internal_scores =
        vec![0u32; len];

    for internal in 2..len {
        let original =
            node_map[internal];

        internal_scores[internal] =
            scores[original];
    }

    /*
     * Costs in the internal representation.
     */
    let mut costs =
        SquareMatrix::<f32>::new(len);

    for i in 0..len {
        for j in 0..len {
            let original_i =
                node_map[i];

            let original_j =
                node_map[j];

            costs[(i, j)] =
                original_costs[original_i][original_j] as f32;
        }
    }

    Problem {
        tmax: cost_limit as f32,
        costs,
        scores: internal_scores,
        len,
    }
}

fn header_value<'a>(
    line: &'a str,
    key: &str
) -> Option<&'a str> {
    let (left, right) =
        line.split_once(':')?;

    if left.trim() == key {
        Some(right.trim())
    } else {
        None
    }
}

/*
 * Coordinate-based distance matrix.
 */
fn build_coordinate_matrix(
    points: &[Point],
    edge_weight_type: EdgeWeightType,
) -> Vec<Vec<f64>> {
    let n =
        points.len();

    let distance:
        fn(&Point, &Point) -> f64 =
        match edge_weight_type {
            EdgeWeightType::Euc2D => euc_2d,
            EdgeWeightType::Ceil2D => ceil_2d,
            EdgeWeightType::Geo => geo,
            EdgeWeightType::Att => att,

            EdgeWeightType::Explicit => {
                unreachable!()
            }
        };

    let mut costs =
        vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            costs[i][j] =
                distance(
                    &points[i],
                    &points[j],
                );
        }
    }

    costs
}

/*
 * EUC_2D
 */
fn euc_2d(
    a: &Point,
    b: &Point
) -> f64 {
    let xd =
        a.x - b.x;

    let yd =
        a.y - b.y;

    (
        (xd * xd + yd * yd).sqrt()
        + 0.5
    ).trunc()
}

/*
 * CEIL_2D
 */
fn ceil_2d(
    a: &Point,
    b: &Point
) -> f64 {
    let xd =
        a.x - b.x;

    let yd =
        a.y - b.y;

    (
        xd * xd
        + yd * yd
    )
        .sqrt()
        .ceil()
}

/*
 * ATT
 */
fn att(
    a: &Point,
    b: &Point
) -> f64 {
    let xd =
        a.x - b.x;

    let yd =
        a.y - b.y;

    let rij =
        (
            (
                xd * xd
                + yd * yd
            )
            / 10.0
        )
        .sqrt();

    let tij =
        rij.trunc();

    if tij < rij {
        tij + 1.0
    } else {
        tij
    }
}

/*
 * GEO
 */
fn geo(
    a: &Point,
    b: &Point
) -> f64 {
    const PI: f64 =
        3.141592;

    const RRR: f64 =
        6378.388;

    fn to_radians(
        coord: f64
    ) -> f64 {
        let deg =
            coord.trunc();

        let min =
            coord - deg;

        PI
            * (
                deg
                + 5.0 * min / 3.0
            )
            / 180.0
    }

    let latitude_a =
        to_radians(a.x);

    let longitude_a =
        to_radians(a.y);

    let latitude_b =
        to_radians(b.x);

    let longitude_b =
        to_radians(b.y);

    let q1 =
        (
            longitude_a
            - longitude_b
        )
        .cos();

    let q2 =
        (
            latitude_a
            - latitude_b
        )
        .cos();

    let q3 =
        (
            latitude_a
            + latitude_b
        )
        .cos();

    (
        RRR
            * (
                0.5
                    * (
                        (1.0 + q1) * q2
                        - (1.0 - q1) * q3
                    )
            )
            .acos()
        + 1.0
    )
    .trunc()
}

/*
 * EXPLICIT
 */
fn build_explicit_matrix(
    n: usize,
    format: &str,
    values: &[f64],
) -> Vec<Vec<f64>> {
    let mut matrix =
        vec![vec![0.0; n]; n];

    let mut values =
        values.iter().copied();

    match format {
        "FULL_MATRIX" => {
            for i in 0..n {
                for j in 0..n {
                    matrix[i][j] =
                        next_weight(&mut values);
                }
            }
        }

        "UPPER_ROW" => {
            for i in 0..n {
                for j in (i + 1)..n {
                    let w =
                        next_weight(&mut values);

                    set_symmetric(
                        &mut matrix,
                        i,
                        j,
                        w,
                    );
                }
            }
        }

        "LOWER_ROW" => {
            for i in 0..n {
                for j in 0..i {
                    let w =
                        next_weight(&mut values);

                    set_symmetric(
                        &mut matrix,
                        i,
                        j,
                        w,
                    );
                }
            }
        }

        "UPPER_DIAG_ROW" => {
            for i in 0..n {
                for j in i..n {
                    let w =
                        next_weight(&mut values);

                    set_symmetric(
                        &mut matrix,
                        i,
                        j,
                        w,
                    );
                }
            }
        }

        "LOWER_DIAG_ROW" => {
            for i in 0..n {
                for j in 0..=i {
                    let w =
                        next_weight(&mut values);

                    set_symmetric(
                        &mut matrix,
                        i,
                        j,
                        w,
                    );
                }
            }
        }

        "UPPER_COL" => {
            for j in 0..n {
                for i in 0..j {
                    let w =
                        next_weight(&mut values);

                    set_symmetric(
                        &mut matrix,
                        i,
                        j,
                        w,
                    );
                }
            }
        }

        "LOWER_COL" => {
            for j in 0..n {
                for i in (j + 1)..n {
                    let w =
                        next_weight(&mut values);

                    set_symmetric(
                        &mut matrix,
                        i,
                        j,
                        w,
                    );
                }
            }
        }

        "UPPER_DIAG_COL" => {
            for j in 0..n {
                for i in 0..=j {
                    let w =
                        next_weight(&mut values);

                    set_symmetric(
                        &mut matrix,
                        i,
                        j,
                        w,
                    );
                }
            }
        }

        "LOWER_DIAG_COL" => {
            for j in 0..n {
                for i in j..n {
                    let w =
                        next_weight(&mut values);

                    set_symmetric(
                        &mut matrix,
                        i,
                        j,
                        w,
                    );
                }
            }
        }

        other => {
            panic!(
                "unsupported EDGE_WEIGHT_FORMAT: {other}"
            );
        }
    }

    assert!(
        values.next().is_none(),
        "too many values in EDGE_WEIGHT_SECTION"
    );

    matrix
}

fn next_weight<I>(
    values: &mut I
) -> f64
where
    I: Iterator<Item = f64>,
{
    values
        .next()
        .expect(
            "not enough values in EDGE_WEIGHT_SECTION"
        )
}

fn set_symmetric(
    matrix: &mut [Vec<f64>],
    i: usize,
    j: usize,
    value: f64,
) {
    matrix[i][j] =
        value;

    matrix[j][i] =
        value;
}

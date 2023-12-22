use std::collections::{HashMap, VecDeque};
use eyre::eyre;
use axum::http::StatusCode;

pub async fn integers(int: String) -> String {
    let mut int: Vec<u64> = int.split_ascii_whitespace()
        .filter_map(|n| n.parse().ok()).collect();
    // All integers appear twice, except for one. Find it.
    int.sort_unstable();
    let mut n = 0;
    for i in int.chunks(2) {
        if i.len() == 1 /* ⇒ last chunk */ || i[0] != i[1] {
            n = i[0];
            break
        }
    }

    let mut s = String::with_capacity(n.try_into().unwrap());
    for _ in 0 .. n { s.push('🎁'); }
    s
}

#[derive(Debug)]
struct Portals {
    star: Vec<[f32; 3]>,
    portal: HashMap<usize, Vec<usize>>,
}

impl Portals {
    fn from_str(data: &str) -> eyre::Result<Self> {
    let mut lines = data.lines();
        let n: usize = lines.next().ok_or(eyre!("n"))?.parse()?;
        if n < 2 { return Err(eyre!("n = {n} < 2")) }
        let mut star = Vec::with_capacity(n);
        for i in 0 .. n {
            let line = lines.next().ok_or(eyre!("star {i}"))?;
            let c: Vec<f32> = line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            if c.len() == 3 {
                star.push([c[0], c[1], c[2]])
            } else {
                return Err(eyre!("star {i}: {:?}", c))
            }
        }

        let k: usize = lines.next().ok_or(eyre!("k"))?.parse()?;
        if k == 0 { return Err(eyre!("k = 0")) }
        let mut portal = HashMap::with_capacity(2 * k);
        for i in 0 .. k {
            let line = lines.next().ok_or(eyre!("portal {i}"))?;
            let c: Vec<usize> = line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            if c.len() == 2 {
                // The portals are bidirectional.
                portal.entry(c[0])
                    .and_modify(|v: &mut Vec<usize>| v.push(c[1]))
                    .or_insert(vec![c[1]]);
                portal.entry(c[1])
                    .and_modify(|v: &mut Vec<usize>| v.push(c[0]))
                    .or_insert(vec![c[0]]);
            } else {
                return Err(eyre!("portal {i}: {:?}", c))
            }
        }

        Ok(Self { star, portal })
    }
}

fn shortest_path(p: &Portals) -> Vec<usize> {
    let target = p.star.len() - 1;
    // Breadth-first search to find the shortest path 0 → `target`.
    let mut paths = VecDeque::new();
    paths.push_back(vec![0]);
    while let Some(path) = paths.pop_front() {
        let end = *path.last().unwrap();
        if end == target {
            return path
        }
        // Add a new portal (cannot go back along the path).
        let neighbors = p.portal[&end].iter()
            .filter(|n| !path.contains(n));
        for &nbh in neighbors {
            let mut new_path = path.clone();
            new_path.push(nbh);
            paths.push_back(new_path);
        }
    }
    vec![]
}

fn dist_stars(p: &Portals, s0: usize, s1: usize) -> f32 {
    let [x0, y0, z0] = p.star[s0];
    let [x1, y1, z1] = p.star[s1];
    // Naive computation (may over & underflow):
    ((x0 - x1).powi(2) + (y0 - y1).powi(2) + (z0 - z1).powi(2)).sqrt()
}

pub async fn rocket(data: String) -> Result<String, StatusCode> {
    let p = Portals::from_str(&data)
        .or(Err(StatusCode::BAD_REQUEST))?;
    let path = shortest_path(&p);
    if path.len() == 0 { return Err(StatusCode::BAD_REQUEST) }
    let n_portals = path.len() - 1;
    let path_len: f32 = path.windows(2)
        .map(|s| dist_stars(&p, s[0], s[1]))
        .sum();
    Ok(format!("{n_portals} {path_len:.3}"))
}

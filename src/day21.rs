use std::fmt::Display;
use axum::{extract::Path, http::StatusCode};
use s2::{cellid::CellID, point::Point as Point3D, s1::Angle};
use reverse_geocoder::ReverseGeocoder;

pub async fn country(Path
    (s2_id): Path<String>
) -> Result<String, StatusCode> {
    let p: Point3D = CellID(u64_of_binary(&s2_id)).into();
    let lat = p.latitude().deg();
    let long = p.longitude().deg();
    let rg = ReverseGeocoder::new();
    let cc = &rg.search((lat, long)).record.cc;
    let country = rust_iso3166::from_alpha2(cc)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(format!("{:?}", country))
}

pub async fn coords(Path(s2_id): Path<String>) -> String {
    let id = u64_of_binary(&s2_id);
    let p: Point3D = CellID(id).into();
    let lat = p.latitude();
    let long = p.longitude();
    let ns = if lat.rad() > 0. { 'N' } else { 'S' };
    let ew = if long.rad() > 0. { 'E' } else { 'W' };
    //format!("{id} {} {} ", lat.deg(), long.deg())
    format!("{}{ns} {}{ew}", Degree(lat.abs()), Degree(long.abs()))
}

// See also https://docs.s2cell.aliddell.com/en/stable/s2_concepts.html

struct Degree(Angle);

impl Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Degree(s) = self;
        let deg = s.deg();
        let d = deg.trunc();
        let min = (deg - d).abs() * 60.;
        let m = min.trunc();
        let s = (min - m) * 60.;
        write!(f, "{d}°{m}'{s:.3}''")
    }
}


fn u64_of_binary(s: &str) -> u64 {
    let mut x = 0;
    for (i, d) in s.as_bytes().iter().rev().enumerate() {
        if d == &b'1' {
            x += 1 << i;
        }
    }
    x
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u64_of_binary() {
        assert_eq!(u64_of_binary("0"), 0);
        assert_eq!(u64_of_binary("1"), 1);
        assert_eq!(u64_of_binary("10"), 2);
        assert_eq!(u64_of_binary("11"), 3);
        assert_eq!(u64_of_binary("100"), 4);
        assert_eq!(u64_of_binary("101"), 5);
    }

    #[test]
    fn test_degree() {

    }
}

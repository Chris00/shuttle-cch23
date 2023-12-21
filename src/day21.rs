use std::{collections::HashMap, fmt::Display};
use axum::{extract::Path, http::StatusCode};
use s2::{cellid::CellID, point::Point as Point3D, s1::Angle};
use reverse_geocoder::ReverseGeocoder;
use lazy_static::lazy_static;


pub async fn country(Path
    (s2_id): Path<String>
) -> Result<&'static str, StatusCode> {
    let p: Point3D = CellID(u64_of_binary(&s2_id)).into();
    let lat = p.latitude().deg();
    let long = p.longitude().deg();
    let rg = ReverseGeocoder::new();
    let cc: &str = &rg.search((lat, long)).record.cc;
    Ok(COUNTRIES[cc])
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
    let mut x: u64 = 0;
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

lazy_static!{
// https://www.iban.com/country-codes but tweaked to accomodate the task
   static ref COUNTRIES: HashMap<&'static str, &'static str> = {
       HashMap::from([
 	   ("AF",	"Afghanistan"),
 	   ("AL",	"Albania"),
           ("DZ",	"Algeria"),
           ("AS",	"American Samoa"),
           ("AD",	"Andorra"),
           ("AO",	"Angola"),
           ("AI",	"Anguilla"),
           ("AQ",	"Antarctica"),
           ("AG",	"Antigua and Barbuda"),
           ("AR",	"Argentina" 	),
           ("AM",	"Armenia" 	),
           ("AW",	"Aruba" 	),
           ("AU",	"Australia" 	),
           ("AT",	"Austria" 	),
           ("AZ",	"Azerbaijan" 	),
           ("BS",	"Bahamas"),
           ("BH",	"Bahrain" 	),
           ("BD",	"Bangladesh" 	),
           ("BB",	"Barbados" 	),
           ("BY",	"Belarus" 	),
           ("BE",	"Belgium" 	),
           ("BZ",	"Belize" 	),
           ("BJ",	"Benin" 	),
           ("BM",	"Bermuda" 	),
           ("BT",	"Bhutan" 	),
           ("BO",	"Bolivia" 	),
           ("BQ",	"Bonaire"),
           ("BA",	"Bosnia and Herzegovina"),
           ("BW",	"Botswana"),
           ("BV",	"Bouvet Island"),
           ("BR",	"Brazil"),
           ("IO",	"British Indian Ocean Territory"),
           ("BN",	"Brunei"), // Brunei Darussalam
           ("BG",	"Bulgaria"),
           ("BF",	"Burkina Faso"),
           ("BI",	"Burundi" 	),
           ("CV",	"Cabo Verde" 	),
           ("KH",	"Cambodia" 	),
           ("CM",	"Cameroon" 	),
           ("CA",	"Canada" 	),
           ("KY", 	"Cayman Islands"),
           ("CF",	"Central African Republic"),
           ("TD",	"Chad"),
           ("CL",	"Chile" 	),
           ("CN",	"China" 	),
           ("CX",	"Christmas Island"),
           ("CC",	"Cocos (Keeling) Islands"),
           ("CO",	"Colombia" 	),
           ("KM",	"Comoros"),
           ("CD",	"Congo"),
           ("CG",	"Congo"),
           ("CK",	"Cook Islands"),
           ("CR",	"Costa Rica" 	),
           ("HR",	"Croatia" 	),
           ("CU",	"Cuba" 	),
           ("CW",	"Curaçao" 	),
           ("CY",	"Cyprus" 	),
           ("CZ",	"Czechia" 	),
           ("CI",	"Côte d'Ivoire"),
           ("DK",	"Denmark" 	),
           ("DJ",	"Djibouti" 	),
           ("DM",	"Dominica" 	),
           ("DO",	"Dominican Republic"),
           ("EC",	"Ecuador" 	),
           ("EG",	"Egypt" 	),
           ("SV",	"El Salvador" 	),
           ("GQ",	"Equatorial Guinea"),
           ("ER",	"Eritrea" 	),
           ("EE",	"Estonia" 	),
           ("SZ",	"Eswatini" 	),
           ("ET",	"Ethiopia" 	),
           ("FK",	"Falkland Islands"),
           ("FO",	"Faroe Islands"),
           ("FJ",	"Fiji"),
           ("FI",	"Finland" 	),
           ("FR",	"France" 	),
           ("GF",	"French Guiana" ),
           ("PF",	"French Polynesia"),
           ("TF",	"French Southern Territories"),
           ("GA",	"Gabon" 	),
           ("GM",	"Gambia" 	),
           ("GE",	"Georgia" 	),
           ("DE",	"Germany" 	),
           ("GH",	"Ghana" 	),
           ("GI",	"Gibraltar" 	),
           ("GR",	"Greece" 	),
           ("GL",	"Greenland" 	),
           ("GD",	"Grenada" 	),
           ("GP",	"Guadeloupe" 	),
           ("GU",	"Guam"),
           ("GT",	"Guatemala" 	),
           ("GG",	"Guernsey" 	),
           ("GN",	"Guinea" 	),
           ("GW",	"Guinea-Bissau" ),
           ("GY",	"Guyana" 	),
           ("HT",	"Haiti" 	),
           ("HM",	"Heard Island and McDonald Islands"),
           ("VA",	"Holy See"	),
           ("HN",	"Honduras" 	),
           ("HK",	"Hong Kong" 	),
           ("HU",	"Hungary" 	),
           ("IS",	"Iceland" 	),
           ("IN",	"India" 	),
           ("ID",	"Indonesia" 	),
           ("IR",	"Iran" 	),
           ("IQ",	"Iraq" 	),
           ("IE",	"Ireland" 	),
           ("IM",	"Isle of Man" 	),
           ("IL",	"Israel" 	),
           ("IT",	"Italy" 	),
           ("JM",	"Jamaica" 	),
           ("JP",	"Japan" 	),
           ("JE",	"Jersey" 	),
           ("JO",	"Jordan" 	),
           ("KZ",	"Kazakhstan" 	),
           ("KE",	"Kenya" 	),
           ("KI",	"Kiribati" 	),
           ("KP",	"Korea"), // the Democratic People's Republic of
           ("KR",	"Korea" 	), //the Republic of
           ("KW",	"Kuwait" 	),
           ("KG",	"Kyrgyzstan" 	),
           ("LA",	"Lao People's Democratic Republic"),
           ("LV",	"Latvia" 	),
           ("LB",	"Lebanon" 	),
           ("LS",	"Lesotho" 	),
           ("LR",	"Liberia" 	),
           ("LY",	"Libya" 	),
           ("LI",	"Liechtenstein"),
           ("LT",	"Lithuania" 	),
           ("LU",	"Luxembourg" 	),
           ("MO",	"Macao" 	),
           ("MG",	"Madagascar" 	),
           ("MW",	"Malawi" 	),
           ("MY",	"Malaysia" 	),
           ("MV",	"Maldives" 	),
           ("ML",	"Mali"),
           ("MT",	"Malta" 	),
           ("MH",	"Marshall Islands"),
           ("MQ",	"Martinique" 	),
           ("MR",	"Mauritania" 	),
           ("MU",	"Mauritius" 	),
           ("YT",	"Mayotte" 	),
           ("MX",	"Mexico" 	),
           ("FM",	"Micronesia "	), // Federated States of
           ("MD",	"Moldova" 	), // the Republic of
           ("MC",	"Monaco" 	),
           ("MN",	"Mongolia" 	),
           ("ME",	"Montenegro" 	),
           ("MS",	"Montserrat" 	),
           ("MA",	"Morocco" 	),
           ("MZ",	"Mozambique" 	),
           ("MM",	"Myanmar" 	),
           ("NA",	"Namibia" 	),
           ("NR",	"Nauru" 	),
           ("NP",	"Nepal" 	),
           // https://nominatim.openstreetmap.org/reverse?lat=51.4365999975148&lon=4.928289977262877
           // returns Belgium but Google https://www.google.co.uk/maps/place/
           // and `reverse_geocoder` say "Netherlands"
           ("NL",	"Belgium"), // The Netherlands
           ("NC",	"New Caledonia"),
           ("NZ",	"New Zealand" 	),
           ("NI",	"Nicaragua" 	),
           ("NE",	"Niger" 	),
           ("NG",	"Nigeria" 	),
           ("NU",	"Niue" 	),
           ("NF",	"Norfolk Island"),
           ("MP",	"Northern Mariana Islands"),
           ("NO",	"Norway" 	),
           ("OM",	"Oman" 	),
           ("PK",	"Pakistan" 	),
           ("PW",	"Palau" 	),
           ("PS",	"Palestine" 	),
           ("PA",	"Panama" 	),
           ("PG",	"Papua New Guinea"),
           ("PY",	"Paraguay" 	),
           ("PE",	"Peru" 	),
           ("PH",	"Philippines" 	),
           ("PN",	"Pitcairn" 	),
           ("PL",	"Poland" 	),
           ("PT",	"Portugal" 	),
           ("PR",	"Puerto Rico" 	),
           ("QA",	"Qatar" 	),
           ("MK",	"Republic of North Macedonia"),
           ("RO",	"Romania" 	),
           ("RU",	"Russian Federation"),
           ("RW",	"Rwanda" 	),
           ("RE",	"Réunion" 	),
           ("BL",	"Saint Barthélemy"),
           ("SH",	"Saint Helena, Ascension and Tristan da Cunha"),
           ("KN",	"Saint Kitts and Nevis"),
           ("LC",	"Saint Lucia"),
           ("MF",	"Saint Martin"), // French part
           ("PM",	"Saint Pierre and Miquelon"),
           ("VC",	"Saint Vincent and the Grenadines"),
           ("WS",	"Samoa" 	),
           ("SM",	"San Marino" 	),
           ("ST",	"Sao Tome and Principe"),
           ("SA",	"Saudi Arabia"),
           ("SN",	"Senegal" 	),
           ("RS",	"Serbia" 	),
           ("SC",	"Seychelles" 	),
           ("SL",	"Sierra Leone" ),
           ("SG",	"Singapore" 	),
           ("SX",	"Sint Maarten"	), // Dutch part
           ("SK",	"Slovakia" 	),
           ("SI",	"Slovenia" 	),
           ("SB",	"Solomon Islands"),
           ("SO",	"Somalia" 	),
           ("ZA",	"South Africa" 	),
           ("GS",	"South Georgia and the South Sandwich Islands"),
           ("SS",	"South Sudan" 	),
           ("ES",	"Spain" 	),
           ("LK",	"Sri Lanka" 	),
           ("SD",	"Sudan" 	),
           ("SR",	"Suriname" 	),
           ("SJ",	"Svalbard and Jan Mayen"),
           ("SE",	"Sweden" 	),
           ("CH",	"Switzerland" 	),
           ("SY",	"Syrian Arab Republic"),
           ("TW",	"Taiwan" 	), // Province of China
           ("TJ",	"Tajikistan" 	),
           ("TZ",	"Tanzania, United Republic of"),
           ("TH",	"Thailand" 	),
           ("TL",	"Timor-Leste" 	),
           ("TG",	"Togo" 	),
           ("TK",	"Tokelau" 	),
           ("TO",	"Tonga" 	),
           ("TT",	"Trinidad and Tobago"),
           ("TN",	"Tunisia" 	),
           ("TR",	"Turkey" 	),
           ("TM",	"Turkmenistan"),
           ("TC",	"Turks and Caicos Islands"),
           ("TV",	"Tuvalu" 	),
           ("UG",	"Uganda" 	),
           ("UA",	"Ukraine" 	),
           ("AE",	"United Arab Emirates"),
           ("GB", "United Kingdom of Great Britain and Northern Ireland"),
           ("UM",	"United States Minor Outlying Islands"),
           ("US",	"United States of America"),
           ("UY",	"Uruguay" 	),
           ("UZ",	"Uzbekistan" 	),
           ("VU",	"Vanuatu" 	),
           ("VE",	"Venezuela"), // Bolivarian Republic of
           ("VN",	"Vietnam"),
           ("VG",	"Virgin Islands"), // British
           ("VI",	"Virgin Islands"), // U.S.
           ("WF",	"Wallis and Futuna"),
           ("EH",	"Western Sahara"),
           ("YE",	"Yemen" 	),
           ("ZM",	"Zambia" 	),
           ("ZW",	"Zimbabwe" 	),
           ("AX",	"Åland Islands"),
       ])
    };
}


use std::io::BufRead;
use std::io::Write;
use std::str::FromStr;

const MMDB_FILE_PATH: &'static str = "./GeoIP2-Country.mmdb";

fn main() {
    let mmdb = match std::fs::read(MMDB_FILE_PATH) {
        Ok(i)  => i,
        Err(e) => panic!("failed to open GeoIP database {}: {:?}", MMDB_FILE_PATH, e),
    };
    let geodb_country = geoip2::Reader::<geoip2::Country>::from_bytes(&mmdb).unwrap();

    let infh = match std::env::args().nth(1) {
        Some(filename) => {
            match std::fs::File::open(&filename) {
                Ok(i)  => i,
                Err(e) => panic!("failed to open input file {}: {:?}", filename, e),
            }
        },
        None => panic!("input filename is not specified"),
    };

    let mut outfh = std::io::BufWriter::new(std::io::stdout().lock());

    for l in std::io::BufReader::new(infh).lines() {
        let line = l.unwrap();

        let mut row = line.splitn(2, ',');
        let ipaddr = row.next().unwrap();
        let rest_columns = row.next().unwrap();

        let cc = match geodb_country.lookup(std::net::IpAddr::from_str(ipaddr).unwrap()) {
            Ok(i) => match i.country {
                Some(c) => c.iso_code.unwrap(),
                None    => "ZZ",
            },
            Err(geoip2::Error::NotFound) => "ZZ",
            Err(e)                       => panic!("{:?}", e),
        };

        writeln!(outfh, "{},{},{}", ipaddr, cc, rest_columns).unwrap();
    }
}

// vim: set fileencoding=utf-8 nobomb fileformat=unix filetype=rust number expandtab tabstop=8 softtabstop=4 shiftwidth=4 autoindent smartindent :

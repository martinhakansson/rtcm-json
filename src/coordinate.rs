use chrono::prelude::*;
use std::f64::consts::PI;
use std::io::Write;

#[derive(Clone)]
pub struct Coordinate {
    latitude: f64,
    longitude: f64,
    height: f64,
}
impl Coordinate {
    pub fn from_llh(latitude: f64, longitude: f64, height: f64) -> Self {
        Coordinate {
            latitude,
            longitude,
            height,
        }
    }
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        const A: f64 = 6378137.0;
        const F: f64 = 1.0 / 298.257223563;
        const E2: f64 = F * (2.0 - F);
        let r2 = x * x + y * y;
        let mut z0 = z;
        let mut zk = z0 + 1.0;
        let mut v = 0.0;
        while (z0 - zk).abs() > 1.0e-4 {
            zk = z0;
            let sinphi = z0 / (r2 + z0 * z0).sqrt();
            v = A / (1.0 - E2 * sinphi * sinphi).sqrt();
            z0 = z + v * E2 * sinphi;
        }
        let (latitude, longitude) = if r2 > 1.0e-12 {
            (
                (z0 / r2.sqrt()).atan() * (180.0 / PI),
                y.atan2(x) * (180.0 / PI),
            )
        } else {
            (if z > 0.0 { 90.0 } else { -90.0 }, 0.0)
        };
        let height = (r2 + z0 * z0).sqrt() - v;
        Coordinate {
            latitude,
            longitude,
            height,
        }
    }
    pub fn write_to_stream<T: Write>(&self, stream: &mut T) -> std::io::Result<()> {
        //Create nmea
        let utc = Utc::now().round_subsecs(2);
        let lat = self.latitude.abs();
        let lon = self.longitude.abs();
        let lat_dir = if self.latitude >= 0.0 { 'N' } else { 'S' };
        let lon_dir = if self.longitude >= 0.0 { 'E' } else { 'W' };
        let gga = format!("GPGGA,{:02}{:02}{:02}.{:02},{:02}{:02}.{:06},{},{:03}{:02}.{:06},{},0,0,1.0,{:.3},M,0.0,M,,",

            utc.hour(),
            utc.minute(),
            utc.second(),
            if utc.timestamp_subsec_millis() < 1000 {
                utc.timestamp_subsec_millis() / 10
            } else { utc.timestamp_subsec_millis() / 10 - 100 },
            lat.trunc(),
            (lat.fract() * 60.0).trunc(),
            ((lat * 60.0).fract() * 1000000.0).trunc(),
            lat_dir,
            lon.trunc(),
            (lon.fract() * 60.0).trunc(),
            ((lon * 60.0).fract() * 1000000.0).trunc(),
            lon_dir,
            self.height,
        );
        let mut checksum: u8 = 0;
        for b in gga.as_bytes() {
            checksum ^= b;
        }
        stream.write_all(&[b'$'])?;
        stream.write_all(gga.as_bytes())?;
        stream.write_all(format!("*{:02X}\r\n", checksum).as_bytes())?;
        Ok(())
    }
}

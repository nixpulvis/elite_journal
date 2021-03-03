use serde::Deserialize;

#[cfg(feature = "with-postgis-sqlx")]
use std::io::Read;
#[cfg(feature = "with-postgis-sqlx")]
use geozero::{
    wkb::{FromWkb, WkbDialect},
    CoordDimensions,
    GeomProcessor,
    GeozeroGeometry
};

#[derive(Deserialize, Debug, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[cfg(feature = "with-postgis-sqlx")]
impl GeomProcessor for Coordinate {
    fn dimensions(&self) -> CoordDimensions {
        CoordDimensions::xyz()
    }

    fn coordinate(&mut self, x: f64, y: f64, z: Option<f64>,
        _m: Option<f64>, _t: Option<f64>, _tm: Option<u64>, _idx: usize,)
        -> geozero::error::Result<()>
    {
        self.x = x;
        self.y = y;
        self.z = z.unwrap_or(0.0);
        Ok(())
    }
}

#[cfg(feature = "with-postgis-sqlx")]
impl GeozeroGeometry for Coordinate {
    fn process_geom<P: GeomProcessor>(&self, processor: &mut P)
        -> std::result::Result<(), geozero::error::GeozeroError>
    {
        processor.point_begin(0)?;
        processor.coordinate(self.x, self.y, Some(self.z), None, None, None, 0)?;
        processor.point_end(0)
    }

    fn dims(&self) -> CoordDimensions {
        CoordDimensions::xyz()
    }
}

#[cfg(feature = "with-postgis-sqlx")]
impl FromWkb for Coordinate {
    fn from_wkb<R: Read>(rdr: &mut R, dialect: WkbDialect) -> geozero::error::Result<Self> {
        let mut pt = Coordinate { x: 0., y: 0., z: 0. };
        geozero::wkb::process_wkb_type_geom(rdr, &mut pt, dialect)?;
        Ok(pt)
    }
}


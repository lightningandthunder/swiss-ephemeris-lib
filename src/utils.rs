use std::f64::consts::PI;

pub fn radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

pub fn dms_to_decimal(degree: i32, minute: i32, second: i32) -> f64 {
    degree as f64 + ((minute as f64) / 60.0) + ((second as f64) / 3600.0)
}

pub fn decimal_to_dms(decimal: f64) -> (i32, i32, i32) {
    let degree = i32::abs(decimal as i32);
    let minute = i32::abs(((decimal - degree as f64) * 60.0) as i32);
    let second = i32::abs(((((decimal - degree as f64) * 60.0) - (minute as f64)) * 60.0) as i32);
    (degree, minute, second)
}

pub fn calculate_prime_vertical_longitude(
    planet_longitude: f64,
    planet_latitude: f64,
    ramc: f64,
    obliquity: f64,
    svp: f64,
    geo_latitude: f64,
) -> f64 {
    let calc_ax = (planet_longitude + (360.0 - (330.0 + svp)))
        .to_radians()
        .cos();

    let precessed_declination = ((planet_latitude.to_radians().sin()
        * obliquity.to_radians().cos())
        + (planet_latitude.to_radians().cos()
            * obliquity.to_radians().sin()
            * (planet_longitude + (360.0 - (330.0 + svp)))
                .to_radians()
                .sin()))
    .asin()
    .to_degrees();

    let calc_ay = (planet_longitude + (360.0 - (330.0 + svp)))
        .to_radians()
        .sin()
        * obliquity.to_radians().cos()
        - planet_latitude.to_radians().tan() * obliquity.to_radians().sin();

    let calc_ayx_deg = (calc_ay / calc_ax).atan().to_degrees();

    let precessed_right_ascension = if calc_ax < 0.0 {
        calc_ayx_deg + 180.0
    } else if calc_ay < 0.0 {
        calc_ayx_deg + 360.0
    } else {
        calc_ayx_deg
    };

    let hour_angle_degree = ramc - precessed_right_ascension;

    let calc_cz = (1.0
        / (geo_latitude.to_radians().cos() / hour_angle_degree.to_radians().tan()
            + geo_latitude.to_radians().sin() * precessed_declination.to_radians().tan()
                / hour_angle_degree.to_radians().sin()))
    .atan()
    .to_degrees();

    let calc_cx = geo_latitude.to_radians().cos() * hour_angle_degree.to_radians().cos()
        + geo_latitude.to_radians().sin() * precessed_declination.to_radians().tan();

    let campanus_longitude = if calc_cx < 0.0 {
        90.0 - calc_cz
    } else {
        270.0 - calc_cz
    };

    campanus_longitude
}

pub fn calculate_right_ascension(
    planet_longitude: f64,
    planet_latitude: f64,
    svp: f64,
    obliquity: f64,
) -> f64 {
    let circle_minus_ayanamsa = 360.0 - (330.0 + svp);
    let precessed_longitude = planet_longitude + circle_minus_ayanamsa;

    let calcs_ay = precessed_longitude.to_radians().sin() * obliquity.to_radians().cos()
        - planet_latitude.to_radians().tan() * obliquity.to_radians().sin();

    let calcs_ax = precessed_longitude.to_radians().cos();
    let calcs_o = (calcs_ay / calcs_ax).atan().to_degrees();

    let precessed_right_ascension = if calcs_ax < 0.0 {
        calcs_o + 180.0
    } else if calcs_ay < 0.0 {
        calcs_o + 360.0
    } else {
        calcs_o
    };

    precessed_right_ascension
}

pub fn parse_angularity_pvl(pvl: f64, orb: f64) -> Option<f64> {
    let orb_int = orb as i64;
    match pvl as i64 {
        pvl_int if pvl_int >= 0 && pvl_int <= 0 + orb_int => Some(f64::abs(orb - pvl)),
        pvl_int if pvl_int >= 360 - orb_int && pvl_int <= 360 => Some(f64::abs(360.0 - pvl)),
        pvl_int if pvl_int >= 90 - orb_int && pvl_int <= 90 + orb_int => Some(f64::abs(90.0 - pvl)),
        pvl_int if pvl_int >= 180 - orb_int && pvl_int <= 180 + orb_int => {
            Some(f64::abs(180.0 - pvl))
        }
        pvl_int if pvl_int >= 270 - orb_int && pvl_int <= 270 + orb_int => {
            Some(f64::abs(270.0 - pvl))
        }
        _ => None,
    }
}

pub fn parse_angularity_ra(ra: f64, ramc: f64, orb: f64) -> Option<f64> {
    let orb = f64::max(ra, ramc) - f64::min(ra, ramc);
    let orb_int = orb as i64;

    match orb as i64 {
        90 if orb_int >= 90 - orb_int && orb_int <= 90 + orb_int => Some(90.0 - orb),
        270 if orb_int >= 270 - orb_int && orb_int <= 270 + orb_int => Some(270.0 - orb),
        _ => None,
    }
}

pub fn parse_angularity_longitude(longitude: f64, asc: f64, mc: f64) -> Option<f64> {
    // normalize orbs
    let asc_orb = {
        match (longitude - asc) as i64 {
            0.. => longitude - asc,
            _ => (longitude + 360.0) - asc,
        }
    };

    let mc_orb = {
        match (longitude - mc) as i64 {
            0.. => longitude - mc,
            _ => (longitude + 360.0) - mc,
        }
    };

    match asc_orb as i64 {
        88..=90 => Some(90.0 - asc_orb),
        268..=270 => Some(270.0 - asc_orb),
        _ => match mc_orb as i64 {
            88..=90 => Some(90.0 - mc_orb),
            268..=270 => Some(270.0 - mc_orb),
            _ => None,
        },
    }
}

pub fn round_to_digit(number: f64, digits: i8) -> f64 {
    let rounding_factor = 10.0 * digits as f64;
    (number * rounding_factor).round() / (10.0 * rounding_factor)
}
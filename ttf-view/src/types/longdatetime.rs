use crate::util::DisplayBuffer;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeDelta, Utc};
use std::fmt::Write;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct LongDateTime([u8; 8]);

const EPOCH_NAIVE: NaiveDateTime =
    NaiveDate::from_ymd_opt(1904, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();

impl LongDateTime {
    pub const EPOCH: DateTime<Utc> = EPOCH_NAIVE.and_utc();

    pub const fn new(datetime: DateTime<Utc>) -> Self {
        let delta = datetime.naive_utc().signed_duration_since(EPOCH_NAIVE);
        Self::from_epoch_seconds(delta.num_seconds())
    }
    pub const fn datetime(&self) -> Option<DateTime<Utc>> {
        let delta = TimeDelta::try_seconds(self.epoch_seconds())?;
        Some(EPOCH_NAIVE.checked_add_signed(delta)?.and_utc())
    }

    pub const fn from_epoch_seconds(secs: i64) -> Self {
        Self(i64::to_be_bytes(secs))
    }
    pub const fn epoch_seconds(&self) -> i64 {
        i64::from_be_bytes(self.0)
    }

    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0
    }
}

// TODO: When [u8; 8]'s Default is constified, replace this impl with #[derive_const]
#[allow(clippy::derivable_impls)]
const impl Default for LongDateTime {
    fn default() -> Self {
        Self([0; 8])
    }
}
// Note: PartialEq + Eq impls need to be explicit, because [u8; 8] is compared unsignedly.
const impl PartialOrd for LongDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
const impl Ord for LongDateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.epoch_seconds().cmp(&other.epoch_seconds())
    }
}

impl std::fmt::Debug for LongDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt_longdatetime(*self, true, f)
    }
}
impl std::fmt::Display for LongDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt_longdatetime(*self, false, f)
    }
}

fn fmt_longdatetime(ldt: LongDateTime, iso: bool, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let epoch_secs = ldt.epoch_seconds();

    const SECS_PER_DAY: i64 = 24 * 60 * 60;
    let epoch_days = epoch_secs.div_euclid(SECS_PER_DAY);
    let epoch_secs = epoch_secs.rem_euclid(SECS_PER_DAY);

    // Convert "days since 1904" to "days since 1970"
    let (year, month, day) = ymd_from_days(epoch_days - 24107);

    // Max length is 32: "-292277022723-01-25 08:29:52 UTC"
    let mut buf = DisplayBuffer::<32>::new();

    if year > 9999 {
        // ISO 8601 requires the explicit sign for out-of-range years
        buf.write_byte_unchecked(b'+');
    } else if year < 0 {
        // Write minus separately to avoid counting it towards min width in "{:04}"
        buf.write_byte_unchecked(b'-');
    }
    let year = year.unsigned_abs();

    // Write "{:04}" with year (fast path for 0..=9999)
    if matches!(year, 0..=9999) {
        buf.write_two_digits_unchecked((year / 100) as u8);
        buf.write_two_digits_unchecked((year % 100) as u8);
    } else {
        write!(buf, "{:04}", year)?;
    }

    // Write "-{:02}-{:02}" with month and day
    buf.write_byte_unchecked(b'-');
    buf.write_two_digits_unchecked(month);
    buf.write_byte_unchecked(b'-');
    buf.write_two_digits_unchecked(day);

    // In Debug use 'T' as separator, and in Display - ' '
    buf.write_byte_unchecked(if iso { b'T' } else { b' ' });

    // Write "{:02}:{:02}:{:02}" with hour, min, sec
    buf.write_two_digits_unchecked((epoch_secs / 3600) as u8);
    buf.write_byte_unchecked(b':');
    buf.write_two_digits_unchecked(((epoch_secs % 3600) / 60) as u8);
    buf.write_byte_unchecked(b':');
    buf.write_two_digits_unchecked((epoch_secs % 60) as u8);

    // In Debug use 'Z' for UTC, and in Display - " UTC"
    if iso {
        buf.write_byte_unchecked(b'Z');
    } else {
        buf.write_str_unchecked(" UTC");
    }

    f.write_str(buf.as_str())
}

fn ymd_from_days(unix_days: i64) -> (i64, u8, u8) {
    // See https://howardhinnant.github.io/date_algorithms.html#civil_from_days
    let z = unix_days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (y + (m <= 2) as i64, m as u8, d as u8)
}

const impl From<DateTime<Utc>> for LongDateTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self::new(value)
    }
}
const impl TryFrom<LongDateTime> for DateTime<Utc> {
    type Error = ();
    fn try_from(value: LongDateTime) -> Result<Self, Self::Error> {
        value.datetime().ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longdatetimes() {
        let dates: [(u64, &'static str); _] = [
            (0x8000000000000000, "-292277022723-01-25T08:29:52Z"),
            (0xFFFFF00000000000, "-555571-10-24T14:19:44Z"),
            (0xFFFFFF0000000000, "-32939-11-10T23:23:44Z"),
            (0xFFFFFFF000000000, "-0274-05-13T16:27:44Z"),
            (0xFFFFFFF200D00100, "-0001-01-01T00:00:00Z"),
            (0xFFFFFFF202B13480, "0000-01-01T00:00:00Z"),
            (0xFFFFFFF20493B980, "0001-01-01T00:00:00Z"),
            (0x0000000000000000, "1904-01-01T00:00:00Z"),
            (0x00000000E3D1B1DE, "2025-02-12T02:03:10Z"),
            (0x00000000E6B686E6, "2026-08-28T00:29:26Z"),
            (0x00000000FFFFFFFF, "2040-02-06T06:28:15Z"),
            (0x000000FFFFFFFFFF, "+36746-02-19T00:36:15Z"),
            (0x0000FFFFFFFFFFFF, "+8921490-12-06T10:44:15Z"),
            (0x00FFFFFFFFFFFFFF, "+2283416158-11-23T12:52:15Z"),
            (0x7FFFFFFFFFFFFFFF, "+292277026530-12-04T15:30:07Z"),
        ];

        for (stamp, iso_date) in dates {
            let stamp = LongDateTime::from_epoch_seconds(stamp as i64);
            assert_eq!(format!("{:?}", stamp), iso_date);

            if let Some(chrono_date) = stamp.datetime() {
                assert_eq!(format!("{:?}", chrono_date), iso_date);
                assert_eq!(stamp, LongDateTime::new(chrono_date));
            }
        }
    }
}

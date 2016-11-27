use std::fmt;
use std::time::Duration;
use std::time::{UNIX_EPOCH, SystemTime};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use chrono;
use futures::{Async, Future, Poll, Stream};

use DataType;
use Row;

pub fn duration_to_ms(duration: &Duration) -> u64 {
    duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000_000
}

pub fn fmt_hex<T>(f: &mut fmt::Formatter, bytes: &[T]) -> fmt::Result where T: fmt::LowerHex {
    if bytes.is_empty() {
        return write!(f, "0x")
    }
    try!(write!(f, "{:#x}", bytes[0]));
    for b in &bytes[1..] {
        try!(write!(f, "{:x}", b));
    }
    Ok(())
}

pub fn time_to_us(time: &SystemTime) -> i64 {
    // TODO: do overflow checking
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            (duration.as_secs() * 1000_000 + duration.subsec_nanos() as u64 / 1000) as i64
        },
        Err(error) => {
            let duration = error.duration();
            (- ((duration.as_secs() * 1000_000 + duration.subsec_nanos() as u64 / 1000) as i64))
        }
    }

}

pub fn us_to_time(us: i64) -> SystemTime {
    let abs = us.abs() as u64;

    let s = abs / 1000_000;
    let ns = (abs % 1000_000) as u32 * 1000;

    if us.is_negative() {
        UNIX_EPOCH - Duration::new(s, ns)
    } else {
        UNIX_EPOCH + Duration::new(s, ns)
    }
}

pub fn fmt_timestamp(f: &mut fmt::Formatter, timestamp: SystemTime) -> fmt::Result {
    let datetime = if timestamp < UNIX_EPOCH {
        chrono::NaiveDateTime::from_timestamp(0, 0) -
            chrono::Duration::from_std(UNIX_EPOCH.duration_since(timestamp).unwrap()).unwrap()
    } else {
        chrono::NaiveDateTime::from_timestamp(0, 0) +
            chrono::Duration::from_std(timestamp.duration_since(UNIX_EPOCH).unwrap()).unwrap()
    };

    write!(f, "{}", datetime.format("%Y-%m-%dT%H:%M:%S%.6fZ"))
}

pub fn fmt_cell(f: &mut fmt::Formatter, row: &Row, idx: usize) -> fmt::Result {
    match row.schema().columns()[idx].data_type() {
        DataType::Bool => write!(f, "{}", row.get::<bool>(idx).unwrap()),
        DataType::Int8 => write!(f, "{}", row.get::<i8>(idx).unwrap()),
        DataType::Int16 => write!(f, "{}", row.get::<i16>(idx).unwrap()),
        DataType::Int32 => write!(f, "{}", row.get::<i32>(idx).unwrap()),
        DataType::Int64 => write!(f, "{}", row.get::<i64>(idx).unwrap()),
        DataType::Timestamp => fmt_timestamp(f, row.get::<SystemTime>(idx).unwrap()),
        DataType::Float => write!(f, "{}", row.get::<f32>(idx).unwrap()),
        DataType::Double => write!(f, "{}", row.get::<f64>(idx).unwrap()),
        DataType::Binary => fmt_hex(f, row.get::<&[u8]>(idx).unwrap()),
        DataType::String => write!(f, "{:?}", row.get::<&str>(idx).unwrap()),
    }
}

pub fn dummy_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0)
}

/// Creates a new stream from a collection of futures, yielding items in order
/// of completion.
pub fn select_stream<F: Future>(futures: Vec<F>) -> SelectStream<F> {
    SelectStream {
        futures: futures,
    }
}

/// Stream which yields items from a collection of futures in completion order.
pub struct SelectStream<F: Future> {
    futures: Vec<F>,
}

impl<F: Future> Stream for SelectStream<F> {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.futures.is_empty() {
            return Ok(Async::Ready(None));
        }
        let item = self.futures.iter_mut().enumerate().filter_map(|(i, f)| {
            match f.poll() {
                Ok(Async::NotReady) => None,
                Ok(Async::Ready(e)) => Some((i, Ok(e))),
                Err(e) => Some((i, Err(e))),
            }
        }).next();
        match item {
            Some((idx, res)) => {
                self.futures.swap_remove(idx);
                match res {
                    Ok(item) => Ok(Async::Ready(Some(item))),
                    Err(error) => Err(error),
                }
            },
            None => Ok(Async::NotReady),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::time::{Duration, UNIX_EPOCH};

    use futures::sync::oneshot;
    use futures;
    use quickcheck::{quickcheck, TestResult};

    use schema;
    use super::*;

    #[test]
    fn timestamp_conversion() {

        fn roundtrip(us: i64) -> TestResult {
            TestResult::from_bool(us == time_to_us(&us_to_time(us)))
        }

        quickcheck(roundtrip as fn(i64) -> TestResult);
    }

    #[test]
    fn test_format_timestamp() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();

        row.set_by_name("timestamp", UNIX_EPOCH - Duration::from_millis(1234)).unwrap();
        assert_eq!("Timestamp \"timestamp\"=1969-12-31T23:59:58.766000Z",
                   &format!("{:?}", row));

        row.set_by_name("timestamp", UNIX_EPOCH + Duration::from_millis(1234)).unwrap();
        assert_eq!("Timestamp \"timestamp\"=1970-01-01T00:00:01.234000Z",
                   &format!("{:?}", row));
    }

    #[test]
    fn test_select_stream() {
        let (a_tx, a_rx) = oneshot::channel::<u32>();
        let (b_tx, b_rx) = oneshot::channel::<u32>();
        let (c_tx, c_rx) = oneshot::channel::<u32>();

        let stream = select_stream(vec![a_rx, b_rx, c_rx]);

        let mut spawn = futures::executor::spawn(stream);
        b_tx.complete(99);
        assert_eq!(Some(Ok(99)), spawn.wait_stream());

        a_tx.complete(33);
        c_tx.complete(33);
        assert_eq!(Some(Ok(33)), spawn.wait_stream());
        assert_eq!(Some(Ok(33)), spawn.wait_stream());
        assert_eq!(None, spawn.wait_stream());
    }
}

#[macro_use]
extern crate criterion;

use resp::Message;
use bytes::BytesMut;

use criterion::Criterion;

fn bench_resp(c: &mut Criterion) {
    // fn bench_parse_plain(b:&mut Bencher) {
    c.bench_function("resp parse plain", |b| {
        let sdata = "+baka for you\r\n".as_bytes();
        b.iter(|| {
            Message::parse(&mut BytesMut::from(&sdata[..])).unwrap().unwrap();
        })
    });

    c.bench_function("resp parse bulk", |b| {
        let sdata = "$5\r\nojbK\n\r\n".as_bytes();
        b.iter(|| {
            Message::parse(&mut BytesMut::from(&sdata[..])).unwrap().unwrap();
        })
    });

    c.bench_function("resp parse array", |b| {
        let sdata = "*2\r\n$1\r\na\r\n$5\r\nojbK\n\r\n".as_bytes();
        b.iter(|| {
            Message::parse(&mut BytesMut::from(&sdata[..])).unwrap().unwrap();
        })
    });
}

criterion_group!(benches, bench_resp);
criterion_main!(benches);

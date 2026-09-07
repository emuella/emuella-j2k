use emuella_jpip::*;
use std::collections::BTreeMap;
fn key(id: u64) -> BinKey {
    BinKey::new(0, id).unwrap()
}
fn limits(bytes: usize) -> CacheLimits {
    CacheLimits {
        bytes,
        bins: 8,
        ranges_per_bin: 4,
        max_bin_length: 4096,
    }
}
fn cache(bytes: usize) -> Cache {
    let mut c = Cache::new(limits(bytes));
    c.bind_identity("immutable-a").unwrap();
    c
}
fn msg(id: u64, offset: u64, final_bin: bool, bytes: &[u8]) -> DataMessage<'_> {
    DataMessage {
        key: key(id),
        offset,
        final_bin,
        auxiliary: None,
        bytes,
    }
}
fn receive(cache: &mut Cache, bytes: &[u8], fragment: usize, complete: bool) {
    let mut decoder = Decoder::new(4096);
    for part in bytes.chunks(fragment) {
        decoder
            .push(part, |event| {
                if let Event::Data(m) = event {
                    cache.insert(m)?;
                }
                Ok(())
            })
            .unwrap();
    }
    assert_eq!(
        decoder.finish(),
        if complete {
            Ok(())
        } else {
            Err(Error::Truncated)
        }
    );
}
#[test]
fn generated_message_extremes_survive_every_fragment_boundary() {
    for id in [0, 15, 16, 2047, 2048, u64::MAX] {
        let payload: Vec<u8> = (0..=255).collect();
        let mut message = encode_message(DataMessage {
            auxiliary: Some(3),
            ..msg(id, 0, true, &payload)
        })
        .unwrap();
        message.extend(encode_end(2));
        for size in 1..=message.len() {
            let mut c = cache(512);
            receive(&mut c, &message, size, true);
            assert!(c.is_complete(key(id)));
            let mut out = vec![0; 256];
            c.read(key(id), 0, &mut out).unwrap();
            assert_eq!(out, payload);
        }
    }
}
#[test]
fn independent_wire_layout_inheritance_unknown_class_and_eor_body() {
    // Authored values: explicit class/codestream, then inherited fields.
    let bytes = [
        0x61, 0, 0, 0, 2, 7, 8, 0x31, 2, 1, 9, 0x60, 10, 0, 0, 2, 99, 98, 0, 2, 2, 50, 51,
    ];
    let mut c = cache(32);
    receive(&mut c, &bytes, 1, true);
    assert!(c.is_complete(key(1)));
    let mut out = [0; 3];
    c.read(key(1), 0, &mut out).unwrap();
    assert_eq!(out, [7, 8, 9]);
    assert_eq!(c.bin_count(), 1);
    let ext = encode_message(DataMessage {
        auxiliary: Some(2),
        ..msg(1, 0, true, &[7])
    })
    .unwrap();
    assert_eq!(ext, [0x71, 1, 0, 0, 1, 2, 7]);
}
#[test]
fn truncation_at_every_byte_never_falsely_completes_response() {
    let mut bytes = encode_message(msg(1, 0, true, &[1, 2, 3, 4, 5])).unwrap();
    bytes.extend(encode_end(2));
    for cut in 0..bytes.len() {
        let mut c = cache(32);
        receive(&mut c, &bytes[..cut], 1, false);
        assert!(c.bytes() <= 5);
    }
    let mut decoder = Decoder::new(32);
    decoder.push(&bytes, |_| Ok(())).unwrap();
    assert_eq!(decoder.push(&[1], |_| Ok(())), Err(Error::Malformed));
}
#[test]
fn decoder_limits_overflows_invalid_presence_and_failed_state() {
    for bytes in [
        vec![0x10],
        vec![0x60, 0, 0, 0, 33],
        vec![0xe0; 11],
        vec![
            0x60, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ],
    ] {
        let mut d = Decoder::new(32);
        assert!(d.push(&bytes, |_| Ok(())).is_err());
        assert_eq!(d.push(&[], |_| Ok(())), Err(Error::Malformed));
    }
}
#[test]
fn sparse_final_holes_conflicts_and_atomic_rejection() {
    let mut c = cache(8);
    c.insert(msg(0, 4, true, &[5, 6])).unwrap();
    assert!(!c.is_complete(key(0)));
    assert!(c.model().is_empty());
    c.insert(msg(0, 0, false, &[1, 2])).unwrap();
    assert_eq!(c.model()[&key(0)], Known::Prefix(2));
    let before = c.model();
    assert_eq!(
        c.insert(msg(0, 1, false, &[99, 3, 4])),
        Err(Error::Conflict)
    );
    assert_eq!(c.model(), before);
    assert_eq!(c.bytes(), 4);
    assert_eq!(c.insert(msg(0, 6, false, &[7])), Err(Error::Conflict));
    assert_eq!(c.insert(msg(0, 0, true, &[1, 2])), Err(Error::Conflict));
    c.insert(msg(0, 1, false, &[2, 3, 4, 5])).unwrap();
    assert!(c.is_complete(key(0)));
    assert_eq!(c.bytes(), 6);
    c.insert(msg(0, 0, true, &[1, 2, 3, 4, 5, 6])).unwrap();
    assert_eq!(c.bytes(), 6);
    let mut out = [0; 6];
    c.read(key(0), 0, &mut out).unwrap();
    assert_eq!(out, [1, 2, 3, 4, 5, 6]);
}
#[test]
fn eviction_metadata_limits_empty_bins_and_identity() {
    let mut c = cache(5);
    c.insert(msg(0, 0, true, &[1, 2, 3])).unwrap();
    c.insert(msg(1, 0, true, &[4, 5])).unwrap();
    c.read(key(0), 0, &mut [0]).unwrap();
    c.insert(msg(2, 0, true, &[6, 7])).unwrap();
    assert!(!c.model().contains_key(&key(1)));
    assert_eq!(c.eviction_count(), 1);
    assert!(c.model().contains_key(&key(0)));
    assert_eq!(c.insert(msg(3, 0, true, &[0; 6])), Err(Error::Limit));
    assert_eq!(c.bytes(), 5);
    c.insert(DataMessage {
        key: BinKey::new(8, 0).unwrap(),
        ..msg(0, 0, true, &[])
    })
    .unwrap();
    assert!(c.is_complete(BinKey::new(8, 0).unwrap()));
    assert!(!c.bind_identity("immutable-a").unwrap());
    assert_eq!(c.bytes(), 5);
    assert!(c.bind_identity("immutable-b").unwrap());
    assert_eq!(c.bytes(), 0);
    assert_eq!(c.eviction_count(), 1);
    assert!(c.model().is_empty());
    let mut small = Cache::new(CacheLimits {
        bytes: 10,
        bins: 1,
        ranges_per_bin: 1,
        max_bin_length: 10,
    });
    small.bind_identity("x").unwrap();
    small.insert(msg(0, 0, false, &[1])).unwrap();
    assert_eq!(small.insert(msg(0, 2, false, &[3])), Err(Error::Limit));
    assert_eq!(small.insert(msg(0, 10, false, &[3])), Err(Error::Limit));
    small.insert(msg(1, 0, true, &[])).unwrap();
    assert!(!small.model().contains_key(&key(0)));
}
fn request() -> Request {
    Request {
        target: "image-v1".into(),
        tid: "immutable-a".into(),
        frame: [512, 256],
        offset: [3, 4],
        size: [100, 101],
        components: vec![0, 2],
        layers: Some(1),
        max_length: 512,
        extended: true,
        model: CacheModel::new(),
    }
}
#[test]
fn request_and_response_roundtrip_and_strict_profile() {
    let mut r = request();
    r.model = parse_model("Hm,H3:17,P2048:33,M0", 4).unwrap();
    let q = r.query().unwrap();
    assert_eq!(Request::parse(&q, 4096, 4).unwrap(), r);
    for suffix in ["&cid=1", "&cnew=http", "&len=9", "&bogus=1"] {
        assert!(Request::parse(&(q.clone() + suffix), 4096, 4).is_err());
    }
    for bad in ["Hm,Hm", "P2:L3", "-P2", "P*", "[1],P2", "H0:+3", "M0:"] {
        assert!(parse_model(bad, 16).is_err());
    }
    assert_eq!(Request::parse(&q, 5, 16), Err(Error::Limit));
    assert!(r.model_for_identity("changed").is_empty());
    assert_eq!(r.model_for_identity("immutable-a"), r.model);
    r.tid = "0".into();
    assert!(r.model_for_identity("0").is_empty());
    let response = ResponseFields::parse("image-b", "512,256", "3,4", "100,101").unwrap();
    let h = response.headers().unwrap();
    assert_eq!(h[0], ("JPIP-tid".into(), "image-b".into()));
    assert!(ResponseFields::parse("x\r\ny", "1,1", "0,0", "1,1").is_err());
    assert_eq!(precinct_id(2, 5, 1, 3, 4).unwrap(), 67);
}
struct Source {
    bins: BTreeMap<BinKey, Vec<u8>>,
    read_bytes: usize,
}
impl BinSource for Source {
    fn length(&self, k: BinKey) -> Result<u64, Error> {
        Ok(self.bins.get(&k).ok_or(Error::Source)?.len() as u64)
    }
    fn read(&mut self, k: BinKey, o: u64, out: &mut [u8]) -> Result<(), Error> {
        let b = self.bins.get(&k).ok_or(Error::Source)?;
        let r = b
            .get(o as usize..o as usize + out.len())
            .ok_or(Error::Source)?;
        out.copy_from_slice(r);
        self.read_bytes += out.len();
        Ok(())
    }
    fn completed_packets(&self, k: BinKey, p: u64) -> Option<u64> {
        if k.class == 0 { Some(p / 128) } else { None }
    }
}
fn source() -> Source {
    Source {
        bins: BTreeMap::from([
            (key(0), (0..400).map(|v| v as u8).collect()),
            (BinKey::new(2, 0).unwrap(), vec![]),
            (BinKey::new(8, 0).unwrap(), vec![]),
        ]),
        read_bytes: 0,
    }
}
fn demands() -> Vec<Demand> {
    vec![
        Demand {
            key: BinKey::new(2, 0).unwrap(),
            prefix: None,
        },
        Demand {
            key: BinKey::new(8, 0).unwrap(),
            prefix: None,
        },
        Demand {
            key: key(0),
            prefix: None,
        },
    ]
}
#[test]
fn bounded_delivery_interrupt_retry_reconnect_and_warm_reuse() {
    let mut s = source();
    let mut c = cache(1024);
    let limits = DeliveryLimits {
        response_bytes: 110,
        message_bytes: 67,
        messages: 50,
    };
    let mut rounds = 0;
    while !c.is_complete(key(0)) {
        let model = c.model();
        let mut bytes = Vec::new();
        let stats = deliver(&mut s, &demands(), &model, limits, true, |p| {
            bytes.extend_from_slice(p);
            Ok(())
        })
        .unwrap();
        assert!(stats.wire_bytes <= 113);
        assert_eq!(stats.wire_bytes, bytes.len() as u64);
        // First response interrupted inside a body; its bytes remain useful.
        if rounds == 0 {
            let cut = bytes.len() - 8;
            receive(&mut c, &bytes[..cut], 3, false);
        } else {
            receive(&mut c, &bytes, 3, true);
        }
        rounds += 1;
        assert!(rounds < 10);
        assert!(!c.bind_identity("immutable-a").unwrap());
    }
    assert!(c.is_complete(BinKey::new(2, 0).unwrap()));
    assert!(c.is_complete(BinKey::new(8, 0).unwrap()));
    let mut out = vec![0; 400];
    c.read(key(0), 0, &mut out).unwrap();
    assert_eq!(out, s.bins[&key(0)]);
    let stats = deliver(&mut s, &demands(), &c.model(), limits, false, |_| Ok(())).unwrap();
    assert_eq!(stats.source_bytes, 0);
    assert_eq!(stats.wire_bytes, 3);
    assert!(stats.window_complete);
    c.evict(key(0));
    let stats = deliver(&mut s, &demands(), &c.model(), limits, false, |_| Ok(())).unwrap();
    assert!(stats.source_bytes > 0);
}
#[test]
fn exact_len_boundaries_empty_final_and_source_failures() {
    for budget in 0..280 {
        let mut s = source();
        let mut bytes = Vec::new();
        let stats = deliver(
            &mut s,
            &demands(),
            &CacheModel::new(),
            DeliveryLimits {
                response_bytes: budget,
                message_bytes: 1024,
                messages: 8,
            },
            true,
            |p| {
                bytes.extend_from_slice(p);
                Ok(())
            },
        )
        .unwrap();
        assert!(stats.wire_bytes <= budget + 3);
        let mut c = cache(1024);
        receive(&mut c, &bytes, 7, true);
    }
    let empty = Demand {
        key: BinKey::new(2, 0).unwrap(),
        prefix: None,
    };
    let stats = deliver(
        &mut source(),
        &[empty],
        &CacheModel::new(),
        DeliveryLimits {
            response_bytes: 5,
            message_bytes: 1,
            messages: 1,
        },
        false,
        |_| Ok(()),
    )
    .unwrap();
    assert!(stats.window_complete);
    assert_eq!(stats.wire_bytes, 8);
    assert_eq!(
        deliver(
            &mut source(),
            &demands(),
            &CacheModel::new(),
            DeliveryLimits {
                response_bytes: 1000,
                message_bytes: 10,
                messages: 99
            },
            false,
            |_| Err(Error::Source)
        ),
        Err(Error::Source)
    );
}

#[test]
fn zero_identity_and_metadata_csn_do_not_leak_reusable_state() {
    let mut c = cache(32);
    // Metadata ignores CSn; an unknown odd class still has an auxiliary field.
    let bytes = [0x70, 8, 5, 0, 1, 42, 0x60, 11, 0, 0, 1, 9, 99, 0, 2, 0];
    receive(&mut c, &bytes, 1, true);
    assert!(c.is_complete(BinKey::new(8, 0).unwrap()));
    assert_eq!(c.bytes(), 1);
    c.bind_identity("0").unwrap();
    c.insert(msg(0, 0, true, &[1])).unwrap();
    assert!(c.model().is_empty());
    c.bind_identity("0").unwrap();
    assert_eq!(c.bytes(), 0);
}

#[test]
fn sparse_arrival_permutations_preserve_only_actual_prefixes() {
    let orders = [
        [0, 1, 2, 3, 4, 5],
        [5, 4, 3, 2, 1, 0],
        [2, 0, 4, 1, 5, 3],
        [1, 3, 5, 0, 2, 4],
    ];
    for order in orders {
        let mut c = cache(32);
        let mut present = [false; 6];
        for at in order {
            c.insert(msg(0, (at * 2) as u64, at == 5, &[at as u8, at as u8]))
                .unwrap();
            present[at] = true;
            let prefix = present.iter().take_while(|v| **v).count() * 2;
            assert_eq!(c.bytes(), present.iter().filter(|v| **v).count() * 2);
            let model = c.model();
            assert_eq!(
                model.get(&key(0)).copied(),
                if prefix == 12 {
                    Some(Known::Complete)
                } else if prefix == 0 {
                    None
                } else {
                    Some(Known::Prefix(prefix as u64))
                }
            );
        }
    }
}

#[test]
fn source_error_and_response_message_limit_are_explicit() {
    struct Broken;
    impl BinSource for Broken {
        fn length(&self, _: BinKey) -> Result<u64, Error> {
            Ok(5)
        }
        fn read(&mut self, _: BinKey, _: u64, _: &mut [u8]) -> Result<(), Error> {
            Err(Error::Source)
        }
    }
    let limits = DeliveryLimits {
        response_bytes: 100,
        message_bytes: 2,
        messages: 1,
    };
    assert_eq!(
        deliver(
            &mut Broken,
            &[Demand {
                key: key(0),
                prefix: None
            }],
            &CacheModel::new(),
            limits,
            false,
            |_| Ok(())
        ),
        Err(Error::Source)
    );
    let mut bytes = Vec::new();
    let stats = deliver(
        &mut source(),
        &demands(),
        &CacheModel::new(),
        limits,
        false,
        |p| {
            bytes.extend_from_slice(p);
            Ok(())
        },
    )
    .unwrap();
    assert!(!stats.window_complete);
    assert_eq!(&bytes[bytes.len() - 3..], &[0, 7, 0]);
}

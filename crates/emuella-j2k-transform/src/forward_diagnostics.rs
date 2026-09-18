//! Feature-only observations of the ordinary bounded forward transform body.
use std::{cell::RefCell, time::Instant};

/// Nested intervals, excluding clock/accounting and loop overhead. These are
/// diagnostic observations, not ordinary throughput measurements.
#[derive(Debug, Clone, Default)]
pub struct ForwardTransformDiagnostic {
    pub validation_ns: u128,
    pub vertical_gather_ns: u128,
    pub vertical_lifting_ns: u128,
    pub vertical_store_ns: u128,
    pub horizontal_lifting_ns: u128,
    pub horizontal_copy_ns: u128,
}

thread_local! {
    static CURRENT: RefCell<Option<ForwardTransformDiagnostic>> = const { RefCell::new(None) };
}

/// Observe synchronous calls to the existing bounded forward transform.
/// Nested observation is rejected; state is removed on error or unwind.
pub fn observe_forward_transform<T>(call: impl FnOnce() -> T) -> (T, ForwardTransformDiagnostic) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            CURRENT.with(|c| {
                c.borrow_mut().take();
            });
        }
    }
    CURRENT.with(|c| {
        assert!(c.borrow().is_none(), "nested forward transform observation");
        *c.borrow_mut() = Some(ForwardTransformDiagnostic::default());
    });
    let reset = Reset;
    let result = call();
    let observation = CURRENT.with(|c| c.borrow_mut().take().expect("active observation"));
    drop(reset);
    (result, observation)
}

pub(super) struct Clock(Option<Instant>);
impl Clock {
    pub(super) fn start() -> Self {
        Self(CURRENT.with(|c| c.borrow().is_some()).then(Instant::now))
    }
    pub(super) fn finish(self, field: impl FnOnce(&mut ForwardTransformDiagnostic) -> &mut u128) {
        if let Some(start) = self.0 {
            let ns = start.elapsed().as_nanos();
            CURRENT.with(|c| {
                if let Some(d) = c.borrow_mut().as_mut() {
                    *field(d) += ns;
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn observed_forward_matches_ordinary_and_checked_coefficients() {
        for (width, height) in [(1, 1), (1, 9), (8, 1), (7, 9), (8, 10), (65, 64)] {
            for (x_origin, y_origin) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let config = Reversible53Config {
                    width,
                    height,
                    stride: width + 3,
                    edges: Reversible53Edges::from_tile_origin(x_origin, y_origin, width, height),
                    sample_range: ComponentSampleRange::signed(32),
                };
                let source: Vec<i32> = (0..config.stride * height)
                    .map(|n| ((n * 7919 + n / 7) % 131071) as i32 - 65535)
                    .collect();
                let mut plain = source.clone();
                let mut checked = source.clone();
                let mut observed = source;
                let mut scratch = vec![0; config.scratch_len()];
                forward_reversible_5_3(&mut checked, config, &mut scratch).unwrap();
                forward_reversible_5_3_bounded(&mut plain, config, &mut scratch).unwrap();
                let start = Instant::now();
                let (result, detail) = observe_forward_transform(|| {
                    forward_reversible_5_3_bounded(&mut observed, config, &mut scratch)
                });
                result.unwrap();
                assert_eq!(observed, plain);
                assert_eq!(observed, checked);
                let sum = detail.validation_ns
                    + detail.vertical_gather_ns
                    + detail.vertical_lifting_ns
                    + detail.vertical_store_ns
                    + detail.horizontal_lifting_ns
                    + detail.horizontal_copy_ns;
                assert!(sum <= start.elapsed().as_nanos());
            }
        }
    }

    #[test]
    fn observer_releases_after_unwind() {
        assert!(
            std::panic::catch_unwind(|| observe_forward_transform(|| panic!("authored fault")))
                .is_err()
        );
        let (_, detail) = observe_forward_transform(|| ());
        assert_eq!(detail.validation_ns, 0);
    }
}

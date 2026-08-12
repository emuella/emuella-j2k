use emuella_j2k_core::{
    ComponentLayout, DecodeOptions, ImageData, InputFormat, InspectOptions, decode, inspect,
};
use emuella_j2k_test_support::{generate_grayscale_j2k, grayscale_gradient};

#[test]
fn generated_j2k_is_deterministic_and_self_describing() {
    let first = generate_grayscale_j2k(17, 19).expect("project generator succeeds");
    let second = generate_grayscale_j2k(17, 19).expect("project generator is repeatable");
    assert_eq!(first, second);
    assert!(first.starts_with(&[0xff, 0x4f]));

    let metadata = inspect(&first, &InspectOptions::default()).expect("generated J2K parses");
    assert_eq!(metadata.format, InputFormat::J2kCodestream);
    let image = metadata.image.expect("generated J2K has image metadata");
    assert_eq!((image.width, image.height, image.components), (17, 19, 1));
}

#[test]
fn generated_j2k_round_trips_without_external_fixtures() {
    let encoded = generate_grayscale_j2k(17, 19).expect("project generator succeeds");
    let decoded = decode(
        &encoded,
        &DecodeOptions {
            target_layout: ComponentLayout::Interleaved,
            ..DecodeOptions::default()
        },
    )
    .expect("generated J2K decodes");
    assert_eq!(
        decoded.data,
        ImageData::Interleaved(grayscale_gradient(17, 19))
    );
}

#[test]
fn truncated_input_fails_closed() {
    assert!(inspect(&[], &InspectOptions::default()).is_err());
    assert!(inspect(&[0xff, 0x4f], &InspectOptions::default()).is_err());
}

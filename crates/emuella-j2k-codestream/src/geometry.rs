//! Checked JPEG 2000 Part 1 native geometry.
//!
//! The public decode API keeps image-relative requests for compatibility. This
//! module gives the codestream implementation distinct absolute domains before
//! those requests reach component or reduced-resolution arithmetic.

use crate::{CodestreamError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HalfOpenRect {
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
}

impl HalfOpenRect {
    fn new(x0: u32, y0: u32, x1: u32, y1: u32) -> Result<Self> {
        if x0 >= x1 || y0 >= y1 {
            return Err(CodestreamError::SizeOverflow);
        }
        Ok(Self { x0, y0, x1, y1 })
    }

    fn from_origin_and_size(x: u32, y: u32, width: u32, height: u32) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(CodestreamError::SizeOverflow);
        }
        Self::new(
            x,
            y,
            x.checked_add(width).ok_or(CodestreamError::SizeOverflow)?,
            y.checked_add(height).ok_or(CodestreamError::SizeOverflow)?,
        )
    }

    fn width(self) -> u32 {
        self.x1 - self.x0
    }

    fn height(self) -> u32 {
        self.y1 - self.y0
    }

    fn intersection(self, other: Self) -> Option<Self> {
        Self::new(
            self.x0.max(other.x0),
            self.y0.max(other.y0),
            self.x1.min(other.x1),
            self.y1.min(other.y1),
        )
        .ok()
    }

    fn translate(self, x: u32, y: u32) -> Result<Self> {
        Self::new(
            self.x0
                .checked_add(x)
                .ok_or(CodestreamError::SizeOverflow)?,
            self.y0
                .checked_add(y)
                .ok_or(CodestreamError::SizeOverflow)?,
            self.x1
                .checked_add(x)
                .ok_or(CodestreamError::SizeOverflow)?,
            self.y1
                .checked_add(y)
                .ok_or(CodestreamError::SizeOverflow)?,
        )
    }

    fn ceil_div(self, x_divisor: u32, y_divisor: u32) -> Result<Self> {
        Self::new(
            checked_ceil_div(self.x0, x_divisor)?,
            checked_ceil_div(self.y0, y_divisor)?,
            checked_ceil_div(self.x1, x_divisor)?,
            checked_ceil_div(self.y1, y_divisor)?,
        )
    }
}

fn checked_ceil_div(value: u32, divisor: u32) -> Result<u32> {
    if divisor == 0 {
        return Err(CodestreamError::SizeOverflow);
    }
    Ok(value.div_ceil(divisor))
}

/// Non-empty absolute half-open rectangle on the Part 1 reference grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReferenceGridRect(HalfOpenRect);

impl ReferenceGridRect {
    pub fn new(x0: u32, y0: u32, x1: u32, y1: u32) -> Result<Self> {
        HalfOpenRect::new(x0, y0, x1, y1).map(Self)
    }

    pub fn from_origin_and_size(x: u32, y: u32, width: u32, height: u32) -> Result<Self> {
        HalfOpenRect::from_origin_and_size(x, y, width, height).map(Self)
    }

    /// Translate a checked full-resolution image-relative request to the
    /// absolute reference grid.
    pub fn from_image_relative(
        image: Self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let relative = HalfOpenRect::from_origin_and_size(x, y, width, height)?;
        if relative.x1 > image.width() || relative.y1 > image.height() {
            return Err(CodestreamError::SizeOverflow);
        }
        relative
            .translate(image.x0(), image.y0())
            .map(ReferenceGridRect)
    }

    pub fn intersection(self, other: Self) -> Option<Self> {
        self.0.intersection(other.0).map(Self)
    }

    pub fn to_component_grid(
        self,
        horizontal_separation: u8,
        vertical_separation: u8,
    ) -> Result<ComponentGridRect> {
        self.0
            .ceil_div(
                u32::from(horizontal_separation),
                u32::from(vertical_separation),
            )
            .map(ComponentGridRect)
    }

    pub fn x0(self) -> u32 {
        self.0.x0
    }

    pub fn y0(self) -> u32 {
        self.0.y0
    }

    pub fn x1(self) -> u32 {
        self.0.x1
    }

    pub fn y1(self) -> u32 {
        self.0.y1
    }

    pub fn width(self) -> u32 {
        self.0.width()
    }

    pub fn height(self) -> u32 {
        self.0.height()
    }
}

/// A clipped nominal tile in absolute reference-grid coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileReferenceRect {
    pub tile_index: u16,
    pub tile_x: u32,
    pub tile_y: u32,
    bounds: ReferenceGridRect,
}

impl TileReferenceRect {
    #[allow(clippy::too_many_arguments)]
    pub fn clipped_to_image(
        image: ReferenceGridRect,
        tile_origin_x: u32,
        tile_origin_y: u32,
        tile_width: u32,
        tile_height: u32,
        tile_x: u32,
        tile_y: u32,
        tile_index: u16,
    ) -> Result<Option<Self>> {
        let x = tile_origin_x
            .checked_add(
                tile_x
                    .checked_mul(tile_width)
                    .ok_or(CodestreamError::SizeOverflow)?,
            )
            .ok_or(CodestreamError::SizeOverflow)?;
        let y = tile_origin_y
            .checked_add(
                tile_y
                    .checked_mul(tile_height)
                    .ok_or(CodestreamError::SizeOverflow)?,
            )
            .ok_or(CodestreamError::SizeOverflow)?;
        let nominal = ReferenceGridRect::from_origin_and_size(x, y, tile_width, tile_height)?;
        Ok(image.intersection(nominal).map(|bounds| Self {
            tile_index,
            tile_x,
            tile_y,
            bounds,
        }))
    }

    pub fn bounds(self) -> ReferenceGridRect {
        self.bounds
    }

    pub fn to_component_grid(
        self,
        horizontal_separation: u8,
        vertical_separation: u8,
    ) -> Result<ComponentGridRect> {
        self.bounds
            .to_component_grid(horizontal_separation, vertical_separation)
    }
}

/// Non-empty absolute half-open rectangle on one native component grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentGridRect(HalfOpenRect);

impl ComponentGridRect {
    pub fn reduce(self, discard_levels: u8) -> Result<ReducedComponentRect> {
        let scale = 1_u32
            .checked_shl(u32::from(discard_levels))
            .ok_or(CodestreamError::SizeOverflow)?;
        self.0
            .ceil_div(scale, scale)
            .map(|bounds| ReducedComponentRect {
                bounds,
                discard_levels,
            })
    }

    pub fn x0(self) -> u32 {
        self.0.x0
    }

    pub fn y0(self) -> u32 {
        self.0.y0
    }

    pub fn x1(self) -> u32 {
        self.0.x1
    }

    pub fn y1(self) -> u32 {
        self.0.y1
    }

    pub fn width(self) -> u32 {
        self.0.width()
    }

    pub fn height(self) -> u32 {
        self.0.height()
    }
}

/// Non-empty absolute component rectangle at one stated discard level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReducedComponentRect {
    bounds: HalfOpenRect,
    discard_levels: u8,
}

impl ReducedComponentRect {
    pub fn discard_levels(self) -> u8 {
        self.discard_levels
    }

    pub fn x0(self) -> u32 {
        self.bounds.x0
    }

    pub fn y0(self) -> u32 {
        self.bounds.y0
    }

    pub fn x1(self) -> u32 {
        self.bounds.x1
    }

    pub fn y1(self) -> u32 {
        self.bounds.y1
    }

    pub fn width(self) -> u32 {
        self.bounds.width()
    }

    pub fn height(self) -> u32 {
        self.bounds.height()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_non_zero_origins_unequal_sampling_odd_edges_and_reduction() {
        let image = ReferenceGridRect::new(3, 5, 20, 18).unwrap();
        let request = ReferenceGridRect::from_image_relative(image, 2, 1, 9, 8).unwrap();
        assert_eq!(
            (request.x0(), request.y0(), request.x1(), request.y1()),
            (5, 6, 14, 14)
        );

        let component = request.to_component_grid(2, 3).unwrap();
        assert_eq!(
            (
                component.x0(),
                component.y0(),
                component.x1(),
                component.y1()
            ),
            (3, 2, 7, 5)
        );

        let reduced = component.reduce(1).unwrap();
        assert_eq!(reduced.discard_levels(), 1);
        assert_eq!(
            (reduced.x0(), reduced.y0(), reduced.x1(), reduced.y1()),
            (2, 1, 4, 3)
        );
    }

    #[test]
    fn clips_non_zero_origin_tiles_to_the_image() {
        let image = ReferenceGridRect::new(3, 5, 20, 18).unwrap();
        let tile = TileReferenceRect::clipped_to_image(image, 1, 2, 8, 7, 0, 0, 0)
            .unwrap()
            .unwrap();
        assert_eq!(tile.bounds(), ReferenceGridRect::new(3, 5, 9, 9).unwrap());
        assert_eq!(tile.to_component_grid(2, 3).unwrap().width(), 3);
    }

    #[test]
    fn rejects_empty_out_of_bounds_zero_divisors_and_overflow() {
        assert!(ReferenceGridRect::new(1, 1, 1, 2).is_err());
        assert!(ReferenceGridRect::from_origin_and_size(u32::MAX, 0, 2, 1).is_err());
        let image = ReferenceGridRect::new(3, 5, 20, 18).unwrap();
        assert!(ReferenceGridRect::from_image_relative(image, 16, 0, 2, 1).is_err());
        assert!(image.to_component_grid(0, 1).is_err());
        assert!(image.to_component_grid(1, 1).unwrap().reduce(32).is_err());
    }

    #[test]
    fn partition_and_stitch_mapping_matches_the_full_mapping() {
        let image = ReferenceGridRect::new(3, 5, 28, 24).unwrap();
        let full = ReferenceGridRect::from_image_relative(image, 1, 2, 19, 13)
            .unwrap()
            .to_component_grid(2, 3)
            .unwrap()
            .reduce(2)
            .unwrap();
        let left = ReferenceGridRect::from_image_relative(image, 1, 2, 7, 13)
            .unwrap()
            .to_component_grid(2, 3)
            .unwrap()
            .reduce(2)
            .unwrap();
        let right = ReferenceGridRect::from_image_relative(image, 8, 2, 12, 13)
            .unwrap()
            .to_component_grid(2, 3)
            .unwrap()
            .reduce(2)
            .unwrap();

        assert_eq!((left.x0(), right.x1()), (full.x0(), full.x1()));
        assert_eq!((left.y0(), left.y1()), (full.y0(), full.y1()));
        assert_eq!((right.y0(), right.y1()), (full.y0(), full.y1()));
        assert_eq!(left.x1(), right.x0());
    }
}

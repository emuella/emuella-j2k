//! Checked JPEG 2000 Part 1 native and rendered-canvas geometry.
//!
//! The public decode API keeps image-relative requests for compatibility. This
//! module gives the codestream implementation distinct absolute domains before
//! those requests reach component or reduced-resolution arithmetic. Rendered
//! canvas planning selects geometry only; it does not select registration,
//! interpolation, sample conversion, or storage policy.

use alloc::vec::Vec;

use crate::{CodestreamError, ComponentParameters, Result};

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

    fn to_rendered_grid(
        self,
        horizontal_spacing: u8,
        vertical_spacing: u8,
    ) -> Result<RenderedRect> {
        self.0
            .ceil_div(u32::from(horizontal_spacing), u32::from(vertical_spacing))
            .map(RenderedRect)
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

/// Non-empty absolute half-open rectangle on the selected rendered canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderedRect(HalfOpenRect);

impl RenderedRect {
    pub fn intersection(self, other: Self) -> Option<Self> {
        self.0.intersection(other.0).map(Self)
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

/// Checked full-resolution default rendered-canvas geometry.
///
/// The axis spacing is the greatest common divisor of the corresponding SIZ
/// component separations. Component mappings retain their absolute native
/// bounds for a later, separately selected projection policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedCanvasPlan {
    reference_bounds: ReferenceGridRect,
    bounds: RenderedRect,
    horizontal_spacing: u8,
    vertical_spacing: u8,
    component_bounds: Vec<ComponentGridRect>,
}

impl RenderedCanvasPlan {
    pub fn new(
        reference_bounds: ReferenceGridRect,
        components: &[ComponentParameters],
    ) -> Result<Self> {
        if components.is_empty() {
            return Err(CodestreamError::SizeOverflow);
        }

        let mut horizontal_spacing = 0;
        let mut vertical_spacing = 0;
        let mut component_bounds = Vec::with_capacity(components.len());
        for component in components {
            if component.horizontal_separation == 0 || component.vertical_separation == 0 {
                return Err(CodestreamError::SizeOverflow);
            }
            horizontal_spacing =
                greatest_common_divisor(horizontal_spacing, component.horizontal_separation);
            vertical_spacing =
                greatest_common_divisor(vertical_spacing, component.vertical_separation);
            component_bounds.push(reference_bounds.to_component_grid(
                component.horizontal_separation,
                component.vertical_separation,
            )?);
        }

        let bounds = reference_bounds.to_rendered_grid(horizontal_spacing, vertical_spacing)?;
        Ok(Self {
            reference_bounds,
            bounds,
            horizontal_spacing,
            vertical_spacing,
            component_bounds,
        })
    }

    pub fn bounds(&self) -> RenderedRect {
        self.bounds
    }

    pub fn horizontal_spacing(&self) -> u8 {
        self.horizontal_spacing
    }

    pub fn vertical_spacing(&self) -> u8 {
        self.vertical_spacing
    }

    pub fn width(&self) -> u32 {
        self.bounds.width()
    }

    pub fn height(&self) -> u32 {
        self.bounds.height()
    }

    pub fn component_bounds(&self, component_index: u16) -> Option<ComponentGridRect> {
        self.component_bounds
            .get(usize::from(component_index))
            .copied()
    }

    /// Map one non-empty absolute reference-grid partition to this canvas.
    pub fn map_reference_rect(&self, reference: ReferenceGridRect) -> Result<RenderedRect> {
        if self.reference_bounds.intersection(reference) != Some(reference) {
            return Err(CodestreamError::SizeOverflow);
        }
        reference.to_rendered_grid(self.horizontal_spacing, self.vertical_spacing)
    }
}

fn greatest_common_divisor(mut left: u8, mut right: u8) -> u8 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
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

    fn component(horizontal_separation: u8, vertical_separation: u8) -> ComponentParameters {
        ComponentParameters {
            bits_per_sample: 8,
            signed: false,
            horizontal_separation,
            vertical_separation,
        }
    }

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

    #[test]
    fn rendered_canvas_uses_axis_specific_gcd_and_absolute_odd_bounds() {
        let image = ReferenceGridRect::new(3, 5, 28, 26).unwrap();
        let plan = RenderedCanvasPlan::new(
            image,
            &[component(4, 6), component(6, 9), component(10, 12)],
        )
        .unwrap();
        assert_eq!((plan.horizontal_spacing(), plan.vertical_spacing()), (2, 3));
        assert_eq!(
            (
                plan.bounds().x0(),
                plan.bounds().y0(),
                plan.bounds().x1(),
                plan.bounds().y1(),
                plan.width(),
                plan.height(),
            ),
            (2, 2, 14, 9, 12, 7)
        );

        let unit_horizontal =
            RenderedCanvasPlan::new(image, &[component(2, 4), component(3, 6), component(5, 8)])
                .unwrap();
        assert_eq!(
            (
                unit_horizontal.horizontal_spacing(),
                unit_horizontal.vertical_spacing()
            ),
            (1, 2)
        );
        assert_eq!(
            (unit_horizontal.bounds().x0(), unit_horizontal.bounds().x1()),
            (3, 28)
        );

        let unit_vertical =
            RenderedCanvasPlan::new(image, &[component(4, 2), component(6, 3), component(10, 5)])
                .unwrap();
        assert_eq!(
            (
                unit_vertical.horizontal_spacing(),
                unit_vertical.vertical_spacing()
            ),
            (2, 1)
        );
        assert_eq!(
            (unit_vertical.bounds().y0(), unit_vertical.bounds().y1()),
            (5, 26)
        );
    }

    #[test]
    fn rendered_canvas_retains_each_unequal_native_component_mapping() {
        let plan = RenderedCanvasPlan::new(
            ReferenceGridRect::new(3, 5, 28, 26).unwrap(),
            &[component(4, 6), component(6, 9), component(10, 12)],
        )
        .unwrap();
        let mappings = (0..3)
            .map(|index| {
                let bounds = plan.component_bounds(index).unwrap();
                (
                    bounds.x0(),
                    bounds.y0(),
                    bounds.x1(),
                    bounds.y1(),
                    bounds.width(),
                    bounds.height(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            mappings,
            [(1, 1, 7, 5, 6, 4), (1, 1, 5, 3, 4, 2), (1, 1, 3, 3, 2, 2)]
        );
        assert_eq!(plan.component_bounds(3), None);
    }

    #[test]
    fn rendered_clipped_partitions_map_to_exact_stitch_geometry() {
        let image = ReferenceGridRect::new(3, 5, 28, 26).unwrap();
        let plan = RenderedCanvasPlan::new(
            image,
            &[component(4, 6), component(6, 9), component(10, 12)],
        )
        .unwrap();
        let left = plan
            .map_reference_rect(ReferenceGridRect::new(3, 5, 15, 26).unwrap())
            .unwrap();
        let right = plan
            .map_reference_rect(ReferenceGridRect::new(15, 5, 28, 26).unwrap())
            .unwrap();
        assert_eq!(left.x1(), right.x0());
        assert_eq!(left.width() + right.width(), plan.width());
        assert_eq!((left.y0(), left.y1()), (right.y0(), right.y1()));

        let clipped = TileReferenceRect::clipped_to_image(image, 1, 2, 10, 9, 0, 0, 0)
            .unwrap()
            .unwrap();
        let rendered_tile = plan.map_reference_rect(clipped.bounds()).unwrap();
        assert_eq!(
            (
                rendered_tile.x0(),
                rendered_tile.y0(),
                rendered_tile.x1(),
                rendered_tile.y1(),
            ),
            (2, 2, 6, 4)
        );
        assert_eq!(
            plan.bounds().intersection(rendered_tile),
            Some(rendered_tile)
        );
    }

    #[test]
    fn rendered_canvas_rejects_malformed_empty_and_overflowing_geometry() {
        let image = ReferenceGridRect::new(3, 5, 28, 26).unwrap();
        assert!(RenderedCanvasPlan::new(image, &[]).is_err());
        assert!(RenderedCanvasPlan::new(image, &[component(0, 1)]).is_err());
        assert!(RenderedCanvasPlan::new(image, &[component(1, 0)]).is_err());

        let collapsed = ReferenceGridRect::new(1, 1, 2, 2).unwrap();
        assert!(RenderedCanvasPlan::new(collapsed, &[component(2, 2)]).is_err());

        let plan = RenderedCanvasPlan::new(image, &[component(2, 3)]).unwrap();
        assert!(
            plan.map_reference_rect(ReferenceGridRect::new(2, 5, 4, 8).unwrap())
                .is_err()
        );
        assert!(ReferenceGridRect::from_origin_and_size(u32::MAX, 0, 2, 1).is_err());
    }
}

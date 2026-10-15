// Copyright 2026 the CrownOS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use kurbo_expand::{BezPath, Diagonal2, Join, PathEl, Point, expand_path};
use skrifa::outline::OutlinePen;

/// Outline growth per side, in pixels at the rasterised size.
///
/// Thin stems of unhinted fonts (Inter, 1.1px at 13px) otherwise spread over
/// two half-lit columns and read as blurry next to bytecode-hinted fonts.
const STEM_DARKENING: f64 = 0.07;
const MITER_LIMIT: f64 = 4.0;
const TOLERANCE: f64 = 0.1;

#[derive(Default)]
pub(crate) struct DarkeningPen {
    outline: BezPath,
}

impl DarkeningPen {
    pub(crate) fn finish(self, target: &mut impl OutlinePen) {
        let darkened = expand_path(
            &self.outline,
            Diagonal2::new(STEM_DARKENING, STEM_DARKENING),
            Join::Miter,
            MITER_LIMIT,
            TOLERANCE,
        );
        for element in darkened.elements() {
            match *element {
                PathEl::MoveTo(to) => target.move_to(to.x as f32, to.y as f32),
                PathEl::LineTo(to) => target.line_to(to.x as f32, to.y as f32),
                PathEl::QuadTo(control, to) => {
                    target.quad_to(control.x as f32, control.y as f32, to.x as f32, to.y as f32);
                }
                PathEl::CurveTo(first, second, to) => target.curve_to(
                    first.x as f32,
                    first.y as f32,
                    second.x as f32,
                    second.y as f32,
                    to.x as f32,
                    to.y as f32,
                ),
                PathEl::ClosePath => target.close(),
            }
        }
    }
}

fn point(x: f32, y: f32) -> Point {
    Point::new(x.into(), y.into())
}

impl OutlinePen for DarkeningPen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.outline.move_to(point(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.outline.line_to(point(x, y));
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.outline.quad_to(point(cx0, cy0), point(x, y));
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.outline
            .curve_to(point(cx0, cy0), point(cx1, cy1), point(x, y));
    }

    fn close(&mut self) {
        self.outline.close_path();
    }
}

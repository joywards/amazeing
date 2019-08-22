use std::collections::HashSet;
use crate::region::Region;
use crate::build::make_circle;

// This value is used by both renderer and maze builder.
// That's why using global value is simplier than passing it through call stack.
pub fn visibility_radius() -> i32 {
    12
}

// This region is used by both renderer and maze builder too.
// Moreover it is expensive to copy, so reference to the same object is used.
pub fn visible_area() -> &'static Region {
    lazy_static! {
        static ref VISIBLE_AREA: Region = {
            make_circle(visibility_radius()).collect::<HashSet<_>>().into()
        };
    }
    &VISIBLE_AREA
}

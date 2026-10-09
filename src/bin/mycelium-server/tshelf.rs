// 78.152 TU-7 — THIS DOOR'S TUNIVERSAL SHELF: one per process, beside the
// 78.151 published reading and outside the kernel. The keepers' quorum raises
// its final point; every frame read at the door is heard on it; every reply
// the door writes carries its final point.

use mycelium::chronometry::tuniversal::{TPoint, TShelf};
use std::sync::LazyLock;

static SHELF: LazyLock<TShelf> = LazyLock::new(TShelf::new);

/// The door's shelf.
pub(crate) fn shelf() -> &'static TShelf {
    &SHELF
}

/// A frame read at the door, with the point its speaker carried.
pub(crate) fn heard_at_door(point: Option<TPoint>) {
    SHELF.heard_at_door(point);
}

/// The final point every reply of this door carries.
pub(crate) fn final_point() -> Option<TPoint> {
    SHELF.final_point()
}

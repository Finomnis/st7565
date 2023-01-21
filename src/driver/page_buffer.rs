/// A graphics page buffer, required for graphics mode.
///
/// Needs to be provided by the user.
pub struct GraphicsPageBuffer<const WIDTH: usize, const PAGES: usize>(
    pub [([u8; WIDTH], Option<(usize, usize)>); PAGES],
);

impl<const WIDTH: usize, const PAGES: usize> GraphicsPageBuffer<WIDTH, PAGES> {
    /// Creates a graphics page buffer.
    pub const fn new() -> Self {
        Self(
            // Fill with full dirty flags to force an initial synchronization
            [([0; WIDTH], Some((0, WIDTH))); PAGES],
        )
    }

    /// Marks the entire buffer as dirty, causing a full retransmission of
    /// all data at next `flush()`.
    pub fn mark_dirty(&mut self) {
        for page in &mut self.0 {
            page.1 = Some((0, WIDTH));
        }
    }
}

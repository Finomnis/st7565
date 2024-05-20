#[derive(Copy, Clone)]
pub(crate) struct BufferPage<const WIDTH: usize> {
    pub data: [u8; WIDTH],
    pub dirty: Option<(usize, usize)>,
}

/// A graphics page buffer, required for graphics mode.
///
/// Needs to be provided by the user.
#[derive(Clone)]
pub struct GraphicsPageBuffer<const WIDTH: usize, const PAGES: usize> {
    pub(crate) pages: [BufferPage<WIDTH>; PAGES],
}

impl<const WIDTH: usize, const PAGES: usize> GraphicsPageBuffer<WIDTH, PAGES> {
    /// Creates a graphics page buffer.
    pub const fn new() -> Self {
        Self {
            // Fill with full dirty flags to force an initial synchronization
            pages: [BufferPage {
                data: [0; WIDTH],
                dirty: Some((0, WIDTH)),
            }; PAGES],
        }
    }

    /// Marks the entire buffer as dirty, causing a full retransmission of
    /// all data at next [`flush()`](crate::ST7565::flush).
    pub fn mark_dirty(&mut self) {
        for page in &mut self.pages {
            page.dirty = Some((0, WIDTH));
        }
    }
}

impl<const WIDTH: usize, const PAGES: usize> Default for GraphicsPageBuffer<WIDTH, PAGES> {
    fn default() -> Self {
        Self::new()
    }
}

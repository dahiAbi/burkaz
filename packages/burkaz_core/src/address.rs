#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BurkazObjectAddr(u64);

impl std::fmt::Display for BurkazObjectAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BurkazObjectAddr(segment_ord: {}, doc_id: {})",
            self.segment_ord(),
            self.doc_id()
        )
    }
}

impl BurkazObjectAddr {
    #[inline]
    pub const fn segment_ord(&self) -> tantivy::SegmentOrdinal {
        (self.0 >> 32) as u32
    }

    #[inline]
    pub const fn doc_id(&self) -> u32 {
        (self.0 & u32::MAX as u64) as u32
    }

    #[inline]
    pub const fn val(&self) -> u64 {
        self.0
    }
}

impl From<u64> for BurkazObjectAddr {
    fn from(val: u64) -> Self {
        Self(val)
    }
}

impl From<tantivy::DocAddress> for BurkazObjectAddr {
    fn from(addr: tantivy::DocAddress) -> Self {
        Self((addr.segment_ord as u64) << 32 | addr.doc_id as u64)
    }
}

impl Into<tantivy::DocAddress> for BurkazObjectAddr {
    fn into(self) -> tantivy::DocAddress {
        tantivy::DocAddress::new(self.segment_ord(), self.doc_id())
    }
}

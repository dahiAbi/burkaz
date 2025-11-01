use tantivy::{
    Searcher,
    collector::{Count, TopDocs},
    query::Query,
};

use crate::{
    address::BurkazObjectAddr,
    error::BurkazError,
    index::{BurkazIndex, WeakBurkazIndex},
    query::BurkazQuery,
};

pub struct QueryRunner {
    _index: WeakBurkazIndex,
    _query: BurkazQuery,
}

impl QueryRunner {
    #[inline]
    pub fn new(index: WeakBurkazIndex, query: BurkazQuery) -> Self {
        Self {
            _index: index,
            _query: query,
        }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    #[inline]
    fn index(&self) -> Option<BurkazIndex> {
        self._index.upgrade()
    }

    #[inline]
    fn searcher(&self) -> Option<Searcher> {
        self._index.upgrade().map(move |index| index.searcher())
    }

    fn query(&self) -> Option<Box<dyn Query>> {
        self.index()
            .map(|index| self._query.to_tantivy_query(&index))
    }

    pub fn count(&self) -> crate::Result<usize> {
        self.searcher()
            .ok_or(BurkazError::IndexClosed)?
            .search(&self.query().ok_or(BurkazError::IndexClosed)?, &Count)
            .map_err(Into::into)
    }

    pub fn search(&self, offset: usize, limit: usize) -> crate::Result<Vec<BurkazObjectAddr>> {
        let query = self.query().ok_or(BurkazError::IndexClosed)?;
        let collector = TopDocs::with_limit(limit).and_offset(offset);
        let score_and_addrs = self
            .searcher()
            .ok_or(BurkazError::IndexClosed)?
            .search(&query, &collector)?;
        Ok(score_and_addrs
            .iter()
            .map(move |(_, addr)| (*addr).into())
            .collect())
    }

    pub fn delete_all(&self) -> crate::Result<()> {
        self.index()
            .ok_or(BurkazError::IndexClosed)?
            .delete_all_by_query(self.query().ok_or(BurkazError::IndexClosed)?)
    }
}

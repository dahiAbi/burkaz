use tantivy::{
    Searcher,
    collector::{Count, TopDocs},
    query::Query,
};

use crate::{address::BurkazObjectAddr, index::BurkazIndex, query::BurkazQuery};

pub struct QueryRunner {
    _index: BurkazIndex,
    _query: BurkazQuery,
}

impl QueryRunner {
    #[inline]
    pub fn new(index: BurkazIndex, query: BurkazQuery) -> Self {
        Self {
            _index: index,
            _query: query,
        }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    #[inline]
    fn searcher(&self) -> Searcher {
        self._index.searcher()
    }

    fn query(&self) -> Box<dyn Query> {
        self._query.to_tantivy_query(&self._index)
    }

    pub fn count(&self) -> crate::Result<usize> {
        self.searcher()
            .search(&self.query(), &Count)
            .map_err(Into::into)
    }

    pub fn search(&self, offset: usize, limit: usize) -> crate::Result<Vec<BurkazObjectAddr>> {
        let query = self.query();
        let collector = TopDocs::with_limit(limit).and_offset(offset);
        let score_and_addrs = self.searcher().search(&query, &collector)?;
        Ok(score_and_addrs
            .iter()
            .map(move |(_, addr)| (*addr).into())
            .collect())
    }

    pub fn delete_all(&self) -> crate::Result<()> {
        self._index.delete_all_by_query(self.query())
    }
}

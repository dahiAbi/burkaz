use tantivy::query::{
    AllQuery, BooleanQuery, BoostQuery, EmptyQuery, FuzzyTermQuery, Occur, PhraseQuery, Query,
    RegexPhraseQuery, TermQuery, TermSetQuery,
};

use crate::{index::BurkazIndex, schema::BurkazIndexingStrategy, term::BurkazTerm};

/// A query that can be executed against the index.
pub enum BurkazQuery {
    /// Matches all documents.
    All,
    /// Matches no documents.
    Empty,
    /// All child queries must match (AND).
    And(Vec<BurkazQuery>),
    /// At least one child query must match (OR).
    Or(Vec<BurkazQuery>),
    /// The child query must not match (NOT).
    Not(Box<BurkazQuery>),
    /// Matches a term.
    Term {
        term: BurkazTerm,
        indexing_strategy: BurkazIndexingStrategy,
    },
    /// Matches a set of terms.
    TermSet {
        terms: Vec<BurkazTerm>,
    },
    /// Matches a fuzzy term.
    FuzzyTerm {
        term: BurkazTerm,
        distance: u8,
        transposition_cost_one: bool,
        prefix: bool,
    },
    /// Matches a phrase.
    Phase {
        terms: Vec<BurkazTerm>,
        slop: u32,
    },
    RegexPhase {
        field_id: u32,
        terms: Vec<String>,
        slop: u32,
        max_expansions: u32,
    },
    Boost {
        query: Box<BurkazQuery>,
        boost: f32,
    },
    Parse {
        query_text: String,
    },
}

impl BurkazQuery {
    pub unsafe fn from_raw(ptr: *mut Self) -> Self {
        unsafe { *Box::from_raw(ptr) }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }
}

impl BurkazQuery {
    pub fn to_tantivy_query(&self, index: &BurkazIndex) -> Box<dyn Query> {
        match self {
            Self::All => Box::new(AllQuery),
            Self::Empty => Box::new(EmptyQuery),
            Self::And(queries) => Box::new(BooleanQuery::new(
                queries
                    .iter()
                    .map(|query| (Occur::Must, query.to_tantivy_query(index)))
                    .collect(),
            )),
            Self::Or(queries) => Box::new(BooleanQuery::new(
                queries
                    .iter()
                    .map(|query| (Occur::Should, query.to_tantivy_query(index)))
                    .collect(),
            )),
            Self::Not(query) => Box::new(BooleanQuery::new(vec![(
                Occur::MustNot,
                query.to_tantivy_query(index),
            )])),
            Self::Term {
                term,
                indexing_strategy,
            } => Box::new(TermQuery::new(
                term.to_tantivy_term(),
                (*indexing_strategy).into(),
            )),
            Self::TermSet { terms } => Box::new(TermSetQuery::new(
                terms
                    .iter()
                    .map(|term| term.to_tantivy_term())
                    .collect::<Vec<_>>(),
            )),
            Self::FuzzyTerm {
                term,
                distance,
                transposition_cost_one,
                prefix,
            } => {
                if *prefix {
                    Box::new(FuzzyTermQuery::new_prefix(
                        term.to_tantivy_term(),
                        *distance,
                        *transposition_cost_one,
                    ))
                } else {
                    Box::new(FuzzyTermQuery::new(
                        term.to_tantivy_term(),
                        *distance,
                        *transposition_cost_one,
                    ))
                }
            }
            Self::Phase { terms, slop } => Box::new(PhraseQuery::new_with_offset_and_slop(
                terms
                    .iter()
                    .enumerate()
                    .map(|(index, term)| (index, term.to_tantivy_term()))
                    .collect(),
                *slop,
            )),
            Self::RegexPhase {
                field_id,
                terms,
                slop,
                max_expansions,
            } => {
                let mut query = Box::new(RegexPhraseQuery::new_with_offset_and_slop(
                    tantivy::schema::Field::from_field_id(*field_id),
                    terms.iter().cloned().enumerate().collect(),
                    *slop,
                ));
                query.set_max_expansions(*max_expansions);
                query
            }
            Self::Boost { query, boost } => {
                Box::new(BoostQuery::new(query.to_tantivy_query(index), *boost))
            }
            Self::Parse { query_text } => {
                let parsed_query = index
                    .query_parser()
                    .parse_query(&query_text)
                    .unwrap_or(Box::new(EmptyQuery));

                parsed_query
            }
        }
    }
}

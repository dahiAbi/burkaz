import 'schema_field_options.dart';

import 'term.dart';

typedef Score = double;

/// A query on a burkaz index.
sealed class Query {
  const Query();
}

/// A query that matches all documents.
class AllQuery extends Query {
  const AllQuery();
}

/// A query that matches no documents.
class EmptyQuery extends Query {
  const EmptyQuery();
}

/// A query that matches a term.
class TermQuery extends Query {
  const TermQuery({
    required this.term,
    this.indexingStrategy = IndexingStrategy.basic,
  });

  /// The term to match.
  final Term term;

  /// The indexing strategy to use.
  final IndexingStrategy indexingStrategy;
}

/// A query that matches a set of terms.
class TermSetQuery extends Query {
  const TermSetQuery({required this.terms});

  /// The terms to match.
  final List<Term> terms;
}

/// A query that matches a fuzzy term.
class FuzzyTermQuery extends Query {
  const FuzzyTermQuery({
    required this.term,
    required this.distance,
    required this.transpositionCostOne,
    this.prefix = false,
  });

  /// The term to match.
  final Term term;

  /// The distance to match.
  final int distance;

  /// Whether to match the transposition cost one.
  final bool transpositionCostOne;

  /// Whether to match the prefix.
  final bool prefix;
}

/// A query that matches a phrase.
class PhaseQuery extends Query {
  const PhaseQuery({required this.terms, this.slop = 0});

  /// The terms to match.
  final List<Term> terms;

  /// The slop to match.
  final int slop;
}

/// A query that matches a regular expression phrase.
class RegexPhaseQuery extends Query {
  const RegexPhaseQuery({
    required this.field,
    required this.terms,
    this.slop = 0,
    this.maxExpansions = 1 << 14,
  });

  /// The field to match.
  final String field;

  /// The terms to match.
  final List<String> terms;

  /// The slop to match.
  final int slop;

  /// The max expansions to match.
  final int maxExpansions;
}

/// A query that matches all the child queries.
class AndQuery extends Query {
  const AndQuery(this.queries);

  /// The child queries to match.
  final List<Query> queries;
}

/// A query that matches at least one of the child queries.
class OrQuery extends Query {
  const OrQuery(this.queries);

  /// The child queries to match.
  final List<Query> queries;
}

/// A query that negates the child query.
class NotQuery extends Query {
  const NotQuery(this.query);

  /// The child query to negate.
  final Query query;
}

/// A query that boosts the score of the query.
class BoostQuery extends Query {
  const BoostQuery({required this.query, required this.boost});

  /// The child query to boost.
  final Query query;

  /// The boost to apply.
  final Score boost;
}

part of 'native.dart';

extension QueryListToNativeExtension on List<Query> {
  /// Converts a list of queries to a native burkaz query.
  (Pointer<Pointer<CBurkazQuery>>, int) toNative(
    Schema schema, {
    Allocator allocator = malloc,
  }) {
    final queryArrayLength = length;
    final queryArrayPointer = allocator.allocate<Pointer<CBurkazQuery>>(
      queryArrayLength,
    );
    for (int index = 0; index < queryArrayLength; index++) {
      queryArrayPointer[index] = this[index].toNative(
        schema,
        allocator: allocator,
      );
    }
    return (queryArrayPointer, queryArrayLength);
  }
}

extension QueryToNativeExtension on Query {
  /// Converts a query to a native burkaz query.
  Pointer<CBurkazQuery> toNative(
    Schema schema, {
    Allocator allocator = malloc,
  }) {
    switch (this) {
      case AllQuery():
        return burkaz_all_query();
      case EmptyQuery():
        return burkaz_empty_query();
      case AndQuery(:final queries):
        final (queryArrayPointer, queryArrayLength) = queries.toNative(
          schema,
          allocator: allocator,
        );
        return burkaz_and_query(queryArrayPointer, queryArrayLength);
      case OrQuery(:final queries):
        final (queryArrayPointer, queryArrayLength) = queries.toNative(
          schema,
          allocator: allocator,
        );
        return burkaz_or_query(queryArrayPointer, queryArrayLength);
      case NotQuery(:final query):
        return burkaz_not_query(query.toNative(schema, allocator: allocator));
      case BoostQuery(:final query, :final boost):
        return burkaz_boost_query(
          query.toNative(schema, allocator: allocator),
          boost,
        );
      case TermQuery(:final term, :final indexingStrategy):
        return burkaz_term_query(
          term.toNative(schema, allocator: allocator),
          indexingStrategy.code,
        );
      case FuzzyTermQuery(
        :final term,
        :final distance,
        :final transpositionCostOne,
        :final prefix,
      ):
        return burkaz_fuzzy_term_query(
          term.toNative(schema, allocator: allocator),
          distance,
          transpositionCostOne,
          prefix,
        );
      case TermSetQuery(:final terms):
        final (termArrayPointer, termArrayLength) = terms.toNative(
          schema,
          allocator: allocator,
        );
        return burkaz_term_set_query(termArrayPointer, termArrayLength);
      case PhaseQuery(:final terms, :final slop):
        final (termArrayPointer, termArrayLength) = terms.toNative(
          schema,
          allocator: allocator,
        );
        return burkaz_phrase_query(termArrayPointer, termArrayLength, slop);
      case RegexPhaseQuery(
        :final field,
        :final terms,
        :final slop,
        :final maxExpansions,
      ):
        final fieldId = schema.getFieldIndex(field);
        if (fieldId == null) return nullptr;

        final termArrayPointer = allocator.allocate<Pointer<Char>>(
          terms.length,
        );
        for (int index = 0; index < terms.length; index++) {
          termArrayPointer[index] = terms[index]
              .toNativeUtf8(allocator: allocator)
              .cast();
        }

        return burkaz_regex_phrase_query(
          fieldId,
          termArrayPointer,
          terms.length,
          slop,
          maxExpansions,
        );
    }
  }
}

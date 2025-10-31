import 'address.dart';

/// A runner for a query on a burkaz index.
abstract class QueryRunner<T> {
  const QueryRunner();

  /// Counts the number of results for the query.
  int count();

  /// Counts the number of results for the query asynchronously.
  Future<int> countAsync();

  /// Searches for results for the query.
  List<T> search({int offset = 0, int limit = 1});

  /// Searches for results for the query asynchronously.
  Future<List<T>> searchAsync({int offset = 0, int limit = 1});

  /// Searches for addresses for the query.
  List<Address> addresses({int offset = 0, int limit = 1});

  /// Searches for addresses for the query asynchronously.
  Future<List<Address>> addressesAsync({int offset = 0, int limit = 1});

  /// Deletes all results for the query.
  void deleteAll();

  /// Deletes all results for the query asynchronously.
  Future<void> deleteAllAsync();
}

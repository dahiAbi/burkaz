import 'dart:io';

import 'schema.dart';
import 'query.dart';
import 'query_runner.dart';
import 'address.dart';

import 'package:burkaz/src/native/native.dart';

/// A burkaz index.
abstract class BurkazIndex<T> {
  const BurkazIndex();

  /// Opens a burkaz index.
  static BurkazIndex<T> open<T>({
    required String name,
    required Schema<T> schema,
    Directory? directory,
  }) {
    return NativeBurkazIndex.open<T>(
      name: name,
      schema: schema,
      directory: directory,
    );
  }

  /// The name of the index.
  String get name;

  /// The schema of the index.
  Schema<T> get schema;

  /// Gets an object from the index.
  T get(Address address);

  /// Gets an object from the index asynchronously.
  Future<T> getAsync(Address address);

  /// Adds an object to the index.
  void add(T object);

  /// Adds an object to the index asynchronously.
  Future<void> addAsync(T object);

  /// Adds multiple objects to the index.
  void addAll(Iterable<T> objects);

  /// Adds multiple objects to the index asynchronously.
  Future<void> addAllAsync(Iterable<T> objects);

  /// Clears the index.
  void clear();

  /// Clears the index asynchronously.
  Future<void> clearAsync();

  /// Runs a query on the index.
  QueryRunner<T> query(Query query);

  /// Closes the index.
  void close();

  /// Closes the index asynchronously.
  Future<void> closeAsync();
}

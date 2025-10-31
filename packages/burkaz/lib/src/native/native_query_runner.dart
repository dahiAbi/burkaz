part of 'native.dart';

/// A native burkaz query runner.
class NativeQueryRunner<T> extends QueryRunner<T> {
  /// The pointer of the native burkaz query runner.
  final Pointer<CBurkazQueryRunner> _ptr;

  /// The native burkaz index.
  final NativeBurkazIndex<T> _index;

  const NativeQueryRunner._(this._index, this._ptr);

  /// The finalizer of the native burkaz query runner.
  static final _finalizer = Finalizer(burkaz_free_query_runner);

  /// Creates a native burkaz query runner from a pointer.
  factory NativeQueryRunner.fromPointer(
    NativeBurkazIndex<T> index,
    Pointer<CBurkazQueryRunner> ptr,
  ) {
    final runner = NativeQueryRunner<T>._(index, ptr);
    _finalizer.attach(runner, runner._ptr, detach: runner);
    return runner;
  }

  @override
  int count() {
    return using((arena) {
      final countPointer = arena<Size>();
      burkaz_query_runner_count(_ptr, countPointer).checkError();
      return countPointer.value;
    });
  }

  @override
  Future<int> countAsync() => Isolate.run(count);

  @override
  List<T> search({int offset = 0, int limit = 1}) {
    if (limit == 0) return const [];

    assert(offset >= 0, 'Offset must be greater than or equal to 0');
    assert(limit > 0, 'Limit must be greater than 0');

    return using((arena) {
      final resultArrayPointer = arena<Pointer<Uint64>>();
      final resultArrayLengthPointer = arena<Size>();

      burkaz_query_runner_search(
        _ptr,
        offset,
        limit,
        resultArrayPointer,
        resultArrayLengthPointer,
      ).checkError();

      final resultArray = resultArrayPointer.value;
      final resultArrayLength = resultArrayLengthPointer.value;
      if (resultArrayLength == 0) {
        // Nothing to free or iterate
        return const [];
      }

      final List<T> result = [];

      final objectPointerPointer = arena<Pointer<CBurkazObject>>();

      try {
        for (int index = 0; index < resultArrayLength; index++) {
          final rawAddress = resultArray[index];
          burkaz_index_get(
            _index.ptr,
            rawAddress,
            objectPointerPointer,
          ).checkError();
          final object = _index.objectFromNative(objectPointerPointer.value);
          result.add(object);
        }
      } finally {
        burkaz_free_query_runner_search_result(resultArray, resultArrayLength);
      }

      return result;
    });
  }

  @override
  Future<List<T>> searchAsync({int offset = 0, int limit = 1}) =>
      Isolate.run(() => search(offset: offset, limit: limit));

  @override
  List<Address> addresses({int offset = 0, int limit = 1}) {
    if (limit == 0) return const [];

    assert(offset >= 0, 'Offset must be greater than or equal to 0');
    assert(limit > 0, 'Limit must be greater than 0');

    return using((arena) {
      final resultArrayPointer = arena<Pointer<Uint64>>();
      final resultArrayLengthPointer = arena<Size>();

      burkaz_query_runner_search(
        _ptr,
        offset,
        limit,
        resultArrayPointer,
        resultArrayLengthPointer,
      ).checkError();

      final resultArray = resultArrayPointer.value;
      final resultArrayLength = resultArrayLengthPointer.value;
      if (resultArrayLength == 0) {
        // Nothing to free or iterate
        return const [];
      }

      final List<Address> result = [];

      try {
        for (int index = 0; index < resultArrayLength; index++) {
          final rawAddress = resultArray[index];
          final address = Address.raw(rawAddress);
          result.add(address);
        }
      } finally {
        burkaz_free_query_runner_search_result(resultArray, resultArrayLength);
      }

      return result;
    });
  }

  @override
  Future<List<Address>> addressesAsync({int offset = 0, int limit = 1}) =>
      Isolate.run(() => addresses(offset: offset, limit: limit));

  @override
  void deleteAll() {
    burkaz_query_runner_delete_all(_ptr).checkError();
  }

  @override
  Future<void> deleteAllAsync() => Isolate.run(deleteAll);
}

part of 'native.dart';

/// A native burkaz index.
class NativeBurkazIndex<T> extends BurkazIndex<T> {
  Pointer<CBurkazIndex>? _ptr;

  /// The pointer of the native burkaz index.
  Pointer<CBurkazIndex> get ptr {
    return switch (_ptr) {
      final ptr? => ptr,
      null => throw StateError('Burkaz index is closed'),
    };
  }

  NativeBurkazIndex._(this._ptr, this._schema);

  /// Opens a native burkaz index.
  static NativeBurkazIndex<T> open<T>({
    required String name,
    required Schema<T> schema,
    Directory? directory,
  }) {
    return using((arena) {
      final namePointer = name.toNativeUtf8(allocator: arena);
      final nativeSchemaPointer = schema.toNative(allocator: arena);
      final indexPointerPointer = arena<Pointer<CBurkazIndex>>();

      final directoryPath = directory?.path;
      final directoryPathPointer = directoryPath?.toNativeUtf8(
        allocator: arena,
      );

      burkaz_index_open(
        namePointer.cast(),
        namePointer.length,
        directoryPathPointer?.cast() ?? nullptr,
        directoryPathPointer?.length ?? 0,
        nativeSchemaPointer,
        indexPointerPointer,
      ).checkError();

      return NativeBurkazIndex<T>._(indexPointerPointer.value, schema);
    });
  }

  @override
  Schema<T> get schema => _schema;
  final Schema<T> _schema;

  @override
  String get name => _name;
  late final String _name = _getIndexName();

  /// Gets the name of the index.
  String _getIndexName() {
    final namePointer = burkaz_index_name(ptr);
    if (namePointer.address == 0) return '';
    final name = namePointer.cast<Utf8>().toDartString();
    burkaz_free_string(namePointer);
    return name;
  }

  @override
  void add(T object) {
    final nativeObject = objectToNative(object);
    burkaz_index_add(ptr, nativeObject).checkError();
  }

  @override
  void addAll(Iterable<T> objects) {
    final objectList = objects is List<T>
        ? objects
        : objects.toList(growable: false);

    return using((arena) {
      final objectArrayPointer = arena<Pointer<CBurkazObject>>(
        objectList.length,
      );
      for (int index = 0; index < objectList.length; index++) {
        objectArrayPointer[index] = objectToNative(objectList[index]);
      }

      burkaz_index_add_all(
        ptr,
        objectArrayPointer,
        objectList.length,
      ).checkError();
    });
  }

  @override
  void clear() {
    burkaz_index_clear(ptr).checkError();
  }

  @override
  QueryRunner<T> query(Query query) {
    return using((arena) {
      final queryRunnerPointerPointer = arena<Pointer<CBurkazQueryRunner>>();
      burkaz_query_runner_new(
        ptr,
        query.toNative(schema, allocator: arena),
        queryRunnerPointerPointer,
      ).checkError();
      return NativeQueryRunner<T>.fromPointer(
        this,
        queryRunnerPointerPointer.value,
      );
    });
  }

  @override
  T get(Address address) {
    return using((arena) {
      final objectPointerPointer = arena<Pointer<CBurkazObject>>();
      burkaz_index_get(ptr, address.value, objectPointerPointer).checkError();
      final object = objectFromNative(objectPointerPointer.value);
      burkaz_free_object(objectPointerPointer.value);
      return object;
    });
  }

  @override
  void close() {
    final ptr = _ptr;
    if (ptr != null) {
      _ptr = null;
      burkaz_index_close(ptr);
    }
  }

  /// Converts an object to a native burkaz object.
  @protected
  Pointer<CBurkazObject> objectToNative(T object) {
    final writer = NativeObject();
    _schema.serialize(writer, object);
    return writer._ptr;
  }

  /// Converts a native burkaz object to an object.
  @protected
  T objectFromNative(Pointer<CBurkazObject> objectPointer) {
    final reader = NativeObject.fromPointer(objectPointer);
    return _schema.deserialize(reader);
  }
}

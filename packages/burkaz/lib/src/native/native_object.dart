part of 'native.dart';

/// A native burkaz object.
class NativeObject implements ObjectWriter, ObjectReader {
  const NativeObject.fromPointer(this._ptr);

  /// Creates a native burkaz object.
  factory NativeObject() {
    return NativeObject.fromPointer(burkaz_object_create());
  }

  /// The pointer of the native burkaz object.
  final Pointer<CBurkazObject> _ptr;

  @override
  int? readInt(int index) {
    return using((arena) {
      final valuePointer = arena<Int64>();
      final exists = burkaz_object_read_int(_ptr, index, valuePointer);
      if (!exists) return null;
      return valuePointer.value;
    });
  }

  @override
  bool? readBoolean(int index) {
    return using((arena) {
      final valuePointer = arena<Bool>();
      final exists = burkaz_object_read_boolean(_ptr, index, valuePointer);
      if (!exists) return null;
      return valuePointer.value;
    });
  }

  @override
  String? readString(int index) {
    return using((arena) {
      final valuePointer = arena<Pointer<Char>>();
      final exists = burkaz_object_read_text(_ptr, index, valuePointer);
      if (!exists) return null;
      final ptr = valuePointer.value.cast<Utf8>();
      final dartStr = ptr.toDartString();
      // Free the C string allocated by Rust after conversion
      burkaz_free_string(ptr.cast());
      return dartStr;
    });
  }

  @override
  void writeInt(int index, int value) {
    burkaz_object_write_int(_ptr, index, value);
  }

  @override
  void writeBoolean(int index, bool value) {
    burkaz_object_write_boolean(_ptr, index, value);
  }

  @override
  void writeString(int index, String value) {
    return using((arena) {
      burkaz_object_write_text(
        _ptr,
        index,
        value.toNativeUtf8(allocator: arena).cast(),
      );
    });
  }
}

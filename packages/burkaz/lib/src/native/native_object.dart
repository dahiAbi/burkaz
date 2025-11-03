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
  List<int>? readIntList(int index) {
    return using((arena) {
      final resultArrayPointer = arena<Pointer<Int64>>();
      final resultArrayLengthPointer = arena<Size>();
      final exists = burkaz_object_read_int_list(
        _ptr,
        index,
        resultArrayPointer,
        resultArrayLengthPointer,
      );
      if (!exists) return null;
      final resultArray = resultArrayPointer.value;
      final resultArrayLength = resultArrayLengthPointer.value;
      if (resultArrayLength == 0) return null;
      final List<int> result = [];
      for (int index = 0; index < resultArrayLength; index++) {
        result.add(resultArray[index]);
      }
      return result;
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
  List<bool>? readBooleanList(int index) {
    return using((arena) {
      final resultArrayPointer = arena<Pointer<Bool>>();
      final resultArrayLengthPointer = arena<Size>();
      final exists = burkaz_object_read_boolean_list(
        _ptr,
        index,
        resultArrayPointer,
        resultArrayLengthPointer,
      );
      if (!exists) return null;
      final resultArray = resultArrayPointer.value;
      final resultArrayLength = resultArrayLengthPointer.value;
      if (resultArrayLength == 0) return null;
      final List<bool> result = [];
      for (int index = 0; index < resultArrayLength; index++) {
        result.add(resultArray[index]);
      }
      return result;
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
  List<String>? readStringList(int index) {
    return using((arena) {
      final resultArrayPointer = arena<Pointer<Pointer<Char>>>();
      final resultArrayLengthPointer = arena<Size>();
      final exists = burkaz_object_read_text_list(
        _ptr,
        index,
        resultArrayPointer,
        resultArrayLengthPointer,
      );
      if (!exists) return null;
      final resultArray = resultArrayPointer.value;
      final resultArrayLength = resultArrayLengthPointer.value;
      if (resultArrayLength == 0) return null;
      final List<String> result = [];
      for (int index = 0; index < resultArrayLength; index++) {
        final valuePointer = resultArray[index];
        final value = valuePointer.cast<Utf8>().toDartString();
        burkaz_free_string(valuePointer);
        result.add(value);
      }
      return result;
    });
  }

  @override
  void writeInt(int index, int value) {
    burkaz_object_write_int(_ptr, index, value);
  }

  @override
  void writeIntList(int index, List<int> values) {
    return using((arena) {
      final valueArrayLength = values.length;
      final valueArrayPointer = arena<Int64>(valueArrayLength);
      for (int index = 0; index < values.length; index++) {
        valueArrayPointer[index] = values[index];
      }
      burkaz_object_write_int_list(
        _ptr,
        index,
        valueArrayPointer,
        valueArrayLength,
      );
    });
  }

  @override
  void writeBoolean(int index, bool value) {
    burkaz_object_write_boolean(_ptr, index, value);
  }

  @override
  void writeBooleanList(int index, List<bool> values) {
    return using((arena) {
      final valueArrayLength = values.length;
      final valueArrayPointer = arena<Bool>(valueArrayLength);
      for (int index = 0; index < values.length; index++) {
        valueArrayPointer[index] = values[index];
      }
      burkaz_object_write_boolean_list(
        _ptr,
        index,
        valueArrayPointer,
        valueArrayLength,
      );
    });
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

  @override
  void writeStringList(int index, List<String> values) {
    return using((arena) {
      final valueArrayLength = values.length;
      final valueArrayPointer = arena<Pointer<Char>>(valueArrayLength);
      for (int index = 0; index < values.length; index++) {
        valueArrayPointer[index] = values[index]
            .toNativeUtf8(allocator: arena)
            .cast();
      }
      burkaz_object_write_text_list(
        _ptr,
        index,
        valueArrayPointer,
        valueArrayLength,
      );
    });
  }
}

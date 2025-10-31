part of 'native.dart';

extension CheckNativeResultCodeExtension on int {
  /// Checks if the result code is an error.
  void checkError() {
    if (this != 0) {
      throw burkazGetLastError() ?? 'Unknown error';
    }
  }
}

/// Gets the last error from the native burkaz library.
String? burkazGetLastError() {
  final messagePointer = burkaz_get_last_error();
  if (messagePointer.address == 0) return null;
  final message = messagePointer.cast<Utf8>().toDartString();
  burkaz_free_error_string(messagePointer);
  return message;
}

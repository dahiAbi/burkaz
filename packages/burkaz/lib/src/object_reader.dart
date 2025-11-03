/// A reader for a burkaz object.
abstract class ObjectReader {
  const ObjectReader();

  /// Reads an integer from the object.
  int? readInt(int index);

  /// Reads an integer list from the object.
  List<int>? readIntList(int index);

  /// Reads a string from the object.
  String? readString(int index);

  /// Reads a string list from the object.
  List<String>? readStringList(int index);

  /// Reads a boolean from the object.
  bool? readBoolean(int index);

  /// Reads a boolean list from the object.
  List<bool>? readBooleanList(int index);
}

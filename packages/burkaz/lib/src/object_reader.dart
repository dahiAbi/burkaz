/// A reader for a burkaz object.
abstract class ObjectReader {
  const ObjectReader();

  /// Reads an integer from the object.
  int? readInt(int index);

  /// Reads a string from the object.
  String? readString(int index);
}

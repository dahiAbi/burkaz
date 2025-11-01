/// A writer for a burkaz object.
abstract class ObjectWriter {
  const ObjectWriter();

  /// Writes an integer to the object.
  void writeInt(int index, int value);

  /// Writes a string to the object.
  void writeString(int index, String value);

  /// Writes a boolean to the object.
  void writeBoolean(int index, bool value);
}

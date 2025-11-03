/// A writer for a burkaz object.
abstract class ObjectWriter {
  const ObjectWriter();

  /// Writes an integer to the object.
  void writeInt(int index, int value);

  /// Writes an integer list to the object.
  void writeIntList(int index, List<int> values);

  /// Writes a string to the object.
  void writeString(int index, String value);

  /// Writes a string list to the object.
  void writeStringList(int index, List<String> values);

  /// Writes a boolean to the object.
  void writeBoolean(int index, bool value);

  /// Writes a boolean list to the object.
  void writeBooleanList(int index, List<bool> values);
}

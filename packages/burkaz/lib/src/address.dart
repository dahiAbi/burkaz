/// A address of the object in the burkaz index.
extension type const Address.raw(int value) {
  const Address(int segmentOrdinal, int documentId)
    : this.raw((segmentOrdinal << 32) | documentId);

  /// The segment ordinal.
  int get segmentOrdinal => value >> 32;

  /// The document id.
  int get documentId => value & 0xFFFFFFFF;
}

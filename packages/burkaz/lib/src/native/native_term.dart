part of 'native.dart';

extension TermToNativeExtension on Term {
  /// Converts a term to a native burkaz term.
  Pointer<CBurkazTerm> toNative(Schema schema, {Allocator allocator = malloc}) {
    final fieldId = schema.getFieldIndex(field);
    if (fieldId == null) return nullptr;
    switch ((valueType, value)) {
      case (ValueType.int, final int value):
        return burkaz_term_int(fieldId, value);
      case (ValueType.text, final String value):
        final valuePointer = value.toNativeUtf8(allocator: allocator);
        return burkaz_term_text(fieldId, valuePointer.cast(), value.length);
      default:
        throw UnsupportedError('Unsupported term: $this');
    }
  }
}

extension TermListToNativeExtension on List<Term> {
  /// Converts a list of terms to a native burkaz term.
  (Pointer<Pointer<CBurkazTerm>>, int) toNative(
    Schema schema, {
    Allocator allocator = malloc,
  }) {
    final termArrayLength = length;
    final termArrayPointer = allocator.allocate<Pointer<CBurkazTerm>>(
      termArrayLength,
    );
    for (int index = 0; index < termArrayLength; index++) {
      termArrayPointer[index] = this[index].toNative(
        schema,
        allocator: allocator,
      );
    }
    return (termArrayPointer, termArrayLength);
  }
}

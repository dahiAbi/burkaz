part of 'native.dart';

extension SchemaToNativeExtension on Schema {
  /// Converts a schema to a native burkaz schema.
  Pointer<CBurkazSchema> toNative({Allocator allocator = malloc}) {
    final fields = switch (this.fields) {
      final List<Field> listFields => listFields,
      final iterableFields => iterableFields.toList(growable: false),
    };

    final fieldArrayLength = fields.length;
    final fieldArrayPointer = allocator.allocate<Pointer<CBurkazSchemaField>>(
      fieldArrayLength,
    );

    for (int index = 0; index < fieldArrayLength; index++) {
      fieldArrayPointer[index] = fields[index].toNative(allocator: allocator);
    }

    return burkaz_schema_new(fieldArrayPointer, fields.length);
  }
}

extension FieldToNativeExtension on Field {
  /// Converts a field to a native burkaz field.
  Pointer<CBurkazSchemaField> toNative({Allocator allocator = malloc}) {
    final fieldPointer = allocator<CBurkazSchemaField>();
    final namePointer = name.toNativeUtf8(allocator: allocator);
    fieldPointer.ref.namePointer = namePointer.cast();
    fieldPointer.ref.nameLength = name.length;
    fieldPointer.ref.optionsPointer = options.toNative(allocator: allocator);
    return fieldPointer;
  }
}

extension FieldOptionsToNativeExtension on FieldOptions {
  /// Converts a field options to a native burkaz field options.
  Pointer<CBurkazSchemaFieldOptions> toNative({Allocator allocator = malloc}) {
    final optionsPointer = allocator<CBurkazSchemaFieldOptions>();
    optionsPointer.ref.type = type.index + 1;
    if (isStored) optionsPointer.ref.stored = 1;
    if (shouldCoerce) optionsPointer.ref.coerce = 1;
    if (isIndexed) optionsPointer.ref.indexed = 1;
    if (fieldnorms) optionsPointer.ref.fieldnorms = 1;
    if (isFast) optionsPointer.ref.fast = 1;
    if (indexingStrategy case final strategy?) {
      optionsPointer.ref.indexingStrategy = strategy.code;
    }
    if (indexingTokenizer case final tokenizer?) {
      final tokenizerPointer = tokenizer.toNativeUtf8(allocator: allocator);
      optionsPointer.ref.indexingTokenizerPointer = tokenizerPointer.cast();
      optionsPointer.ref.indexingTokenizerLength = tokenizer.length;
    }
    if (fastTokenizer case final fastTokenizer?) {
      final fastTokenizerPointer = fastTokenizer.toNativeUtf8(
        allocator: allocator,
      );
      optionsPointer.ref.fastTokenizerPointer = fastTokenizerPointer.cast();
      optionsPointer.ref.fastTokenizerLength = fastTokenizer.length;
    }
    return optionsPointer;
  }
}

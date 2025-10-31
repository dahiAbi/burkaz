import 'schema_field_options.dart';

import 'object_reader.dart';
import 'object_writer.dart';

/// A schema for a burkaz index.
abstract class Schema<T> {
  const Schema();

  /// The fields of the schema.
  Iterable<Field> get fields;

  /// Get the index of a field in the schema.
  int? getFieldIndex(String field) {
    return fields.indexed
        .where((meta) => meta.$2.name == field)
        .firstOrNull
        ?.$1;
  }

  /// Serializes an object to a burkaz object.
  void serialize(ObjectWriter writer, T object);

  /// Deserializes a burkaz object to an object.
  T deserialize(ObjectReader reader);
}

/// A strict schema for a burkaz index.
class StrictSchema<T> extends Schema<T> {
  const StrictSchema({
    required this.fields,
    required void Function(ObjectWriter writer, T object) serialize,
    required T Function(ObjectReader reader) deserialize,
  }) : _serialize = serialize,
       _deserialize = deserialize;

  final void Function(ObjectWriter writer, T object) _serialize;
  final T Function(ObjectReader reader) _deserialize;

  @override
  final List<Field> fields;

  @override
  void serialize(ObjectWriter writer, T object) {
    _serialize(writer, object);
  }

  @override
  T deserialize(ObjectReader reader) => _deserialize(reader);
}

/// A field in a schema.
class Field {
  /// The name of the field.
  final String name;

  /// The options of the field.
  final FieldOptions options;

  const Field({required this.name, required this.options});

  @override
  String toString() {
    return 'Field(name: $name, options: $options)';
  }
}

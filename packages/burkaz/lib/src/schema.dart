import 'schema_field_options.dart';

/// A schema for a burkaz index.
abstract class Schema {
  const Schema();

  /// The fields of the schema.
  Iterable<Field> get fields;
}

/// A strict schema for a burkaz index.
class StrictSchema extends Schema {
  const StrictSchema({required this.fields});

  @override
  final List<Field> fields;
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

import 'schema_field_options.dart';

/// A raw value of a term.
typedef RawValue = Object;

/// A term on a burkaz index.
class Term {
  const Term({
    required this.field,
    required this.valueType,
    required this.value,
  });

  /// The field of the term.
  final String field;

  /// The type of the value.
  final ValueType valueType;

  /// The value.
  final RawValue value;

  /// Creates a term with an integer value.
  const Term.int(String field, int value)
    : this(field: field, valueType: ValueType.int, value: value);

  /// Creates a term with a text value.
  const Term.text(String field, String value)
    : this(field: field, valueType: ValueType.text, value: value);
}

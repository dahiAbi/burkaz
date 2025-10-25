/// The options for a field in a schema.
abstract class FieldOptions {
  const FieldOptions();

  /// The type of the field.
  FieldType get type;

  /// Whether the field is stored.
  bool get isStored => false;

  /// Whether the field should be coerced.
  bool get shouldCoerce => false;

  /// Whether the field is fast.
  bool get isFast => false;

  /// The tokenizer for the fast field.
  String? get fastTokenizer => null;

  /// Whether the field is indexed.
  bool get isIndexed => false;

  /// The indexing strategy for the field.
  IndexingStrategy? get indexingStrategy => null;

  /// The tokenizer for the indexed field.
  String? get indexingTokenizer => null;

  /// Whether the field has field norms.
  bool get fieldnorms => isIndexed;
}

/// The options for a numeric field in a schema.
abstract class NumericFieldOptions extends FieldOptions {
  const NumericFieldOptions({
    required this.type,
    bool indexed = false,
    bool fieldnorms = false,
    bool fast = false,
    bool stored = false,
    bool coerce = false,
  }) : assert(indexed ? true : !fieldnorms),
       _indexed = indexed,
       _fieldnorms = fieldnorms,
       _fast = fast,
       _stored = stored,
       _coerce = coerce;

  @override
  final FieldType type;

  final bool _indexed;
  @override
  bool get isIndexed => _indexed;

  final bool _fieldnorms;
  @override
  bool get fieldnorms => _fieldnorms;

  final bool _fast;
  @override
  bool get isFast => _fast;

  final bool _stored;
  @override
  bool get isStored => _stored;

  final bool _coerce;
  @override
  bool get shouldCoerce => _coerce;
}

/// The options for a 64-bit integer field in a schema.
class Int64FieldOptions extends NumericFieldOptions {
  const Int64FieldOptions({
    super.indexed,
    super.fieldnorms,
    super.fast,
    super.stored,
    super.coerce,
  }) : super(type: FieldType.int64);
}

/// The options for a text field in a schema.
class TextFieldOptions extends FieldOptions {
  const TextFieldOptions({
    IndexedTextFieldOptions indexed = const IndexedTextFieldOptions.disabled(),
    FastTextFieldOptions fast = const FastTextFieldOptions.disabled(),
    bool stored = false,
    bool coerce = false,
  }) : _indexed = indexed,
       _stored = stored,
       _fast = fast,
       _coerce = coerce;

  @override
  final FieldType type = FieldType.text;

  final IndexedTextFieldOptions? _indexed;
  @override
  bool get isIndexed => _indexed != null;

  @override
  String? get indexingTokenizer => _indexed?.tokenizer;

  final bool _stored;
  @override
  bool get isStored => _stored;

  final FastTextFieldOptions _fast;
  @override
  bool get isFast => _fast.enabled;
  @override
  String? get fastTokenizer => _fast.tokenizer;

  final bool _coerce;
  @override
  bool get shouldCoerce => _coerce;
}

/// The options for a fast text field in a schema.
class FastTextFieldOptions {
  /// Whether the fast text field is enabled.
  final bool enabled;

  /// The tokenizer for the fast text field.
  final String? tokenizer;

  const FastTextFieldOptions({required this.enabled, required this.tokenizer});

  /// The constructor for a enabled fast text field.
  const FastTextFieldOptions.enabled([String? tokenizer])
    : this(enabled: true, tokenizer: tokenizer);

  /// The constructor for a disabled fast text field.
  const FastTextFieldOptions.disabled() : this(enabled: false, tokenizer: null);
}

/// The options for a indexed text field in a schema.
class IndexedTextFieldOptions {
  const IndexedTextFieldOptions({
    required this.enabled,
    required this.strategy,
    required this.fieldnorms,
    required this.tokenizer,
  });

  /// The constructor for a enabled indexed text field.
  const IndexedTextFieldOptions.enabled({
    IndexingStrategy strategy = IndexingStrategy.basic,
    bool fieldnorms = true,
    String? tokenizer,
  }) : this(
         enabled: true,
         strategy: strategy,
         fieldnorms: fieldnorms,
         tokenizer: tokenizer,
       );

  /// The constructor for a disabled indexed text field.
  const IndexedTextFieldOptions.disabled()
    : this(enabled: false, strategy: null, fieldnorms: null, tokenizer: null);

  /// Whether the indexed text field is enabled.
  final bool enabled;

  /// The indexing strategy for the indexed text field.
  final IndexingStrategy? strategy;

  /// Whether the indexed text field has field norms.
  final bool? fieldnorms;

  /// The tokenizer for the indexed text field.
  final String? tokenizer;
}

/// The indexing strategy for a indexed text field.
enum IndexingStrategy { basic, frequencies, frequenciesAndPositions }

/// The type of a field.
enum FieldType { int64, text }

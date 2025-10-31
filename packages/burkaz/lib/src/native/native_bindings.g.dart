// ignore_for_file: non_constant_identifier_names

part of 'native.dart';

typedef ResultCode = Uint8;

final class CBurkazIndex extends Opaque {}

final class CBurkazObject extends Opaque {}

final class CBurkazQueryRunner extends Opaque {}

final class CBurkazSchema extends Struct {
  external Pointer<Pointer<CBurkazSchemaField>> fieldArrayPointer;

  @Size()
  external int fieldArrayLength;
}

final class CBurkazSchemaField extends Struct {
  external Pointer<Char> namePointer;

  @Size()
  external int nameLength;

  external Pointer<CBurkazSchemaFieldOptions> optionsPointer;
}

final class CBurkazSchemaFieldOptions extends Struct {
  @Uint8()
  external int type;

  @Uint8()
  external int stored;

  @Uint8()
  external int coerce;

  @Uint8()
  external int indexed;

  @Uint8()
  external int fieldnorms;

  @Uint8()
  external int fast;

  @Uint8()
  external int indexingStrategy;

  external Pointer<Char> fastTokenizerPointer;

  @Size()
  external int fastTokenizerLength;

  external Pointer<Char> indexingTokenizerPointer;

  @Size()
  external int indexingTokenizerLength;
}

@Native<
  ResultCode Function(
    Pointer<Char>,
    Size,
    Pointer<Char>,
    Size,
    Pointer<CBurkazSchema>,
    Pointer<Pointer<CBurkazIndex>>,
  )
>()
external int burkaz_index_open(
  Pointer<Char> namePointer,
  int nameLength,
  Pointer<Char> directoryPathPointer,
  int directoryPathLength,
  Pointer<CBurkazSchema> schemaPointer,
  Pointer<Pointer<CBurkazIndex>> indexPointerPointer,
);

@Native<Void Function(Pointer<CBurkazIndex>)>()
external void burkaz_index_close(Pointer<CBurkazIndex> indexPointer);

@Native<Pointer<Char> Function(Pointer<CBurkazIndex>)>()
external Pointer<Char> burkaz_index_name(Pointer<CBurkazIndex> indexPointer);

@Native<Void Function(Pointer<Char>)>()
external void burkaz_free_string(Pointer<Char> stringPointer);

@Native()
external Pointer<Char> burkaz_get_last_error();

@Native()
external void burkaz_free_error_string(Pointer<Char> errorPointer);

// @Native<Pointer<CBurkazObject> Function(Pointer<Uint8>, Size)>(isLeaf: true)
// external Pointer<CBurkazObject> burkaz_object_new(
//   Pointer<Uint8> dataArrayPointer,
//   int dataArrayLength,
// );

@Native<Pointer<CBurkazObject> Function()>()
external Pointer<CBurkazObject> burkaz_object_create();

@Native<Bool Function(Pointer<CBurkazObject>, Uint32, Pointer<Int64>)>(
  isLeaf: true,
)
external bool burkaz_object_read_int(
  Pointer<CBurkazObject> objectPointer,
  int fieldId,
  Pointer<Int64> valuePointer,
);

@Native<Void Function(Pointer<CBurkazObject>, Uint32, Int64)>(isLeaf: true)
external void burkaz_object_write_int(
  Pointer<CBurkazObject> objectPointer,
  int fieldId,
  int valuePointer,
);

@Native<Bool Function(Pointer<CBurkazObject>, Uint32, Pointer<Pointer<Char>>)>(
  isLeaf: true,
)
external bool burkaz_object_read_text(
  Pointer<CBurkazObject> objectPointer,
  int fieldId,
  Pointer<Pointer<Char>> valuePointerPointer,
);

@Native<Void Function(Pointer<CBurkazObject>, Uint32, Pointer<Char>)>(
  isLeaf: true,
)
external void burkaz_object_write_text(
  Pointer<CBurkazObject> objectPointer,
  int fieldId,
  Pointer<Char> valuePointer,
);

// @Native<
//   Void Function(Pointer<CBurkazObject>, Pointer<Pointer<Uint8>>, Pointer<Size>)
// >()
// external void burkaz_object_bytes(
//   Pointer<CBurkazObject> objectPointer,
//   Pointer<Pointer<Uint8>> dataPointerPointer,
//   Pointer<Size> dataLengthPointer,
// );

@Native<Void Function(Pointer<CBurkazObject>)>()
external void burkaz_free_object(Pointer<CBurkazObject> objectPointer);

@Native<
  ResultCode Function(
    Pointer<CBurkazIndex>,
    Pointer<CBurkazQuery>,
    Pointer<Pointer<CBurkazQueryRunner>>,
  )
>()
external int burkaz_query_runner_new(
  Pointer<CBurkazIndex> indexPointer,
  Pointer<CBurkazQuery> queryPointer,
  Pointer<Pointer<CBurkazQueryRunner>> queryRunnerPointerPointer,
);

@Native<Void Function(Pointer<CBurkazQueryRunner>)>()
external void burkaz_free_query_runner(
  Pointer<CBurkazQueryRunner> queryRunnerPointer,
);

@Native<ResultCode Function(Pointer<CBurkazQueryRunner>, Pointer<Size>)>()
external int burkaz_query_runner_count(
  Pointer<CBurkazQueryRunner> queryRunnerPointer,
  Pointer<Size> resultPointer,
);

@Native<
  ResultCode Function(
    Pointer<CBurkazQueryRunner>,
    Size,
    Size,
    Pointer<Pointer<Uint64>>,
    Pointer<Size>,
  )
>()
external int burkaz_query_runner_search(
  Pointer<CBurkazQueryRunner> queryRunnerPointer,
  int offset,
  int limit,
  Pointer<Pointer<Uint64>> resultArrayPointer,
  Pointer<Size> resultArrayLengthPointer,
);

@Native<Void Function(Pointer<Uint64>, Size)>()
external void burkaz_free_query_runner_search_result(
  Pointer<Uint64> resultArrayPointer,
  int resultArrayLength,
);

@Native<ResultCode Function(Pointer<CBurkazQueryRunner>)>()
external int burkaz_query_runner_delete_all(
  Pointer<CBurkazQueryRunner> queryRunnerPointer,
);

@Native<ResultCode Function(Pointer<CBurkazIndex>, Pointer<CBurkazObject>)>()
external int burkaz_index_add(
  Pointer<CBurkazIndex> indexPointer,
  Pointer<CBurkazObject> objectPointer,
);

@Native<
  ResultCode Function(
    Pointer<CBurkazIndex>,
    Pointer<Pointer<CBurkazObject>>,
    Size,
  )
>()
external int burkaz_index_add_all(
  Pointer<CBurkazIndex> indexPointer,
  Pointer<Pointer<CBurkazObject>> objectArrayPointer,
  int objectArrayLength,
);

@Native<
  ResultCode Function(
    Pointer<CBurkazIndex>,
    Uint64,
    Pointer<Pointer<CBurkazObject>>,
  )
>()
external int burkaz_index_get(
  Pointer<CBurkazIndex> indexPointer,
  int address,
  Pointer<Pointer<CBurkazObject>> objectPointerPointer,
);

@Native<ResultCode Function(Pointer<CBurkazIndex>)>()
external int burkaz_index_clear(Pointer<CBurkazIndex> indexPointer);

@Native<
  Pointer<CBurkazSchema> Function(Pointer<Pointer<CBurkazSchemaField>>, Size)
>()
external Pointer<CBurkazSchema> burkaz_schema_new(
  Pointer<Pointer<CBurkazSchemaField>> fieldArrayPointer,
  int fieldArrayLength,
);

final class CBurkazQuery extends Opaque {}

@Native<Pointer<CBurkazQuery> Function()>()
external Pointer<CBurkazQuery> burkaz_all_query();

@Native<Pointer<CBurkazQuery> Function()>()
external Pointer<CBurkazQuery> burkaz_empty_query();

@Native<Pointer<CBurkazQuery> Function(Pointer<Pointer<CBurkazQuery>>, Size)>()
external Pointer<CBurkazQuery> burkaz_and_query(
  Pointer<Pointer<CBurkazQuery>> queryArrayPointer,
  int queryArrayLength,
);

@Native<Pointer<CBurkazQuery> Function(Pointer<Pointer<CBurkazQuery>>, Size)>()
external Pointer<CBurkazQuery> burkaz_or_query(
  Pointer<Pointer<CBurkazQuery>> queryArrayPointer,
  int queryArrayLength,
);

@Native<Pointer<CBurkazQuery> Function(Pointer<CBurkazQuery>)>()
external Pointer<CBurkazQuery> burkaz_not_query(
  Pointer<CBurkazQuery> queryPointer,
);

@Native<Pointer<CBurkazQuery> Function(Pointer<CBurkazQuery>, Float)>()
external Pointer<CBurkazQuery> burkaz_boost_query(
  Pointer<CBurkazQuery> queryPointer,
  double boost,
);

@Native<Pointer<CBurkazQuery> Function(Pointer<CBurkazTerm>, Uint8)>()
external Pointer<CBurkazQuery> burkaz_term_query(
  Pointer<CBurkazTerm> termPointer,
  int indexingStrategy,
);

@Native<
  Pointer<CBurkazQuery> Function(Pointer<CBurkazTerm>, Uint8, Bool, Bool)
>()
external Pointer<CBurkazQuery> burkaz_fuzzy_term_query(
  Pointer<CBurkazTerm> termPointer,
  int distance,
  bool transpositionCostOne,
  bool prefix,
);

@Native<Pointer<CBurkazQuery> Function(Pointer<Pointer<CBurkazTerm>>, Size)>()
external Pointer<CBurkazQuery> burkaz_term_set_query(
  Pointer<Pointer<CBurkazTerm>> termArrayPointer,
  int termArrayLength,
);

@Native<Pointer<CBurkazQuery> Function(Pointer<Char>, Size)>()
external Pointer<CBurkazQuery> burkaz_parse_query(
  Pointer<Char> queryTextPointer,
  int queryTextLength,
);

@Native<
  Pointer<CBurkazQuery> Function(Pointer<Pointer<CBurkazTerm>>, Size, Uint32)
>()
external Pointer<CBurkazQuery> burkaz_phrase_query(
  Pointer<Pointer<CBurkazTerm>> termArrayPointer,
  int termArrayLength,
  int slop,
);

@Native<
  Pointer<CBurkazQuery> Function(
    Uint32,
    Pointer<Pointer<Char>>,
    Size,
    Uint32,
    Uint32,
  )
>()
external Pointer<CBurkazQuery> burkaz_regex_phrase_query(
  int fieldId,
  Pointer<Pointer<Char>> termArrayPointer,
  int termArrayLength,
  int slop,
  int maxExpansions,
);

final class CBurkazTerm extends Opaque {}

@Native<Pointer<CBurkazTerm> Function(Uint32, Pointer<Char>, Size)>()
external Pointer<CBurkazTerm> burkaz_term_text(
  int fieldId,
  Pointer<Char> valuePointer,
  int valueLength,
);

@Native<Pointer<CBurkazTerm> Function(Uint32, Int64)>()
external Pointer<CBurkazTerm> burkaz_term_int(int fieldId, int value);

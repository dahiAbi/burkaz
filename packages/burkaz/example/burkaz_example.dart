import 'dart:io';

import 'package:burkaz/burkaz.dart';

class Category {
  const Category({required this.id, required this.name, this.description = ''});

  final int id;

  final String name;

  final String description;

  @override
  String toString() {
    return 'Category(id: $id, name: $name, description: $description)';
  }
}

T performance<T>(String name, T Function() task) {
  final stopwatch = Stopwatch();
  stopwatch.start();
  final result = task();
  stopwatch.stop();
  print('$name: ${stopwatch.elapsed.inMicroseconds} μs');
  return result;
}

void main() {
  final schema = StrictSchema<Category>(
    fields: const [
      Field(
        name: 'id',
        options: IntFieldOptions(
          stored: true,
          indexed: true,
          fast: true,
          fieldnorms: true,
        ),
      ),
      Field(
        name: 'name',
        options: TextFieldOptions(
          stored: true,
          fast: FastTextFieldOptions.enabled('default'),
          indexed: IndexedTextFieldOptions.enabled(
            strategy: IndexingStrategy.frequenciesAndPositions,
            fieldnorms: true,
            tokenizer: 'default',
          ),
        ),
      ),
      Field(name: 'description', options: TextFieldOptions(stored: true)),
    ],
    serialize: (writer, object) {
      writer.writeInt(0, object.id);
      writer.writeString(1, object.name);
      writer.writeString(2, object.description);
    },
    deserialize: (reader) {
      return Category(
        id: reader.readInt(0) ?? 0,
        name: reader.readString(1) ?? '?',
        description: reader.readString(2) ?? '?',
      );
    },
  );

  final directory = Directory.current;
  print('directory: ${directory.path}');

  final BurkazIndex<Category> index = BurkazIndex.open<Category>(
    name: 'test_index',
    schema: schema,
  );

  print('index name: ${index.name}');

  performance('addAll', () {
    index.addAll(const [
      Category(id: 1, name: 'Ayakkabi'),
      Category(id: 2, name: 'Spor Ayakkabi'),
      Category(id: 3, name: 'Hali saha Spor Ayakkabi'),
      Category(id: 4, name: 'Futbol Ayakkabi'),
      Category(id: 5, name: 'Terlik Ayakkabi'),
      Category(id: 6, name: 'Giyim'),
      Category(id: 7, name: 'Elektronik'),
      Category(id: 8, name: 'Kitap'),
      Category(id: 9, name: 'Müzik'),
      Category(id: 10, name: 'Oyuncak'),
      Category(id: 11, name: 'Oyun'),
      Category(id: 12, name: 'Kozmetik'),
      Category(id: 13, name: 'Spor'),
      Category(id: 14, name: 'Bebek'),
      Category(id: 15, name: 'Icecek'),
      Category(id: 16, name: 'Yemek'),
      Category(id: 17, name: 'Kisisel Bakim'),
      Category(id: 18, name: 'Hayvan Bakim'),
      Category(id: 19, name: 'Gazli icecek'),
      Category(id: 20, name: 'Alkollu icecek'),
      Category(id: 21, name: 'Temizlik'),
      Category(id: 22, name: 'Iphone'),
      Category(id: 23, name: 'Iphone 17'),
      Category(id: 24, name: 'Iphone 17 Air'),
      Category(id: 25, name: 'Iphone 17 Pro'),
      Category(id: 26, name: 'Iphone 17 Pro Max'),
      Category(id: 27, name: 'Iphone 16 Pro Max'),
      Category(id: 28, name: 'Iphone 16 Pro'),
      Category(id: 29, name: 'Iphone 16'),
      Category(id: 30, name: 'iMac 2025'),
      Category(id: 31, name: 'Mac mini 2025'),
      Category(id: 32, name: 'Mac mini 2024'),
      Category(id: 33, name: 'Macbook Pro 2025'),
      Category(id: 34, name: 'Macbook Air 2025'),
      Category(id: 35, name: 'Macbook Pro 2024'),
      Category(id: 36, name: 'Macbook Air 2024'),
    ]);
  });

  final object1 = index.get(const Address(0, 0));
  print('object 0/0: $object1');

  final object2 = index.get(const Address(0, 1));
  print('object 0/1: $object2');

  final count = performance(
    'count',
    () => index.query(const AllQuery()).count(),
  );
  print('count: $count');
  {
    final categories = performance('fuzzy term query', () {
      return index
          .query(
            // const TermQuery(
            //   term: Term.text('name', 'category 1'),
            //   indexingStrategy: IndexingStrategy.frequenciesAndPositions,
            // ),
            const TermSetQuery(
              terms: [
                Term.text('name', 'sipor'),
                Term.text('name', 'ayakkabi'),
              ],
            ),
          )
          .search(limit: 100, offset: 0);
    });

    print('fuzzy term query: $categories');
  }

  index.close();
}

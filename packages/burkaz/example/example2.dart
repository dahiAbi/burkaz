import 'package:burkaz/burkaz.dart';

class Product {
  final int id;

  final String name;

  const Product({required this.id, required this.name});

  @override
  String toString() {
    return 'Product(id: $id, name: $name)';
  }
}

void serialize(ObjectWriter writer, Product product) {
  writer.writeInt(0, product.id);
  writer.writeString(1, product.name);
}

Product deserialize(ObjectReader reader) {
  return Product(id: reader.readInt(0) ?? 0, name: reader.readString(1) ?? '');
}

void printProducts(Iterable<Product> products) {
  for (final product in products) {
    print('${product.id}: ${product.name}');
  }
}

void main() async {
  const Schema<Product> schema = StrictSchema<Product>(
    fields: [
      Field(
        name: 'id',
        options: IntFieldOptions(
          fast: true,
          indexed: true,
          stored: true,
          //
        ),
      ),
      Field(
        name: 'name',
        options: TextFieldOptions(
          stored: true,
          fast: FastTextFieldOptions.enabled(),
          indexed: IndexedTextFieldOptions.enabled(
            strategy: IndexingStrategy.frequenciesAndPositions,
          ),
          //
        ),
      ),
    ],
    serialize: serialize,
    deserialize: deserialize,
  );

  final index = BurkazIndex.open(name: 'product_index', schema: schema);

  int idCounter = 1;
  int generateId() => idCounter++;

  index.add(Product(id: generateId(), name: 'Mary Corek'));
  index.add(Product(id: generateId(), name: 'Ahal Kici Corek'));
  index.add(Product(id: generateId(), name: 'Ahal Suytli Corek'));
  index.add(Product(id: generateId(), name: 'Iphone 15 pro max'));
  index.add(Product(id: generateId(), name: 'Iphone 16 pro max'));
  index.add(Product(id: generateId(), name: 'Iphone 17 pro max'));
  index.add(Product(id: generateId(), name: 'Iphone 17 pro'));
  index.add(Product(id: generateId(), name: 'Iphone 17e'));
  index.add(Product(id: generateId(), name: 'Redmi Note 14 pro'));
  index.add(Product(id: generateId(), name: 'Redmi Note 14 pro+'));
  index.add(Product(id: generateId(), name: 'Redmi Note 14'));

  print('products count: ${await index.query(const AllQuery()).countAsync()}');

  printProducts(
    await index
        .query(
          const RegexPhaseQuery(field: 'name', terms: ['i', 'pro'], slop: 2),
        )
        .searchAsync(limit: 100),
  );
}

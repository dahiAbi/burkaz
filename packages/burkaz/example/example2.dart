import 'package:burkaz/burkaz.dart';

class Product {
  final int id;

  final String name;

  final bool isActive;

  final List<String> tags;

  const Product({
    required this.id,
    required this.name,
    this.isActive = true,
    this.tags = const [],
  });

  @override
  String toString() {
    return 'Product(id: $id, name: $name, isActive: $isActive, tags: $tags)';
  }
}

void serialize(ObjectWriter writer, Product product) {
  writer.writeInt(0, product.id);
  writer.writeString(1, product.name);
  writer.writeBoolean(2, product.isActive);
  writer.writeStringList(3, product.tags);
}

Product deserialize(ObjectReader reader) {
  return Product(
    id: reader.readInt(0) ?? 0,
    name: reader.readString(1) ?? '',
    isActive: reader.readBoolean(2) ?? false,
    tags: reader.readStringList(3) ?? [],
  );
}

void writeMoney(ObjectWriter writer, String money) {
  // writer.writeInt(0, money.numerator);
  // writer.writeInt(1, money.denominator);
}

void printProducts(Iterable<Product> products) {
  print('products count: ${products.length}');
  for (final product in products) {
    print(
      '${product.id}: ${product.name} '
      '(${product.isActive ? 'active' : 'inactive'})'
      'tags: ${product.tags.join(', ')}',
    );
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
      Field(
        name: 'is_active',
        options: BooleanFieldOptions(stored: true, indexed: true),
      ),
      Field(name: 'tags', options: TextFieldOptions(stored: true)),
    ],
    serialize: serialize,
    deserialize: deserialize,
  );

  final index = BurkazIndex.open(name: 'product_index', schema: schema);

  int idCounter = 1;
  int generateId() => idCounter++;

  index.add(
    Product(id: generateId(), name: 'Mary Corek', tags: ['corek', 'nanlar']),
  );
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

  printProducts(index.query(const AllQuery()).search(limit: 100));

  index.close();

  print('index closed');
}

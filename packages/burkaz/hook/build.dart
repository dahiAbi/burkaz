import 'package:hooks/hooks.dart';
import 'package:native_toolchain_rs/native_toolchain_rs.dart';

void main(List<String> args) async {
  await build(args, (input, output) async {
    const rustBuilder = RustBuilder(
      assetName: 'src/native/native.dart',
      cratePath: '../burkaz_core',
    );

    await rustBuilder.run(input: input, output: output);
  });
}

// import 'package:idl/ffi_rpc/dart_impl.dart';
import 'dart:ffi';
import 'dart:typed_data';

import 'package:idl/ffi_rpc/dart_impl.dart';
import 'package:test/test.dart';

void main() {
  test('ulid uuid v7', () {
    {
      var id = ffiRpc.generateUlid();
      print(id);
    }

    {
      var id = ffiRpc.generateUuidV7();
      print(id);
    }
  });
}

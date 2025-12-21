// import 'package:idl/ffi_rpc/dart_impl.dart';
import 'dart:developer' as developer;

import 'package:idl/ffi_rpc/dart_impl.dart';
import 'package:test/test.dart';

void main() {
  test('ulid uuid v7', () {
    {
      var id = ffiRpc.generateUlid();
      developer.log(id.toString());
    }

    {
      var id = ffiRpc.generateUuidV7();
      developer.log(id.toString());
    }
  });
}

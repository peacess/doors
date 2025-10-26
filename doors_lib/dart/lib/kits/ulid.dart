import 'dart:ffi';
import 'dart:typed_data';
import 'package:ffi/ffi.dart' as ffi;
import 'package:flat_buffers/flat_buffers.dart';
import 'package:idl/idl/base_base_generated.dart';

import '../ffi_rpc/idl.dart';

final class UlidFfi {
  static final len = 16;
  Pointer<Uint8> pointer;
  UlidFfi(this.pointer);
  factory UlidFfi.generate() {
    var p = ffi.calloc<Uint8>(UlidFfi.len);
    idlBindings.generate_ulid(p);
    return UlidFfi(p);
  }

  static int finish(Builder builder) {
    var p = ffi.calloc<Uint8>(UlidFfi.len);
    idlBindings.generate_ulid(p);
    var bc = BufferContext.fromBytes(p.asTypedList(UlidFfi.len));
    var ulidBytes = UlidBytes.reader.read(bc, 0);
    builder.putInt64(ulidBytes.high);
    builder.putInt64(ulidBytes.low);
    ffi.calloc.free(p);
    return builder.offset;
  }

  void free() {
    if (pointer != nullptr) {
      ffi.calloc.free(pointer);
      pointer = nullptr;
    }
  }

  Uint8List toList() {
    return pointer.asTypedList(UlidFfi.len);
  }

  UlidBytes toUlidBytes() {
    var bc = BufferContext.fromBytes(pointer.asTypedList(UlidFfi.len));
    var ulidBytes = UlidBytes.reader.read(bc, 0);
    return ulidBytes;
  }
}

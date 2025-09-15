import 'package:idl/idl/base_base_generated.dart';
import 'package:uuid/uuid.dart';
import 'package:flat_buffers/flat_buffers.dart';

abstract class UuidV7 {
  static const len = 16;
  // byte 16
  static List<int> generate() {
    const id = Uuid();
    return id.v7buffer(List.filled(len, 0));
  }

  static Ubyte16 generateUByte16() {
    var id = Uuid().v7buffer(List.filled(len, 0));
    var ctx = BufferContext.fromBytes(id);
    return Ubyte16.reader.read(ctx, 0);
  }
}

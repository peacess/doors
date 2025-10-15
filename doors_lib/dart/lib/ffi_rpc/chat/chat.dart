import 'dart:ffi';

import 'package:ffi/ffi.dart' as ffi;
import 'package:idl/ffi_rpc/dart_impl.dart';
import 'package:flat_buffers/flat_buffers.dart' as fb;
import 'package:idl/ffi_rpc/idl.dart';
import 'package:idl/idl_bindings_generated.dart';

import '../../idl/base_base_generated.dart';
import '../../idl/chat_chat_generated.dart';

enum ChatType {
  textMessage(1),
  textMessageAck(2);

  final int value;
  const ChatType(this.value);

  factory ChatType.from(int chatType) {
    for (var it in ChatType.values) {
      if (it.value == chatType) {
        return it;
      }
    }
    throw ArgumentError('Unknown header type: $chatType');
  }
}

base class Chat {
  // dont call the TextMessageBuilder.finish()
  void textMessage(TextMessageBuilder builder) {
    var chatBuilder = ChatTextMessageBuilder(builder.fbBuilder);
    chatBuilder.begin();
    var offset = builder.finish();
    chatBuilder.addMessageOffset(offset);

    //todo
    // headerBuild.finish(HeaderType.chat.value, ChatType.textMessage.value, text.fbBuilder.size());
  }

  // dont call the TextMessageAckBuilder.finish()
  void textMessageAck(TextMessageAckBuilder ack) {}

  void call(fb.Builder builder) {
    var buffer = builder.buffer;
    // final call = dylib.lookupFunction<FfiBytes Function(Pointer<Uint8>, Uint64), FfiBytes Function(Pointer<Uint8>, int)>('call',isLeaf: true);
    final pointer = ffi.calloc<Uint8>(buffer.length);
    try {
      pointer.asTypedList(buffer.length).setAll(0, buffer);
      var ffiBytes = ffiRpc.call(pointer, buffer.length);
      ffiRpc.bytesFree(ffiBytes);
    } finally {
      ffi.calloc.free(pointer);
    }
  }
}

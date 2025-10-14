import 'dart:ffi';

import 'package:idl/ffi_rpc/dart_impl.dart';
import 'package:idl/idl/net_discovery_net_discovery_generated.dart';
import 'package:flat_buffers/flat_buffers.dart' as fb;

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
  void textMessage(TextMessageBuilder text) {
    var builder = fb.Builder(initialSize: Header.reader.size);
    var headerBuild = HeaderBuilder(builder);
    //todo
    // headerBuild.finish(HeaderType.chat.value, ChatType.textMessage.value, text.fbBuilder.size());
  }

  void textMessageAck(TextMessageAck ack) {}

  void call(Header header, fb.BufferContext data) {
    var ffiBytes = FfiBytesHelper.from(data);
    //todo
    // ffiRpc.call(header,ffiBytes.ref);
  }
}

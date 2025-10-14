import 'package:flat_buffers/flat_buffers.dart' as fb;

import '../../idl/base_base_generated.dart';
import '../../idl/chat_chat_generated.dart';

enum ChatCallbackType {
  textMessage(1),
  textMessageAck(2);

  final int callback;
  const ChatCallbackType(this.callback);

  factory ChatCallbackType.from(int chatCallbackType) {
    for (var it in ChatCallbackType.values) {
      if (it.callback == chatCallbackType) {
        return it;
      }
    }
    throw ArgumentError('Unknown header type: $chatCallbackType');
  }
}

base class ChatCallback {
  void textMessage(Header header, TextMessage text) {}
  void textMessageAck(Header header, TextMessageAck ack) {}

  void callback(fb.BufferContext buffer, Header header, int offset) {
    var tempType = ChatCallbackType.from(header.frameType);
    switch (tempType) {
      case ChatCallbackType.textMessage:
        var hi = TextMessage.reader.read(buffer, offset);
        textMessage(header, hi);
        break;
      case ChatCallbackType.textMessageAck:
        var hi = TextMessageAck.reader.read(buffer, offset);
        textMessageAck(header, hi);
        break;
    }
  }
}

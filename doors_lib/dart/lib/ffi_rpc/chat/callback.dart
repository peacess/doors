import 'package:flat_buffers/flat_buffers.dart' as fb;

import '../../idl/base_base_generated.dart';
import '../../idl/chat_chat_generated.dart';
import '../partner/partners.dart';

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
  final Partners partners;
  ChatCallback(this.partners);
  void textMessage(ChatTextMessage chatText) {
    if (chatText.header != null && chatText.message != null) {
      var partner = partners.getByTerminal(chatText.header!.toTerminalId.unpack());
      if (partner != null) {
        partner.addTextMessage(chatText.message!.unpack());
      }
    }
  }

  void textMessageAck(ChatTextMessageAck ack) {}

  void callback(fb.BufferContext buffer, Header header) {
    var tempType = ChatCallbackType.from(header.frameType);
    switch (tempType) {
      case ChatCallbackType.textMessage:
        var text = ChatTextMessage.reader.read(buffer, 0);
        textMessage(text);
        break;
      case ChatCallbackType.textMessageAck:
        var ack = ChatTextMessageAck.reader.read(buffer, 0);
        textMessageAck(ack);
        break;
    }
  }
}

import 'package:flat_buffers/flat_buffers.dart' as fb;

import '../../idl/base_base_generated.dart';
import '../../idl/chat_chat_generated.dart';
import '../partner/partners.dart';

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
    var tempType = ChatType.fromValue(header.frameType);
    switch (tempType) {
      case ChatType.none:
        break;
      case ChatType.text_message:
        var text = ChatTextMessage.reader.read(buffer, 0);
        textMessage(text);
        break;
      case ChatType.text_message_ack:
        var ack = ChatTextMessageAck.reader.read(buffer, 0);
        textMessageAck(ack);
        break;
    }
  }
}

import 'package:idl/ffi_rpc/chat/callback.dart';
import 'package:idl/idl/base_base_generated.dart';
import 'package:idl/idl/chat_chat_generated.dart';

final class AppChatCallback extends ChatCallback {
  @override
  void textMessage(Header header, TextMessage text) {
    // print('textMessage: ${text.fromId}, ${text.toId}, ${text.text}');
  }

  @override
  void textMessageAck(Header header, TextMessageAck ack) {
    // print('textMessageAck: ${ack.messageId}, ${ack.status}');
  }
}

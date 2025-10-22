import 'package:idl/ffi_rpc/chat/callback.dart';
import 'package:idl/idl/base_base_generated.dart';
import 'package:idl/idl/chat_chat_generated.dart';

final class AppChatCallback extends ChatCallback {
  @override
  void textMessage(ChatTextMessage text) {
    // print('textMessage: ${text.fromId}, ${text.toId}, ${text.text}');
  }

  @override
  void textMessageAck(ChatTextMessageAck ack) {
    // print('textMessageAck: ${ack.messageId}, ${ack.status}');
  }
}

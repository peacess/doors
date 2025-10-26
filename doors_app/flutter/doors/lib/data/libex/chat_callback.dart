import 'package:idl/ffi_rpc/chat/chat.dart';
import 'package:idl/ffi_rpc/chat/chat_callback.dart';
import 'package:idl/idl/chat_chat_generated.dart';

final class AppChatCallback extends ChatCallback {
  AppChatCallback(super.partners);

  @override
  void textMessage(ChatTextMessage chatText) {
    // print('textMessage: ${text.fromId}, ${text.toId}, ${text.text}');
  }

  @override
  void textMessageAck(ChatTextMessageAck ack) {
    // print('textMessageAck: ${ack.messageId}, ${ack.status}');
  }
}

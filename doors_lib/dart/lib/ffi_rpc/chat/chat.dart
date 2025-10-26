import 'package:flat_buffers/flat_buffers.dart' as fb;
import 'package:idl/ffi_rpc/partner/partner.dart';

import '../../idl/base_base_generated.dart';
import '../../idl/chat_chat_generated.dart';
import '../partner/partners.dart';

abstract class Chat {
  static void textMessage(String text, Partner from, Partner to) {
    var fbBuilder = fb.Builder(initialSize: 1024);
    var textMessage = TextMessageBuilder(fbBuilder);
    {
      var id = UlidBytesBuilder(fbBuilder).finish(0, 0);
      var fromPartner = PartnerIdBuilder(fbBuilder).finish(from.partnerId.low, from.partnerId.high);
      var toPartner = PartnerIdBuilder(fbBuilder).finish(to.partnerId.low, to.partnerId.high);
      // var fromTeminal = TerminalIdBuilder(fbBuilder).finish(from, high)
    }
    // if (chatText.header != null && chatText.message != null) {
    //   var partner = partners.getByTerminal(chatText.header!.toTerminalId.unpack());
    //   if (partner != null) {
    //     partner.addTextMessage(chatText.message!.unpack());
    //   }
    // }
  }

  static void textMessageAck(ChatTextMessageAck ack) {}

  void call(fb.BufferContext buffer, Header header) {}
}

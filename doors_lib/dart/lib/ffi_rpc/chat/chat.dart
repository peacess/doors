import 'dart:ffi';

import 'package:flat_buffers/flat_buffers.dart' as fb;
import 'package:idl/ffi_rpc/partner/partner.dart';
import 'package:idl/idl.dart';
import 'package:ffi/ffi.dart' as ffi;

import '../../idl/base_base_generated.dart';
import '../../idl/chat_chat_generated.dart';
import '../../kits/ulid.dart';

abstract class Chat {
  static void textMessage(String text, Partner from, Partner to) {
    var messageId = UlidFfi.generate();
    var messageIdUlid = messageId.toUlidBytes();
    for (var toTerminal in to.terminals) {
      if (toTerminal.terminalId == null || from.terminals.firstOrNull == null) {
        continue;
      }
      var fbBuilder = fb.Builder(initialSize: 1024);
      var textMessageOffset = 0;
      {
        var textMessage = TextMessageBuilder(fbBuilder);
        var id = UlidFfi.finish(fbBuilder);
        var fromPartnerOffset = PartnerIdBuilder(fbBuilder).finish(from.partnerId.low, from.partnerId.high);
        var fromTerminalOffset = TerminalIdBuilder(fbBuilder).finish(from.terminals.first.terminalId!.low, from.terminals.first.terminalId!.high);

        var toPartnerOffet = PartnerIdBuilder(fbBuilder).finish(to.partnerId.low, to.partnerId.high);
        var toTerminalOffset = TerminalIdBuilder(fbBuilder).finish(toTerminal.terminalId!.low, toTerminal.terminalId!.high);
        var messageIdOffset = UlidBytesBuilder(fbBuilder).finish(messageIdUlid.low, messageIdUlid.high);
        var textOffset = fbBuilder.writeString(text);
        textMessage.begin();
        textMessage.addId(id);
        textMessage.addFromPartnerId(fromPartnerOffset);
        textMessage.addFromTerminalId(fromTerminalOffset);
        textMessage.addToPartnerId(toPartnerOffet);
        textMessage.addFromTerminalId(toTerminalOffset);
        textMessage.addMessageId(messageIdOffset);
        textMessage.addTextOffset(textOffset);
        textMessageOffset = textMessage.finish();
      }
      var headerOffset = 0;
      {
        var header = HeaderBuilder(fbBuilder);
        headerOffset = header.finish(
          textMessageOffset + Header.reader.size,
          HeaderType.chat.value,
          ChatType.text_message.value,
          () {
            TerminalIdBuilder(fbBuilder).finish(toTerminal.terminalId!.low, toTerminal.terminalId!.high);
          },
          () {
            X25519PublicBuilder(fbBuilder).finish(0, 0, 0, 0);
          },
        );
      }

      var chat = ChatTextMessageBuilder(fbBuilder);
      // var bytesOffset = 0;
      {
        chat.begin();
        chat.addHeader(headerOffset);
        chat.addMessageOffset(textMessageOffset);
        // bytesOffset = chat.finish();
      }
      call(fbBuilder);
    }
    messageId.free();
  }

  static void textMessageAck(ChatTextMessageAck ack) {}

  static void call(fb.Builder builder) {
    final len = builder.size() - builder.offset;
    final pointer = ffi.calloc<Uint8>(len);
    pointer.asTypedList(len).setAll(builder.offset, builder.buffer);
    ffiRpc.call(pointer, len);
    ffi.calloc.free(pointer);
  }
}

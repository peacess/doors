import 'package:idl/ffi_rpc/idl_ex/base.dart';
import 'package:idl/idl/base_base_generated.dart';
import 'package:idl/idl/chat_chat_generated.dart';
import 'package:idl/idl/net_discovery_net_discovery_generated.dart';
import 'package:logger/logger.dart';

import '../../idl/partner_partner_generated.dart';
import '../../kits/value_notifier_ex.dart';

final _logger = Logger();

class Partner {
  UlidBytesT id;
  PartnerIdT partnerId;
  String showName;
  List<DnsTerminalT> terminals = [];
  ValueNotifierEx<List<TextMessageT>> texts = ValueNotifierEx([]);

  Partner(this.id, this.partnerId, {this.showName = ""});

  factory Partner.fromHi(Hi hi) {
    var data = PartnerT();
    data.id = hi.id!.unpack();
    var dnsTerminal = hi.dnsTerminal!;
    var showName = hi.showName ?? dnsTerminal.showName ?? dnsTerminal.hostName ?? "";
    final partner = Partner(hi.id!.unpack(), dnsTerminal.partnerId!.unpack(), showName: showName);
    {
      var terminal = dnsTerminal.unpack();
      partner.terminals.add(terminal);
    }
    return partner;
  }

  void addTextMessage(TextMessageT text) {
    var notfind = true;
    if (text.id != null) {
      for (var it in texts.value) {
        if (it.id!.eq(text.id!)) {
          notfind = false;
          break;
        }
      }
    }
    if (notfind) {
      texts.value.add(text);
      texts.notifyListeners();
    }
  }
}

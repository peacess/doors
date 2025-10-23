import 'package:idl/idl/base_base_generated.dart';
import 'package:idl/idl/net_discovery_net_discovery_generated.dart';
import 'package:logger/logger.dart';

import '../../idl/partner_partner_generated.dart';

final _logger = Logger();

class Partner {
  UlidBytesT id;
  PartnerIdT partnerId;
  String showName;
  List<DnsTerminalT> terminals = [];

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

  void sendTextMessage(String text, Partner to) {
    _logger.i('Send message to $showName: $text');
  }
}

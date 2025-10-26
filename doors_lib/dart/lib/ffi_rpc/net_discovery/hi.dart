import 'package:idl/ffi_rpc/partner/partner.dart';
import 'package:idl/ffi_rpc/partner/partners.dart';
import 'package:idl/idl/net_discovery_net_discovery_generated.dart';
import 'package:flat_buffers/flat_buffers.dart' as fb;

import '../../idl/base_base_generated.dart';

base class NetDiscoveryCallback {
  final Partners partners;
  NetDiscoveryCallback(this.partners);

  void hi(HiFrame hi) {
    partners.add(Partner.fromHi(hi.hi!));
  }

  void callback(fb.BufferContext buffer, Header header) {
    var tempType = NetDiscoveryType.fromValue(header.frameType);
    switch (tempType) {
      case NetDiscoveryType.none:
        break;
      case NetDiscoveryType.hi:
        var hiFrame = HiFrame.reader.read(buffer, 0);
        hi(hiFrame);
        break;
    }
  }
}

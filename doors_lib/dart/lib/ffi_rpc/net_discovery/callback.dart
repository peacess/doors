import 'package:idl/idl/net_discovery_net_discovery_generated.dart';
import 'package:flat_buffers/flat_buffers.dart' as fb;

import '../../idl/base_base_generated.dart';

enum NetDiscoveryCallbackType {
  hiRecv(1);

  final int callback;
  const NetDiscoveryCallbackType(this.callback);

  factory NetDiscoveryCallbackType.from(int netDiscoveryCallbackType) {
    for (var it in NetDiscoveryCallbackType.values) {
      if (it.callback == netDiscoveryCallbackType) {
        return it;
      }
    }
    throw ArgumentError('Unknown header type: $netDiscoveryCallbackType');
  }
}

base class NetDiscoveryCallback {
  void hiRecv(Header header, HiRecv hi) {}

  void callback(fb.BufferContext buffer, Header header, int offset) {
    var tempType = NetDiscoveryCallbackType.from(header.frameType);
    switch (tempType) {
      case NetDiscoveryCallbackType.hiRecv:
        var hi = HiRecv.reader.read(buffer, offset);
        hiRecv(header, hi);
        break;
    }
  }
}

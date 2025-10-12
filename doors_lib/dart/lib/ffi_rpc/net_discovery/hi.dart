import 'package:idl/idl/net_discovery_net_discovery_generated.dart';
import 'package:flat_buffers/flat_buffers.dart' as fb;

base class NetDiscoveryCallback {
  void hiRecv(HiRecv hi) {}

  void callback(fb.BufferContext buffer, DiscoveryHeader header, int offset) {
    var tempType = NetDiscoveryType.from(header.discoveryType);
    switch (tempType) {
      case NetDiscoveryType.hiRecv:
        var hi = HiRecv.reader.read(buffer, offset);
        hiRecv(hi);
        break;
    }
  }
}

enum NetDiscoveryType {
  hiRecv(1);

  final int value;
  const NetDiscoveryType(this.value);

  factory NetDiscoveryType.from(int netDiscoveryType) {
    for (var it in NetDiscoveryType.values) {
      if (it.value == netDiscoveryType) {
        return it;
      }
    }
    throw ArgumentError('Unknown header type: $netDiscoveryType');
  }
}

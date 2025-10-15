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

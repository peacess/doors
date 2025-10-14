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

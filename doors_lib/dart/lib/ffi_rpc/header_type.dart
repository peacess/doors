enum HeaderType {
  netDiscovery(1),
  chat(2);

  final int value;
  const HeaderType(this.value);

  factory HeaderType.from(int headerType) {
    for (var it in HeaderType.values) {
      if (it.value == headerType) {
        return it;
      }
    }
    throw ArgumentError('Unknown header type: $headerType');
  }
}

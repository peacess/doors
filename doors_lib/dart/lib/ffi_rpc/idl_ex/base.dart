import 'package:idl/idl/base_base_generated.dart';

extension UlidBytesEx on UlidBytes {
  bool eq(UlidBytes other) {
    if (identical(this, other)) return true;

    return other.ts1 == ts1 &&
        other.ts2 == ts2 &&
        other.ts3 == ts3 &&
        other.ts4 == ts4 &&
        other.ts5 == ts5 &&
        other.ts6 == ts6 &&
        other.rand7 == rand7 &&
        other.rand8 == rand8 &&
        other.rand9 == rand9 &&
        other.rand10 == rand10 &&
        other.rand11 == rand11 &&
        other.rand12 == rand12 &&
        other.rand13 == rand13 &&
        other.rand14 == rand14 &&
        other.rand15 == rand15 &&
        other.rand16 == rand16;
  }
}

extension UlidBytesTEx on UlidBytesT {
  bool eq(UlidBytesT other) {
    if (identical(this, other)) return true;

    return other.ts1 == ts1 &&
        other.ts2 == ts2 &&
        other.ts3 == ts3 &&
        other.ts4 == ts4 &&
        other.ts5 == ts5 &&
        other.ts6 == ts6 &&
        other.rand7 == rand7 &&
        other.rand8 == rand8 &&
        other.rand9 == rand9 &&
        other.rand10 == rand10 &&
        other.rand11 == rand11 &&
        other.rand12 == rand12 &&
        other.rand13 == rand13 &&
        other.rand14 == rand14 &&
        other.rand15 == rand15 &&
        other.rand16 == rand16;
  }
}

extension PartnerIdEx on PartnerId {
  bool eq(PartnerId other) {
    if (identical(this, other)) return true;

    return other.low == low && other.high == high;
  }
}

extension PartnerIdTEx on PartnerIdT {
  bool eq(PartnerIdT other) {
    if (identical(this, other)) return true;

    return other.low == low && other.high == high;
  }
}

extension TerminalIdEx on TerminalId {
  bool eq(TerminalId other) {
    if (identical(this, other)) return true;

    return other.low == low && other.high == high;
  }
}

extension TerminalIdTEx on TerminalIdT {
  bool eq(TerminalIdT other) {
    if (identical(this, other)) return true;

    return other.low == low && other.high == high;
  }
}

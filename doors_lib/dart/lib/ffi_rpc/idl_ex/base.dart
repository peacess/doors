import 'package:idl/idl/base_base_generated.dart';

extension UlidBytesEx on UlidBytes {
  bool eq(UlidBytes other) {
    if (identical(this, other)) return true;

    return other.low == low && other.high == high;
  }
}

extension UlidBytesTEx on UlidBytesT {
  bool eq(UlidBytesT other) {
    if (identical(this, other)) return true;

    return other.low == low && other.high == high;
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

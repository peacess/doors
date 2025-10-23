import 'package:idl/ffi_rpc/idl_ex/base.dart';
import 'package:idl/kits/value_notifier_ex.dart';

import './partner.dart';

class Partners {
  final patterns = ValueNotifierEx<List<Partner>>([]);

  void add(Partner pattern) {
    for (final it in patterns.value) {
      if (it.id.eq(pattern.id)) {
        if (_merge(it, pattern)) {
          patterns.notifyListeners();
        }
        return;
      }
    }
    patterns.value.add(pattern);
    patterns.notifyListeners();
  }

  void remove(Partner partner) {
    // remove first find partner

    patterns.value.removeWhere((it) => it.id.eq(partner.id));
    patterns.notifyListeners();
  }

  // the terminal is not too much, so use the list, not map
  bool _merge(Partner pattern, Partner newPattern) {
    bool changed = false;
    final currentLen = pattern.terminals.length;
    for (int i = 0; i < newPattern.terminals.length; i++) {
      final newTerminal = pattern.terminals[i];
      bool notFind = true;
      for (int index = 0; index < currentLen; index++) {
        final oldTerminal = newPattern.terminals[index];
        if (newTerminal.terminalId!.eq(oldTerminal.terminalId!)) {
          pattern.terminals[index] = newTerminal;
          notFind = false;
          changed = true;
          break;
        }
      }
      if (notFind) {
        pattern.terminals.add(newTerminal);
        changed = true;
      }
    }
    return changed;
  }
}

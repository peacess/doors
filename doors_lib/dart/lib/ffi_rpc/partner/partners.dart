import 'package:idl/kits/value_notifier_ex.dart';

import './partner.dart';

class Partners {
  final patterns = ValueNotifierEx<List<Partner>>([]);

  void add(Partner pattern) {
    for (final it in patterns.value) {
      if (it.id == pattern.id) {
        _merge(it, pattern);
        return;
      }
    }
    patterns.value.add(pattern);
    patterns.notifyListeners();
  }

  void remove(Partner partner) {
    // remove first find partner

    patterns.value.removeWhere((it) => it.id == partner.id);
    patterns.notifyListeners();
  }

  void _merge(Partner pattern, Partner newPattern) {}
}

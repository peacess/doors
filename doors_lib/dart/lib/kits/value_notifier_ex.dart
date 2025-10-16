import 'package:flutter/material.dart';

class ValueNotifierEx<T> extends ValueNotifier<T> {
  ValueNotifierEx(super.value);
  @override
  void notifyListeners() {
    super.notifyListeners();
  }
}

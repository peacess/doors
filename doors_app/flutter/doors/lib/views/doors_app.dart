import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

typedef MakeWidget<T extends Widget> = T Function();

class DoorsApp {
  final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();
  final ValueNotifier<String> title = ValueNotifier("Doors");
  final ValueNotifier<String> subTitle = ValueNotifier("");

  // final ValueNotifier<MakeWidget<Widget>> mainWidget = ValueNotifier(HomePage.make);

  final ValueNotifier<ThemeMode> themeMode = ValueNotifier(ThemeMode.dark);

  late MediaQueryData mediaQueryData;

  // MediaQueryData get mediaQueryData => _mediaQueryData;

  static final DoorsApp app = DoorsApp();

  bool widthIsMobile() {
    return mediaQueryData.size.width < 600;
  }

  bool widthIsTablet() {
    return mediaQueryData.size.width >= 600;
  }

  bool systemIsPc() {
    return Platform.isWindows || Platform.isLinux || Platform.isMacOS;
  }

  bool isWeb() {
    return kIsWeb;
  }
}

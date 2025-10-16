import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:idl/ffi_rpc/partner/partners.dart';

typedef MakeWidget<T extends Widget> = T Function();

class DoorsApp {
  final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();
  final title = ValueNotifier("Doors");
  final subTitle = ValueNotifier("");

  // final ValueNotifier<MakeWidget<Widget>> mainWidget = ValueNotifier(HomePage.make);

  final themeMode = ValueNotifier(ThemeMode.dark);

  late MediaQueryData mediaQueryData;

  // MediaQueryData get mediaQueryData => _mediaQueryData;

  final partners = Partners();

  static final app = DoorsApp();

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

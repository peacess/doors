import 'package:doors/views/home_page.dart';
import 'package:flutter/material.dart';

typedef MakeWidget<T extends Widget> = T Function();

class DoorsApp {
  final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();
  final ValueNotifier<String> title = ValueNotifier("Doors");
  final ValueNotifier<String> subTitle = ValueNotifier("");

  final ValueNotifier<MakeWidget<Widget>> mainWidget = ValueNotifier(HomePage.make);

  void changeMainWidget(MakeWidget newWidget) {
    mainWidget.value = newWidget;
  }

  static final DoorsApp app = DoorsApp();
}

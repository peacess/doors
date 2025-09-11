import 'package:doors/views/home_page.dart';
import 'package:doors/main_page.dart';
import 'package:flutter/material.dart';

class DoorsAppPage extends StatelessWidget {
  const DoorsAppPage({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorKey: DoorsApp.app.navigatorKey,
      title: 'Doors',
      theme: ThemeData(
        // colorScheme: ColorScheme.fromSeed(seedColor: Colors.black),
        colorScheme: ColorScheme.dark(),
        useMaterial3: true,
      ),
      home: MainPage(),
    );
  }
}

class DoorsApp {
  final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();
  final ValueNotifier<String> title = ValueNotifier("Doors");
  final ValueNotifier<String> subTitle = ValueNotifier("");

  final ValueNotifier<Widget Function()> mainPage = ValueNotifier(HomePage.make);

  void changeMainView(Widget Function() newView) {
    mainPage.value = newView;
  }

  static final DoorsApp app = DoorsApp();
}

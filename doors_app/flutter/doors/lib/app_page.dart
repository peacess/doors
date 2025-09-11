import 'package:doors/views/category.dart';
import 'package:doors/views/doors_app.dart';
import 'package:doors/views/title_bar.dart';
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
      home: buildHome(),
    );
  }

  Widget buildHome() {
    final ValueNotifier<MakeWidget<Widget>> mainWidget = DoorsApp.app.mainWidget;
    return Scaffold(
      appBar: TitleBar(),
      body: ValueListenableBuilder(
        valueListenable: mainWidget,
        builder: (context, value, child) {
          return mainWidget.value();
        },
      ),
      drawer: Category(),
    );
  }
}

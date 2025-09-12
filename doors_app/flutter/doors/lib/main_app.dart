import 'package:doors/views/category.dart';
import 'package:doors/views/doors_app.dart';
import 'package:doors/views/title_bar.dart';
import 'package:flutter/material.dart';

class DoorsMainApp extends StatelessWidget {
  const DoorsMainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorKey: DoorsApp.app.navigatorKey,
      title: 'Doors',
      theme: ThemeData(
        // colorScheme: ColorScheme.fromSeed(seedColor: Colors.black),
        colorScheme: ColorScheme.dark(),
        useMaterial3: true,
        textButtonTheme: TextButtonThemeData(
          style: TextButton.styleFrom(shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero)),
        ),
        iconButtonTheme: IconButtonThemeData(
          style: IconButton.styleFrom(shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero)),
        ),
      ),
      home: Scaffold(
        appBar: TitleBar(),
        body: ValueListenableBuilder(
          valueListenable: DoorsApp.app.mainWidget,
          builder: (context, makeWidget, child) {
            return makeWidget();
          },
        ),
        drawer: Category(),
      ),
    );
  }
}

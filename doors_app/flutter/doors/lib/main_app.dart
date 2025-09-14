import 'package:doors/views/category.dart';
import 'package:doors/views/doors_app.dart';
import 'package:doors/views/title_bar.dart';
import 'package:flutter/material.dart';

class DoorsMainApp extends StatefulWidget {
  const DoorsMainApp({super.key});

  @override
  State<DoorsMainApp> createState() => _DoorsMainApp();
}

class _DoorsMainApp extends State<DoorsMainApp> {
  final ValueNotifier<ThemeMode> themeMode = DoorsApp.app.themeMode;

  @override
  void initState() {
    super.initState();
    themeMode.addListener(_refresh);
  }

  @override
  void dispose() {
    themeMode.removeListener(_refresh);
    super.dispose();
  }

  void _refresh() {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorKey: DoorsApp.app.navigatorKey,
      title: 'Doors',
      theme: ThemeData(
        // colorScheme: ColorScheme.fromSeed(seedColor: Colors.black),
        colorScheme: ColorScheme.light(),
        useMaterial3: true,
        textButtonTheme: TextButtonThemeData(
          style: TextButton.styleFrom(shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero)),
        ),
        iconButtonTheme: IconButtonThemeData(
          style: IconButton.styleFrom(shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero)),
        ),
      ),
      darkTheme: ThemeData(
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

      themeMode: themeMode.value,

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

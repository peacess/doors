import 'package:doors/views/category.dart';
import 'package:doors/views/doors_app.dart';
import 'package:doors/views/title_bar.dart';
import 'package:flutter/material.dart';

class MainPage extends StatefulWidget {
  const MainPage({super.key});

  @override
  State<MainPage> createState() => _MainPageState();
}

class _MainPageState extends State<MainPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: TitleBar(),
      body: ValueListenableBuilder(
        valueListenable: DoorsApp.app.mainPage,
        builder: (context, value, child) {
          return DoorsApp.app.mainPage.value();
        },
      ),
      drawer: Category(),
    );
  }
}

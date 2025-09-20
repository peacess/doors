import 'package:doors/views/category.dart';
import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';

class BluePage extends StatefulWidget {
  const BluePage({super.key});

  @override
  State<BluePage> createState() => _BluePageState();

  static MakeWidget<BluePage> make = () => BluePage();
  static void open() {
    DoorsApp.app.navigatorKey.currentState?.pushAndRemoveUntil(MaterialPageRoute(builder: (context) => BluePage()), (route) => false);
  }
}

class _BluePageState extends State<BluePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: Center(child: Text("Blue Page")),
      drawer: Category(),
    );
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      DoorsApp.app.subTitle.value = "Blue";
    });
  }
}

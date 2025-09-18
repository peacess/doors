import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';

class BluePage extends StatefulWidget {
  const BluePage({super.key});

  @override
  State<BluePage> createState() => _BluePageState();

  static MakeWidget<BluePage> make = () => BluePage();
}

class _BluePageState extends State<BluePage> {
  @override
  Widget build(BuildContext context) {
    return Center(child: Text("Blue Page"));
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      DoorsApp.app.subTitle.value = "Blue";
    });
  }
}

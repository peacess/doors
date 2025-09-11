import 'package:doors/views/category.dart';
import 'package:doors/views/doors_app.dart';
import 'package:doors/views/title_bar.dart';
import 'package:flutter/material.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();

  static MakeWidget<HomePage> make = () => HomePage();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return Center(child: Text("Home Page"));
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      DoorsApp.app.subTitle.value = "Home";
    });
  }
}

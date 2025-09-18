import 'package:doors/views/blue/blue_page.dart';
import 'package:doors/views/home_page.dart';
import 'package:doors/views/chat/chat_page.dart';
import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';

class Category extends StatefulWidget {
  const Category({super.key});

  @override
  State<Category> createState() => _CategoryState();
}

class _CategoryState extends State<Category> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: Column(
        spacing: 10.0,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          IconButton(onPressed: () => Scaffold.of(context).openEndDrawer(), icon: Icon(Icons.chevron_left)),
          Expanded(
            child: Wrap(
              spacing: 10.0,
              runSpacing: 10.0,
              children: [
                TextButton.icon(
                  onPressed: () async {
                    Scaffold.of(context).openEndDrawer();
                    DoorsApp.app.changeMainWidget(HomePage.make);
                  },
                  label: Text("Home"),
                  icon: Icon(Icons.home),
                ),
                TextButton.icon(
                  onPressed: () async {
                    Scaffold.of(context).openEndDrawer();
                    DoorsApp.app.changeMainWidget(ChatPage.make);
                  },
                  label: Text("Chat"),
                  icon: Icon(Icons.chat),
                ),
                TextButton.icon(
                  onPressed: () async {
                    Scaffold.of(context).openEndDrawer();
                    DoorsApp.app.changeMainWidget(BluePage.make);
                  },
                  label: Text("Blue"),
                  icon: Icon(Icons.bluetooth),
                ),
              ],
            ),
          ),
          Row(
            children: [
              for (var mode in ThemeMode.values)
                IconButton(
                  onPressed: () {
                    DoorsApp.app.themeMode.value = mode;
                  },
                  icon: Icon(switch (mode) {
                    ThemeMode.system => Icons.brightness_auto,
                    ThemeMode.light => Icons.brightness_7,
                    ThemeMode.dark => Icons.brightness_2,
                  }),
                ),
            ],
          ),
        ],
      ),
    );
  }
}

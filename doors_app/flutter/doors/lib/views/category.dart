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
                    DoorsApp.app.changeMainView(HomePage.make);
                  },
                  label: Text("Home"),
                  icon: Icon(Icons.home),
                ),
                TextButton.icon(
                  onPressed: () async {
                    Scaffold.of(context).openEndDrawer();
                    DoorsApp.app.changeMainView(ChatPage.make);
                  },
                  label: Text("Chat"),
                  icon: Icon(Icons.chat),
                ),
              ],
            ),
          ),
          IconButton(onPressed: () => Scaffold.of(context).openEndDrawer(), icon: Icon(Icons.chevron_left)),
        ],
      ),
    );
  }
}

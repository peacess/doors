import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';

class ChatPage extends StatefulWidget {
  const ChatPage({super.key});

  @override
  State<ChatPage> createState() => _ChatPageState();

  static MakeWidget<ChatPage> make = () => ChatPage();
}

class _ChatPageState extends State<ChatPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(child: Text("Chat Page")),
      drawer: Drawer(
        child: Column(
          spacing: 10.0,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [IconButton(onPressed: () => Scaffold.of(context).openEndDrawer(), icon: Icon(Icons.chevron_left))],
        ),
      ),
    );
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      DoorsApp.app.subTitle.value = "Chat";
    });
  }
}

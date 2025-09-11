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
    return Center(child: Text("Chat Page"));
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      DoorsApp.app.subTitle.value = "Chat";
    });
  }
}

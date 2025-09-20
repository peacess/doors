import 'package:doors/views/category.dart';
import 'package:doors/views/chat/chat_title_bar.dart';
import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';

class ChatPage extends StatefulWidget {
  const ChatPage({super.key});

  @override
  State<ChatPage> createState() => _ChatPageState();

  static MakeWidget<ChatPage> make = () => ChatPage();
  static void open() {
    DoorsApp.app.navigatorKey.currentState?.pushAndRemoveUntil(MaterialPageRoute(builder: (context) => ChatPage()), (route) => false);
  }
}

class _ChatPageState extends State<ChatPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: ChatTitleBar(),
      body: Center(child: Text("Chat Page")),
      drawer: Category(),
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

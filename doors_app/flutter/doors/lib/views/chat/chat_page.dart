import 'package:doors/views/category.dart';
import 'package:doors/views/chat/chat_title_bar.dart';
import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';

class ChatPage extends StatefulWidget {
  ChatPage({super.key});
  final ValueNotifier<bool> showLeft = ValueNotifier(true);

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
      appBar: ChatTitleBar(widget.showLeft),
      body: SizedBox.expand(
        child: Row(
          children: [
            if (!DoorsApp.app.widthIsMobile()) makeLeft(),
            Expanded(child: Center(child: Text("chat"))),
          ],
        ),
      ),
      bottomNavigationBar: makeBotton(),
      drawer: Category(),
    );
  }

  Widget makeLeft() {
    return ValueListenableBuilder(
      valueListenable: widget.showLeft,
      builder: (context, value, child) {
        return Visibility(
          visible: value,
          child: Container(
            width: 360,
            padding: EdgeInsets.only(left: 6),
            child: ValueListenableBuilder(
              valueListenable: DoorsApp.app.partners.patterns,
              builder: (_, value, _) {
                return Wrap(
                  spacing: 8.0,
                  runSpacing: 8.0,
                  children: [for (var it in value) TextButton.icon(onPressed: () {}, label: Text(it.showName))],
                );
              },
            ),
          ),
        );
      },
    );
  }

  Widget? makeBotton() {
    if (DoorsApp.app.widthIsMobile()) {}
    return null;
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      DoorsApp.app.subTitle.value = "Chat";
    });
  }
}

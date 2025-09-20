import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';

class ChatTitleBar extends StatefulWidget implements PreferredSizeWidget {
  ChatTitleBar(this.showLeft, {super.key});
  final ValueNotifier<String> title = DoorsApp.app.title;
  final ValueNotifier<String> subTitle = DoorsApp.app.subTitle;
  final ValueNotifier<bool> showLeft;

  @override
  State<ChatTitleBar> createState() => _ChatTitleBarState();

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}

class _ChatTitleBarState extends State<ChatTitleBar> {
  @override
  Widget build(BuildContext context) {
    return AppBar(
      automaticallyImplyLeading: false,
      leading: null,
      title: Row(
        spacing: 6.0,
        children: [
          IconButton(
            icon: const Icon(Icons.menu),
            onPressed: () {
              Scaffold.of(context).openDrawer();
            },
          ),
          ValueListenableBuilder(
            valueListenable: widget.showLeft,
            builder: (_, value, _) {
              return IconButton(
                icon: Icon(value ? Icons.arrow_back : Icons.arrow_forward),
                onPressed: () {
                  widget.showLeft.value = !widget.showLeft.value;
                },
              );
            },
          ),
          ValueListenableBuilder(
            valueListenable: widget.subTitle,
            builder: (_, value, _) {
              return Text(value);
            },
          ),
        ],
      ),
    );
  }
}

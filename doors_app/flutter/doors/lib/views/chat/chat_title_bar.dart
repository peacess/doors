import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';

class ChatTitleBar extends StatefulWidget implements PreferredSizeWidget {
  ChatTitleBar({super.key});
  final ValueNotifier<String> title = DoorsApp.app.title;
  final ValueNotifier<String> subTitle = DoorsApp.app.subTitle;

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
          IconButton(
            icon: const Icon(Icons.arrow_forward),
            onPressed: () {
              Navigator.of(context).pop();
            },
          ),
          ValueListenableBuilder(
            valueListenable: widget.title,
            builder: (_, _, _) {
              return Text(widget.title.value);
            },
          ),
          ValueListenableBuilder(
            valueListenable: widget.subTitle,
            builder: (_, _, _) {
              var t = "";
              if (widget.subTitle.value.isNotEmpty) {
                t = " - ${widget.subTitle.value}";
              }
              return Text(t);
            },
          ),
        ],
      ),
    );
  }
}

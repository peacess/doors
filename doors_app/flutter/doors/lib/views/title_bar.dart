import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';

class TitleBar extends StatefulWidget implements PreferredSizeWidget {
  TitleBar({super.key});
  final ValueNotifier<String> title = DoorsApp.app.title;
  final ValueNotifier<String> subTitle = DoorsApp.app.subTitle;

  @override
  State<TitleBar> createState() => _TitleBarState();

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}

class _TitleBarState extends State<TitleBar> {
  @override
  Widget build(BuildContext context) {
    return AppBar(
      title: Row(
        children: [
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

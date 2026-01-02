import 'package:doors/views/doors_app.dart';
import 'package:flutter/material.dart';
import 'package:idl/kits/platform.dart';
import 'package:window_manager/window_manager.dart';

class MainTitleBar extends StatefulWidget implements PreferredSizeWidget {
  MainTitleBar({super.key});
  final ValueNotifier<String> title = DoorsApp.app.title;
  final ValueNotifier<String> subTitle = DoorsApp.app.subTitle;

  @override
  State<MainTitleBar> createState() => _MainTitleBarState();

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}

class _MainTitleBarState extends State<MainTitleBar> {
  @override
  Widget build(BuildContext context) {
    return PlatformEx.isDesktop
        ? GestureDetector(
            onDoubleTap: () async {
              bool isMaximized = await windowManager.isMaximized();
              if (isMaximized) {
                await windowManager.unmaximize();
              } else {
                await windowManager.maximize();
              }
            },
            child: DragToMoveArea(child: makeAppBar()),
          )
        : makeAppBar();
  }

  AppBar makeAppBar() {
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
      actions: PlatformEx.isDesktop
          ? [
              IconButton(
                icon: const Icon(Icons.minimize),
                onPressed: () {
                  windowManager.minimize();
                },
              ),
              IconButton(
                icon: const Icon(Icons.crop_square),
                onPressed: () async {
                  bool isMaximized = await windowManager.isMaximized();
                  if (isMaximized) {
                    await windowManager.unmaximize();
                  } else {
                    await windowManager.maximize();
                  }
                },
              ),
              IconButton(
                icon: const Icon(Icons.close),
                onPressed: () {
                  windowManager.close();
                },
              ),
            ]
          : null,
    );
  }
}

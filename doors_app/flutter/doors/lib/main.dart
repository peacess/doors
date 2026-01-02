import 'package:doors/main_app.dart';
import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';
import 'package:idl/kits/platform.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  if (PlatformEx.isDesktop) {
    await windowManager.ensureInitialized();
    WindowOptions windowOptions = const WindowOptions(
      center: true,
      backgroundColor: Colors.transparent,
      skipTaskbar: false,
      titleBarStyle: TitleBarStyle.hidden,
    );

    windowManager.waitUntilReadyToShow(windowOptions, () async {
      await windowManager.show();
      await windowManager.focus();
      await windowManager.setResizable(true);
    });
  }

  runApp(DoorsMainApp());
}

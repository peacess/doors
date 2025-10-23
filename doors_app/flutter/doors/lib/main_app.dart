import 'package:doors/data/libex/chat_callback.dart';
import 'package:doors/data/libex/net_discovery_callback.dart';
import 'package:doors/l10n/gen_code/app_localizations.dart';
import 'package:doors/views/doors_app.dart';
import 'package:doors/views/home_page.dart';
import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:idl/idl.dart';

class DoorsMainApp extends StatefulWidget {
  const DoorsMainApp({super.key});

  @override
  State<DoorsMainApp> createState() => _DoorsMainApp();
}

class _DoorsMainApp extends State<DoorsMainApp> {
  final ValueNotifier<ThemeMode> themeMode = DoorsApp.app.themeMode;

  @override
  void initState() {
    super.initState();
    themeMode.addListener(_refresh);
    ffiRpc.init(netDiscoveryCallback: AppNetDiscoveryCallback(DoorsApp.app.partners), chatCallback: AppChatCallback(DoorsApp.app.partners));
  }

  @override
  void didChangeDependencies() {
    DoorsApp.app.mediaQueryData = MediaQuery.of(context);
    super.didChangeDependencies();
  }

  @override
  void dispose() {
    themeMode.removeListener(_refresh);
    super.dispose();
  }

  void _refresh() {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorKey: DoorsApp.app.navigatorKey,
      title: 'Doors',
      theme: ThemeData(
        // colorScheme: ColorScheme.fromSeed(seedColor: Colors.black),
        colorScheme: ColorScheme.light(),
        useMaterial3: true,
        textButtonTheme: TextButtonThemeData(
          style: TextButton.styleFrom(shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero)),
        ),
        iconButtonTheme: IconButtonThemeData(
          style: IconButton.styleFrom(shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero)),
        ),
      ),
      darkTheme: ThemeData(
        // colorScheme: ColorScheme.fromSeed(seedColor: Colors.black),
        colorScheme: ColorScheme.dark(),
        useMaterial3: true,
        textButtonTheme: TextButtonThemeData(
          style: TextButton.styleFrom(shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero)),
        ),
        iconButtonTheme: IconButtonThemeData(
          style: IconButton.styleFrom(shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero)),
        ),
      ),

      themeMode: themeMode.value,

      localizationsDelegates: [
        AppLocalizations.delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      // 支持的语言列表
      supportedLocales: [
        const Locale('en', ''), // English
      ],

      // 指定默认语言（可选）
      // locale: const Locale('zh', ''),
      home: HomePage(),
    );
  }
}

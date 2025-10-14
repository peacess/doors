import 'package:flutter/material.dart';

import 'package:idl/idl.dart' as idl;
import 'package:idl/idl/base_base_generated.dart';
import 'package:idl/idl/net_discovery_net_discovery_generated.dart';
import 'package:logger/logger.dart';

final logger = Logger();
void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late int sumResult;

  @override
  void initState() {
    super.initState();
    sumResult = 0;
    idl.ffiRpc.init(netDiscoveryCallback: Net());
  }

  @override
  Widget build(BuildContext context) {
    const textStyle = TextStyle(fontSize: 25);
    const spacerSmall = SizedBox(height: 10);
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('Native Packages')),
        body: SingleChildScrollView(
          child: Container(
            padding: const EdgeInsets.all(10),
            child: Column(
              children: [
                const Text(
                  'This calls a native function through FFI that is shipped as source in the package. '
                  'The native code is built as part of the Flutter Runner build.',
                  style: textStyle,
                  textAlign: TextAlign.center,
                ),
                spacerSmall,
                Text('sum(1, 2) = $sumResult', style: textStyle, textAlign: TextAlign.center),
                spacerSmall,
                // FutureBuilder<int>(
                //   future: sumAsyncResult,
                //   builder: (BuildContext context, AsyncSnapshot<int> value) {
                //     final displayValue =
                //         (value.hasData) ? value.data : 'loading';
                //     return Text(
                //       'await sumAsync(3, 4) = $displayValue',
                //       style: textStyle,
                //       textAlign: TextAlign.center,
                //     );
                //   },
                // ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

final class Net extends idl.NetDiscoveryCallback {
  @override
  void hiRecv(Header header, HiRecv hi) {
    logger.i("call back data");
  }
}

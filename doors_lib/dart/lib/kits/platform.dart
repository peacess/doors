import 'dart:io';

import 'package:flutter/foundation.dart';

abstract class PlatformEx {
  static get isDesktop => !kIsWeb && (Platform.isWindows || Platform.isLinux || Platform.isMacOS);
}

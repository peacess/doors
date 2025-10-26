import 'dart:io';

abstract class Environment {
  static bool get isRunningTest => Platform.environment.containsKey('FLUTTER_TEST') || Platform.environment.containsKey('TEST');
}

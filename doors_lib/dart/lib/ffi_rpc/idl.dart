import 'dart:ffi';
import 'dart:io';

import '../idl_bindings_generated.dart';
import '../kits/environment.dart';

const String _libName = 'ffi_rpc';

/// The dynamic library in which the symbols for [IdlBindings] can be found.
final DynamicLibrary dylib = () {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.open('$_libName.framework/$_libName');
  }
  if (Platform.isAndroid || Platform.isLinux) {
    if (Environment.isRunningTest) {
      return DynamicLibrary.open('linux/lib$_libName.so');
    } else {
      return DynamicLibrary.open('lib$_libName.so');
    }
  }
  if (Platform.isWindows) {
    return DynamicLibrary.open('$_libName.dll');
  }
  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

final IdlBindings idlBindings = IdlBindings(dylib);

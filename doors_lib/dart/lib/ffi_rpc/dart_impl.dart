import 'dart:ffi';

import '../idl_bindings_generated.dart';
import './idl.dart';

class FfiRpcDart {
  final IdlBindings _idlBindings;
  FfiRpcDart(this._idlBindings);

  void init() {
    var re = _idlBindings.init();
    _idlBindings.bytes_free(re);
  }

  void unInit() {
    var re = _idlBindings.un_init();
    _idlBindings.bytes_free(re);
  }

  Bytes call(int methodId, Pointer<Bytes> inParameter) {
    return _idlBindings.call(methodId, inParameter);
  }

  void setCallback(DartCallBackFunction callback) {
    final nativeCallable = NativeCallable<CallBackFunction>.listener(callback);
    _idlBindings.set_call_back(nativeCallable.nativeFunction);
  }
}

final ffiRpc = FfiRpcDart(idlBindings);

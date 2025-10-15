import 'dart:ffi';
import 'package:ffi/ffi.dart' as ffi;
import 'package:idl/ffi_rpc/chat/callback.dart';
import 'package:logger/logger.dart';

import '../idl/base_base_generated.dart';
import '../idl/net_discovery_net_discovery_generated.dart';
import '../idl_bindings_generated.dart';
import './net_discovery/hi.dart';
import './idl.dart';
import 'package:flat_buffers/flat_buffers.dart' as fb;

final logger = Logger();

class FfiRpcDart {
  final IdlBindings _idlBindings;
  FfiRpcDart(this._idlBindings);
  late NetDiscoveryCallback netDiscoveryCallback;
  late ChatCallback chatCallback;

  void init({NetDiscoveryCallback? netDiscoveryCallback, ChatCallback? chatCallback}) {
    this.netDiscoveryCallback = netDiscoveryCallback ?? NetDiscoveryCallback();
    this.chatCallback = chatCallback ?? ChatCallback();
    final nativeCallable = NativeCallable<CallBackFunction>.listener(callback);
    var re = _idlBindings.init(nativeCallable.nativeFunction);
    _idlBindings.bytes_free(re);
  }

  void unInit() {
    var re = _idlBindings.un_init();
    _idlBindings.bytes_free(re);
  }

  FfiBytes call(FfiBytes inParameter) {
    return _idlBindings.call(inParameter);
  }

  void callback(FfiBytes data) {
    final buffer = data.attach();
    var header = Header.reader.read(buffer, 0);
    try {
      var headType = HeaderType.from(header.headerType);
      switch (headType) {
        case HeaderType.netDiscovery:
          netDiscoveryCallback.callback(buffer, header, Header.reader.size);
          break;
        case HeaderType.chat:
          chatCallback.callback(buffer, header, Header.reader.size);
          break;
      }
    } catch (e) {
      logger.e(e);
    }
  }

  void bytesFree(FfiBytes data) {
    _idlBindings.bytes_free(data);
    data.bytes = nullptr;
    data.len = 0;
    data.capacity = 0;
    data.offset = 0;
  }
}

enum HeaderType {
  netDiscovery(1),
  chat(1);

  final int value;
  const HeaderType(this.value);

  factory HeaderType.from(int headerType) {
    for (var it in HeaderType.values) {
      if (it.value == headerType) {
        return it;
      }
    }
    throw ArgumentError('Unknown header type: $headerType');
  }
}

final ffiRpc = FfiRpcDart(idlBindings);

final Finalizer<FfiBytes> _bytesFinalizer = Finalizer((FfiBytes data) {
  ffiRpc.bytesFree(data);
  data.bytes = nullptr;
});

extension BytesEx on FfiBytes {
  fb.BufferContext attach() {
    final buffer = fb.BufferContext.fromBytes(bytes.asTypedList(len));
    _bytesFinalizer.attach(buffer, this, detach: this);
    return buffer;
  }

  void detach() {
    _bytesFinalizer.detach(this);
    ffiRpc.bytesFree(this);
  }
}

abstract class FfiBytesHelper {
  static Pointer<FfiBytes> from(fb.BufferContext data) {
    var p = ffi.calloc<FfiBytes>();
    p.ref.len = data.buffer.elementSizeInBytes;
    //todo
    return p;
  }
}

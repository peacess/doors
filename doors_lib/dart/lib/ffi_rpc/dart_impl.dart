import 'dart:ffi';
import 'dart:typed_data';

import 'package:flutter/foundation.dart';
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

  void init({NetDiscoveryCallback? netDiscoveryCallback}) {
    this.netDiscoveryCallback = netDiscoveryCallback ?? NetDiscoveryCallback();
    final nativeCallable = NativeCallable<CallBackFunction>.listener(callback);
    var re = _idlBindings.init(nativeCallable.nativeFunction);
    _idlBindings.bytes_free(re);
  }

  void unInit() {
    var re = _idlBindings.un_init();
    _idlBindings.bytes_free(re);
  }

  Bytes call(int methodId, Pointer<Bytes> inParameter) {
    return _idlBindings.call(methodId, inParameter);
  }

  void callback(Bytes data) {
    final buffer = data.attach();
    var header = DiscoveryHeader.reader.read(buffer, 0);
    try {
      var headType = HeaderType.from(header.headerType);
      switch (headType) {
        case HeaderType.netDiscovery:
          netDiscoveryCallback.callback(buffer, header, DiscoveryHeader.reader.size);
          break;
      }
    } catch (e) {
      logger.e(e);
    }
  }

  void bytesFree(Bytes data) {
    _idlBindings.bytes_free(data);
  }
}

enum HeaderType {
  netDiscovery(1);

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

final Finalizer<Bytes> _bytesFinalizer = Finalizer((Bytes data) {
  ffiRpc.bytesFree(data);
  data.bytes = nullptr;
});

extension BytesEx on Bytes {
  fb.BufferContext attach() {
    final buffer = fb.BufferContext.fromBytes(bytes.asTypedList(len));
    _bytesFinalizer.attach(buffer, this, detach: this);
    return buffer;
  }

  void detach() {
    _bytesFinalizer.detach(this);
    ffiRpc.bytesFree(this);
    bytes = nullptr;
  }
}

import 'dart:ffi';
import 'package:ffi/ffi.dart' as ffi;
import 'package:idl/ffi_rpc/chat/callback.dart';
import 'package:idl/ffi_rpc/header_type.dart';
import 'package:logger/logger.dart';

import '../idl/base_base_generated.dart';
import '../idl_bindings_generated.dart';
import './idl.dart';
import 'package:flat_buffers/flat_buffers.dart' as fb;

import 'net_discovery/callback.dart';
import 'chat/chat.dart';

final logger = Logger();

class FfiRpcDart {
  final IdlBindings _idlBindings;
  FfiRpcDart(this._idlBindings);
  late final NetDiscoveryCallback netDiscoveryCallback;
  late final ChatCallback chatCallback;
  late final Chat chat;

  void init({required NetDiscoveryCallback netDiscoveryCallback, ChatCallback? chatCallback, Chat? chat}) {
    this.netDiscoveryCallback = netDiscoveryCallback;
    this.chatCallback = chatCallback ?? ChatCallback();
    this.chat = chat ?? Chat();
    final nativeCallable = NativeCallable<CallBackFunction>.listener(callback);
    var re = _idlBindings.init(nativeCallable.nativeFunction);
    _idlBindings.bytes_free(re);
  }

  void unInit() {
    var re = _idlBindings.un_init();
    _idlBindings.bytes_free(re);
  }

  // @Native<FfiBytes Function(Pointer<Uint8>, Uint64)>(symbol: 'call',isLeaf: true)
  // external FfiBytes call(Pointer<Uint8> bytes, int length);
  late final call = dylib.lookupFunction<FfiBytes Function(Pointer<Uint8>, Uint64), FfiBytes Function(Pointer<Uint8>, int)>('call', isLeaf: true);

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

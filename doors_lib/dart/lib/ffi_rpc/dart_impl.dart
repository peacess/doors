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

final _logger = Logger();

class FfiRpcDart {
  final IdlBindings _idlBindings;
  FfiRpcDart(this._idlBindings);
  late final NetDiscoveryCallback netDiscoveryCallback;
  late final ChatCallback chatCallback;
  late final Chat chat;

  void init({required NetDiscoveryCallback netDiscoveryCallback, required ChatCallback chatCallback, Chat? chat}) {
    this.netDiscoveryCallback = netDiscoveryCallback;
    this.chatCallback = chatCallback;
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
    // _logger.d("callback data:　${data.describe()}");
    final buffer = data.attach();
    var header = Frame.reader.read(buffer, 0).header!;
    try {
      var headType = HeaderType.from(header.headerType);
      switch (headType) {
        case HeaderType.netDiscovery:
          netDiscoveryCallback.callback(buffer, header);
          break;
        case HeaderType.chat:
          chatCallback.callback(buffer, header);
          break;
      }
    } catch (e) {
      _logger.e(e);
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
    _bytesFinalizer.attach(buffer, this, detach: FfiBytesT(bytes, len));
    return buffer;
  }

  void detach() {
    _bytesFinalizer.detach(FfiBytesT(bytes, len));
    ffiRpc.bytesFree(this);
  }

  String describe() {
    final tem = bytes.asTypedList(len);
    return 'FfiBytes(len: $len, capacity: $capacity, offset: $offset, bytes: $tem)';
  }
}

class FfiBytesT {
  Pointer<Uint8> bytes;
  int len;
  int capacity;
  int offset;
  FfiBytesT(this.bytes, this.len, {int? capacity, int? offset}) : capacity = capacity ?? len, offset = offset ?? len;

  FfiBytes to() {
    return Struct.create()
      ..len = len
      ..offset = offset
      ..capacity = capacity
      ..bytes = bytes;
  }

  @override
  bool operator ==(Object other) {
    // 如果引用相同，直接相等
    if (identical(this, other)) return true;

    // 判断类型和字段是否相同
    return other is FfiBytesT && other.bytes == bytes;
  }

  @override
  int get hashCode => bytes.hashCode;
}

abstract class FfiBytesHelper {
  static Pointer<FfiBytes> from(fb.BufferContext data) {
    var p = ffi.calloc<FfiBytes>();
    p.ref.len = data.buffer.elementSizeInBytes;
    //todo
    return p;
  }
}

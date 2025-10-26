import 'dart:ffi';
import 'dart:typed_data';
import 'package:ffi/ffi.dart' as ffi;
import 'package:idl/ffi_rpc/chat/chat_callback.dart';
import 'package:logger/logger.dart';

import '../idl/base_base_generated.dart';
import '../idl/net_data_type_net_data_type_generated.dart';
import '../idl_bindings_generated.dart';
import './idl.dart';
import 'package:flat_buffers/flat_buffers.dart' as fb;

import 'net_discovery/hi.dart';

final _logger = Logger();

class FfiRpcDart {
  final IdlBindings _idlBindings;
  FfiRpcDart(this._idlBindings);
  late final NetDiscoveryCallback netDiscoveryCallback;
  late final ChatCallback chatCallback;

  void init({required NetDiscoveryCallback netDiscoveryCallback, required ChatCallback chatCallback}) {
    this.netDiscoveryCallback = netDiscoveryCallback;
    this.chatCallback = chatCallback;
    final nativeCallable = NativeCallable<CallBackFunction>.listener(_callback);
    var re = _idlBindings.init(nativeCallable.nativeFunction);
    _idlBindings.bytes_free(re);
  }

  void unInit() {
    var re = _idlBindings.un_init();
    _idlBindings.bytes_free(re);
  }

  void _callback(FfiBytes data) {
    // _logger.d("callback data:　${data.describe()}");
    final buffer = data.attach();
    var header = Frame.reader.read(buffer, 0).header!;
    try {
      var headType = HeaderType.fromValue(header.headerType);
      switch (headType) {
        case HeaderType.net_discovery:
          netDiscoveryCallback.callback(buffer, header);
          break;
        case HeaderType.chat:
          chatCallback.callback(buffer, header);
          break;
        case HeaderType.none:
          _logger.e("the header type is none(0)");
          break;
        case HeaderType.ffi_rpc:
          // TODO: Handle this case.
          throw UnimplementedError();
      }
    } catch (e) {
      _logger.e(e);
    }
  }

  late final call = _idlBindings.call;

  void bytesFree(FfiBytes data) {
    _idlBindings.bytes_free(data);
    data.bytes = nullptr;
    data.len = 0;
    data.capacity = 0;
    data.offset = 0;
  }

  Uint8List generateUlid() {
    final p = ffi.calloc<Uint8>(16);
    _idlBindings.generate_ulid(p);
    var d = p.asTypedList(16).sublist(0);
    ffi.calloc.free(p);
    return d;
  }

  Uint8List generateUuidV7() {
    final p = ffi.calloc<Uint8>(16);
    _idlBindings.generate_uuid_v7(p);
    var d = p.asTypedList(16).sublist(0);
    ffi.calloc.free(p);
    return d;
  }
}

// @Native<Void Function(Pointer<Uint8>)>(symbol: 'generate_ulid', isLeaf: true)
// external void _generateUlid(Pointer<Uint8> bytes);
//
// @Native<Void Function(Pointer<Uint8>)>(symbol: 'generate_uuid_v7', isLeaf: true)
// external void _generateUuidV7(Pointer<Uint8> bytes);

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

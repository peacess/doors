// // hook/build.dart
//
// import 'dart:io';
// import 'package:hooks/hooks.dart';
// import 'package:code_assets/code_assets.dart'; // 导入 NativeCodeAsset 所在的包
//
// void main(List<String> args) async {
//   await build(args, (input, output) async {
//     // 1. 获取目标操作系统和架构
//     final os = input.config.code.targetOS;
//     final arch = input.config.code.targetArchitecture;
//
//     // 2. 确定库的名称和文件后缀
//     final libraryName = 'ffi_rpc';
//     final assetId = 'package:idl/$libraryName'; // 用于 Dart 代码中 @Native 的 ID
//
//     String libraryFileName;
//     String libraryPath;
//
//     // 3. 根据目标平台选择正确的预构建库文件
//     switch (os) {
//       case OS.android:
//       // Android 库通常在特定架构的文件夹中，并使用 .so 后缀
//         libraryFileName = 'lib$libraryName.so';
//         libraryPath = 'android/$arch/$libraryFileName';
//         break;
//       case OS.iOS:
//       // iOS/macOS 的动态库（通常是 .dylib 或 Framework）
//         libraryFileName = 'lib$libraryName.dylib';
//         libraryPath = 'ios/$libraryFileName';
//         // 注意：iOS/macOS 上的动态库处理可能更复杂，涉及 Frameworks 或安装名称（install_name）
//         break;
//       case OS.windows:
//       // Windows 库使用 .dll
//         libraryFileName = '$libraryName.dll';
//         libraryPath = 'windows/$libraryFileName';
//         break;
//       case OS.linux:
//       // Linux 库使用 .so
//         libraryFileName = 'lib$libraryName.so';
//         libraryPath = 'linux/$libraryFileName';
//         break;
//     // ... 处理其他平台
//       default:
//       // 对于不支持的平台，可以不生成资产
//         return;
//     }
//
//     // 检查文件是否存在
//     final absolutePath = input.packageRoot.resolve(libraryPath).toFilePath();
//     // if (!await File(absolutePath).exists()) {
//     //   // 如果找不到预构建库，可能需要抛出错误或跳过
//     //   throw Exception('Prebuilt library not found for $os/$arch at $libraryPath');
//     // }
//
//     final libraryUri = Uri.file(libraryPath);
//     // 4. 将预构建库声明为 NativeCodeAsset
//     output.assets.code.add(CodeAsset(package: input.packageName,
//       name: '',
//       linkMode: DynamicLoadingBundled(),
//       file: libraryUri,
//     ));
//
//     // 可以添加一个 DataAsset 来确保文件也被打包，尽管对于 CodeAsset，构建系统通常会处理打包。
//     // output.data.add(DataAsset(...));
//   });
// }

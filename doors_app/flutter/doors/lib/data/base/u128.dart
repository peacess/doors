import 'package:isar_community/isar.dart';

part 'u128.g.dart';

@embedded
class U128 {
  static final zero = U128();
  int low = 0;
  int hight = 0;
}

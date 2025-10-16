import 'package:doors/data/base/u128.dart';
import 'package:isar_community/isar.dart';

part 'partner.g.dart';

@collection
class Partner {
  // @Id()
  Id id = Isar.autoIncrement;
  U128 partnerId = U128.zero;
  String name = "";
}

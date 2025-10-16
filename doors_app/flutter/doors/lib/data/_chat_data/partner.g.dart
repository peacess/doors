// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'partner.dart';

// **************************************************************************
// IsarCollectionGenerator
// **************************************************************************

// coverage:ignore-file
// ignore_for_file: duplicate_ignore, non_constant_identifier_names, constant_identifier_names, invalid_use_of_protected_member, unnecessary_cast, prefer_const_constructors, lines_longer_than_80_chars, require_trailing_commas, inference_failure_on_function_invocation, unnecessary_parenthesis, unnecessary_raw_strings, unnecessary_null_checks, join_return_with_assignment, prefer_final_locals, avoid_js_rounded_ints, avoid_positional_boolean_parameters, always_specify_types

extension GetPartnerCollection on Isar {
  IsarCollection<Partner> get partners => this.collection();
}

const PartnerSchema = CollectionSchema(
  name: r'Partner',
  id: -1675268894051260210,
  properties: {
    r'name': PropertySchema(id: 0, name: r'name', type: IsarType.string),
    r'partnerId': PropertySchema(id: 1, name: r'partnerId', type: IsarType.object, target: r'U128'),
  },

  estimateSize: _partnerEstimateSize,
  serialize: _partnerSerialize,
  deserialize: _partnerDeserialize,
  deserializeProp: _partnerDeserializeProp,
  idName: r'id',
  indexes: {},
  links: {},
  embeddedSchemas: {r'U128': U128Schema},

  getId: _partnerGetId,
  getLinks: _partnerGetLinks,
  attach: _partnerAttach,
  version: '3.3.0-dev.3',
);

int _partnerEstimateSize(Partner object, List<int> offsets, Map<Type, List<int>> allOffsets) {
  var bytesCount = offsets.last;
  bytesCount += 3 + object.name.length * 3;
  bytesCount += 3 + U128Schema.estimateSize(object.partnerId, allOffsets[U128]!, allOffsets);
  return bytesCount;
}

void _partnerSerialize(Partner object, IsarWriter writer, List<int> offsets, Map<Type, List<int>> allOffsets) {
  writer.writeString(offsets[0], object.name);
  writer.writeObject<U128>(offsets[1], allOffsets, U128Schema.serialize, object.partnerId);
}

Partner _partnerDeserialize(Id id, IsarReader reader, List<int> offsets, Map<Type, List<int>> allOffsets) {
  final object = Partner();
  object.id = id;
  object.name = reader.readString(offsets[0]);
  object.partnerId = reader.readObjectOrNull<U128>(offsets[1], U128Schema.deserialize, allOffsets) ?? U128();
  return object;
}

P _partnerDeserializeProp<P>(IsarReader reader, int propertyId, int offset, Map<Type, List<int>> allOffsets) {
  switch (propertyId) {
    case 0:
      return (reader.readString(offset)) as P;
    case 1:
      return (reader.readObjectOrNull<U128>(offset, U128Schema.deserialize, allOffsets) ?? U128()) as P;
    default:
      throw IsarError('Unknown property with id $propertyId');
  }
}

Id _partnerGetId(Partner object) {
  return object.id;
}

List<IsarLinkBase<dynamic>> _partnerGetLinks(Partner object) {
  return [];
}

void _partnerAttach(IsarCollection<dynamic> col, Id id, Partner object) {
  object.id = id;
}

extension PartnerQueryWhereSort on QueryBuilder<Partner, Partner, QWhere> {
  QueryBuilder<Partner, Partner, QAfterWhere> anyId() {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(const IdWhereClause.any());
    });
  }
}

extension PartnerQueryWhere on QueryBuilder<Partner, Partner, QWhereClause> {
  QueryBuilder<Partner, Partner, QAfterWhereClause> idEqualTo(Id id) {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(IdWhereClause.between(lower: id, upper: id));
    });
  }

  QueryBuilder<Partner, Partner, QAfterWhereClause> idNotEqualTo(Id id) {
    return QueryBuilder.apply(this, (query) {
      if (query.whereSort == Sort.asc) {
        return query
            .addWhereClause(IdWhereClause.lessThan(upper: id, includeUpper: false))
            .addWhereClause(IdWhereClause.greaterThan(lower: id, includeLower: false));
      } else {
        return query
            .addWhereClause(IdWhereClause.greaterThan(lower: id, includeLower: false))
            .addWhereClause(IdWhereClause.lessThan(upper: id, includeUpper: false));
      }
    });
  }

  QueryBuilder<Partner, Partner, QAfterWhereClause> idGreaterThan(Id id, {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(IdWhereClause.greaterThan(lower: id, includeLower: include));
    });
  }

  QueryBuilder<Partner, Partner, QAfterWhereClause> idLessThan(Id id, {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(IdWhereClause.lessThan(upper: id, includeUpper: include));
    });
  }

  QueryBuilder<Partner, Partner, QAfterWhereClause> idBetween(Id lowerId, Id upperId, {bool includeLower = true, bool includeUpper = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(IdWhereClause.between(lower: lowerId, includeLower: includeLower, upper: upperId, includeUpper: includeUpper));
    });
  }
}

extension PartnerQueryFilter on QueryBuilder<Partner, Partner, QFilterCondition> {
  QueryBuilder<Partner, Partner, QAfterFilterCondition> idEqualTo(Id value) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.equalTo(property: r'id', value: value));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> idGreaterThan(Id value, {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.greaterThan(include: include, property: r'id', value: value));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> idLessThan(Id value, {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.lessThan(include: include, property: r'id', value: value));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> idBetween(Id lower, Id upper, {bool includeLower = true, bool includeUpper = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        FilterCondition.between(property: r'id', lower: lower, includeLower: includeLower, upper: upper, includeUpper: includeUpper),
      );
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameEqualTo(String value, {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.equalTo(property: r'name', value: value, caseSensitive: caseSensitive));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameGreaterThan(String value, {bool include = false, bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.greaterThan(include: include, property: r'name', value: value, caseSensitive: caseSensitive));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameLessThan(String value, {bool include = false, bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.lessThan(include: include, property: r'name', value: value, caseSensitive: caseSensitive));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameBetween(
    String lower,
    String upper, {
    bool includeLower = true,
    bool includeUpper = true,
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        FilterCondition.between(
          property: r'name',
          lower: lower,
          includeLower: includeLower,
          upper: upper,
          includeUpper: includeUpper,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameStartsWith(String value, {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.startsWith(property: r'name', value: value, caseSensitive: caseSensitive));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameEndsWith(String value, {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.endsWith(property: r'name', value: value, caseSensitive: caseSensitive));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameContains(String value, {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.contains(property: r'name', value: value, caseSensitive: caseSensitive));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameMatches(String pattern, {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.matches(property: r'name', wildcard: pattern, caseSensitive: caseSensitive));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameIsEmpty() {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.equalTo(property: r'name', value: ''));
    });
  }

  QueryBuilder<Partner, Partner, QAfterFilterCondition> nameIsNotEmpty() {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.greaterThan(property: r'name', value: ''));
    });
  }
}

extension PartnerQueryObject on QueryBuilder<Partner, Partner, QFilterCondition> {
  QueryBuilder<Partner, Partner, QAfterFilterCondition> partnerId(FilterQuery<U128> q) {
    return QueryBuilder.apply(this, (query) {
      return query.object(q, r'partnerId');
    });
  }
}

extension PartnerQueryLinks on QueryBuilder<Partner, Partner, QFilterCondition> {}

extension PartnerQuerySortBy on QueryBuilder<Partner, Partner, QSortBy> {
  QueryBuilder<Partner, Partner, QAfterSortBy> sortByName() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(r'name', Sort.asc);
    });
  }

  QueryBuilder<Partner, Partner, QAfterSortBy> sortByNameDesc() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(r'name', Sort.desc);
    });
  }
}

extension PartnerQuerySortThenBy on QueryBuilder<Partner, Partner, QSortThenBy> {
  QueryBuilder<Partner, Partner, QAfterSortBy> thenById() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(r'id', Sort.asc);
    });
  }

  QueryBuilder<Partner, Partner, QAfterSortBy> thenByIdDesc() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(r'id', Sort.desc);
    });
  }

  QueryBuilder<Partner, Partner, QAfterSortBy> thenByName() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(r'name', Sort.asc);
    });
  }

  QueryBuilder<Partner, Partner, QAfterSortBy> thenByNameDesc() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(r'name', Sort.desc);
    });
  }
}

extension PartnerQueryWhereDistinct on QueryBuilder<Partner, Partner, QDistinct> {
  QueryBuilder<Partner, Partner, QDistinct> distinctByName({bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addDistinctBy(r'name', caseSensitive: caseSensitive);
    });
  }
}

extension PartnerQueryProperty on QueryBuilder<Partner, Partner, QQueryProperty> {
  QueryBuilder<Partner, int, QQueryOperations> idProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addPropertyName(r'id');
    });
  }

  QueryBuilder<Partner, String, QQueryOperations> nameProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addPropertyName(r'name');
    });
  }

  QueryBuilder<Partner, U128, QQueryOperations> partnerIdProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addPropertyName(r'partnerId');
    });
  }
}

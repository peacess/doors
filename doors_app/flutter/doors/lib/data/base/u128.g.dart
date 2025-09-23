// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'u128.dart';

// **************************************************************************
// IsarEmbeddedGenerator
// **************************************************************************

// coverage:ignore-file
// ignore_for_file: duplicate_ignore, non_constant_identifier_names, constant_identifier_names, invalid_use_of_protected_member, unnecessary_cast, prefer_const_constructors, lines_longer_than_80_chars, require_trailing_commas, inference_failure_on_function_invocation, unnecessary_parenthesis, unnecessary_raw_strings, unnecessary_null_checks, join_return_with_assignment, prefer_final_locals, avoid_js_rounded_ints, avoid_positional_boolean_parameters, always_specify_types

const U128Schema = Schema(
  name: r'U128',
  id: -2897760067270826786,
  properties: {
    r'hight': PropertySchema(id: 0, name: r'hight', type: IsarType.long),
    r'low': PropertySchema(id: 1, name: r'low', type: IsarType.long),
  },

  estimateSize: _u128EstimateSize,
  serialize: _u128Serialize,
  deserialize: _u128Deserialize,
  deserializeProp: _u128DeserializeProp,
);

int _u128EstimateSize(U128 object, List<int> offsets, Map<Type, List<int>> allOffsets) {
  var bytesCount = offsets.last;
  return bytesCount;
}

void _u128Serialize(U128 object, IsarWriter writer, List<int> offsets, Map<Type, List<int>> allOffsets) {
  writer.writeLong(offsets[0], object.hight);
  writer.writeLong(offsets[1], object.low);
}

U128 _u128Deserialize(Id id, IsarReader reader, List<int> offsets, Map<Type, List<int>> allOffsets) {
  final object = U128();
  object.hight = reader.readLong(offsets[0]);
  object.low = reader.readLong(offsets[1]);
  return object;
}

P _u128DeserializeProp<P>(IsarReader reader, int propertyId, int offset, Map<Type, List<int>> allOffsets) {
  switch (propertyId) {
    case 0:
      return (reader.readLong(offset)) as P;
    case 1:
      return (reader.readLong(offset)) as P;
    default:
      throw IsarError('Unknown property with id $propertyId');
  }
}

extension U128QueryFilter on QueryBuilder<U128, U128, QFilterCondition> {
  QueryBuilder<U128, U128, QAfterFilterCondition> hightEqualTo(int value) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.equalTo(property: r'hight', value: value));
    });
  }

  QueryBuilder<U128, U128, QAfterFilterCondition> hightGreaterThan(int value, {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.greaterThan(include: include, property: r'hight', value: value));
    });
  }

  QueryBuilder<U128, U128, QAfterFilterCondition> hightLessThan(int value, {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.lessThan(include: include, property: r'hight', value: value));
    });
  }

  QueryBuilder<U128, U128, QAfterFilterCondition> hightBetween(int lower, int upper, {bool includeLower = true, bool includeUpper = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        FilterCondition.between(property: r'hight', lower: lower, includeLower: includeLower, upper: upper, includeUpper: includeUpper),
      );
    });
  }

  QueryBuilder<U128, U128, QAfterFilterCondition> lowEqualTo(int value) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.equalTo(property: r'low', value: value));
    });
  }

  QueryBuilder<U128, U128, QAfterFilterCondition> lowGreaterThan(int value, {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.greaterThan(include: include, property: r'low', value: value));
    });
  }

  QueryBuilder<U128, U128, QAfterFilterCondition> lowLessThan(int value, {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.lessThan(include: include, property: r'low', value: value));
    });
  }

  QueryBuilder<U128, U128, QAfterFilterCondition> lowBetween(int lower, int upper, {bool includeLower = true, bool includeUpper = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        FilterCondition.between(property: r'low', lower: lower, includeLower: includeLower, upper: upper, includeUpper: includeUpper),
      );
    });
  }
}

extension U128QueryObject on QueryBuilder<U128, U128, QFilterCondition> {}

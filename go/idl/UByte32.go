// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package idl

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type UByte32 struct {
	_tab flatbuffers.Struct
}

func (rcv *UByte32) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *UByte32) Table() flatbuffers.Table {
	return rcv._tab.Table
}

func (rcv *UByte32) Ub1(obj *UByte8) *UByte8 {
	if obj == nil {
		obj = new(UByte8)
	}
	obj.Init(rcv._tab.Bytes, rcv._tab.Pos+0)
	return obj
}
func (rcv *UByte32) Ub2(obj *UByte8) *UByte8 {
	if obj == nil {
		obj = new(UByte8)
	}
	obj.Init(rcv._tab.Bytes, rcv._tab.Pos+8)
	return obj
}
func (rcv *UByte32) Ub3(obj *UByte8) *UByte8 {
	if obj == nil {
		obj = new(UByte8)
	}
	obj.Init(rcv._tab.Bytes, rcv._tab.Pos+16)
	return obj
}
func (rcv *UByte32) Ub4(obj *UByte8) *UByte8 {
	if obj == nil {
		obj = new(UByte8)
	}
	obj.Init(rcv._tab.Bytes, rcv._tab.Pos+24)
	return obj
}

func CreateUByte32(builder *flatbuffers.Builder, ub1_ub1 byte, ub1_ub2 byte, ub1_ub3 byte, ub1_ub4 byte, ub1_ub5 byte, ub1_ub6 byte, ub1_ub7 byte, ub1_ub8 byte, ub2_ub1 byte, ub2_ub2 byte, ub2_ub3 byte, ub2_ub4 byte, ub2_ub5 byte, ub2_ub6 byte, ub2_ub7 byte, ub2_ub8 byte, ub3_ub1 byte, ub3_ub2 byte, ub3_ub3 byte, ub3_ub4 byte, ub3_ub5 byte, ub3_ub6 byte, ub3_ub7 byte, ub3_ub8 byte, ub4_ub1 byte, ub4_ub2 byte, ub4_ub3 byte, ub4_ub4 byte, ub4_ub5 byte, ub4_ub6 byte, ub4_ub7 byte, ub4_ub8 byte) flatbuffers.UOffsetT {
	builder.Prep(1, 32)
	builder.Prep(1, 8)
	builder.PrependByte(ub4_ub8)
	builder.PrependByte(ub4_ub7)
	builder.PrependByte(ub4_ub6)
	builder.PrependByte(ub4_ub5)
	builder.PrependByte(ub4_ub4)
	builder.PrependByte(ub4_ub3)
	builder.PrependByte(ub4_ub2)
	builder.PrependByte(ub4_ub1)
	builder.Prep(1, 8)
	builder.PrependByte(ub3_ub8)
	builder.PrependByte(ub3_ub7)
	builder.PrependByte(ub3_ub6)
	builder.PrependByte(ub3_ub5)
	builder.PrependByte(ub3_ub4)
	builder.PrependByte(ub3_ub3)
	builder.PrependByte(ub3_ub2)
	builder.PrependByte(ub3_ub1)
	builder.Prep(1, 8)
	builder.PrependByte(ub2_ub8)
	builder.PrependByte(ub2_ub7)
	builder.PrependByte(ub2_ub6)
	builder.PrependByte(ub2_ub5)
	builder.PrependByte(ub2_ub4)
	builder.PrependByte(ub2_ub3)
	builder.PrependByte(ub2_ub2)
	builder.PrependByte(ub2_ub1)
	builder.Prep(1, 8)
	builder.PrependByte(ub1_ub8)
	builder.PrependByte(ub1_ub7)
	builder.PrependByte(ub1_ub6)
	builder.PrependByte(ub1_ub5)
	builder.PrependByte(ub1_ub4)
	builder.PrependByte(ub1_ub3)
	builder.PrependByte(ub1_ub2)
	builder.PrependByte(ub1_ub1)
	return builder.Offset()
}

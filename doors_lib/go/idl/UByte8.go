// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package idl

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type UByte8 struct {
	_tab flatbuffers.Struct
}

func (rcv *UByte8) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *UByte8) Table() flatbuffers.Table {
	return rcv._tab.Table
}

func (rcv *UByte8) Ub1() byte {
	return rcv._tab.GetByte(rcv._tab.Pos + flatbuffers.UOffsetT(0))
}
func (rcv *UByte8) MutateUb1(n byte) bool {
	return rcv._tab.MutateByte(rcv._tab.Pos+flatbuffers.UOffsetT(0), n)
}

func (rcv *UByte8) Ub2() byte {
	return rcv._tab.GetByte(rcv._tab.Pos + flatbuffers.UOffsetT(1))
}
func (rcv *UByte8) MutateUb2(n byte) bool {
	return rcv._tab.MutateByte(rcv._tab.Pos+flatbuffers.UOffsetT(1), n)
}

func (rcv *UByte8) Ub3() byte {
	return rcv._tab.GetByte(rcv._tab.Pos + flatbuffers.UOffsetT(2))
}
func (rcv *UByte8) MutateUb3(n byte) bool {
	return rcv._tab.MutateByte(rcv._tab.Pos+flatbuffers.UOffsetT(2), n)
}

func (rcv *UByte8) Ub4() byte {
	return rcv._tab.GetByte(rcv._tab.Pos + flatbuffers.UOffsetT(3))
}
func (rcv *UByte8) MutateUb4(n byte) bool {
	return rcv._tab.MutateByte(rcv._tab.Pos+flatbuffers.UOffsetT(3), n)
}

func (rcv *UByte8) Ub5() byte {
	return rcv._tab.GetByte(rcv._tab.Pos + flatbuffers.UOffsetT(4))
}
func (rcv *UByte8) MutateUb5(n byte) bool {
	return rcv._tab.MutateByte(rcv._tab.Pos+flatbuffers.UOffsetT(4), n)
}

func (rcv *UByte8) Ub6() byte {
	return rcv._tab.GetByte(rcv._tab.Pos + flatbuffers.UOffsetT(5))
}
func (rcv *UByte8) MutateUb6(n byte) bool {
	return rcv._tab.MutateByte(rcv._tab.Pos+flatbuffers.UOffsetT(5), n)
}

func (rcv *UByte8) Ub7() byte {
	return rcv._tab.GetByte(rcv._tab.Pos + flatbuffers.UOffsetT(6))
}
func (rcv *UByte8) MutateUb7(n byte) bool {
	return rcv._tab.MutateByte(rcv._tab.Pos+flatbuffers.UOffsetT(6), n)
}

func (rcv *UByte8) Ub8() byte {
	return rcv._tab.GetByte(rcv._tab.Pos + flatbuffers.UOffsetT(7))
}
func (rcv *UByte8) MutateUb8(n byte) bool {
	return rcv._tab.MutateByte(rcv._tab.Pos+flatbuffers.UOffsetT(7), n)
}

func CreateUByte8(builder *flatbuffers.Builder, ub1 byte, ub2 byte, ub3 byte, ub4 byte, ub5 byte, ub6 byte, ub7 byte, ub8 byte) flatbuffers.UOffsetT {
	builder.Prep(1, 8)
	builder.PrependByte(ub8)
	builder.PrependByte(ub7)
	builder.PrependByte(ub6)
	builder.PrependByte(ub5)
	builder.PrependByte(ub4)
	builder.PrependByte(ub3)
	builder.PrependByte(ub2)
	builder.PrependByte(ub1)
	return builder.Offset()
}
// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package idl

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type Header struct {
	_tab flatbuffers.Struct
}

func (rcv *Header) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *Header) Table() flatbuffers.Table {
	return rcv._tab.Table
}

func (rcv *Header) Len() int32 {
	return rcv._tab.GetInt32(rcv._tab.Pos + flatbuffers.UOffsetT(0))
}
func (rcv *Header) MutateLen(n int32) bool {
	return rcv._tab.MutateInt32(rcv._tab.Pos+flatbuffers.UOffsetT(0), n)
}

func (rcv *Header) Type() FrameType {
	return FrameType(rcv._tab.GetInt16(rcv._tab.Pos + flatbuffers.UOffsetT(4)))
}
func (rcv *Header) MutateType(n FrameType) bool {
	return rcv._tab.MutateInt16(rcv._tab.Pos+flatbuffers.UOffsetT(4), int16(n))
}

func (rcv *Header) Version() int16 {
	return rcv._tab.GetInt16(rcv._tab.Pos + flatbuffers.UOffsetT(6))
}
func (rcv *Header) MutateVersion(n int16) bool {
	return rcv._tab.MutateInt16(rcv._tab.Pos+flatbuffers.UOffsetT(6), n)
}

func (rcv *Header) ToId(obj *Int128) *Int128 {
	if obj == nil {
		obj = new(Int128)
	}
	obj.Init(rcv._tab.Bytes, rcv._tab.Pos+8)
	return obj
}

func CreateHeader(builder *flatbuffers.Builder, len int32, type_ FrameType, version int16, to_id_i1 int8, to_id_i2 int8, to_id_i3 int8, to_id_i4 int8, to_id_i5 int8, to_id_i6 int8, to_id_i7 int8, to_id_i8 int8) flatbuffers.UOffsetT {
	builder.Prep(4, 16)
	builder.Prep(1, 8)
	builder.PrependInt8(to_id_i8)
	builder.PrependInt8(to_id_i7)
	builder.PrependInt8(to_id_i6)
	builder.PrependInt8(to_id_i5)
	builder.PrependInt8(to_id_i4)
	builder.PrependInt8(to_id_i3)
	builder.PrependInt8(to_id_i2)
	builder.PrependInt8(to_id_i1)
	builder.PrependInt16(version)
	builder.PrependInt16(int16(type_))
	builder.PrependInt32(len)
	return builder.Offset()
}

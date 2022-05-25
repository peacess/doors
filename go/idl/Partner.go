// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package idl

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type Partner struct {
	_tab flatbuffers.Table
}

func GetRootAsPartner(buf []byte, offset flatbuffers.UOffsetT) *Partner {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &Partner{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsPartner(buf []byte, offset flatbuffers.UOffsetT) *Partner {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &Partner{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *Partner) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *Partner) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *Partner) Id(obj *UByte16) *UByte16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		x := o + rcv._tab.Pos
		if obj == nil {
			obj = new(UByte16)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func (rcv *Partner) TerminalId(obj *UByte16) *UByte16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		x := o + rcv._tab.Pos
		if obj == nil {
			obj = new(UByte16)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func (rcv *Partner) PartnerId(obj *UByte16) *UByte16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		x := o + rcv._tab.Pos
		if obj == nil {
			obj = new(UByte16)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func (rcv *Partner) Name() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(10))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func (rcv *Partner) Ip() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(12))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func (rcv *Partner) Port() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(14))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *Partner) MutatePort(n int16) bool {
	return rcv._tab.MutateInt16Slot(14, n)
}

func (rcv *Partner) CreateTs(obj *Timestamp) *Timestamp {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(16))
	if o != 0 {
		x := o + rcv._tab.Pos
		if obj == nil {
			obj = new(Timestamp)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func PartnerStart(builder *flatbuffers.Builder) {
	builder.StartObject(7)
}
func PartnerAddId(builder *flatbuffers.Builder, id flatbuffers.UOffsetT) {
	builder.PrependStructSlot(0, flatbuffers.UOffsetT(id), 0)
}
func PartnerAddTerminalId(builder *flatbuffers.Builder, terminalId flatbuffers.UOffsetT) {
	builder.PrependStructSlot(1, flatbuffers.UOffsetT(terminalId), 0)
}
func PartnerAddPartnerId(builder *flatbuffers.Builder, partnerId flatbuffers.UOffsetT) {
	builder.PrependStructSlot(2, flatbuffers.UOffsetT(partnerId), 0)
}
func PartnerAddName(builder *flatbuffers.Builder, name flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(3, flatbuffers.UOffsetT(name), 0)
}
func PartnerAddIp(builder *flatbuffers.Builder, ip flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(4, flatbuffers.UOffsetT(ip), 0)
}
func PartnerAddPort(builder *flatbuffers.Builder, port int16) {
	builder.PrependInt16Slot(5, port, 0)
}
func PartnerAddCreateTs(builder *flatbuffers.Builder, createTs flatbuffers.UOffsetT) {
	builder.PrependStructSlot(6, flatbuffers.UOffsetT(createTs), 0)
}
func PartnerEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
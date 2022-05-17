// automatically generated by the FlatBuffers compiler, do not modify



use crate::base_generated::*;
use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod idl {

  use crate::base_generated::*;
  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum PartnerOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Partner<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Partner<'a> {
    type Inner = Partner<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Partner<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Partner { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args PartnerArgs<'args>) -> flatbuffers::WIPOffset<Partner<'bldr>> {
      let mut builder = PartnerBuilder::new(_fbb);
      if let Some(x) = args.create_ts { builder.add_create_ts(x); }
      if let Some(x) = args.ip { builder.add_ip(x); }
      if let Some(x) = args.name { builder.add_name(x); }
      if let Some(x) = args.partner_id { builder.add_partner_id(x); }
      if let Some(x) = args.terminal_id { builder.add_terminal_id(x); }
      if let Some(x) = args.id { builder.add_id(x); }
      builder.add_port(args.port);
      builder.finish()
    }

    pub const VT_ID: flatbuffers::VOffsetT = 4;
    pub const VT_TERMINAL_ID: flatbuffers::VOffsetT = 6;
    pub const VT_PARTNER_ID: flatbuffers::VOffsetT = 8;
    pub const VT_NAME: flatbuffers::VOffsetT = 10;
    pub const VT_IP: flatbuffers::VOffsetT = 12;
    pub const VT_PORT: flatbuffers::VOffsetT = 14;
    pub const VT_CREATE_TS: flatbuffers::VOffsetT = 16;

  #[inline]
  pub fn id(&self) -> Option<&'a UByte16> {
    self._tab.get::<UByte16>(Partner::VT_ID, None)
  }
  #[inline]
  pub fn terminal_id(&self) -> Option<&'a UByte16> {
    self._tab.get::<UByte16>(Partner::VT_TERMINAL_ID, None)
  }
  #[inline]
  pub fn partner_id(&self) -> Option<&'a UByte16> {
    self._tab.get::<UByte16>(Partner::VT_PARTNER_ID, None)
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Partner::VT_NAME, None)
  }
  #[inline]
  pub fn ip(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Partner::VT_IP, None)
  }
  #[inline]
  pub fn port(&self) -> i16 {
    self._tab.get::<i16>(Partner::VT_PORT, Some(0)).unwrap()
  }
  #[inline]
  pub fn create_ts(&self) -> Option<&'a Timestamp> {
    self._tab.get::<Timestamp>(Partner::VT_CREATE_TS, None)
  }
}

impl flatbuffers::Verifiable for Partner<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<UByte16>(&"id", Self::VT_ID, false)?
     .visit_field::<UByte16>(&"terminal_id", Self::VT_TERMINAL_ID, false)?
     .visit_field::<UByte16>(&"partner_id", Self::VT_PARTNER_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"name", Self::VT_NAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"ip", Self::VT_IP, false)?
     .visit_field::<i16>(&"port", Self::VT_PORT, false)?
     .visit_field::<Timestamp>(&"create_ts", Self::VT_CREATE_TS, false)?
     .finish();
    Ok(())
  }
}
pub struct PartnerArgs<'a> {
    pub id: Option<&'a UByte16>,
    pub terminal_id: Option<&'a UByte16>,
    pub partner_id: Option<&'a UByte16>,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub ip: Option<flatbuffers::WIPOffset<&'a str>>,
    pub port: i16,
    pub create_ts: Option<&'a Timestamp>,
}
impl<'a> Default for PartnerArgs<'a> {
    #[inline]
    fn default() -> Self {
        PartnerArgs {
            id: None,
            terminal_id: None,
            partner_id: None,
            name: None,
            ip: None,
            port: 0,
            create_ts: None,
        }
    }
}
pub struct PartnerBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PartnerBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: &UByte16) {
    self.fbb_.push_slot_always::<&UByte16>(Partner::VT_ID, id);
  }
  #[inline]
  pub fn add_terminal_id(&mut self, terminal_id: &UByte16) {
    self.fbb_.push_slot_always::<&UByte16>(Partner::VT_TERMINAL_ID, terminal_id);
  }
  #[inline]
  pub fn add_partner_id(&mut self, partner_id: &UByte16) {
    self.fbb_.push_slot_always::<&UByte16>(Partner::VT_PARTNER_ID, partner_id);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Partner::VT_NAME, name);
  }
  #[inline]
  pub fn add_ip(&mut self, ip: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Partner::VT_IP, ip);
  }
  #[inline]
  pub fn add_port(&mut self, port: i16) {
    self.fbb_.push_slot::<i16>(Partner::VT_PORT, port, 0);
  }
  #[inline]
  pub fn add_create_ts(&mut self, create_ts: &Timestamp) {
    self.fbb_.push_slot_always::<&Timestamp>(Partner::VT_CREATE_TS, create_ts);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PartnerBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PartnerBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Partner<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Partner<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Partner");
      ds.field("id", &self.id());
      ds.field("terminal_id", &self.terminal_id());
      ds.field("partner_id", &self.partner_id());
      ds.field("name", &self.name());
      ds.field("ip", &self.ip());
      ds.field("port", &self.port());
      ds.field("create_ts", &self.create_ts());
      ds.finish()
  }
}
pub enum OnLineOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct OnLine<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for OnLine<'a> {
    type Inner = OnLine<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> OnLine<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        OnLine { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args OnLineArgs<'args>) -> flatbuffers::WIPOffset<OnLine<'bldr>> {
      let mut builder = OnLineBuilder::new(_fbb);
      if let Some(x) = args.ts { builder.add_ts(x); }
      if let Some(x) = args.partner { builder.add_partner(x); }
      if let Some(x) = args.header { builder.add_header(x); }
      builder.finish()
    }

    pub const VT_HEADER: flatbuffers::VOffsetT = 4;
    pub const VT_PARTNER: flatbuffers::VOffsetT = 6;
    pub const VT_TS: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn header(&self) -> Option<&'a Header> {
    self._tab.get::<Header>(OnLine::VT_HEADER, None)
  }
  #[inline]
  pub fn partner(&self) -> Option<Partner<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Partner>>(OnLine::VT_PARTNER, None)
  }
  #[inline]
  pub fn ts(&self) -> Option<&'a Timestamp> {
    self._tab.get::<Timestamp>(OnLine::VT_TS, None)
  }
}

impl flatbuffers::Verifiable for OnLine<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Header>(&"header", Self::VT_HEADER, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<Partner>>(&"partner", Self::VT_PARTNER, false)?
     .visit_field::<Timestamp>(&"ts", Self::VT_TS, false)?
     .finish();
    Ok(())
  }
}
pub struct OnLineArgs<'a> {
    pub header: Option<&'a Header>,
    pub partner: Option<flatbuffers::WIPOffset<Partner<'a>>>,
    pub ts: Option<&'a Timestamp>,
}
impl<'a> Default for OnLineArgs<'a> {
    #[inline]
    fn default() -> Self {
        OnLineArgs {
            header: None,
            partner: None,
            ts: None,
        }
    }
}
pub struct OnLineBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> OnLineBuilder<'a, 'b> {
  #[inline]
  pub fn add_header(&mut self, header: &Header) {
    self.fbb_.push_slot_always::<&Header>(OnLine::VT_HEADER, header);
  }
  #[inline]
  pub fn add_partner(&mut self, partner: flatbuffers::WIPOffset<Partner<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Partner>>(OnLine::VT_PARTNER, partner);
  }
  #[inline]
  pub fn add_ts(&mut self, ts: &Timestamp) {
    self.fbb_.push_slot_always::<&Timestamp>(OnLine::VT_TS, ts);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> OnLineBuilder<'a, 'b> {
    let start = _fbb.start_table();
    OnLineBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<OnLine<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for OnLine<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("OnLine");
      ds.field("header", &self.header());
      ds.field("partner", &self.partner());
      ds.field("ts", &self.ts());
      ds.finish()
  }
}
pub enum OnLineAckOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct OnLineAck<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for OnLineAck<'a> {
    type Inner = OnLineAck<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> OnLineAck<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        OnLineAck { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args OnLineAckArgs<'args>) -> flatbuffers::WIPOffset<OnLineAck<'bldr>> {
      let mut builder = OnLineAckBuilder::new(_fbb);
      if let Some(x) = args.ts { builder.add_ts(x); }
      if let Some(x) = args.partner { builder.add_partner(x); }
      if let Some(x) = args.id { builder.add_id(x); }
      if let Some(x) = args.header { builder.add_header(x); }
      builder.finish()
    }

    pub const VT_HEADER: flatbuffers::VOffsetT = 4;
    pub const VT_ID: flatbuffers::VOffsetT = 6;
    pub const VT_PARTNER: flatbuffers::VOffsetT = 8;
    pub const VT_TS: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn header(&self) -> Option<&'a Header> {
    self._tab.get::<Header>(OnLineAck::VT_HEADER, None)
  }
  #[inline]
  pub fn id(&self) -> Option<&'a UByte16> {
    self._tab.get::<UByte16>(OnLineAck::VT_ID, None)
  }
  #[inline]
  pub fn partner(&self) -> Option<Partner<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Partner>>(OnLineAck::VT_PARTNER, None)
  }
  #[inline]
  pub fn ts(&self) -> Option<&'a Timestamp> {
    self._tab.get::<Timestamp>(OnLineAck::VT_TS, None)
  }
}

impl flatbuffers::Verifiable for OnLineAck<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Header>(&"header", Self::VT_HEADER, false)?
     .visit_field::<UByte16>(&"id", Self::VT_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<Partner>>(&"partner", Self::VT_PARTNER, false)?
     .visit_field::<Timestamp>(&"ts", Self::VT_TS, false)?
     .finish();
    Ok(())
  }
}
pub struct OnLineAckArgs<'a> {
    pub header: Option<&'a Header>,
    pub id: Option<&'a UByte16>,
    pub partner: Option<flatbuffers::WIPOffset<Partner<'a>>>,
    pub ts: Option<&'a Timestamp>,
}
impl<'a> Default for OnLineAckArgs<'a> {
    #[inline]
    fn default() -> Self {
        OnLineAckArgs {
            header: None,
            id: None,
            partner: None,
            ts: None,
        }
    }
}
pub struct OnLineAckBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> OnLineAckBuilder<'a, 'b> {
  #[inline]
  pub fn add_header(&mut self, header: &Header) {
    self.fbb_.push_slot_always::<&Header>(OnLineAck::VT_HEADER, header);
  }
  #[inline]
  pub fn add_id(&mut self, id: &UByte16) {
    self.fbb_.push_slot_always::<&UByte16>(OnLineAck::VT_ID, id);
  }
  #[inline]
  pub fn add_partner(&mut self, partner: flatbuffers::WIPOffset<Partner<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Partner>>(OnLineAck::VT_PARTNER, partner);
  }
  #[inline]
  pub fn add_ts(&mut self, ts: &Timestamp) {
    self.fbb_.push_slot_always::<&Timestamp>(OnLineAck::VT_TS, ts);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> OnLineAckBuilder<'a, 'b> {
    let start = _fbb.start_table();
    OnLineAckBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<OnLineAck<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for OnLineAck<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("OnLineAck");
      ds.field("header", &self.header());
      ds.field("id", &self.id());
      ds.field("partner", &self.partner());
      ds.field("ts", &self.ts());
      ds.finish()
  }
}
}  // pub mod idl


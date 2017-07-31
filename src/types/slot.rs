//! Minecraft item stack (inventory slot) data type

use std::io;
use std::io::prelude::*;

use nbt;

use packet::Protocol;

#[derive(Debug)]
pub struct Slot {
    pub id: u16,
    pub count: u8,
    pub damage: i16,
    pub tag: Option<nbt::Blob>
}

impl Protocol for Option<Slot> {
    type Clean = Option<Slot>;

    fn proto_len(value: &Option<Slot>) -> usize {
        match *value {
            Some(ref slot) => if let Some(ref tag) = slot.tag {
                    2 + 1 + 2 + <nbt::Blob as Protocol>::proto_len(&tag) // id, count, damage, tag
                } else {
                    2 + 1 + 2 + 1
                },
            None => 2
        }
    }

    fn proto_encode(value: &Option<Slot>, dst: &mut Write) -> io::Result<()> {
        match *value {
            Some(Slot { id, count, damage, ref tag }) => {
                try!(<i16 as Protocol>::proto_encode(&(id as i16), dst));
                try!(<u8 as Protocol>::proto_encode(&count, dst));
                try!(<i16 as Protocol>::proto_encode(&damage, dst));
                if let &Some(ref tag) = tag {
                    try!(<nbt::Blob as Protocol>::proto_encode(&tag, dst))
                } else {
                    try!(<u8 as Protocol>::proto_encode(&0, dst))
                }
            }
            None => { try!(<i16 as Protocol>::proto_encode(&-1, dst)) }
        }
        Ok(())
    }

    fn proto_decode(src: &mut Read) -> io::Result<Option<Slot>> {
        let id = try!(<i16 as Protocol>::proto_decode(src));
        Ok(if id == -1 {
            None
        } else {
            Some(Slot {
                id: id as u16,
                count: try!(<u8 as Protocol>::proto_decode(src)),
                damage: try!(<i16 as Protocol>::proto_decode(src)),
                tag: if let Ok(tag) = <nbt::Blob as Protocol>::proto_decode(src) {
                    Some(tag)
                } else {
                    None
                }
            })
        })
    }
}

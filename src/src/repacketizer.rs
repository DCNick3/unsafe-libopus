use crate::externs::{free, malloc};
use ::libc;

#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/opus_private.h:33"]
pub mod opus_private_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:8"]
    pub struct OpusRepacketizer {
        pub toc: libc::c_uchar,
        pub nb_frames: libc::c_int,
        pub frames: [*const libc::c_uchar; 48],
        pub len: [i16; 48],
        pub framesize: libc::c_int,
    }
    extern "C" {
        #[c2rust::src_loc = "165:1"]
        pub fn opus_packet_parse_impl(
            data: *const libc::c_uchar,
            len: i32,
            self_delimited: libc::c_int,
            out_toc: *mut libc::c_uchar,
            frames: *mut *const libc::c_uchar,
            size: *mut i16,
            payload_offset: *mut libc::c_int,
            packet_offset: *mut i32,
        ) -> libc::c_int;
        #[c2rust::src_loc = "140:1"]
        pub fn encode_size(size: libc::c_int, data: *mut libc::c_uchar) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:32"]
pub mod opus_defines_h {
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_BAD_ARG: libc::c_int = -(1 as libc::c_int);
    #[c2rust::src_loc = "50:9"]
    pub const OPUS_BUFFER_TOO_SMALL: libc::c_int = -(2 as libc::c_int);
    #[c2rust::src_loc = "54:9"]
    pub const OPUS_INVALID_PACKET: libc::c_int = -(4 as libc::c_int);
    #[c2rust::src_loc = "46:9"]
    pub const OPUS_OK: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus.h:32"]
pub mod opus_h {
    extern "C" {
        #[c2rust::src_loc = "556:1"]
        pub fn opus_packet_get_samples_per_frame(
            data: *const libc::c_uchar,
            Fs: i32,
        ) -> libc::c_int;
        #[c2rust::src_loc = "572:1"]
        pub fn opus_packet_get_nb_frames(packet: *const libc::c_uchar, len: i32) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
use self::arch_h::celt_fatal;
pub use self::opus_defines_h::{OPUS_BAD_ARG, OPUS_BUFFER_TOO_SMALL, OPUS_INVALID_PACKET, OPUS_OK};
use self::opus_h::{opus_packet_get_nb_frames, opus_packet_get_samples_per_frame};
pub use self::opus_private_h::{encode_size, opus_packet_parse_impl, OpusRepacketizer};
pub use self::stddef_h::{size_t, NULL};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::types_h::{__int16_t, __int32_t};
use crate::externs::memmove;

#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn opus_repacketizer_get_size() -> libc::c_int {
    return ::core::mem::size_of::<OpusRepacketizer>() as libc::c_ulong as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn opus_repacketizer_init(
    mut rp: *mut OpusRepacketizer,
) -> *mut OpusRepacketizer {
    (*rp).nb_frames = 0 as libc::c_int;
    return rp;
}
#[no_mangle]
#[c2rust::src_loc = "48:1"]
pub unsafe extern "C" fn opus_repacketizer_create() -> *mut OpusRepacketizer {
    let mut rp: *mut OpusRepacketizer = 0 as *mut OpusRepacketizer;
    rp = malloc(opus_repacketizer_get_size() as size_t) as *mut OpusRepacketizer;
    if rp.is_null() {
        return NULL as *mut OpusRepacketizer;
    }
    return opus_repacketizer_init(rp);
}
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn opus_repacketizer_destroy(rp: *mut OpusRepacketizer) {
    free(rp as *mut libc::c_void);
}
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn opus_repacketizer_cat_impl(
    mut rp: *mut OpusRepacketizer,
    data: *const libc::c_uchar,
    len: i32,
    self_delimited: libc::c_int,
) -> libc::c_int {
    let mut tmp_toc: libc::c_uchar = 0;
    let mut curr_nb_frames: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if len < 1 as libc::c_int {
        return OPUS_INVALID_PACKET;
    }
    if (*rp).nb_frames == 0 as libc::c_int {
        (*rp).toc = *data.offset(0 as libc::c_int as isize);
        (*rp).framesize = opus_packet_get_samples_per_frame(data, 8000 as libc::c_int);
    } else if (*rp).toc as libc::c_int & 0xfc as libc::c_int
        != *data.offset(0 as libc::c_int as isize) as libc::c_int & 0xfc as libc::c_int
    {
        return OPUS_INVALID_PACKET;
    }
    curr_nb_frames = opus_packet_get_nb_frames(data, len);
    if curr_nb_frames < 1 as libc::c_int {
        return OPUS_INVALID_PACKET;
    }
    if (curr_nb_frames + (*rp).nb_frames) * (*rp).framesize > 960 as libc::c_int {
        return OPUS_INVALID_PACKET;
    }
    ret = opus_packet_parse_impl(
        data,
        len,
        self_delimited,
        &mut tmp_toc,
        &mut *((*rp).frames).as_mut_ptr().offset((*rp).nb_frames as isize),
        &mut *((*rp).len).as_mut_ptr().offset((*rp).nb_frames as isize),
        NULL as *mut libc::c_int,
        NULL as *mut i32,
    );
    if ret < 1 as libc::c_int {
        return ret;
    }
    (*rp).nb_frames += curr_nb_frames;
    return OPUS_OK;
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn opus_repacketizer_cat(
    rp: *mut OpusRepacketizer,
    data: *const libc::c_uchar,
    len: i32,
) -> libc::c_int {
    return opus_repacketizer_cat_impl(rp, data, len, 0 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn opus_repacketizer_get_nb_frames(rp: *mut OpusRepacketizer) -> libc::c_int {
    return (*rp).nb_frames;
}
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub unsafe extern "C" fn opus_repacketizer_out_range_impl(
    rp: *mut OpusRepacketizer,
    begin: libc::c_int,
    end: libc::c_int,
    data: *mut libc::c_uchar,
    maxlen: i32,
    self_delimited: libc::c_int,
    pad: libc::c_int,
) -> i32 {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut tot_size: i32 = 0;
    let mut len: *mut i16 = 0 as *mut i16;
    let mut frames: *mut *const libc::c_uchar = 0 as *mut *const libc::c_uchar;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if begin < 0 as libc::c_int || begin >= end || end > (*rp).nb_frames {
        return OPUS_BAD_ARG;
    }
    count = end - begin;
    len = ((*rp).len).as_mut_ptr().offset(begin as isize);
    frames = ((*rp).frames).as_mut_ptr().offset(begin as isize);
    if self_delimited != 0 {
        tot_size = 1 as libc::c_int
            + (*len.offset((count - 1 as libc::c_int) as isize) as libc::c_int
                >= 252 as libc::c_int) as libc::c_int;
    } else {
        tot_size = 0 as libc::c_int;
    }
    ptr = data;
    if count == 1 as libc::c_int {
        tot_size += *len.offset(0 as libc::c_int as isize) as libc::c_int + 1 as libc::c_int;
        if tot_size > maxlen {
            return OPUS_BUFFER_TOO_SMALL;
        }
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = ((*rp).toc as libc::c_int & 0xfc as libc::c_int) as libc::c_uchar;
    } else if count == 2 as libc::c_int {
        if *len.offset(1 as libc::c_int as isize) as libc::c_int
            == *len.offset(0 as libc::c_int as isize) as libc::c_int
        {
            tot_size += 2 as libc::c_int * *len.offset(0 as libc::c_int as isize) as libc::c_int
                + 1 as libc::c_int;
            if tot_size > maxlen {
                return OPUS_BUFFER_TOO_SMALL;
            }
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            *fresh1 = ((*rp).toc as libc::c_int & 0xfc as libc::c_int | 0x1 as libc::c_int)
                as libc::c_uchar;
        } else {
            tot_size += *len.offset(0 as libc::c_int as isize) as libc::c_int
                + *len.offset(1 as libc::c_int as isize) as libc::c_int
                + 2 as libc::c_int
                + (*len.offset(0 as libc::c_int as isize) as libc::c_int >= 252 as libc::c_int)
                    as libc::c_int;
            if tot_size > maxlen {
                return OPUS_BUFFER_TOO_SMALL;
            }
            let fresh2 = ptr;
            ptr = ptr.offset(1);
            *fresh2 = ((*rp).toc as libc::c_int & 0xfc as libc::c_int | 0x2 as libc::c_int)
                as libc::c_uchar;
            ptr = ptr.offset(
                encode_size(*len.offset(0 as libc::c_int as isize) as libc::c_int, ptr) as isize,
            );
        }
    }
    if count > 2 as libc::c_int || pad != 0 && tot_size < maxlen {
        let mut vbr: libc::c_int = 0;
        let mut pad_amount: libc::c_int = 0 as libc::c_int;
        ptr = data;
        if self_delimited != 0 {
            tot_size = 1 as libc::c_int
                + (*len.offset((count - 1 as libc::c_int) as isize) as libc::c_int
                    >= 252 as libc::c_int) as libc::c_int;
        } else {
            tot_size = 0 as libc::c_int;
        }
        vbr = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < count {
            if *len.offset(i as isize) as libc::c_int
                != *len.offset(0 as libc::c_int as isize) as libc::c_int
            {
                vbr = 1 as libc::c_int;
                break;
            } else {
                i += 1;
            }
        }
        if vbr != 0 {
            tot_size += 2 as libc::c_int;
            i = 0 as libc::c_int;
            while i < count - 1 as libc::c_int {
                tot_size += 1 as libc::c_int
                    + (*len.offset(i as isize) as libc::c_int >= 252 as libc::c_int) as libc::c_int
                    + *len.offset(i as isize) as libc::c_int;
                i += 1;
            }
            tot_size += *len.offset((count - 1 as libc::c_int) as isize) as libc::c_int;
            if tot_size > maxlen {
                return OPUS_BUFFER_TOO_SMALL;
            }
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            *fresh3 = ((*rp).toc as libc::c_int & 0xfc as libc::c_int | 0x3 as libc::c_int)
                as libc::c_uchar;
            let fresh4 = ptr;
            ptr = ptr.offset(1);
            *fresh4 = (count | 0x80 as libc::c_int) as libc::c_uchar;
        } else {
            tot_size +=
                count * *len.offset(0 as libc::c_int as isize) as libc::c_int + 2 as libc::c_int;
            if tot_size > maxlen {
                return OPUS_BUFFER_TOO_SMALL;
            }
            let fresh5 = ptr;
            ptr = ptr.offset(1);
            *fresh5 = ((*rp).toc as libc::c_int & 0xfc as libc::c_int | 0x3 as libc::c_int)
                as libc::c_uchar;
            let fresh6 = ptr;
            ptr = ptr.offset(1);
            *fresh6 = count as libc::c_uchar;
        }
        pad_amount = if pad != 0 {
            maxlen - tot_size
        } else {
            0 as libc::c_int
        };
        if pad_amount != 0 as libc::c_int {
            let mut nb_255s: libc::c_int = 0;
            let ref mut fresh7 = *data.offset(1 as libc::c_int as isize);
            *fresh7 = (*fresh7 as libc::c_int | 0x40 as libc::c_int) as libc::c_uchar;
            nb_255s = (pad_amount - 1 as libc::c_int) / 255 as libc::c_int;
            i = 0 as libc::c_int;
            while i < nb_255s {
                let fresh8 = ptr;
                ptr = ptr.offset(1);
                *fresh8 = 255 as libc::c_int as libc::c_uchar;
                i += 1;
            }
            let fresh9 = ptr;
            ptr = ptr.offset(1);
            *fresh9 =
                (pad_amount - 255 as libc::c_int * nb_255s - 1 as libc::c_int) as libc::c_uchar;
            tot_size += pad_amount;
        }
        if vbr != 0 {
            i = 0 as libc::c_int;
            while i < count - 1 as libc::c_int {
                ptr = ptr.offset(encode_size(*len.offset(i as isize) as libc::c_int, ptr) as isize);
                i += 1;
            }
        }
    }
    if self_delimited != 0 {
        let sdlen: libc::c_int = encode_size(
            *len.offset((count - 1 as libc::c_int) as isize) as libc::c_int,
            ptr,
        );
        ptr = ptr.offset(sdlen as isize);
    }
    i = 0 as libc::c_int;
    while i < count {
        memmove(
            ptr as *mut libc::c_void,
            *frames.offset(i as isize) as *const libc::c_void,
            (*len.offset(i as isize) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * ptr.offset_from(*frames.offset(i as isize)) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        ptr = ptr.offset(*len.offset(i as isize) as libc::c_int as isize);
        i += 1;
    }
    if pad != 0 {
        while ptr < data.offset(maxlen as isize) {
            let fresh10 = ptr;
            ptr = ptr.offset(1);
            *fresh10 = 0 as libc::c_int as libc::c_uchar;
        }
    }
    return tot_size;
}
#[no_mangle]
#[c2rust::src_loc = "230:1"]
pub unsafe extern "C" fn opus_repacketizer_out_range(
    rp: *mut OpusRepacketizer,
    begin: libc::c_int,
    end: libc::c_int,
    data: *mut libc::c_uchar,
    maxlen: i32,
) -> i32 {
    return opus_repacketizer_out_range_impl(
        rp,
        begin,
        end,
        data,
        maxlen,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "235:1"]
pub unsafe extern "C" fn opus_repacketizer_out(
    rp: *mut OpusRepacketizer,
    data: *mut libc::c_uchar,
    maxlen: i32,
) -> i32 {
    return opus_repacketizer_out_range_impl(
        rp,
        0 as libc::c_int,
        (*rp).nb_frames,
        data,
        maxlen,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "240:1"]
pub unsafe extern "C" fn opus_packet_pad(
    data: *mut libc::c_uchar,
    len: i32,
    new_len: i32,
) -> libc::c_int {
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [0 as *const libc::c_uchar; 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut ret: i32 = 0;
    if len < 1 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    if len == new_len {
        return OPUS_OK;
    } else {
        if len > new_len {
            return OPUS_BAD_ARG;
        }
    }
    opus_repacketizer_init(&mut rp);
    memmove(
        data.offset(new_len as isize).offset(-(len as isize)) as *mut libc::c_void,
        data as *const libc::c_void,
        (len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * data
                        .offset(new_len as isize)
                        .offset(-(len as isize))
                        .offset_from(data) as libc::c_long) as libc::c_ulong,
            ),
    );
    ret = opus_repacketizer_cat(
        &mut rp,
        data.offset(new_len as isize).offset(-(len as isize)),
        len,
    );
    if ret != OPUS_OK {
        return ret;
    }
    ret = opus_repacketizer_out_range_impl(
        &mut rp,
        0 as libc::c_int,
        rp.nb_frames,
        data,
        new_len,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    if ret > 0 as libc::c_int {
        return OPUS_OK;
    } else {
        return ret;
    };
}
#[no_mangle]
#[c2rust::src_loc = "263:1"]
pub unsafe extern "C" fn opus_packet_unpad(data: *mut libc::c_uchar, len: i32) -> i32 {
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [0 as *const libc::c_uchar; 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut ret: i32 = 0;
    if len < 1 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    opus_repacketizer_init(&mut rp);
    ret = opus_repacketizer_cat(&mut rp, data, len);
    if ret < 0 as libc::c_int {
        return ret;
    }
    ret = opus_repacketizer_out_range_impl(
        &mut rp,
        0 as libc::c_int,
        rp.nb_frames,
        data,
        len,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if !(ret > 0 as libc::c_int && ret <= len) {
        celt_fatal(
            b"assertion failed: ret > 0 && ret <= len\0" as *const u8 as *const libc::c_char,
            b"src/repacketizer.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
        );
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "278:1"]
pub unsafe extern "C" fn opus_multistream_packet_pad(
    mut data: *mut libc::c_uchar,
    mut len: i32,
    new_len: i32,
    nb_streams: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut toc: libc::c_uchar = 0;
    let mut size: [i16; 48] = [0; 48];
    let mut packet_offset: i32 = 0;
    let mut amount: i32 = 0;
    if len < 1 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    if len == new_len {
        return OPUS_OK;
    } else {
        if len > new_len {
            return OPUS_BAD_ARG;
        }
    }
    amount = new_len - len;
    s = 0 as libc::c_int;
    while s < nb_streams - 1 as libc::c_int {
        if len <= 0 as libc::c_int {
            return OPUS_INVALID_PACKET;
        }
        count = opus_packet_parse_impl(
            data,
            len,
            1 as libc::c_int,
            &mut toc,
            NULL as *mut *const libc::c_uchar,
            size.as_mut_ptr(),
            NULL as *mut libc::c_int,
            &mut packet_offset,
        );
        if count < 0 as libc::c_int {
            return count;
        }
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        s += 1;
    }
    return opus_packet_pad(data, len, len + amount);
}
#[no_mangle]
#[c2rust::src_loc = "309:1"]
pub unsafe extern "C" fn opus_multistream_packet_unpad(
    mut data: *mut libc::c_uchar,
    mut len: i32,
    nb_streams: libc::c_int,
) -> i32 {
    let mut s: libc::c_int = 0;
    let mut toc: libc::c_uchar = 0;
    let mut size: [i16; 48] = [0; 48];
    let mut packet_offset: i32 = 0;
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [0 as *const libc::c_uchar; 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut dst: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dst_len: i32 = 0;
    if len < 1 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    dst = data;
    dst_len = 0 as libc::c_int;
    s = 0 as libc::c_int;
    while s < nb_streams {
        let mut ret: i32 = 0;
        let self_delimited: libc::c_int = (s != nb_streams - 1 as libc::c_int) as libc::c_int;
        if len <= 0 as libc::c_int {
            return OPUS_INVALID_PACKET;
        }
        opus_repacketizer_init(&mut rp);
        ret = opus_packet_parse_impl(
            data,
            len,
            self_delimited,
            &mut toc,
            NULL as *mut *const libc::c_uchar,
            size.as_mut_ptr(),
            NULL as *mut libc::c_int,
            &mut packet_offset,
        );
        if ret < 0 as libc::c_int {
            return ret;
        }
        ret = opus_repacketizer_cat_impl(&mut rp, data, packet_offset, self_delimited);
        if ret < 0 as libc::c_int {
            return ret;
        }
        ret = opus_repacketizer_out_range_impl(
            &mut rp,
            0 as libc::c_int,
            rp.nb_frames,
            dst,
            len,
            self_delimited,
            0 as libc::c_int,
        );
        if ret < 0 as libc::c_int {
            return ret;
        } else {
            dst_len += ret;
        }
        dst = dst.offset(ret as isize);
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        s += 1;
    }
    return dst_len;
}

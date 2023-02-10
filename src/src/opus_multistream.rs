use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/opus_private.h:34"]
pub mod opus_private_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct ChannelLayout {
        pub nb_channels: libc::c_int,
        pub nb_streams: libc::c_int,
        pub nb_coupled_streams: libc::c_int,
        pub mapping: [libc::c_uchar; 256],
    }
}
pub use self::opus_private_h::ChannelLayout;
#[no_mangle]
#[c2rust::src_loc = "41:1"]
pub unsafe extern "C" fn validate_layout(mut layout: *const ChannelLayout) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut max_channel: libc::c_int = 0;
    max_channel = (*layout).nb_streams + (*layout).nb_coupled_streams;
    if max_channel > 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as libc::c_int >= max_channel
            && (*layout).mapping[i as usize] as libc::c_int != 255 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn get_left_channel(
    mut layout: *const ChannelLayout,
    mut stream_id: libc::c_int,
    mut prev: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = if prev < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        prev + 1 as libc::c_int
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as libc::c_int == stream_id * 2 as libc::c_int {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn get_right_channel(
    mut layout: *const ChannelLayout,
    mut stream_id: libc::c_int,
    mut prev: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = if prev < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        prev + 1 as libc::c_int
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as libc::c_int
            == stream_id * 2 as libc::c_int + 1 as libc::c_int
        {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn get_mono_channel(
    mut layout: *const ChannelLayout,
    mut stream_id: libc::c_int,
    mut prev: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = if prev < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        prev + 1 as libc::c_int
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as libc::c_int == stream_id + (*layout).nb_coupled_streams
        {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}

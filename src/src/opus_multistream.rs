#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "47:16"]
pub struct ChannelLayout {
    pub nb_channels: i32,
    pub nb_streams: i32,
    pub nb_coupled_streams: i32,
    pub mapping: [u8; 256],
}
#[c2rust::src_loc = "41:1"]
pub unsafe fn validate_layout(layout: *const ChannelLayout) -> i32 {
    let mut i: i32 = 0;
    let mut max_channel: i32 = 0;
    max_channel = (*layout).nb_streams + (*layout).nb_coupled_streams;
    if max_channel > 255 as i32 {
        return 0 as i32;
    }
    i = 0 as i32;
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as i32 >= max_channel
            && (*layout).mapping[i as usize] as i32 != 255 as i32
        {
            return 0 as i32;
        }
        i += 1;
    }
    return 1 as i32;
}
#[c2rust::src_loc = "57:1"]
pub unsafe fn get_left_channel(layout: *const ChannelLayout, stream_id: i32, prev: i32) -> i32 {
    let mut i: i32 = 0;
    i = if prev < 0 as i32 {
        0 as i32
    } else {
        prev + 1 as i32
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as i32 == stream_id * 2 as i32 {
            return i;
        }
        i += 1;
    }
    return -(1 as i32);
}
#[c2rust::src_loc = "69:1"]
pub unsafe fn get_right_channel(layout: *const ChannelLayout, stream_id: i32, prev: i32) -> i32 {
    let mut i: i32 = 0;
    i = if prev < 0 as i32 {
        0 as i32
    } else {
        prev + 1 as i32
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as i32 == stream_id * 2 as i32 + 1 as i32 {
            return i;
        }
        i += 1;
    }
    return -(1 as i32);
}
#[c2rust::src_loc = "81:1"]
pub unsafe fn get_mono_channel(layout: *const ChannelLayout, stream_id: i32, prev: i32) -> i32 {
    let mut i: i32 = 0;
    i = if prev < 0 as i32 {
        0 as i32
    } else {
        prev + 1 as i32
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as i32 == stream_id + (*layout).nb_coupled_streams {
            return i;
        }
        i += 1;
    }
    return -(1 as i32);
}

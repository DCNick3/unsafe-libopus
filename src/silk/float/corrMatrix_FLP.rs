use crate::silk::float::energy_FLP::silk_energy_FLP;
use crate::silk::float::inner_product_FLP::silk_inner_product_FLP;

#[c2rust::src_loc = "39:1"]
pub unsafe fn silk_corrVector_FLP(
    x: *const libc::c_float,
    t: *const libc::c_float,
    L: libc::c_int,
    Order: libc::c_int,
    Xt: *mut libc::c_float,
) {
    let mut lag: libc::c_int = 0;
    let mut ptr1: *const libc::c_float = 0 as *const libc::c_float;
    ptr1 = &*x.offset((Order - 1 as libc::c_int) as isize) as *const libc::c_float;
    lag = 0 as libc::c_int;
    while lag < Order {
        *Xt.offset(lag as isize) = silk_inner_product_FLP(ptr1, t, L) as libc::c_float;
        ptr1 = ptr1.offset(-1);
        lag += 1;
    }
}
#[c2rust::src_loc = "59:1"]
pub unsafe fn silk_corrMatrix_FLP(
    x: *const libc::c_float,
    L: libc::c_int,
    Order: libc::c_int,
    XX: *mut libc::c_float,
) {
    let mut j: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut energy: libc::c_double = 0.;
    let mut ptr1: *const libc::c_float = 0 as *const libc::c_float;
    let mut ptr2: *const libc::c_float = 0 as *const libc::c_float;
    ptr1 = &*x.offset((Order - 1 as libc::c_int) as isize) as *const libc::c_float;
    energy = silk_energy_FLP(ptr1, L);
    *XX.offset((0 as libc::c_int * Order + 0 as libc::c_int) as isize) = energy as libc::c_float;
    j = 1 as libc::c_int;
    while j < Order {
        energy += (*ptr1.offset(-j as isize) * *ptr1.offset(-j as isize)
            - *ptr1.offset((L - j) as isize) * *ptr1.offset((L - j) as isize))
            as libc::c_double;
        *XX.offset((j * Order + j) as isize) = energy as libc::c_float;
        j += 1;
    }
    ptr2 = &*x.offset((Order - 2 as libc::c_int) as isize) as *const libc::c_float;
    lag = 1 as libc::c_int;
    while lag < Order {
        energy = silk_inner_product_FLP(ptr1, ptr2, L);
        *XX.offset((lag * Order + 0 as libc::c_int) as isize) = energy as libc::c_float;
        *XX.offset((0 as libc::c_int * Order + lag) as isize) = energy as libc::c_float;
        j = 1 as libc::c_int;
        while j < Order - lag {
            energy += (*ptr1.offset(-j as isize) * *ptr2.offset(-j as isize)
                - *ptr1.offset((L - j) as isize) * *ptr2.offset((L - j) as isize))
                as libc::c_double;
            *XX.offset(((lag + j) * Order + j) as isize) = energy as libc::c_float;
            *XX.offset((j * Order + (lag + j)) as isize) = energy as libc::c_float;
            j += 1;
        }
        ptr2 = ptr2.offset(-1);
        lag += 1;
    }
}

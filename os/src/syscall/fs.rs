use log::info;

use crate::batch::{get_current_user_stack_scope, get_current_user_data_scope};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            // if buf as usize == 0x80405000 {
            //     let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            //     let str = core::str::from_utf8(slice).unwrap();
            //     println!("------{}, {}", str, len);
            // }
            if !check_buf_addr_vailid(buf as usize, len) {
                return -1;
            }
            // 程序的空间：
            // 1. 用户栈空间，2.用户堆，3.用户各个段
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => {
            -1
        }
    }
}

fn check_buf_addr_vailid(buf_addr: usize, len: usize)-> bool {
    // 程序的空间：
    // 1. 用户栈空间，2.用户堆，3.用户各个段
    let (lower, higher) = get_current_user_stack_scope();
    let (user_data_lower, user_data_higher) = get_current_user_data_scope();
    info!("({0:#X},{1:#X},{2:#X},{3:#X}) or ({4:#X},{1:#X},{2:#X},{5:#X})", lower, buf_addr, buf_addr + len, higher, user_data_lower, user_data_higher);
    // 1.用户栈
    (buf_addr > lower && buf_addr + len <= higher)
    ||
    (buf_addr >= user_data_lower && buf_addr + len < user_data_higher)
}
// 裸机无std和main
#![no_std]
#![no_main] // 原来所有语言都有运行时，只不过rust和c的马上就结束了

use core::panic::PanicInfo;

mod vga_buffer;

static GREETING: &[u8] = b"Hello, Fly OS!";

#[no_mangle] // 不要名称重整
pub extern "C" fn _start() -> ! {
    // 程序入口，c风格命名
    // 因为链接器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
    // 默认命名为 `_start`

    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in GREETING.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xc;
    //     }
    // }

    vga_buffer::print_test("Hello, fly_os !!!\n6\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // panic 处理
    loop {}
}

#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{
    count_syscall, get_time, println, sleep, trace_read, trace_write,
    SYSCALL_EXIT, SYSCALL_GETTIMEOFDAY, SYSCALL_TRACE, SYSCALL_WRITE, SYSCALL_YIELD,SYSCALL_INIT_COUNT
};

pub fn write_const(var: &u8, new_val: u8) {
    trace_write(var as *const _, new_val);
}

////系统调用:传入一个系统调用号,通过传入的id来将对应的系统调用计数器清零
pub fn init_count(id : usize) -> usize {
    let mut ret: usize;
    let args:[usize ; 3] =[id, 0, 0];
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") SYSCALL_INIT_COUNT
        );
    }
    ret
}

#[no_mangle]
pub fn main() -> usize {
    // init_count();//系统调用:初始化所有的调用计数器
    
    let t1 = get_time() as usize;
   
    get_time();
    // assert_eq!(-1, count_syscall(SYSCALL_WRITE));
    // assert_eq!(-1, count_syscall(SYSCALL_EXIT));
    sleep(500);
    let t2 = get_time() as usize;
    let t3 = get_time() as usize;
    assert!(3 <= count_syscall(SYSCALL_GETTIMEOFDAY));  
    // 注意这次 sys_trace 调用本身也计入
    assert_eq!(2, count_syscall(SYSCALL_TRACE));
    init_count(SYSCALL_WRITE as usize);    
    assert_eq!(0, count_syscall(SYSCALL_WRITE));   
    assert!(0 < count_syscall(SYSCALL_YIELD)); 
    init_count(SYSCALL_EXIT as usize); 
    assert_eq!(0, count_syscall(SYSCALL_EXIT)); 

    // 想想为什么 write 调用是两次
    println!("string from task trace test\n");
    let t4 = get_time() as usize;
    let t5 = get_time() as usize;
    assert!(5 <= count_syscall(SYSCALL_GETTIMEOFDAY));
    assert_eq!(7, count_syscall(SYSCALL_TRACE));
    assert_eq!(2, count_syscall(SYSCALL_WRITE));
    assert!(0 < count_syscall(SYSCALL_YIELD));
    assert_eq!(0, count_syscall(SYSCALL_EXIT));

    #[allow(unused_mut)]
    let mut var = 111u8;
    assert_eq!(Some(111), trace_read(&var as *const u8));
    write_const(&var, (t1 ^ t2 ^ t3 ^ t4 ^ t5) as u8);
    assert_eq!((t1 ^ t2 ^ t3 ^ t4 ^ t5) as u8, unsafe { core::ptr::read_volatile(&var) });

    assert!(None != trace_read(main as *const _));
    println!("Test trace OK!");
    0
}

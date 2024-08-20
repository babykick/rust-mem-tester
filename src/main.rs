use std::alloc::{alloc, dealloc, Layout};
use std::env;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

fn parse_size_with_unit(size_str: &str) -> Result<usize, &'static str> {
    let (num_str, unit) = size_str.split_at(size_str.len() - 1);
    let number = match num_str.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid number format"),
    };

    match unit.to_uppercase().as_str() {
        "G" => Ok(number * 1024 * 1024 * 1024), // GB
        "M" => Ok(number * 1024 * 1024),        // MB
        "K" => Ok(number * 1024),               // KB
        "" => Ok(number),                       // 默认以字节为单位
        _ => Err("Invalid unit. Use G, M, K or leave unit empty"),
    }
}

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了内存大小参数
    if args.len() != 2 {
        eprintln!("Usage: {} <size_with_unit>", args[0]);
        std::process::exit(1);
    }

    // 解析内存大小
    let size_str = &args[1];
    let size_in_bytes = match parse_size_with_unit(size_str) {
        Ok(size) => size,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // 创建内存布局
    let layout = Layout::from_size_align(size_in_bytes, 8).unwrap();

    let mut allocations = Vec::new();
    let mut allocated_size = 0;

    // 分配内存直到达到指定大小
    while allocated_size < size_in_bytes {
        let chunk_size = std::cmp::min(1024 * 1024 * 10, size_in_bytes - allocated_size); // 每次分配 10 MB
        let layout = Layout::from_size_align(chunk_size, 8).unwrap();
        println!("Allocating {} bytes of memory", allocated_size);
        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                eprintln!("Failed to allocate memory");
                std::process::exit(1);
            }
            allocations.push(ptr);
            allocated_size += chunk_size;
            // 写入
            ptr::write_bytes(ptr, 0, chunk_size);
        }
    }

    println!("Allocated {} bytes of memory. Waiting for 10 seconds before deallocating...", size_in_bytes);

    // 等待 10 秒钟
    sleep(Duration::new(10, 0));

    // 释放所有分配的内存
    for ptr in allocations {
        unsafe {
            dealloc(ptr, layout);
        }
    }

    println!("Memory deallocated successfully");
}

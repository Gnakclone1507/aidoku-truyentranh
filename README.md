# Aidoku - Nguồn Truyện goctruyentranhvuiaa.com

Đây là nguồn truyện cho ứng dụng Aidoku lấy từ trang web goctruyentranhvuiaa.com.

## Cài Đặt

1. Cài đặt Rust và wasm-pack:
    
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    cargo install wasm-pack
    

2. Biên dịch sang WebAssembly (WASM):
    
    wasm-pack build --target web
    

3. Thêm nguồn vào Aidoku bằng cách nhập URL của file .wasm.

## Đóng Góp
Nếu bạn muốn cải tiến mã nguồn, hãy tạo pull request hoặc mở issue trên GitHub.

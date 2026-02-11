## Một chút "Insight" về Benchmarking:

Tại sao chúng ta phải làm phức tạp với `Cow` và `Lifetime`?

- **Staff Insight**: Trong một ứng dụng Monitor, bạn sẽ cập nhật UI khoảng 2-5 lần mỗi giây.
  Với ~200 tiến trình, nếu dùng String (copy), bạn sẽ tạo ra **1,000 heap allocations mỗi giây**.
  Điều này không chỉ tốn CPU mà còn tạo ra các "vết răng cưa" (GC spikes) trên biểu đồ RAM của bạn.
  Sử dụng `Cow` giúp biểu đồ của chúng ta trông "mượt" hơn vì nó tái sử dụng vùng nhớ cũ.

## Usage

- Run Unit-Test in `core-engine` from Workspace `rust-pulse`:
  - `cargo test -p core-engine`
- Run Integration-Test in `core-engine` from Workspace `rust-pulse`:
  - `cargo run -p core-engine --example check_core`

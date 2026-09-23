# NEUDating

NEUDating là ứng dụng hẹn hò dành cho sinh viên và người trẻ tại Việt Nam,
giúp họ kết nối với những người có sở thích, giá trị và mục tiêu tương đồng
trong một môi trường an toàn, nghiêm túc và tôn trọng quyền riêng tư.

## Bối cảnh và vấn đề

Tỷ lệ sinh của Việt Nam đang giảm xuống dưới mức sinh thay thế. Theo
[World Bank](https://data.worldbank.org/indicator/SP.DYN.TFRT.IN?locations=VN-KR),
mức sinh của Việt Nam năm 2023 là khoảng 1,9 con/phụ nữ, trong khi Hàn Quốc
đã ở mức thấp hơn rất nhiều. Số liệu chính thức của
[Statistics Korea](https://kostat.go.kr/board.es?mid=a10301010000&bid=204&act=view&list_no=433085)
cho thấy tổng tỷ suất sinh của Hàn Quốc năm 2023 chỉ là 0,72.

Đây là lời cảnh báo về những khó khăn xã hội và kinh tế do dân số già hóa,
thiếu lực lượng lao động và ngày càng ít gia đình trẻ. NEUDating không coi
việc kết hôn hay sinh con là nghĩa vụ của mỗi cá nhân; sản phẩm tập trung vào
việc giúp những người độc thân có thêm cơ hội gặp gỡ phù hợp, xây dựng các
mối quan hệ lành mạnh và lâu dài nếu họ tự nguyện lựa chọn.

## Nhóm

**Nhóm:** QHQ<br>
**Thành viên:** @Nguyễn Sơn Hải · @Nguyễn Minh Quang · @Lê Duy Quyền<br>
**Product Owner (cố định cả kỳ):** @Nguyễn Sơn Hải<br>
**Scrum Master:** @Nguyễn Minh Quang (Sprint 1) · @Lê Duy Quyền (Sprint 2)

**Board:** [NEUDating GitHub Project](https://github.com/orgs/SE-FDA-NEU-AI66B/projects/20)

## Chạy thử

### Yêu cầu

- Rust và `rustup`
- Rust nightly toolchain
- `cargo-leptos`
- Target `wasm32-unknown-unknown`

Hướng dẫn cài đặt đầy đủ cho máy mới được duy trì trong
[SETUP.md](SETUP.md).

### Chạy ở chế độ phát triển

```bash
git clone https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B.git
cd G8-DatingApp-AI66B
cargo leptos watch
```

Sau khi lệnh chạy thành công, mở
<http://localhost:3100>. Chế độ `watch` tự động biên dịch lại khi mã nguồn
thay đổi.

### Chạy bản release

```bash
cargo leptos watch --release
```

### Tạo tài liệu Rust

```bash
cargo doc --open
```

## Tài liệu dự án

- [Hướng dẫn cài đặt](SETUP.md)
- [Definition of Done](docs/definition-of-done.md)
- [Sprint log](docs/sprint-log.md)
- [Traceability](docs/traceability.md)

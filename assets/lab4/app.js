/* =========================================================
   CV cá nhân — app.js          Tuần 4: JavaScript & DOM

   Tuần 3 bạn đã trang trí xong trang này bằng CSS.
   Tuần 4 làm nó SỐNG DẬY: lọc kỹ năng, vẽ lại danh sách từ dữ
   liệu, kiểm tra form liên hệ. Không sửa index.html, không sửa
   style.css — chỉ viết vào 4 phần dưới đây.

   Mở Console (F12 -> tab Console) và để mở suốt buổi.
   Kiểm tra file đã được nối: bỏ chú thích dòng dưới, lưu lại,
   Console phải in ra dòng chữ đó.
   ========================================================= */

// console.log("app.js đã chạy");

/* ===== DỮ LIỆU DÙNG CHUNG CHO CẢ 4 LAB — đã cho sẵn ===== */

const skills = [
  { ten: "Python", mucDo: "Khá", nam: 2, ghiChu: "pandas, numpy, matplotlib" },
  {
    ten: "SQL",
    mucDo: "Khá",
    nam: 2,
    ghiChu: "truy vấn nhiều bảng, window function",
  },
  {
    ten: "HTML & CSS",
    mucDo: "Cơ bản",
    nam: 1,
    ghiChu: "dựng trang tĩnh, responsive",
  },
  { ten: "Tiếng Anh", mucDo: "B2", nam: 5, ghiChu: "đọc tài liệu kỹ thuật" },
];

/* =========================================================
   LAB 1 — CÚ PHÁP JS               (chỉ làm việc với Console)
   ---------------------------------------------------------
   1. locTheoNam(list, toiThieu)   -> mảng các kỹ năng có nam >= toiThieu
                                      (viết bằng vòng for, rồi viết lại bằng filter)
   2. tongSoNam(list)              -> tổng trường nam của cả mảng
   3. lauNhat(list)                -> object có nam lớn nhất
   4. In từng kỹ năng bằng forEach + template literal, dạng:
         `Python: 2 năm (Khá)`

   CONSOLE PHẢI IN RA:
      3 kỹ năng từ 2 năm: Python, SQL, Tiếng Anh
      Tổng số năm: 10
      Lâu nhất: Tiếng Anh (5 năm)
      Python: 2 năm (Khá)
      SQL: 2 năm (Khá)
      HTML & CSS: 1 năm (Cơ bản)
      Tiếng Anh: 5 năm (B2)
   ========================================================= */
// alert("this page have crash trying to load ur mom");
let a = "asdfasdf";
alert(a);
tongSoNam(list);

/* =========================================================
   LAB 2 — CHỌN VÀ SỬA PHẦN TỬ         (chưa tạo phần tử mới)
   ---------------------------------------------------------
   1. document.querySelectorAll(".skill") -> in ra số thẻ (phải là 4)
   2. Đọc số năm của từng thẻ bằng  the.dataset.years
      Lưu ý: giá trị đọc ra là CHUỖI -> Number(...) mới so sánh được
   3. lamMoThePhu(toiThieu)  -> duyệt các thẻ bằng forEach:
         nam <  toiThieu  ->  the.classList.add("dimmed")
         nam >= toiThieu  ->  the.classList.remove("dimmed")
   4. Gọi lamMoThePhu(2)

   PHẢI THẤY: thẻ "HTML & CSS" (1 năm) mờ đi, ba thẻ còn lại bình thường.
   Lớp .dimmed đã có sẵn trong style.css — JS chỉ gắn tên lớp vào.
   ========================================================= */

/* =========================================================
   LAB 3 — TẠO VÀ XÓA PHẦN TỬ
   ---------------------------------------------------------
   1. veKyNang(list) — vẽ lại toàn bộ khu .skills từ dữ liệu:
         a. const khung = document.querySelector(".skills");
         b. khung.innerHTML = "";                  // xóa sạch thẻ cũ
         c. list.forEach(...) với mỗi phần tử:
              const the = document.createElement("div");
              the.classList.add("skill");
              the.dataset.years = s.nam;
              the.innerHTML = `<h3>...</h3><p class="level">...</p>
                               <p class="note">... — ... năm</p>`;
              khung.appendChild(the);
   2. Gọi veKyNang(skills) rồi gọi lại lamMoThePhu(2)
   3. Thử veKyNang(locTheoNam(skills, 2)) -> chỉ còn 3 thẻ

   PHẢI THẤY: giao diện y hệt trước, nhưng bây giờ 4 thẻ do JS
   sinh ra. Xóa một phần tử của mảng skills rồi tải lại trang:
   thẻ tương ứng biến mất mà không cần sửa HTML.
   ========================================================= */

/* =========================================================
   LAB 4 — SỰ KIỆN VÀ KIỂM TRA FORM
   ---------------------------------------------------------
   1. Hai nút lọc:
         #btn-all -> veKyNang(skills)
         #btn-exp -> veKyNang(locTheoNam(skills, 2))
      (dùng addEventListener("click", ...), nhớ gọi lamMoThePhu nếu cần)
   2. #contact-form, sự kiện "submit":
         event.preventDefault();          // chặn tải lại trang
         xoaLoiCu();                      // gỡ hết .error và .error-msg cũ
         Kiểm tra 3 ô (dùng .value.trim()):
            #ten      rỗng                       -> "Vui lòng nhập họ tên."
            #email    không chứa "@" hoặc rỗng   -> "Email không hợp lệ."
            #loi-nhan ngắn hơn 10 ký tự          -> "Lời nhắn cần ít nhất 10 ký tự."
   3. Với mỗi ô sai:  o.classList.add("error")  và chèn ngay dưới nó một
      <p class="error-msg"> tạo bằng createElement.
   4. Không có lỗi -> hiện <p class="ok-msg">Đã gửi! Cảm ơn bạn.</p>

   PHẢI THẤY: bấm Gửi khi form rỗng -> 3 dòng đỏ, viền ô đỏ, trang
   KHÔNG tải lại. Bấm tiếp lần nữa -> vẫn đúng 3 dòng, không nhân đôi.
   Điền đủ và đúng -> lỗi biến mất, hiện dòng xanh.

   XONG SỚM:
   5. Sự kiện "input": gõ lại vào ô nào thì gỡ .error của ô đó.
   6. #theme-btn: document.body.classList.toggle("dark")
   ========================================================= */

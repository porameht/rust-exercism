pub fn factors(n: u64) -> Vec<u64> {
    // สร้างเวกเตอร์ว่างเพื่อเก็บตัวประกอบที่เป็นจำนวนเฉพาะ
    let mut factors_list = Vec::new();
    // กำหนดตัวแปร num เป็นตัวเลข n ที่จะหาตัวประกอบ
    let mut num = n;
    // กำหนดตัวแปร divisor เริ่มต้นที่ 2 เพราะว่า 1 เป็นตัวประกอบของทุกตัวเลข
    let mut divisor = 2;

    // วนลูปจนกว่า num จะเป็น 1
    while num > 1 {
        // วนลูปเพื่อหาร num ด้วย divisor จนกว่า num จะไม่สามารถหารด้วย divisor ได้
        while num % divisor == 0 {
            // เพิ่ม divisor เข้าไปใน factors_list เพราะเป็นตัวประกอบ
            factors_list.push(divisor);
            // หาร num ด้วย divisor
            num /= divisor;
        }
        // เพิ่ม divisor ขึ้นไปหลังจากที่ลูปใน while ภายในหลุดออกมา
        divisor += 1;
    }

    // ส่งคืนรายการ factors_list ที่มีตัวประกอบทั้งหมด
    factors_list
}

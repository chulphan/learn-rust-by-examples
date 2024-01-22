fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation

    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    let mut inferred_type = 12; // type i64 is inferred from another line
    inferred_type = 42944967296i64; // 여기서 다시 할당하면 i64 로 추론 됨. -> i32 넘는 값을 할당해서 그런듯

    let mut mutable = 12; // Mutable i32
    mutable = 21;

    // mutable = true; // error: type of a variable can't be changed

    let mutable = true; // variables can be overwritten with shadowing.
}

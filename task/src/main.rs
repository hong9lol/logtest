use log::generate_complex_function;
// define_variable_with_section!("hello");
// fn main() {
//     // 여러 문자열을 합쳐서 바이트 배열로 변환하고 특정 섹션에 배치
//     // concat_to_static_bytes!("Hello, ", "Rust ", "world!");
//     // hello();
//     // // 바이트 배열 출력
//     // // println!("{:?}", my_string);
//     // // println!("{:?}!!!!!!!!!!", a);

//     // #[link_section = "__DATA,my_custom2"]
//     // static EXG: [u8; 5] = *b"TEST\0";
//     // println!("{:?}", EXG);

//     // let elements = "test String";
//     // let mut root = TokenStream::new();

//     // #[link_section = "__DATA,my_custom5"]
//     // static MY_DATA: [u8; 14] = *b"Hello, world!\0";
//     // println!("{:?}", MY_DATA);

//     // let n = elements.len();
//     // let link_section_name = "__DATA,my_custom10";
//     // let link_section_attr = quote! {
//     //     #[link_section = #link_section_name]
//     // };
//     // root.extend(quote! {

//     // #[link_section = "__DATA,my_custom4"]
//     // static MY_DATA: [u8; 14] = *b"Hello, world!\0";
//     // println!("{:?}", MY_DATA);

//     //     #link_section_attr
//     //     #[no_mangle]
//     //     #[used]
//     //     pub static __INTERRUPTS: [Vector; #n] = [
//     //         #elements
//     //     ];
//     // });
//     // #[unsafe(link_section = ".example_section")]
//     // pub static VAR1: u32 = 1;
// // }
// generate_complex_function!(x);

fn main() {
    generate_complex_function!("TEST");
    // log_info(42);
}

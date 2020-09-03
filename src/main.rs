#![feature(decl_macro)]

#[macro_use] extern crate rocket;
 
#[get("/identity/<name>")]

fn identity(name: String) -> String {
    match name.as_ref() {
        "Haskell" => format!("{first_row}", first_row = "id:: a -> a\nid x = x"),
        "C++" => format!("template <typename T>\n\n\
                    struct identity\n{{\n    T operator()(T x) const {{ return x; }}\n}}"),

        "JavaScript" => format!("function identity(value) {{\n    return value;\n}}"),
        "Agda" => format!("idd : (A : Set) → A → A\nidd A a = a"),
        "Idris" => format!("myId : a -> a\nmyId = \\x => x"),
        "Python" => format!("def identity(x):\n    return x"),
        "C" => format!("int id(x)\n{{\n    return x;\n}}"),
        "Scala" => format!("def id[T](x: T) = x"),
        "Rust" => format!("pub fn id<T>(x: T) -> T {{\n    x\n}}"),
        "Java" => format!("public static <T> T identity(T x) {{\n    return x\n}}"),
        "OCaml" => format!("let id x = x;;"),
        _ => format!("{first_row}", first_row = "Identity is not defined in this case!!")
    }

}

fn main() {
    rocket::ignite().mount("/", routes![identity]).launch();
}


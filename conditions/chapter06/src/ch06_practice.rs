// ch06_practice.rs
// Run with: `rustc --test ch06_practice.rs && ./ch06_practice`
// or put in a Cargo project and `cargo test`.

use std::f64::consts::PI;

// ── EXERCISE 1: enum with data + methods ──────────────────────────────────────
// Pattern-match struct-like and tuple-like variants in a method.
#[derive(Debug, PartialEq)]
enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
    UnitSquare,
}

impl Shape {
    fn area(&self) -> f64 {
        // TODO:
        // - Circle: π r^2
        // - Rect:   w*h
        // - UnitSquare: 1.0
        match self {
            Shape::Circle { r: radius} => {
                return radius.powi(2) * PI;
            }
            Shape::Rect { w: width, h:height } => {
                return width * height;
            }
            Shape::UnitSquare => return 1.0,
        }
        unimplemented!()
    }
}

#[test]
fn ex1_area() {
    let c = Shape::Circle { r: 2.0 };
    let r = Shape::Rect { w: 3.0, h: 4.0 };
    let u = Shape::UnitSquare;
    assert!((c.area() - (PI * 4.0)).abs() < 1e-9);
    assert!((r.area() - 12.0).abs() < 1e-9);
    assert!((u.area() - 1.0).abs() < 1e-9);
}

// ── EXERCISE 2: Option basics ─────────────────────────────────────────────────
// Return double of the first element if it exists; otherwise None.
fn double_first(nums: &[i32]) -> Option<i32> {
    // TODO: use match on nums.first(), or `.first().copied().map(|x| x*2)`
    match nums.first() {
        Some(x) => { 
            return Some(2 * x);
        }
        None => {
            return None;
        }
    }
}

#[test]
fn ex2_option() {
    assert_eq!(double_first(&[]), None);
    assert_eq!(double_first(&[10, 20]), Some(20));
}

// ── EXERCISE 3: if let / else if let + guards ─────────────────────────────────
// Parse preferred port from two strings. Choose the first that parses to a u16
// AND is >= 1025. If neither qualifies, return 8080.
fn pick_port(a: &str, b: &str) -> u16 {
    // TODO:
    // - Try `a.parse::<u16>()`
    // - `else if let Ok(n) = b.parse()` ...
    // - Use a guard: `if n >= 1025`
    let parta = a.parse::<u16>();
    let partb = b.parse::<u16>();

    if let Ok(n) = parta {
        if n >= 1025 {
            return n;
        }
    }

    if let Ok(m) = partb {
        if m >= 1025 {
            return m;
        }
    }
    
    8080
}

#[test]
fn ex3_if_let_chain() {
    assert_eq!(pick_port("99999", "3000"), 3000);
    assert_eq!(pick_port("4444", "3000"), 4444);
    assert_eq!(pick_port("1000", "1024"), 8080);
    assert_eq!(pick_port("not", "numbers"), 8080);
}

// ── EXERCISE 4: match ranges, or-patterns, guards ─────────────────────────────
#[derive(Debug, PartialEq)]
enum Kind { Digit, Alpha, Space, Other }

fn classify(c: char) -> Kind {
    // TODO:
    // - Digits: '0'..='9'
    // - Alpha:  'a'..='z' | 'A'..='Z'  (or-pattern)
    // - Space:  ' ' | '\t' | '\n' | '\r'
    // - Else:   Other
    match c {
        '0'..='9' => {
            return Kind::Digit;
        }
        'a'..='z' | 'A'..='Z' => {
            return Kind::Alpha;
        }
        ' ' | '\t' | '\n' | '\r' => {
            return Kind::Space;
        }
        _ => Kind::Other,
    }
}

#[test]
fn ex4_classify() {
    for d in "0123456789".chars() { assert_eq!(classify(d), Kind::Digit); }
    for a in "aZ".chars() { assert_eq!(classify(a), Kind::Alpha); }
    for s in " \n\t".chars() { assert_eq!(classify(s), Kind::Space); }
    assert_eq!(classify('%'), Kind::Other);
}

// ── EXERCISE 5: while let ─────────────────────────────────────────────────────
// Pop from the end until you find a value > threshold; return it, else None.
fn drain_until_gt(mut v: Vec<i32>, threshold: i32) -> Option<i32> {
    // TODO: while let Some(x) = v.pop() { ... }
    while let Some(x) = v.pop() {
        if x > threshold {
            return Some(x)
        }
    }
    None
}

#[test]
fn ex5_while_let() {
    assert_eq!(drain_until_gt(vec![1, 2, 3], 5), None);
    assert_eq!(drain_until_gt(vec![1, 9, 3], 5), Some(9));
}

// ── EXERCISE 6: enums with methods taking &self vs self ───────────────────────
#[derive(Debug, PartialEq)]
enum Light { Off, On, Dim(u8) }

impl Light {
    fn brightness(&self) -> u8 {
        // TODO:
        // - Off -> 0
        // - On  -> 255
        // - Dim(n) -> n
        match *self {
            Light::Off => 0,
            Light::On => 255,
            Light::Dim(n) => n,
        }
    }

    fn toggle(self) -> Light {
        // TODO (choose a rule, for test below use):
        // - Off -> On
        // - On  -> Off
        // - Dim(_) -> On
        match self {
            Light::Off => {
                return Light::On;
            }
            Light::On => {
                return Light::Off;
            }
            Light::Dim(_) => {
                return Light::On;
            }
        }
    }
}

#[test]
fn ex6_light() {
    assert_eq!(Light::Off.brightness(), 0);
    assert_eq!(Light::On.brightness(), 255);
    assert_eq!(Light::Dim(42).brightness(), 42);

    assert_eq!(Light::Off.toggle(), Light::On);
    assert_eq!(Light::On.toggle(), Light::Off);
    assert_eq!(Light::Dim(10).toggle(), Light::On);
}

// ── EXERCISE 7: matches! macro ────────────────────────────────────────────────
// Count how many chars in s are whitespace using matches!(...)
fn count_ws(s: &str) -> usize {
    // TODO: s.chars().filter(|c| matches!(c, ' ' | '\t' | '\n' | '\r')).count()
    unimplemented!()
}

#[test]
fn ex7_matches_macro() {
    assert_eq!(count_ws("a b\tc\nd\r"), 4);
}

// ── EXERCISE 8: nested destructuring in match ─────────────────────────────────
#[derive(Debug, PartialEq)]
enum Msg {
    Move(Point),
    Resize { w: u32, h: u32 },
    Paint(u8, u8, u8),
    Echo(String),
}
#[derive(Debug, PartialEq)]
struct Point { x: i32, y: i32 }

fn describe(msg: Msg) -> String {
    // TODO:
    // - `Move(Point { x, y })` -> format!("move({x},{y})")
    // - `Resize { w, h }`      -> format!("resize({w}x{h})")
    // - `Paint(r,g,b)`         -> format!("rgb({r},{g},{b})")
    // - `Echo(s)`              -> s
    unimplemented!()
}

#[test]
fn ex8_nested_destructure() {
    assert_eq!(describe(Msg::Move(Point{ x: 3, y: 4 })), "move(3,4)");
    assert_eq!(describe(Msg::Resize { w: 800, h: 600 }), "resize(800x600)");
    assert_eq!(describe(Msg::Paint(255, 0, 128)), "rgb(255,0,128)");
    assert_eq!(describe(Msg::Echo("hi".into())), "hi");
}

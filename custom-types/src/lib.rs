#![allow(dead_code)]

/**
 * Structures -> struct 키워드를 통해 생성할 수 있고 3가지 타입이 있다
 *
 * - tuple (Tuple structs)
 * - 클래식 C structs
 * - Unit structs (field-less, 제너릭에 유용)
 *
 */

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit structure
struct Unit;

// A tuple structure
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

// structs can be reused as fields of another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// ------ Enums ------
// allows the creation of a type which maybe one of a few different varaints.
// struct 에서 유효한 variants 는 enum 에서도 유효하다

// web event 를 분류한 enum 을 생성
// enum 의 각 variant 들은 서로 다르고 독립적이다.

enum WebEvent {
    // unit-like
    PageLoad,
    PageUnload,
    // like tuple structs
    KeyPress(char),
    Paste(String),
    // c-like structs
    Click { x: i64, y: i64 },
}

// WebEvent enum 타입을 인자로 받고 아무것도 반환하지 않는 함수
fn inspect(web_event: WebEvent) {
    match web_event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // destructure 'c' from inside the enum variants
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure 'Click' into 'x' and 'y'
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    // Instantiate a Point
    let point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("x: {}, y: {}", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields
    // of our other one.
    let bottom_right = Point { x: 5.2, ..point };

    // bottom_right.y == point.y because we used that field from point
    println!("second point: {} {}", bottom_right.y, point.y);

    // Destructure the point using a let binding
    let Point {
        x: left_edge,
        y: right_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: right_edge,
        },
        bottom_right: bottom_right,
    };

    // instantiate a unit struct
    let _unit = Unit;

    // instantiate a tuple struct
    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("{}", pair.0);

    // Struct Activity
    // 1. Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring)

    fn rect_area(args: &Rectangle) -> f32 {
        let Rectangle {
            top_left,
            bottom_right,
        } = args;
        let Point { x: tlx, y: tly } = top_left;
        let Point { x: brx, y: bry } = bottom_right;

        return tly - bry * tlx - brx;
    }

    println!("{}", rect_area(&_rectangle).abs());

    /**
     *
     * 2. Add a function square which takes a Point and a f32 as arguments,
     * and returns a Rectangle with its top left corner on the point,
     * and a width and height corresponding to the f32.
     */

    struct Square {
        top_left: Point,
        width: f32,
        height: f32,
    }

    fn square(point: &Point, f: f32) -> Square {
        let Point { x, y } = point;

        let top_left = Point { x: *x, y: *y };

        return Square {
            top_left: top_left,
            width: f,
            height: f,
        };
    }

    // ------ enum ------
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 20 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // -- use -> can be used so manual scoping isn't needed

    // Explicitly 'use' each name so they are available without manual scoping.
    use crate::Status::{Poor, Rich};
    // automatically 'use' each name inside 'Work'
    use crate::Work::*;

    // Equivalent to 'Status::Poor'
    let status = Poor;
    // Equivalent to 'Work::Civilian'
    let work = Civilian;

    match status {
        // note the lack of scoping because of the explicit 'use' above
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money"),
    }

    match work {
        // Note again the lack of scoping
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers Fight!"),
    }
}

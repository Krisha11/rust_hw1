use structs_and_methods::{
    Point, Rect, Figure, Circle
};

// 7. В функции main приведите несколько примеров использование реализованных структур, выводите сообщения на экран пользователю с помощью println!
fn main() {
    let o = Point { x: 0, y: 0 };
    let p1 = Point { x: -6, y: 6 };
    let p2 = Point { x: 6, y: -6 };
    let special_point = Point { x: 3, y: 3 };
    
    let circle = Circle { center: o, radius: 1 };
    let rect = Rect { ul_point: p1, dr_point: p2 };
    let figure : Figure<i32> = Figure::Rect (rect.clone());

    println!("The area of my rectangle is {}", rect.area());

    println!("Statement : 'My circle contains my special point.' is {}", circle.contains(&special_point));
    println!("Statement : 'My figure contains my special point.' is {}", figure.contains(&special_point));
}

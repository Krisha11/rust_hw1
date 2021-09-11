// 1. Структуру Point, состоящую из двух дробных чисел, представляющую точку на плоскости 
#[derive(Clone)]
struct Point
{
    x: f64,
    y: f64
}

// 2. Структуру Rect, состоящую из 4 чисел: декартовых координат левого верхнего угла прямоугольника 
// и длины его сторон (можно использовать другое представление, если удобно) 
#[derive(Clone)]
struct Rect
{
    ul_point: Point,
    dr_point : Point
}

// 3. Структуру Circle, состоящую из декартовых координат середины окружности и её радиуса 
#[derive(Clone)]
struct Circle
{
    center: Point,
    radius: f64
}

// 4. Перечисление Figure, состоящее из вариантов Circle и Rect 
enum Figure   // структура - человек
{
    Circle(Circle),
    Rect(Rect)
}

// 5. Реализуйте для структур Rect и Circle, а также для перечисления Figure метод с сигнатурой contains(&self, p: &Point) -> bool, который определяет, принадлежит ли заданная точка фигуре. 
impl Rect {
    fn contains(&self, p: &Point) -> bool {
        self.ul_point.x <= p.x && p.x <= self.dr_point.x &&
        self.dr_point.y <= p.y && p.y <= self.ul_point.y
    }
}

impl Circle {
    fn contains(&self, p: &Point) -> bool {
        (p.x - self.center.x) * (p.x - self.center.x) 
        + (p.y - self.center.y) * (p.y - self.center.y) 
        <= self.radius * self.radius
    }
}

impl Figure {
    fn contains(&self, p: &Point) -> bool {
        match self {
            Figure::Circle(c) => c.contains(p),
            Figure::Rect(r) => r.contains(p)
        }
    }
}



// 6. Реализуйте для структур Rect и Circle метод area(&self) -> f64, который возвращает площадь фигуры. 
impl Rect {
    fn area(&self) -> f64 {
        let width = self.dr_point.x - self.ul_point.x;
        let height = self.ul_point.y - self.dr_point.y;
        width * height
    }
}

impl Circle {
    fn area(&self) -> f64 {
        let pi = 3.1415;
        pi * self.radius * self.radius
    }
}

// 7. В функции main приведите несколько примеров использование реализованных структур, выводите сообщения на экран пользователю с помощью println!
fn main() {
    let o = Point { x: 0.0, y: 0.0 };
    let p1 = Point { x: -6.0, y: 6.0 };
    let p2 = Point { x: 6.0, y: -6.0 };
    let special_point = Point { x: 3.0, y: 3.0 };
    
    let circle = Circle { center: o, radius: 1.0 };
    let rect = Rect { ul_point: p1, dr_point: p2 };
    let figure : Figure = Figure::Rect (rect.clone());

    println!("The area of my circle is {}", circle.area());
    println!("The area of my rectangle is {}", rect.area());

    println!("Statement : 'My figure contains my special point.' is {}", figure.contains(&special_point));

}

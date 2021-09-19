
use num_traits::{Num};

// 1. Структуру Point, состоящую из двух дробных чисел, представляющую точку на плоскости 
#[derive(Debug, Clone, Hash, Eq)]
pub struct Point<T: Num + std::cmp::PartialOrd + Copy>
{
    pub x: T,
    pub y: T
}

impl<T: Num + std::cmp::PartialOrd + Copy> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Num + std::cmp::PartialOrd + Copy> Default for Point<T> {
    fn default() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }
}

// 2. Структуру Rect, состоящую из 4 чисел: декартовых координат левого верхнего угла прямоугольника 
// и длины его сторон (можно использовать другое представление, если удобно) 
#[derive(Debug, Clone, Hash, Eq)]
pub struct Rect<T: Num + std::cmp::PartialOrd + Copy>
{
    pub ul_point: Point<T>,
    pub dr_point : Point<T>
}

impl<T: Num + std::cmp::PartialOrd + Copy> PartialEq for Rect<T> {
    fn eq(&self, other: &Rect<T>) -> bool {
        self.ul_point == other.ul_point && self.dr_point == other.dr_point
    }
}

impl<T: Num + std::cmp::PartialOrd + Copy> Default for Rect<T> {
    fn default() -> Self {
        Self { ul_point:Point {x: T::one(), y: T::zero() }, dr_point:Point {x: T::zero(), y: T::one() } }
    }
}


// 3. Структуру Circle, состоящую из декартовых координат середины окружности и её радиуса 
#[derive(Debug, Clone, Hash, Eq)]
pub struct Circle<T: Num + std::cmp::PartialOrd + Copy>
{
    pub center: Point<T>,
    pub radius: T
}

impl<T: Num + std::cmp::PartialOrd + Copy> PartialEq for Circle<T> {
    fn eq(&self, other: &Circle<T>) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

impl<T: Num + std::cmp::PartialOrd + Copy> Default for Circle<T> {
    fn default() -> Self {
        Self { center:Point {x: T::zero(), y: T::zero() }, radius:T::one() }
    }
}


// 4. Перечисление Figure, состоящее из вариантов Circle и Rect 
#[derive(Debug, Clone, Hash, Eq)]
pub enum Figure<T: Num + std::cmp::PartialOrd + Copy>
{
    Circle(Circle<T>),
    Rect(Rect<T>)
}

impl<T: Num + std::cmp::PartialOrd + Copy> PartialEq for Figure<T> {
    fn eq(&self, other: &Figure<T>) -> bool {
        match self {
            Figure::Circle(c1) => {
                match other {
                    Figure::Circle(c2) => c1 == c2,
                    Figure::Rect(_) => false
                }
            },
            Figure::Rect(r1) => {
                match other {
                    Figure::Circle(_) => false,
                    Figure::Rect(r2) => r1 == r2
                }
            }
        }
    }


}

impl<T: Num + std::cmp::PartialOrd + Copy> Default for Figure<T> {
    fn default() -> Self {
        Figure::Circle(Circle::default())
    }
}

// 5. Реализуйте для структур Rect и Circle, а также для перечисления Figure метод с сигнатурой contains(&self, p: &Point) -> bool, который определяет, принадлежит ли заданная точка фигуре. 
impl<T: Num + std::cmp::PartialOrd + Copy> Rect<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        self.ul_point.x <= p.x && p.x <= self.dr_point.x &&
        self.dr_point.y <= p.y && p.y <= self.ul_point.y
    }
}

impl<T: Num + std::cmp::PartialOrd + Copy> Circle<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        (p.x - self.center.x) * (p.x - self.center.x) 
        + (p.y - self.center.y) * (p.y - self.center.y) 
        <= self.radius * self.radius
    }
}

impl<T: Num + std::cmp::PartialOrd + Copy> Figure<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        match self {
            Figure::Circle(c) => c.contains(p),
            Figure::Rect(r) => r.contains(p)
        }
    }
}



// 6. Реализуйте для структур Rect и Circle метод area(&self) -> i64, который возвращает площадь фигуры. 
impl<T: Num + std::cmp::PartialOrd + Copy> Rect<T> {
    pub fn area(&self) -> T {
        let width = self.dr_point.x - self.ul_point.x;
        let height = self.ul_point.y - self.dr_point.y;
        width * height
    }
}

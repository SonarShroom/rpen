use crate::mementos;

struct Vector2
{
    pub x: i32,
    pub y: i32
}

impl std::fmt::Display for Vector2
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

struct Shape
{
    origin: Vector2,
    size: Vector2 
}

impl std::fmt::Display for Shape
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(formatter, "Origin: {}, Size: {}", self.origin, self.size)
    }
}

struct CreateShape
{
    target_shape: Shape
}

impl std::fmt::Display for CreateShape
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(formatter, "Create Shape: {}", self.target_shape)
    }
}

impl CreateShape
{
    pub fn new(s: Shape) -> Self
    {
        CreateShape {target_shape: s}
    }
}

impl mementos::Command for CreateShape
{
    fn execute(&self)
    {
        println!("Executed {}", self);
    }
    
    fn undo(&self)
    {
        println!("Undone {}", self);
    }
}
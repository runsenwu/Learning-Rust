#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn size(&self) -> usize {
        self.width * self.height
    }

    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    fn new_rec(self, add_width: usize, add_height: usize) -> Self {
        Self {
            width: self.width + add_width,
            height: self.height + add_height,
        }
    }

    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 10,
        height: 20,
    };

    let mut rec2 = Rectangle {
        ..rec
    };

    println!("{:#?}", rec2);

    rec2.resize(30, 31);

    println!f("{:#?}", rec2);
    println!("{:#?}", rec);

    rec2 = rec;

    println!("{:#?}", rec2);
    println!("{:#?}", rec);


    let rec3 = rec2.new_rec(10, 22);
    println!("{:#?}", rec3);

    let rec4 = Rectangle::new(10, 22);
    println!("{:#?}", rec4);
}
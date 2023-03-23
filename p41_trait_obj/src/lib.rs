use std::str;

pub trait Draw {
    fn draw(&self);
}

// 注意 这里的dyn 表示的是只要实现了Draw的任何类型
// 为什么不用范型T呢？因为如果使用范型T，那么components集合中所有元素都是某一种类型了，因为Screen定义的时候或者push进第一个元素的时候其范型已经确定
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Botton {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Botton {
    fn draw(&self) {
        // 具体的绘制方法
    }
}

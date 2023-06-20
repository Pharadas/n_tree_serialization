use macroquad::prelude::*;
use bitvec::prelude::*;
use ::rand::prelude::*;

const MAX_DEPTH: i32 = 6;

#[derive(Debug)]
pub struct QuadTree {
    pub ul: Vec2,
    pub lr: Vec2,
    depth: i32,
    children: [Option<Box<QuadTree>>; 4],
    filled: bool,
    color: Color
}

impl QuadTree {
    pub fn new(
        ul: Vec2,
        lr: Vec2,
        depth: i32,
        children: [Option<Box<QuadTree>>; 4],
        filled: bool,
        color: Color
    ) -> QuadTree {
        QuadTree {
            ul, lr, depth, children, filled, color
        }
    }

    fn recursve_linear_tree(&mut self, depth: i32, linear_tree: &BitVec<u8>, i: &mut usize) {
        println!("working on depth: {}, linear tree index: {}", depth, i);
        if depth == MAX_DEPTH {
            self.filled = linear_tree[*i];
            *i += 1;

        } else {
            // 1 -> checar que mas
            if linear_tree[*i] {
                *i += 1;
                // completamente lleno (11)
                if linear_tree[*i] {
                    *i += 1;
                    self.filled = true;
                    self.color = RED;

                // parcialmente lleno (10)
                } else {
                    *i += 1;
                    let mut new_children = self.create_children_positions(WHITE);

                    for child_num in (0..4).rev() {
                        let mut mutable_child = new_children.pop().unwrap();
                        mutable_child.recursve_linear_tree(depth + 1, linear_tree, i);
                        self.children[child_num] = Some(Box::new(mutable_child));
                    }
                }

            // completamente vacio (0)
            } else {
                *i += 1;
                self.filled = false;
            }
        }
    }

    pub fn new_from(
        ul: Vec2,
        lr: Vec2,
        linear_tree: BitVec<u8>
    ) -> QuadTree {
        let mut root = QuadTree { ul, lr, depth: 0, children: [None, None, None, None], filled: false, color: BLUE };

        let mut i: usize = 0;
        root.recursve_linear_tree(0, &linear_tree, &mut i);

        root.draw();
        return root;
    }

    // remove all redundant sub-nodes
    pub fn clean(&mut self) -> u8 {
        let mut rng = thread_rng();
        let mut children: u8 = 0;

        if self.filled {
            self.children = [None, None, None, None];
            return 1
        }

        // Save which sub-quads are filled
        for i in 0..4 {
            if !self.children[i].is_none() { // TODO: Make it compare color as well
                children += self.children[i].as_mut().unwrap().clean();
                // children += self.children[i].unwrap().clean();
            }
        }

        // If all sub-quads are filled, remove them and set this quad as filled
        if children == 4 {
            println!("sub-quad removed at depth {}", self.depth);
            self.children = [None, None, None, None];
            self.filled = true;
            self.color = Color::new(rng.gen(), rng.gen(), rng.gen(), 1.);
            return 1;

        } else {
            // If not all sub-quads are filled, leave them as they are
            return 0;
        }
    }

    // 0 -> Empty all the way down
    // 11 -> Filled all the way down
    // 10 -> Partially filled
    // 1 -> This is the last node and it's filled
    pub fn serialize(root: &QuadTree) -> BitVec<u8> {
        // you need to make sure to have cleaned the root node before passing it
        let mut queue: Vec<Option<&QuadTree>> = vec![Some(root)]; // We dont want this reference to be mutable
        let mut vec = BitVec::new();

        while queue.len() != 0 {
            let curr_node = queue.pop().unwrap();

            if curr_node.unwrap().filled {
                vec.push(true); // 1 if last quad
                if curr_node.unwrap().depth != MAX_DEPTH { // 11 if not last quad
                    vec.push(true);
                }

            } else {
                // since we cleaned it and it's not filled, we know that it must be partially
                // filled, otherwise it would've been removed
                let mut has_at_least_one_not_none_child = false;

                for i in 0..4 {
                    let child = &curr_node.unwrap().children[i];

                    if !child.is_none() {
                        queue.push(Some(child.as_deref().unwrap()));
                        has_at_least_one_not_none_child = true;
                    }
                }

                if has_at_least_one_not_none_child {
                    vec.push(true);
                    vec.push(false);
                } else {
                    vec.push(false);
                }
            }

        }

        vec
    }

    pub fn draw(&self) {
        if self.filled {
            draw_rectangle(self.ul.x, self.ul.y, self.lr.x - self.ul.x, self.lr.y - self.ul.y, self.color);
        } else {
            draw_rectangle_lines(self.ul.x, self.ul.y, self.lr.x - self.ul.x, self.lr.y - self.ul.y, 1., RED);

            for i in 0..4 {
                if !self.children[i].is_none() {
                    self.children[i].as_ref().unwrap().draw();
                }
            }
        }
    }

    pub fn divide_at_point(&mut self, point: Vec2, _color: Color) {
        let mut rng = thread_rng();
        let rcolor = Color::new(rng.gen(), rng.gen(), rng.gen(), 1.);

        // checar si puedo seguir dividiendo, si no terminar a esta
        // profundidad y marcar el nodo como completamente lleno
        if (self.depth < MAX_DEPTH) || (self.filled) {
            let mid = vec2(self.ul.x + ((self.lr.x - self.ul.x) / 2.), self.ul.y + ((self.lr.y - self.ul.y) / 2.));

            if point.x <= mid.x { // izquierda
                if point.y <= mid.y { // arriba

                    self.children[0].as_mut().unwrap().create_children(rcolor);
                    self.children[0].as_mut().unwrap().divide_at_point(point, rcolor);
                } else { // abajo

                    self.children[2].as_mut().unwrap().create_children(rcolor);
                    self.children[2].as_mut().unwrap().divide_at_point(point, rcolor);
                }
            } else { // derecha
                if point.y <= mid.y { // arriba

                    self.children[1].as_mut().unwrap().create_children(rcolor);
                    self.children[1].as_mut().unwrap().divide_at_point(point, rcolor);
                } else { // abajo

                    self.children[3].as_mut().unwrap().create_children(rcolor);
                    self.children[3].as_mut().unwrap().divide_at_point(point, rcolor);
                }
            }

            // println!("{}", filled_vec);
        } else {
            self.filled = true;
        }
    }

    fn create_children_positions(&mut self, color: Color) 
    -> Vec<QuadTree> {
        let ul_child = QuadTree {
            children: [None, None, None, None],
            ul: self.ul,
            lr: vec2(
                self.ul.x + ((self.lr.x - self.ul.x) / 2.),
                self.ul.y + ((self.lr.y - self.ul.y) / 2.),
            ),
            depth: self.depth + 1,
            filled: false,
            color
        };

        let ur_child = QuadTree {
            children: [None, None, None, None],
            ul: vec2(
                self.ul.x + ((self.lr.x - self.ul.x) / 2.),
                self.ul.y
            ),
            lr: vec2(
                self.lr.x,
                self.ul.y + ((self.lr.y - self.ul.y) / 2.)
            ),
            depth: self.depth + 1,
            filled: false,
            color
        };

        let ll_child = QuadTree {
            children: [None, None, None, None],
            ul: vec2(
                self.ul.x,
                self.ul.y + ((self.lr.y - self.ul.y) / 2.)
            ),
            lr: vec2(
                self.ul.x + ((self.lr.x - self.ul.x) / 2.),
                self.lr.y
            ),
            depth: self.depth + 1,
            filled: false,
            color
        };

        let lr_child = QuadTree {
            children: [None, None, None, None],
            ul: vec2(
                self.ul.x + ((self.lr.x - self.ul.x) / 2.),
                self.ul.y + ((self.lr.y - self.ul.y) / 2.)
            ),
            lr: self.lr,
            depth: self.depth + 1,
            filled: false,
            color
        };

        return vec![
            ul_child,
            ur_child,
            ll_child,
            lr_child
        ];
    }

    pub fn create_children(&mut self, color: Color) {
        let mut new_children = self.create_children_positions(color);

        for i in (0..4).rev() {
            if self.children[i].is_none() {
                self.children[i] = Some(Box::new(new_children.pop().unwrap()));
            }
        }
    }
}

use macroquad::prelude::*;
use bitvec::prelude::*;
use std::io::{Write, Read}; // bring trait into scope
use std::fs;
use std::mem::transmute;
use std::convert::TryInto;

mod n_tree;
use n_tree::QuadTree;

#[macroquad::main("BasicShapes")]
async fn main() {
    let x_s = (screen_width() / 2.) - (screen_height() / 2.);

    let mut f = fs::File::open("gaming").expect("no file found");
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer);

    let mut linearized_tree: BitVec<u8, Lsb0> = BitVec::from_vec(buffer);
    println!("{}", linearized_tree.len());
    println!("{:?}", linearized_tree);

    let mut root = QuadTree::new_from(vec2(0., 0.), vec2(screen_width(), screen_height()), linearized_tree);

    loop {
        clear_background(BLACK);
        // root.divide_at_point(vec2(rng.gen::<f32>() * screen_height() + root.ul.x, rng.gen::<f32>() * screen_height()), Color {r: rng.gen(), g: rng.gen(), b: rng.gen(), a: 1.});

        if is_key_pressed(KeyCode::K) {
            root = QuadTree::new(
                vec2(x_s, 0.),
                vec2(x_s + screen_height(), screen_height()),
                0,
                [None, None, None, None],
                false,
                BLUE
            );

            root.create_children(BLUE);
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if is_mouse_button_down(MouseButton::Left) {
            root.divide_at_point(vec2(mouse_position().0, mouse_position().1), WHITE);
            root.clean();

            println!();
        }

        root.draw();

        next_frame().await
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .open("./gaming").unwrap();

    // let x: &[u8] = unsafe { transmute(QuadTree::linearize(&mut root).as_raw_slice()) };
    root.clean(); // gotta clean it before linearizing
    let g = QuadTree::serialize(&mut root);
    println!("tree compressed to {} bytes", g.len() / 8);

    file.write_all(g.as_raw_slice()).unwrap();
}

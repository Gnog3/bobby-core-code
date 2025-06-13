use blib::{
    ddi::{self, Pos},
    println_err,
};
use bytemuck::{try_cast_slice, Pod, Zeroable};

#[derive(Debug, Pod, Clone, Copy, Zeroable)]
#[repr(C)]
struct Operation {
    matrix_1: u32,
    matrix_2: u32,
    pos_x: u16,
    pos_y: u16,
    color: u32,
}

pub fn draw_image_at(pos: Pos, image: &[u8]) {
    let opers: &[Operation] = match try_cast_slice(image) {
        Ok(opers) => opers,
        Err(err) => {
            println_err!("Error drawing image: {:?}", err);
            return;
        }
    };
    for oper in opers {
        ddi::set_matrix_1(oper.matrix_1);
        ddi::set_matrix_2(oper.matrix_2);
        ddi::set_target(Pos::new(oper.pos_x + pos.x, oper.pos_y + pos.y));
        ddi::set_color(oper.color);
        ddi::run_matrix();
    }
}

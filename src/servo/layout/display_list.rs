import gfx::geometry::*;
import geom::rect::Rect;
import image::base::image;
import servo_text::text_run::text_run;

enum item_type {
    display_item_solid_color(u8, u8, u8),
    display_item_image(~image),
    display_item_text(text_run),
    // FIXME: Shape code does not understand the alignment without this
    padding(u8, u8, u8, u8)
}

enum display_item = {
    item_type: item_type,
    bounds: Rect<au>
};

type display_list = [display_item];

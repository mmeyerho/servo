#[doc="Creates CSS boxes from a DOM."]

import dom::base::{ElementData, HTMLDivElement, HTMLImageElement, Element, Text, Node};
import dom::style::{display_type, di_block, di_inline, di_none};
import dom::rcu::ReaderMethods;
import gfx::geometry;
import layout::base::{BlockBox, Box, BoxKind, BoxTreeReadMethods, BoxTreeWriteMethods, InlineBox};
import layout::base::{IntrinsicBox, NodeMethods, NodeTreeReadMethods, TextBox};
import layout::base::{appearance, btree, ntree};
import layout::style::style::{style_methods};
import layout::text::text_box;
import util::tree;
import option::is_none;

export box_builder_methods;

enum ctxt = {
    // The parent node that we're scanning.
    parent_node: Node,
    // The parent box that these boxes will be added to.
    parent_box: @Box,

    //
    // The current anonymous box that we're currently appending inline nodes to.
    //
    // See CSS2 9.2.1.1.
    //

    mut anon_box: option<@Box>
};

fn create_context(parent_node: Node, parent_box: @Box) -> ctxt {
    ret ctxt({
        parent_node: parent_node,
        parent_box: parent_box,
        mut anon_box: none
    });
}

impl methods for ctxt {
    #[doc="
        Constructs boxes for the parent's children, when the parent's 'display'
        attribute is 'block'.
    "]
    fn construct_boxes_for_block_children() {
        for ntree.each_child(self.parent_node) {
            |kid|

            // Create boxes for the child. Get its primary box.
            let kid_box = kid.construct_boxes();

            // Determine the child's display.
            let disp = kid.get_computed_style().display;
            if disp != di_inline {
                self.finish_anonymous_box_if_necessary();
            }

            // Add the child's box to the current enclosing box or the current anonymous box.
            alt kid.get_computed_style().display {
                di_block { 
                  btree.add_child(self.parent_box, kid_box);
                }
                di_inline {
                    let anon_box = alt self.anon_box {
                        none {
                          //
                          // The anonymous box inherits the attributes of its parents for now, so
                          // that properties of intrinsic boxes are not spread to their parenting
                          // anonymous box.
                          //
                          // TODO: check what CSS actually specifies
                          //

                          let b = @Box(self.parent_node, InlineBox);
                          self.anon_box = some(b);
                          b
                        }
                        some(b) { b }
                    };
                    btree.add_child(anon_box, kid_box);
                }
                di_none {
                    // Nothing to do.
                }
            }
        }
    }

    #[doc="
        Constructs boxes for the parent's children, when the parent's 'display'
        attribute is 'inline'.
    "]
    fn construct_boxes_for_inline_children() {
        for ntree.each_child(self.parent_node) {
            |kid|

            // Construct boxes for the child. Get its primary box.
            let kid_box = kid.construct_boxes();

            // Determine the child's display.
            let disp = kid.get_computed_style().display;
            if disp != di_inline {
                // TODO
            }

            // Add the child's box to the current enclosing box.
            alt kid.get_computed_style().display {
                di_block {
                    // TODO
                    #warn("TODO: non-inline display found inside inline box");
                    btree.add_child(self.parent_box, kid_box);
                }
                di_inline {
                    btree.add_child(self.parent_box, kid_box);
                }
                di_none {
                    // Nothing to do.
                }
            }
        }
    }

    #[doc="Constructs boxes for the parent's children."]
    fn construct_boxes_for_children() {
        #debug("parent node:");
        self.parent_node.dump();

        alt self.parent_node.get_computed_style().display {
            di_block  { self.construct_boxes_for_block_children();  }
            di_inline { self.construct_boxes_for_inline_children(); }
            di_none   { /* Nothing to do. */                        }
        }

        self.finish_anonymous_box_if_necessary();
        assert is_none(self.anon_box);
    }

    #[doc="
        Flushes the anonymous box we're creating if it exists. This appends the
        anonymous box to the block.
    "]
    fn finish_anonymous_box_if_necessary() {
        alt copy self.anon_box {
            none { /* Nothing to do. */ }
            some(b) { btree.add_child(self.parent_box, b); }
        }
        self.anon_box = none;
    }
}

impl box_builder_priv for Node {
    #[doc="
        Determines the kind of box that this node needs. Also, for images, computes the intrinsic
        size.
    "]
    fn determine_box_kind() -> BoxKind {
        alt self.read({ |n| copy n.kind }) {
            ~Text(string) {
                TextBox(@text_box(string))
            }
            ~Element(element) {
                alt *element.kind {
                    HTMLDivElement           { BlockBox            }
                    HTMLImageElement({size}) { IntrinsicBox(@size) }
                    UnknownElement           { InlineBox           }
                }
            }
        }
    }
}

impl box_builder_methods for Node {
    #[doc="Creates boxes for this node. This is the entry point."]
    fn construct_boxes() -> @Box {
        let box_kind = self.determine_box_kind();
        let my_box = @Box(self, box_kind);
        alt box_kind {
            BlockBox | InlineBox {
                let cx = create_context(self, my_box);
                cx.construct_boxes_for_children();
            }
            _ {
                // Nothing to do.
            }
        }
        ret my_box;
    }
}


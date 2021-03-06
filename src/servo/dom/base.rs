#[doc="The core DOM types. Defines the basic DOM hierarchy as well as all the HTML elements."]

import dom::rcu::WriterMethods;
import gfx::geometry::au;
import geom::size::Size2D;
import layout::base::layout_data;
import util::tree;

import dvec::{dvec, extensions};

enum NodeData = {
    tree: tree::Tree<Node>,
    kind: ~NodeKind,
};

enum NodeKind {
    Element(ElementData),
    Text(str)
}

class ElementData {
    let tag_name: str;
    let kind: ~ElementKind;
    let attrs: dvec<~Attr>;

    new(-tag_name: str, -kind: ~ElementKind) {
        self.tag_name = tag_name;
        self.kind = kind;
        self.attrs = dvec();
    }

    fn get_attr(attr_name: str) -> option<str> {
        let mut i = 0u;
        while i < self.attrs.len() {
            if attr_name == self.attrs[i].name {
                ret some(copy self.attrs[i].value);
            }
            i += 1u;
        }

        none
    }
}

class Attr {
    let name: str;
    let value: str;

    new(-name: str, -value: str) {
        self.name = name;
        self.value = value;
    }
}

enum ElementKind {
    UnknownElement,
    HTMLDivElement,
    HTMLHeadElement,
    HTMLImageElement({mut size: Size2D<au>})
}

#[doc="
    The rd_aux data is a (weak) pointer to the layout data, which contains the CSS info as well as
    the primary box.  Note that there may be multiple boxes per DOM node.
"]

type Node = rcu::Handle<NodeData, layout_data>;

type NodeScope = rcu::Scope<NodeData, layout_data>;

fn NodeScope() -> NodeScope {
    rcu::Scope()
}

impl NodeScope for NodeScope {
    fn new_node(-k: NodeKind) -> Node {
        self.handle(NodeData({tree: tree::empty(), kind: ~k}))
    }
}

impl TreeReadMethods of tree::ReadMethods<Node> for NodeScope {
    fn each_child(node: Node, f: fn(Node) -> bool) {
        tree::each_child(self, node, f)
    }

    fn get_parent(node: Node) -> option<Node> {
        tree::get_parent(self, node)
    }

    fn with_tree_fields<R>(node: Node, f: fn(tree::Tree<Node>) -> R) -> R {
        self.read(node) { |n| f(n.tree) }
    }
}

impl TreeWriteMethods of tree::WriteMethods<Node> for NodeScope {
    fn add_child(node: Node, child: Node) {
        tree::add_child(self, node, child)
    }

    fn with_tree_fields<R>(node: Node, f: fn(tree::Tree<Node>) -> R) -> R {
        self.write(node) { |n| f(n.tree) }
    }
}

